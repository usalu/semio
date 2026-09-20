#!/usr/bin/env bun
/** 🪢️ V3b repair: gives every `unreachable-from-cargo-manifest` leaf the `#[path]` mount it is missing.
 *
 * The gate's question is "does some Cargo-owned module graph reach this file"
 * (`…/📇️registry/🗿️taxonomy-validation/🟦️.ts:285`). The taxonomy's own answer, used by every mounted
 * sibling in the tree, is that the NEAREST ancestor component leaf mounts the child by a literal
 * `#[path]` relative to itself — `🎛️sampler/🚚️move/🦀️.rs:77` mounts `🧪️tests/🔬️direct-leaf/🦀️.rs`
 * exactly that way. This script reproduces that placement mechanically:
 *
 * - host = the closest ancestor directory carrying a `🦀️.rs` that is itself reachable (an unreachable
 *   ancestor would only move the problem up one level), falling back to the plugin root leaf;
 * - a leaf under a `🧪️tests` segment mounts as `#[cfg(test)] mod`, everything else as `pub mod`;
 * - the module identifier is the shortest ASCII suffix of the leaf's own path that is unique inside
 *   the host file, so two `🧪️tests/🧩️example/🦀️.rs` leaves under one artifact cannot collide.
 *
 * Every host file is re-read immediately before it is written (peers edit these crates concurrently),
 * and a leaf whose `#[path]` target is already present anywhere in the host is skipped, so the script
 * is idempotent.
 *
 * Usage: `bun 🐍️v3b-mount-repair.ts <census.txt> [--apply] [--plugin <id>] [--shape <substring>]`
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const pluginsRel = "✏️s/🔌️plugins";
const LEAF = "🦀️.rs";
const TESTS_DIR = "🧪️tests";
const REGION = "🪢️TaxonomyMounts";

const censusPath = process.argv[2]!;
const apply = process.argv.includes("--apply");
const onlyPlugin = process.argv.includes("--plugin") ? process.argv[process.argv.indexOf("--plugin") + 1]! : null;
const onlyShape = process.argv.includes("--shape") ? process.argv[process.argv.indexOf("--shape") + 1]! : null;

type Finding = { readonly plugin: string; readonly rel: string };
const findings: Finding[] = [];
for (const line of readFileSync(censusPath, "utf8").split("\n")) {
  if (!line.includes(" is not reachable from Cargo manifest ")) continue;
  const body = line.replace(/^\s*-\s*/u, "").replace(/ is not reachable from Cargo manifest .*$/u, "");
  const at = body.indexOf(": ");
  findings.push({ plugin: body.slice(0, at), rel: body.slice(at + 2) });
}
if (findings.length === 0) throw new Error(`no unreachable findings parsed out of ${censusPath}`);

const unreachable = new Set(findings.map((row) => `${row.plugin}\0${row.rel}`));

/** 🔤️ The ASCII identifier a taxonomy directory name projects to: emoji and variation selectors are
 * dropped, separators become `_`, and a leading digit takes the `v` prefix the tree already uses for
 * `🔖️2.0` → `v2_0`. */
function identifierOf(segment: string): string {
  const ascii = segment.replace(/[^\x20-\x7E]/gu, "").trim().toLowerCase();
  const slug = ascii.replace(/[^a-z0-9]+/gu, "_").replace(/_+/gu, "_").replace(/^_|_$/gu, "");
  if (slug === "") return "leaf";
  return /^[0-9]/u.test(slug) ? `v${slug}` : slug;
}

const RUST_KEYWORDS = new Set(["as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try"]);

type Mount = { readonly hostAbs: string; readonly target: string; name: string; readonly cfgTest: boolean; readonly leafRel: string; readonly plugin: string };
const mounts: Mount[] = [];
const skipped: { readonly plugin: string; readonly rel: string; readonly reason: string }[] = [];
const takenByHost = new Map<string, Set<string>>();

