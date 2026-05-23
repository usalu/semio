import { mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { pathToFileURL } from "node:url";
import type { BreachRecord } from "./breach.ts";
import { getWorkspaceRoot, runCliGraphql } from "./cli.ts";
import {
  BundleLinter,
  FileLinter,
  FolderLinter,
  type GraphNode,
  TechnologyLinter,
} from "./linter.ts";

export type LintScriptModule<T extends { default?: unknown }> = T & {
  default?: (l: never) => BreachRecord[] | Promise<BreachRecord[]>;
};

function sanitizeCacheKey(id: string): string {
  return id.replace(/[^\w.-]+/g, "_").slice(0, 200);
}

function parseFileLintTarget(scriptPath: string): string {
  const base = basename(scriptPath);
  const m = base.match(/^(.+)\.([^.]+)\.lint\.script\.ts$/);
  if (!m) throw new Error(`[runner] not a file lint script name: ${base}`);
  const stem = m[1];
  const ext = m[2];
  const dir = dirname(scriptPath);
  return join(dir, `${stem}.${ext}`).replaceAll("\\", "/");
}

function fileEntityId(repoRoot: string, fileRel: string): string {
  const rel = relative(repoRoot, fileRel).replaceAll("\\", "/");
  const data = runCliGraphql(
    `query F($p: String!) { file(path: $p) { id } }`,
    { p: rel },
    { repoRoot },
  ) as { file: { id?: string } };
  if (!data.file?.id) throw new Error(`[runner] file id not found for ${rel}`);
  return data.file.id;
}

function norm(p: string): string {
  return p.replaceAll("\\", "/").replace(/\/+$/, "");
}

export type ResolvedLintEntity =
  | { kind: "file"; id: string; path: string }
  | { kind: "folder"; id: string }
  | { kind: "bundle"; id: string }
  | { kind: "technology"; id: string };

/** 🔎Maps lint.script.ts directory to bundle, technology, or folder entity id. */
export function resolveLintScriptEntity(repoRoot: string, scriptPath: string): ResolvedLintEntity {
  const dir = dirname(scriptPath);
  const relDir = norm(relative(repoRoot, dir));
  const folder = runCliGraphql(
    `query Fo($p: String!) { folder(path: $p) { id path } }`,
    { p: relDir },
    { repoRoot },
  ) as { folder: { id?: string; path?: string } };
  if (!folder.folder?.id) throw new Error(`[runner] folder not resolved for ${relDir}`);

  const meta = runCliGraphql(
    `query M { bundles { id root name } technologies { id root name } }`,
    {},
    { repoRoot },
  ) as { bundles: GraphNode[]; technologies: GraphNode[] };
  const d = norm(relDir);
  for (const b of meta.bundles ?? []) {
    if (norm(String(b.root ?? "")) === d) {
      return { kind: "bundle", id: String(b.id) };
    }
  }
  for (const t of meta.technologies ?? []) {
    if (norm(String(t.root ?? "")) === d) {
      return { kind: "technology", id: String(t.id) };
    }
  }
  return { kind: "folder", id: String(folder.folder.id) };
}

export async function runLintScript(scriptPath: string, repoRoot = getWorkspaceRoot()): Promise<{
  entityId: string;
  breachs: BreachRecord[];
  cachePath: string;
}> {
  const absScript = scriptPath.includes(":") || scriptPath.startsWith("/") || /^[A-Za-z]:\\/.test(scriptPath)
    ? scriptPath
    : join(repoRoot, scriptPath);
  const base = basename(absScript);
  let entity: ResolvedLintEntity;
  if (base.endsWith(".lint.script.ts") && base !== "lint.script.ts") {
    const target = parseFileLintTarget(absScript);
    entity = { kind: "file", id: fileEntityId(repoRoot, target), path: target };
  } else if (base === "lint.script.ts") {
    entity = resolveLintScriptEntity(repoRoot, absScript);
  } else {
    throw new Error(`[runner] unsupported lint script name: ${base}`);
  }

  const href = pathToFileURL(absScript).href;
  const mod = (await import(href)) as LintScriptModule<{ default?: (x: never) => BreachRecord[] | Promise<BreachRecord[]> }>;
  const fn = mod.default;
  if (typeof fn !== "function") {
    throw new Error(`[runner] ${absScript} must default-export a lint function`);
  }

  let breachs: BreachRecord[];
  switch (entity.kind) {
    case "file":
      breachs = await fn(new FileLinter(entity.id, repoRoot) as never);
      break;
    case "folder":
      breachs = await fn(new FolderLinter(entity.id, repoRoot) as never);
      break;
    case "bundle":
      breachs = await fn(new BundleLinter(entity.id, repoRoot) as never);
      break;
    case "technology":
      breachs = await fn(new TechnologyLinter(entity.id, repoRoot) as never);
      break;
    default:
      throw new Error(`[runner] unreachable`);
  }

  const cacheDir = join(repoRoot, ".repo", "cache", "breaches");
  mkdirSync(cacheDir, { recursive: true });
  const cacheName = `${sanitizeCacheKey(entity.id)}.json`;
  const cachePath = join(cacheDir, cacheName);
  const payload = {
    entityId: entity.id,
    script: relative(repoRoot, absScript).replaceAll("\\", "/"),
    breachs,
  };
  writeFileSync(cachePath, JSON.stringify(payload, null, 2), "utf8");
  return { entityId: entity.id, breachs, cachePath };
}

/** 📂Collects direct file sizes under a folder (repo-relative paths). */
export function directChildFileSizes(repoRoot: string, folderRel: string): { name: string; bytes: number }[] {
  const abs = join(repoRoot, folderRel);
  const out: { name: string; bytes: number }[] = [];
  for (const name of readdirSync(abs)) {
    const p = join(abs, name);
    const st = statSync(p);
    if (st.isFile()) {
      out.push({ name, bytes: st.size });
    }
  }
  return out;
}
