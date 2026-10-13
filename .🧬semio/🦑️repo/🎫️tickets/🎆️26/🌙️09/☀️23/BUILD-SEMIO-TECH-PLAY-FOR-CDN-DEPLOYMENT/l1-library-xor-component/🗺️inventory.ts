/** 🗺️ Inventories every dual crate-type manifest (and every component manifest) of wave L1 and classifies it. */
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { execSync } from "node:child_process";

export const repoRoot = resolve(import.meta.dir, "../../../../../../../..");

export type Kind = "plugin-component" | "test-component" | "module-split" | "module-only" | "library";
export type Row = {
  kind: Kind;
  name: string;
  manifest: string;
  crateDir: string;
  libPath: string;
  owner: string;
  role?: string;
  pluginId?: string;
  componentKind?: string;
  exportsFile?: string;
  exportsLine?: string;
};

const MODULE_SPLIT = new Set(["semio-framework-editor", "semio-framework-surface", "semio-framework-actor", "semio-framework-os", "semio-framework-os-flow", "semio-s-spatial-kernel-semio-session", "semio-framework-os-node-graph", "semio-framework-os-renderer-wgpu"]);
const MODULE_ONLY = new Set(["semio-s-plugin-trinity-jack-lsp", "semio-s-dev-cad"]);
const LIBRARY = new Set(["semio-framework-os-kernel", "semio-framework-math", "semio-framework-os-shell"]);
const TEST_COMPONENT = new Set(["semio-framework-plugin-host-test-component", "semio-framework-os-scale-fixture"]);

/** 📋️ Every tracked manifest whose lib target declares a cdylib. */
export function dualManifests(): string[] {
  return execSync("git grep -l -E '^crate-type *=.*cdylib' -- '*Cargo.toml' ':!.🧬semio'", { cwd: repoRoot, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim().split("\n").filter(Boolean);
}

/** 🧭️ Finds the export macro invocation of a component library by scanning its crate sources. */
function exportsSite(owner: string, crateDir: string): { file: string; line: string } | undefined {
  const out = execSync(`rg -n -l --no-messages -e '^\\s*semio_framework_plugin::(plugin|extension)_exports!' ${JSON.stringify(owner)} -g '*.rs'`, { cwd: repoRoot, encoding: "utf8" }).trim().split("\n").filter(Boolean).filter((file) => !file.includes("/🧩️component/") && !file.includes("/🧪️tests/") && !file.includes("/🗿️artifacts/"));
  if (out.length !== 1) return out.length ? { file: out.join(" | "), line: "AMBIGUOUS" } : undefined;
  const text = readFileSync(join(repoRoot, out[0]!), "utf8");
  const match = text.match(/^\s*semio_framework_plugin::(plugin|extension)_exports!\s*\(([\s\S]*?)\);/m)!;
  return { file: out[0]!, line: match[0].trim().replace(/\s+/g, " ") };
}

export function inventory(): Row[] {
  const rows: Row[] = [];
  for (const manifest of dualManifests()) {
    if (manifest.includes("/👽️guest/")) continue;
    const text = readFileSync(join(repoRoot, manifest), "utf8"), parsed = Bun.TOML.parse(text) as any;
    if (!/crate-type\s*=\s*\[[^\]]*rlib/.test(text) && !TEST_COMPONENT.has(parsed.package.name)) continue;
    const name = parsed.package.name as string, crateDir = dirname(manifest), libPath = relative(repoRoot, resolve(repoRoot, crateDir, parsed.lib?.path ?? "src/lib.rs"));
    const owner = relative(repoRoot, resolve(repoRoot, crateDir, "../.."));
    const component = parsed.package.metadata?.component?.package as string | undefined;
    const kind: Kind = TEST_COMPONENT.has(name) ? "test-component" : MODULE_SPLIT.has(name) ? "module-split" : MODULE_ONLY.has(name) ? "module-only" : LIBRARY.has(name) ? "library" : component ? "plugin-component" : (() => { throw new Error(`Unclassified dual crate ${name}`); })();
    const row: Row = { kind, name, manifest, crateDir, libPath, owner, role: parsed.package.metadata?.semio?.role, pluginId: component?.slice(6), componentKind: parsed.package.metadata?.semio?.["component-kind"] };
    if (kind === "plugin-component") { const site = exportsSite(owner, crateDir); row.exportsFile = site?.file; row.exportsLine = site?.line; }
    rows.push(row);
  }
  return rows.sort((a, b) => a.kind.localeCompare(b.kind) || a.name.localeCompare(b.name));
}

if (import.meta.main) {
  const rows = inventory();
  for (const row of rows) console.log([row.kind, row.name, row.owner, row.libPath.slice(row.owner.length + 1), row.exportsFile ? row.exportsFile.slice(row.owner.length + 1) : "-", row.exportsLine ?? ""].join(" | "));
  const counts = rows.reduce<Record<string, number>>((acc, row) => ({ ...acc, [row.kind]: (acc[row.kind] ?? 0) + 1 }), {});
  console.log(JSON.stringify(counts), existsSync(repoRoot + "/Cargo.toml"));
}
