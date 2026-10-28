import { existsSync, readFileSync, statSync, realpathSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
const packageDir = realpathSync(process.argv[2]);
const repoRoot = resolve(packageDir, "../../../../../../..");
const repoRelative = (path: string): string => relative(repoRoot, resolve(repoRoot, path)).replaceAll("\\", "/");
const browserContract = JSON.parse(readFileSync(join(packageDir, "../../🧪️tests/🧫️fixtures/🌐️browser-graph.json"), "utf8")) as {
  readonly entry: string;
  readonly configPath: string;
  readonly resolveExtensions: readonly string[];
  readonly conditionNames: readonly string[];
  readonly extensionAlias: Readonly<Record<string, readonly string[]>>;
  readonly alias: readonly { readonly find: string; readonly replacement: string }[];
  readonly denyPackages: readonly string[];
  readonly denySpecifiers: readonly { readonly specifier: string; readonly reason: string }[];
  readonly denyModules: readonly string[];
  readonly requireModules: readonly string[];
  readonly ambiguousDirectories: readonly { readonly directory: string; readonly kindBasename: string; readonly extensionlessResolvesTo: string; readonly browserEntry: string }[];
  readonly maxModules: number;
};
const browserAliases = [...browserContract.alias].sort((left, right) => right.find.length - left.find.length);
const SOURCE_MODULE = /\.(?:[cm]?tsx?|[cm]?jsx?)$/u;

/** 🧭️ Vite's file resolution for one candidate path, in the declared extension order: the exact file, then
 * `<candidate><extension>`, then a directory's own package entry. The ORDER is the contract — `🟦️.mts` sits ahead of
 * `🟦️.ts`/`🟦️.tsx`, so an extensionless `…/🟦️` in a directory that also carries a tool-config entry silently resolves
 * to the tool config rather than to the browser entry beside it. */
function resolveBrowserCandidate(candidate: string): string | null {
  if (existsSync(candidate) && statSync(candidate).isFile()) return candidate;
  for (const [emitted, authored] of Object.entries(browserContract.extensionAlias)) {
    if (!candidate.endsWith(emitted)) continue;
    for (const extension of authored) {
      const twin = `${candidate.slice(0, -emitted.length)}${extension}`;
      if (existsSync(twin) && statSync(twin).isFile()) return twin;
    }
  }
  for (const extension of browserContract.resolveExtensions) {
    const withExtension = `${candidate}${extension}`;
    if (existsSync(withExtension) && statSync(withExtension).isFile()) return withExtension;
  }
  if (!existsSync(candidate) || !statSync(candidate).isDirectory()) return null;
  const manifestPath = join(candidate, "package.json");
  if (existsSync(manifestPath)) {
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { exports?: unknown; module?: string; main?: string };
    const exported = typeof manifest.exports === "string" ? manifest.exports : typeof (manifest.exports as Record<string, unknown> | undefined)?.["."] === "string" ? ((manifest.exports as Record<string, string>)["."]) : undefined;
    const entry = exported ?? manifest.module ?? manifest.main;
    if (entry) return resolveBrowserCandidate(resolve(candidate, entry));
  }
  for (const extension of browserContract.resolveExtensions) {
    const indexPath = join(candidate, `index${extension}`);
    if (existsSync(indexPath)) return indexPath;
  }
  return null;
}

/** ✂️ Every specifier a bundler keeps: comments and fully type-only `import type … from` / `export type … from`
 * statements are erased before any bundler sees them, so they carry no browser edge and are dropped here too. */
function browserSpecifiers(source: string): readonly string[] {
  const body = source
    .replaceAll(/\/\*[\s\S]*?\*\//gu, "")
    .replaceAll(/(^|[^:'"`\\])\/\/[^\n]*/gu, "$1")
    .replaceAll(/\b(?:import|export)\s+type\s[^;]*?\bfrom\s*["'][^"']+["']/gu, "");
  const found = new Set<string>();
  for (const pattern of [/\bfrom\s*["']([^"']+)["']/gu, /\bimport\s*\(\s*["']([^"']+)["']/gu, /\bimport\s+["']([^"']+)["']/gu, /\brequire\(\s*["']([^"']+)["']\)/gu]) {
    for (const match of body.matchAll(pattern)) found.add(match[1]!);
  }
  return [...found];
}

const aliasTarget = (specifier: string): string | null => {
  const hit = browserAliases.find((entry) => specifier === entry.find);
  return hit ? join(repoRoot, hit.replacement) : null;
};

const browserConditions: ReadonlySet<string> = new Set(browserContract.conditionNames);

/** 📦️ Resolves one `exports` target under the browser conditions: a string, the first matching condition of an object, or the first resolvable member of an array. */
function exportsTarget(target: unknown, wildcard: string): string | null {
  if (typeof target === "string") return target.replaceAll("*", wildcard);
  if (Array.isArray(target)) {
    for (const member of target) {
      const hit = exportsTarget(member, wildcard);
      if (hit !== null) return hit;
    }
    return null;
  }
  if (target !== null && typeof target === "object") {
    for (const [condition, member] of Object.entries(target)) {
      if (!browserConditions.has(condition)) continue;
      const hit = exportsTarget(member, wildcard);
      if (hit !== null) return hit;
    }
  }
  return null;
}

/** 🗝️ The `exports` map of a package manifest: an exact subpath, else the longest `*` pattern that covers it; a package that declares `exports` exposes nothing else. */
function exportsEntry(exported: unknown, subpath: string): string | null {
  const key = subpath === "" ? "." : `./${subpath}`;
  const table: Readonly<Record<string, unknown>> = exported !== null && typeof exported === "object" && !Array.isArray(exported) && Object.keys(exported).some((name) => name.startsWith(".")) ? (exported as Record<string, unknown>) : { ".": exported };
  if (Object.hasOwn(table, key)) return exportsTarget(table[key], "");
  for (const pattern of Object.keys(table).filter((name) => name.includes("*")).sort((left, right) => right.length - left.length)) {
    const [head, tail] = pattern.split("*") as [string, string];
    if (key.length >= head.length + tail.length && key.startsWith(head) && key.endsWith(tail)) return exportsTarget(table[pattern], key.slice(head.length, key.length - tail.length));
  }
  return null;
}

/** 🏠️ The workspace package a bare specifier names, found the way Node and Vite find it (the nearest `node_modules/<name>` above the importer) and kept only when it links into this repository: third-party packages stay leaves. */
function workspacePackage(importerDirectory: string, specifier: string): { readonly directory: string; readonly subpath: string } | null {
  const parts = specifier.split("/"), size = specifier.startsWith("@") ? 2 : 1;
  if (parts.length < size) return null;
  const name = parts.slice(0, size).join("/"), checkout = realpathSync(repoRoot);
  for (let directory = importerDirectory; ; directory = dirname(directory)) {
    const candidate = join(directory, "node_modules", name);
    if (existsSync(candidate)) {
      const real = realpathSync(candidate);
      return real.startsWith(checkout + sep) && !real.includes(`${sep}node_modules${sep}`) ? { directory: real, subpath: parts.slice(size).join("/") } : null;
    }
    if (dirname(directory) === directory) return null;
  }
}

/** 🧭️ The file a workspace package's `exports` map (else its directory) names for one bare specifier. */
function workspaceTarget(importerDirectory: string, specifier: string): string | null {
  const found = workspacePackage(importerDirectory, specifier);
  if (found === null) return null;
  const manifestPath = join(found.directory, "package.json");
  const manifest = existsSync(manifestPath) ? (JSON.parse(readFileSync(manifestPath, "utf8")) as { exports?: unknown }) : {};
  if (manifest.exports === undefined) return join(found.directory, found.subpath);
  const entry = exportsEntry(manifest.exports, found.subpath);
  return entry === null ? null : resolve(found.directory, entry);
}

interface BrowserGraph {
  readonly modules: readonly string[];
  readonly packageEdges: ReadonlyMap<string, readonly string[]>;
  readonly resolutions: readonly { readonly importerDirectory: string; readonly specifier: string; readonly resolved: string; readonly exported: boolean }[];
  readonly unresolved: readonly string[];
}

/** 🌐️ The closure Vite serves to the browser: a breadth-first walk of the host entry's relative and aliased
 * imports under the declared resolve order, with every bare specifier recorded as a leaf edge (Vite hands exactly
 * those to its esbuild dependency optimizer, which is where a node-only package's `.node` binding fails the pass). */
function browserGraph(): BrowserGraph {
  const entryPath = join(repoRoot, browserContract.entry);
  const modules = new Set([entryPath]);
  const packageEdges = new Map<string, string[]>();
  const resolutions: { importerDirectory: string; specifier: string; resolved: string; exported: boolean }[] = [];
  const unresolved: string[] = [];
  const queue = [entryPath];
  while (queue.length > 0) {
    const file = queue.shift()!;
    if (!SOURCE_MODULE.test(file)) continue;
    for (const specifier of browserSpecifiers(readFileSync(file, "utf8"))) {
      if (/^(?:node:|bun:|virtual:|data:|https?:)/u.test(specifier)) continue;
      const bare = specifier.split("?")[0]!;
      const aliased = aliasTarget(bare), exported = aliased === null && !bare.startsWith(".") ? workspaceTarget(dirname(file), bare) : null;
      const target = aliased ?? (bare.startsWith(".") ? resolve(dirname(file), bare) : exported);
      const resolved = target === null ? null : resolveBrowserCandidate(target);
      if (resolved === null) {
        (bare.startsWith(".") ? unresolved : []).push(`${repoRelative(file)} → ${bare}`);
        if (!bare.startsWith(".")) packageEdges.set(bare, [...(packageEdges.get(bare) ?? []), repoRelative(file)]);
        continue;
      }
      if (resolved.includes("node_modules")) {
        packageEdges.set(bare, [...(packageEdges.get(bare) ?? []), repoRelative(file)]);
        continue;
      }
      resolutions.push({ importerDirectory: dirname(file), specifier: bare, resolved, exported: exported !== null });
      if (!modules.has(resolved)) {
        modules.add(resolved);
        queue.push(resolved);
      }
    }
  }
  return { modules: [...modules].map(repoRelative).sort(), packageEdges, resolutions, unresolved };
}
const g = browserGraph();
console.log("modules", g.modules.length, "unresolved", g.unresolved.length, "exported", g.resolutions.filter((r) => r.exported).length);
for (const m of browserContract.requireModules) console.log(g.modules.includes(m) ? "HAS    " : "MISSING", m);
const top: Record<string, number> = {};
for (const m of g.modules) { const k = m.split("/").slice(0, 6).join("/"); top[k] = (top[k] || 0) + 1; }
for (const [k, n] of Object.entries(top).sort((x, y) => y[1] - x[1]).slice(0, Number(process.env.TOP ?? 15))) console.log(n, k);

if (process.env.UNRES) console.log(g.unresolved.join("\n"));
