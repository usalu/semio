import { readFileSync } from "node:fs";
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
import type { LintFn } from "./script.ts";

export type LintScriptModule = {
  policy?: LintFn<never>;
};

/** 🔎True when `script.ts` exports a repo policy lint callback. */
export function scriptExportsPolicy(scriptPath: string): boolean {
  const text = readFileSync(scriptPath, "utf8");
  return /\bexport\s+(const|function)\s+policy\b/.test(text);
}

function parsePolicyFileExport(scriptPath: string): string | undefined {
  const text = readFileSync(scriptPath, "utf8");
  const m = text.match(/export\s+const\s+policyFile\s*=\s*["']([^"']+)["']/);
  return m?.[1];
}

function fileEntityId(repoRoot: string, fileRel: string): string {
  const rel = relative(repoRoot, fileRel).replaceAll("\\", "/");
  const data = runCliGraphql(`query F($p: String!) { file(path: $p) { id } }`, { p: rel }, { repoRoot }) as {
    file: { id?: string };
  };
  if (!data.file?.id) throw new Error(`[policy-runner] file id not found for ${rel}`);
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

/** 🔎Maps `script.ts` directory to bundle, technology, or folder entity id. */
export function resolvePolicyScriptEntity(repoRoot: string, scriptPath: string): ResolvedLintEntity {
  const dir = dirname(scriptPath);
  const relDir = norm(relative(repoRoot, dir));
  const folder = runCliGraphql(`query Fo($p: String!) { folder(path: $p) { id path } }`, { p: relDir }, { repoRoot }) as {
    folder: { id?: string; path?: string };
  };
  if (!folder.folder?.id) throw new Error(`[policy-runner] folder not resolved for ${relDir}`);

  const meta = runCliGraphql(`query M { bundles { id root name } technologies { id root name } }`, {}, { repoRoot }) as {
    bundles: GraphNode[];
    technologies: GraphNode[];
  };
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

export async function runPolicyScript(scriptPath: string, repoRoot = getWorkspaceRoot()): Promise<{
  entityId: string;
  breachs: BreachRecord[];
  cachePath: string;
}> {
  const absScript =
    scriptPath.includes(":") || scriptPath.startsWith("/") || /^[A-Za-z]:\\/.test(scriptPath)
      ? scriptPath
      : join(repoRoot, scriptPath);
  const base = basename(absScript);
  if (base !== "script.ts") {
    throw new Error(`[policy-runner] expected script.ts, got ${base}`);
  }

  const policyFile = parsePolicyFileExport(absScript);
  let entity: ResolvedLintEntity;
  if (policyFile) {
    const target = join(dirname(absScript), policyFile).replaceAll("\\", "/");
    entity = { kind: "file", id: fileEntityId(repoRoot, target), path: target };
  } else {
    entity = resolvePolicyScriptEntity(repoRoot, absScript);
  }

  const href = pathToFileURL(absScript).href;
  const mod = (await import(href)) as LintScriptModule;
  const fn = mod.policy;
  if (typeof fn !== "function") {
    throw new Error(`[policy-runner] ${absScript} must export const policy = defineLint(...)`);
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
      throw new Error("[policy-runner] unreachable");
  }

  const { mkdirSync, writeFileSync } = await import("node:fs");
  const sanitizeCacheKey = (id: string) => id.replace(/[^\w.-]+/g, "_").slice(0, 200);
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

/** 🚪Runs `policy` on this `script.ts` and exits 1 when any high-priority breach exists. */
export async function runPolicyExit(scriptPath: string): Promise<void> {
  const { breachs } = await runPolicyScript(scriptPath);
  if (breachs.some((b) => b.priority === "high")) process.exit(1);
}
