// 🧷️ Declaration check: every ui-family crate identifier used in a crate's compiled source (all labels, all features, lib and tests) must be declared by the crate's manifest (normal edge for production use, normal or dev edge for test-only use). Also lists remaining `ui_wgpu` uses. argv[2]: manifest list, argv[3]: JSON out.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {LABELS, ROOT, cfgList, loadNode, readFile, rel, targetsOf} from "./rtd-lib.ts";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {cfgCompiledWithoutTest, inRegions, mask, testRegions} from "./rtd-rust.ts";

const tolerant = async (p: string) => (await readFile(p)) ?? new Uint8Array(0);
const closure = async (entry: string, node: any, cfg: string[]): Promise<string[]> => {
  try { return (await readRustSourceClosureV1([entry], ROOT, cfg, tolerant, {manifestDirectory: node.dir, generated: () => undefined}) as any[]).filter(i => i.kind === "source").map(i => i.path as string); } catch { return []; }
};
const wsDeps: Record<string, any> = (Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
const out: any[] = [];
let problems = 0;
for (const m of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const dir = dirname(join(ROOT, m));
  const node = loadNode(dir);
  if (!node?.libEntry) continue;
  const t = node.parsed;
  const normalKeys = new Set<string>(), devKeys = new Set<string>();
  const addKeys = (tbl: any, set: Set<string>) => { for (const k of Object.keys(tbl ?? {})) set.add(k.replace(/-/g, "_")); };
  addKeys(t.dependencies, normalKeys); addKeys(t["build-dependencies"], normalKeys); addKeys(t["dev-dependencies"], devKeys);
  for (const b of Object.values(t.target ?? {}) as any[]) { addKeys(b.dependencies, normalKeys); addKeys(b["dev-dependencies"], devKeys); }
  const feats = new Set<string>([...Object.keys(node.table), "default"]);
  const normal = new Set<string>(), test = new Set<string>();
  for (const label of LABELS) {
    for (const p of await closure(node.libEntry, node, cfgList(label, feats, false))) normal.add(p);
    for (const tg of targetsOf(node, feats)) if (tg.kind !== "example") for (const p of await closure(tg.entry, node, cfgList(label, feats, tg.test))) test.add(p);
  }
  const rootCfgTest = /#!\[\s*cfg\s*\(\s*test\s*\)\s*\]/.test(mask(readFileSync(node.libEntry, "utf8")));
  const needNormal = new Map<string, string>(), needTest = new Map<string, string>(), aliasUse = {normal: 0, test: 0};
  for (const p of new Set([...normal, ...test])) {
    const abs = p.startsWith("/") ? p : join(ROOT, p);
    if (!existsSync(abs) || !abs.endsWith(".rs")) continue;
    const text = readFileSync(abs, "utf8");
    if (!/semio_framework_ui_|ui_wgpu/.test(text)) continue;
    const mk = mask(text);
    const regions = testRegions(mk);
    const testOnlyFile = !normal.has(p) || rootCfgTest;
    for (const mm of mk.matchAll(/(?<![A-Za-z0-9_:.$])(semio_framework_ui_[a-z0-9_]+|ui_wgpu)\s*::/g)) {
      const reg = inRegions(regions, mm.index!);
      const isTest = testOnlyFile || (reg && !cfgCompiledWithoutTest(reg.attr));
      if (mm[1] === "ui_wgpu") { aliasUse[isTest ? "test" : "normal"]++; }
      (isTest ? needTest : needNormal).set(mm[1]!, rel(abs));
    }
  }
  const missing: string[] = [];
  for (const [id, f] of needNormal) if (!normalKeys.has(id)) missing.push(`production use of ${id} without a normal dependency (${f.split("/").slice(-2).join("/")})`);
  for (const [id, f] of needTest) if (!normalKeys.has(id) && !devKeys.has(id)) missing.push(`test use of ${id} without a dependency (${f.split("/").slice(-2).join("/")})`);
  const unusedUi = [...normalKeys, ...devKeys].filter(k => (k === "ui_wgpu" || k.startsWith("semio_framework_ui_")) && !needNormal.has(k) && !needTest.has(k));
  problems += missing.length;
  out.push({crate: node.name, manifest: m, aliasUse, normalUiKeys: [...normalKeys].filter(k => k === "ui_wgpu" || k.startsWith("semio_framework_ui_")), devUiKeys: [...devKeys].filter(k => k === "ui_wgpu" || k.startsWith("semio_framework_ui_")), missing, unusedUi});
  if (missing.length) console.log(`MISSING ${node.name.replace("semio-framework-os-renderer-", "")}: ${missing.join("; ")}`);
}
writeFileSync(process.argv[3]!, JSON.stringify(out, null, 1));
console.log(problems ? `${problems} MISSING declaration(s)` : "all ui identifiers declared", out.length, "crates");
