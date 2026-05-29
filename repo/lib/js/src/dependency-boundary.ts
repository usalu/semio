import { readFileSync, existsSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import type { BreachRecord } from "./breach.ts";

const ADAPTER_MARKERS = [
  "//#region 🔌adapter",
  "// #region 🔌adapter",
  "#region 🔌adapter",
  "//#region 🔌adapters",
  "// #region 🔌adapters",
  "pub mod adapters",
  "mod adapters ",
];

const INTERNAL_PREFIXES = [
  "@semio/",
  "@ui/",
  "@cad/",
  "@puzzle/",
  "@framework/",
  "@repo/",
  "@elements/",
  "@coda/",
];

/** 🔌Returns true when the file path or content marks an adapter boundary. */
export function isAdapterBoundaryFile(filePath: string, content: string): boolean {
  const n = normalize(filePath).replaceAll("\\", "/").toLowerCase();
  if (n.includes("/adapters/") || n.includes("/external_adapters")) return true;
  if (n.includes("adapter")) return true;
  const lower = content.toLowerCase();
  return ADAPTER_MARKERS.some((m) => lower.includes(m));
}

/** 🔌Skips generated, test, and non-source paths for dependency-boundary lint. */
export function shouldSkipDependencyBoundaryFile(filePath: string): boolean {
  const n = normalize(filePath).replaceAll("\\", "/");
  if (n.includes("/node_modules/") || n.includes("/.repo/") || n.includes("/dist/") || n.includes("/target/")) {
    return true;
  }
  const base = n.split("/").pop() ?? n;
  if (base.endsWith(".gen.ts")) return true;
  if (base.includes(".test.") || base.endsWith(".spec.ts")) return true;
  return /\.(json|md|ya?ml|lock|svg|png|jpe?g|woff2?)$/i.test(base);
}

function isInternalPackage(name: string, version: string): boolean {
  if (INTERNAL_PREFIXES.some((p) => name.startsWith(p))) return true;
  return version.startsWith("workspace:") || version.startsWith("file:") || version.startsWith("link:");
}

/** 🔌Loads third-party package names from the nearest manifest walking up from filePath. */
export function loadThirdPartyDeps(repoRoot: string, filePath: string): Set<string> {
  const deps = new Set<string>();
  let dir = normalize(dirname(join(repoRoot, filePath))).replaceAll("\\", "/");
  const root = normalize(repoRoot).replaceAll("\\", "/");
  while (dir.startsWith(root) || dir === root) {
    const pkg = join(dir, "package.json");
    if (existsSync(pkg)) {
      mergePackageJson(pkg, deps);
      break;
    }
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return deps;
}

function mergePackageJson(path: string, deps: Set<string>): void {
  const raw = JSON.parse(readFileSync(path, "utf8")) as Record<string, Record<string, string> | undefined>;
  for (const key of ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] as const) {
    const block = raw[key];
    if (!block) continue;
    for (const [name, ver] of Object.entries(block)) {
      if (isInternalPackage(name, ver)) continue;
      deps.add(name);
    }
  }
}

/** 🔌Parses import specifiers from a single import line (TS/JS). */
export function parseTsImportSpecs(line: string): string[] {
  const specs: string[] = [];
  const re = /(?:from|import)\s+['"]([^'"]+)['"]/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(line)) !== null) {
    specs.push(m[1]);
  }
  return specs;
}

function isThirdPartySpec(spec: string, deps: Set<string>): boolean {
  if (!spec || spec.startsWith(".") || spec.startsWith("/")) return false;
  if (deps.has(spec)) return true;
  if (spec.startsWith("@")) {
    const parts = spec.split("/");
    if (parts.length >= 2 && deps.has(`${parts[0]}/${parts[1]}`)) return true;
  }
  for (const dep of deps) {
    if (spec === dep || spec.startsWith(`${dep}/`)) return true;
  }
  return false;
}

/** 🔌Builds breach records for direct third-party imports outside adapter boundaries. */
export function dependencyBoundaryBreachesForFile(
  repoRoot: string,
  filePath: string,
  content: string,
  scope: string,
): BreachRecord[] {
  if (shouldSkipDependencyBoundaryFile(filePath)) return [];
  if (isAdapterBoundaryFile(filePath, content)) return [];
  const deps = loadThirdPartyDeps(repoRoot, filePath);
  if (deps.size === 0) return [];
  const breachs: BreachRecord[] = [];
  const lines = content.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (!line.includes("import ")) continue;
    for (const spec of parseTsImportSpecs(line)) {
      if (!isThirdPartySpec(spec, deps)) continue;
      breachs.push({
        id: `dep-boundary-${spec}-${i + 1}`,
        summary: `Direct import of third-party "${spec}" must live in an adapter module`,
        kind: "dependency-boundary/import/direct-third-party",
        priority: "high",
        reason: "Third-party packages must only be imported in adapter regions or adapter paths",
        solution:
          "Move the import into a //#region 🔌Adapter (or /adapters/) module and depend on a first-party port interface elsewhere",
        scope,
      });
    }
  }
  return breachs;
}