for (const { plugin, rel } of findings) {
  if (onlyPlugin && plugin !== onlyPlugin) continue;
  if (onlyShape && !rel.includes(onlyShape)) continue;
  const pluginRoot = join(repoRoot, pluginsRel, plugin);
  const leafAbs = join(pluginRoot, rel);
  if (!existsSync(leafAbs)) {
    skipped.push({ plugin, rel, reason: "leaf no longer on disk" });
    continue;
  }
  if (basename(rel) !== LEAF) {
    skipped.push({ plugin, rel, reason: "not a taxonomy component leaf" });
    continue;
  }
  let dir = dirname(rel);
  let hostRel: string | null = null;
  while (dir !== "." && dir !== "") {
    dir = dirname(dir);
    const candidate = dir === "." ? LEAF : `${dir}/${LEAF}`;
    if (!existsSync(join(pluginRoot, candidate))) continue;
    if (unreachable.has(`${plugin}\0${candidate}`)) continue;
    hostRel = candidate;
    break;
  }
  if (hostRel === null) {
    skipped.push({ plugin, rel, reason: "no reachable ancestor component leaf" });
    continue;
  }
  const hostAbs = join(pluginRoot, hostRel);
  const target = relative(dirname(hostAbs), leafAbs).replaceAll("\\", "/");
  const hostText = readFileSync(hostAbs, "utf8");
  if (hostText.includes(`#[path = "${target}"]`)) {
    skipped.push({ plugin, rel, reason: "already mounted in host" });
    continue;
  }
  const taken = takenByHost.get(hostAbs) ?? new Set<string>([...hostText.matchAll(/^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*[;{]/gmu)].map((match) => match[1]!));
  takenByHost.set(hostAbs, taken);
  mounts.push({ hostAbs, target, name: "", cfgTest: target.split("/").includes(TESTS_DIR), leafRel: rel, plugin });
}

const byHost = new Map<string, Mount[]>();
for (const mount of mounts) byHost.set(mount.hostAbs, [...(byHost.get(mount.hostAbs) ?? []), mount]);

/** 🏷️ One depth for every new module of a host, so sibling lanes read symmetrically: the shortest
 * path suffix at which all of this host's new mounts differ from each other and from the modules the
 * file already declares. `📤️export/🧵️serializers/🗿️artifacts/…` and its `📥️import` twin would
 * otherwise land as `artifacts` and `deserializers_artifacts`, naming the same coordinate two ways. */
for (const [hostAbs, rows] of byHost) {
  const taken = takenByHost.get(hostAbs)!;
  const segmentsOf = (row: Mount): string[] => row.target.split("/").slice(0, -1).filter((segment) => segment !== ".." && segment !== ".");
  const nameAt = (row: Mount, depth: number): string => {
    const segments = segmentsOf(row);
    const base = segments.slice(Math.max(0, segments.length - depth)).map(identifierOf).join("_");
    return RUST_KEYWORDS.has(base) ? `${base}_mod` : base;
  };
  const deepest = Math.max(...rows.map((row) => segmentsOf(row).length));
  let depth = 1;
  for (; depth < deepest; depth++) {
    const names = rows.map((row) => nameAt(row, depth));
    if (new Set(names).size === names.length && names.every((name) => !taken.has(name))) break;
  }
  for (const row of rows) {
    let unique = nameAt(row, depth);
    for (let suffix = 2; taken.has(unique); suffix++) unique = `${nameAt(row, depth)}_${suffix}`;
    taken.add(unique);
    row.name = unique;
  }
}

for (const [hostAbs, rows] of byHost) {
  const text = readFileSync(hostAbs, "utf8");
  const lines = text.replace(/\n*$/u, "\n").split("\n");
  const block = rows.flatMap((row) => [...(row.cfgTest ? ["#[cfg(test)]"] : []), `#[path = "${row.target}"]`, `${row.cfgTest ? "mod" : "pub mod"} ${row.name};`]);
  const endRegion = lines.lastIndexOf(`//#endregion ${REGION}`);
  const next = endRegion === -1 ? [...lines.slice(0, -1), "", `//#region ${REGION}`, ...block, `//#endregion ${REGION}`, ""] : [...lines.slice(0, endRegion), ...block, ...lines.slice(endRegion)];
  if (apply) writeFileSync(hostAbs, next.join("\n"));
}

const perPlugin = new Map<string, number>();
for (const mount of mounts) perPlugin.set(mount.plugin, (perPlugin.get(mount.plugin) ?? 0) + 1);
console.log(`${apply ? "mounted" : "would mount"} ${mounts.length} leaf/leaves across ${byHost.size} host file(s); skipped ${skipped.length}`);
for (const [plugin, count] of [...perPlugin].sort((a, b) => b[1] - a[1])) console.log(`  ${String(count).padStart(4)}  ${plugin}`);
const reasons = new Map<string, number>();
for (const row of skipped) reasons.set(row.reason, (reasons.get(row.reason) ?? 0) + 1);
for (const [reason, count] of [...reasons].sort((a, b) => b[1] - a[1])) console.log(`  skip ${String(count).padStart(4)}  ${reason}`);
console.log("=== skipped rows ===");
for (const row of skipped) console.log(`${row.plugin}\t${row.rel}\t${row.reason}`);
console.log("=== mounts ===");
for (const mount of mounts) console.log(`${mount.plugin}\t${relative(repoRoot, mount.hostAbs)}\t${mount.cfgTest ? "cfg(test) mod" : "pub mod"} ${mount.name}\t${mount.target}`);
