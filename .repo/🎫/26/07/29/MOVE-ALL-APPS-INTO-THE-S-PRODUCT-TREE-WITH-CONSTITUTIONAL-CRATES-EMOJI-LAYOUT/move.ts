#!/usr/bin/env bun
/**
 * 🏛️ Ticket-scoped mover: rewrites path-deps + package aliases across every workspace manifest,
 * then physically `mv`s the crate directories. Temp helper — lives only in the ticket folder,
 * never a permanent repo script (root AGENTS.md: only script.ts files are permanent).
 *
 * Mechanic:
 *   1. Scan ALL workspace member Cargo.toml files (via `cargo metadata`) for `key = { path = "…" }`
 *      dependency lines. For any whose resolved absolute target dir matches a moved oldDir, rewrite
 *      the path relative to the dependent's OWN new location (itself may also be moving) and set/replace
 *      an explicit `package = "<newPkg>"` field so the dependency KEY (and therefore every `use key::…`
 *      in .rs source) stays byte-identical.
 *   2. Rename `[package] name = "…"` for crates that are themselves moving.
 *   3. Rewrite root Cargo.toml `members` list entries for moved dirs.
 *   4. Physically `mv` each oldDir -> newDir (creating parent dirs as needed).
 *
 * Usage: bun move.ts <moves.json>   (dry run by default; pass --apply to actually write/move)
 */
import { execSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const [, , movesPath, ...flags] = process.argv;
const apply = flags.includes("--apply");
const preview = flags.includes("--preview");
if (!movesPath) {
  console.error("usage: bun move.ts <moves.json> [--apply]");
  process.exit(1);
}

type Move = { oldDir: string; newDir: string; newPkg: string };
const moves: Move[] = JSON.parse(readFileSync(movesPath, "utf8"));
const oldToNew = new Map(moves.map((m) => [m.oldDir, m]));

function allMemberManifestDirs(): string[] {
  const raw = execSync("cargo metadata --no-deps --format-version 1", { cwd: repoRoot, maxBuffer: 1024 * 1024 * 64 }).toString();
  const meta = JSON.parse(raw);
  return meta.packages.map((p: { manifest_path: string }) => relative(repoRoot, dirname(p.manifest_path)));
}

const allDirs = allMemberManifestDirs();
console.log(`scanning ${allDirs.length} member manifests for path-deps into ${moves.length} moved dirs`);

const PATH_DEP_RE = /^(\s*)([A-Za-z0-9_.-]+)\s*=\s*\{([^}]*)\}\s*$/;
const PATH_FIELD_RE = /path\s*=\s*"([^"]+)"/;
const PACKAGE_FIELD_RE = /package\s*=\s*"([^"]+)"/;

let editedManifests = 0;
let editedDepLines = 0;

for (const dir of allDirs) {
  const manifestPath = join(repoRoot, dir, "Cargo.toml");
  if (!existsSync(manifestPath)) continue;
  const text = readFileSync(manifestPath, "utf8");
  const lines = text.split("\n");
  let changed = false;

  const selfMove = oldToNew.get(dir);
  const ownNewDir = selfMove ? selfMove.newDir : dir;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const m = line.match(PATH_DEP_RE);
    if (!m) continue;
    const [, indent, depKey, inner] = m;
    const pathMatch = inner.match(PATH_FIELD_RE);
    if (!pathMatch) continue;
    const relPath = pathMatch[1];
    const targetAbsDir = resolve(repoRoot, dir, relPath);
    const targetRelDir = relative(repoRoot, targetAbsDir);
    const move = oldToNew.get(targetRelDir);
    // 🐛 Even when the TARGET isn't moving, if THIS crate is moving, its base dir changed and the
    // relative path must be recomputed — regardless of whether the target itself is also relocating.
    if (!move && !selfMove) continue;
    const newTargetDir = move ? move.newDir : targetRelDir;

    const newRelPath = relative(join(repoRoot, ownNewDir), join(repoRoot, newTargetDir)) || ".";
    let newInner = inner.trim().replace(PATH_FIELD_RE, `path = "${newRelPath}"`);
    if (move) {
      if (PACKAGE_FIELD_RE.test(newInner)) {
        newInner = newInner.replace(PACKAGE_FIELD_RE, `package = "${move.newPkg}"`);
      } else {
        newInner = `${newInner}, package = "${move.newPkg}"`;
      }
    }
    if (preview) console.log(`[${dir}]\n  - ${line.trim()}\n  + ${indent}${depKey} = { ${newInner} }`.trimEnd());
    lines[i] = `${indent}${depKey} = { ${newInner} }`;
    changed = true;
    editedDepLines++;
  }

  // Dotted-table dependency syntax: `[dependencies.foo]` header, `path = "…"` on a later line
  // within the same block (before the next `[` header or EOF).
  const DOTTED_HEADER_RE = /^\[(?:dependencies|dev-dependencies|build-dependencies|target\.[^\]]+\.dependencies)\.[A-Za-z0-9_.-]+\]\s*$/;
  for (let i = 0; i < lines.length; i++) {
    if (!DOTTED_HEADER_RE.test(lines[i])) continue;
    let blockEnd = i + 1;
    while (blockEnd < lines.length && !/^\[/.test(lines[blockEnd])) blockEnd++;
    for (let j = i + 1; j < blockEnd; j++) {
      const pathMatch = lines[j].match(/^path\s*=\s*"([^"]+)"\s*$/);
      if (!pathMatch) continue;
      const targetAbsDir = resolve(repoRoot, dir, pathMatch[1]);
      const targetRelDir = relative(repoRoot, targetAbsDir);
      const move = oldToNew.get(targetRelDir);
      if (!move && !selfMove) continue;
      const newTargetDir = move ? move.newDir : targetRelDir;
      const newRelPath = relative(join(repoRoot, ownNewDir), join(repoRoot, newTargetDir)) || ".";
      const oldLine = lines[j];
      lines[j] = `path = "${newRelPath}"`;
      if (move) {
        let packageLineIdx = -1;
        for (let k = i + 1; k < blockEnd; k++) {
          if (/^package\s*=\s*"/.test(lines[k])) packageLineIdx = k;
        }
        if (packageLineIdx >= 0) {
          lines[packageLineIdx] = `package = "${move.newPkg}"`;
        } else {
          lines.splice(j + 1, 0, `package = "${move.newPkg}"`);
          blockEnd++;
        }
      }
      if (preview) console.log(`[${dir}]\n  - ${oldLine.trim()}\n  + ${lines[j].trim()}${move ? ` (+ package = "${move.newPkg}")` : ""}`);
      changed = true;
      editedDepLines++;
    }
  }

  // Rename [package] name for crates that are themselves moving.
  if (selfMove) {
    for (let i = 0; i < lines.length; i++) {
      if (/^name\s*=\s*"/.test(lines[i])) {
        lines[i] = `name = "${selfMove.newPkg}"`;
        changed = true;
        break;
      }
    }
  }

  if (changed) {
    editedManifests++;
    if (apply) writeFileSync(manifestPath, lines.join("\n"));
  }
}

// [profile.release.package.store] key rename, if store is moving.
const storeMove = oldToNew.get("store/rs");
if (storeMove) {
  const rootManifest = join(repoRoot, "Cargo.toml");
  const text = readFileSync(rootManifest, "utf8");
  const newText = text.replace(/\[profile\.release\.package\.store\]/, `[profile.release.package.${storeMove.newPkg}]`);
  if (newText !== text) {
    editedManifests++;
    if (apply) writeFileSync(rootManifest, newText);
  }
}

// Root Cargo.toml `members` list.
{
  const rootManifest = join(repoRoot, "Cargo.toml");
  let text = readFileSync(rootManifest, "utf8");
  let rootChanged = false;
  for (const m of moves) {
    const oldEntry = `"${m.oldDir}"`;
    const newEntry = `"${m.newDir}"`;
    if (text.includes(oldEntry)) {
      text = text.replace(oldEntry, newEntry);
      rootChanged = true;
    } else {
      console.warn(`⚠️ root Cargo.toml has no members entry for ${m.oldDir}`);
    }
  }
  if (rootChanged) {
    editedManifests++;
    if (apply) writeFileSync(rootManifest, text);
  }
}

console.log(`${apply ? "wrote" : "would write"} ${editedManifests} manifests, ${editedDepLines} dependency lines rewritten`);

// Physical moves.
for (const m of moves) {
  const src = join(repoRoot, m.oldDir);
  const dst = join(repoRoot, m.newDir);
  if (!existsSync(src)) {
    console.warn(`⚠️ missing source dir ${m.oldDir}, skipping move`);
    continue;
  }
  if (apply) {
    mkdirSync(dirname(dst), { recursive: true });
    renameSync(src, dst);
  }
}
console.log(`${apply ? "moved" : "would move"} ${moves.length} directories`);
