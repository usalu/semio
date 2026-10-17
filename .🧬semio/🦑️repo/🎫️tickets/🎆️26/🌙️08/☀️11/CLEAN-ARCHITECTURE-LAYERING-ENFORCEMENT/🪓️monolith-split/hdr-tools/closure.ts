// 🔭️ Per-crate source closure lister: prints every file of the lib and test closure (canonical reader, default features) with bytes and active lines.
import {readFileSync, existsSync} from "node:fs";
import {dirname, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const readFile = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const featureSet = (parsed: any): Set<string> => {
  const table: Record<string, string[]> = parsed.features ?? {};
  const out = new Set<string>(["default"]), queue = [...(table.default ?? [])];
  while (queue.length) {
    const token = queue.pop()!;
    if (out.has(token)) continue;
    out.add(token);
    if (token.startsWith("dep:")) continue;
    const [head, rest] = token.split("/") as [string, string | undefined];
    const name = head.replace(/\?$/u, "");
    if (table[name]) queue.push(...table[name]);
    if (rest !== undefined && !head.endsWith("?")) out.add(name);
  }
  return out;
};
const mode = process.argv[2] ?? "both";
for (const arg of process.argv.slice(3)) {
  const manifest = resolve(ROOT, arg);
  const dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const feats = [...featureSet(parsed)].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f);
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  console.log(`## ${parsed.package.name}`);
  const itests: string[] = [];
  for (const t of parsed.test ?? []) itests.push(resolve(dir, t.path ?? `tests/${t.name}.rs`));
  if (parsed.autotests !== false) {
    const td = resolve(dir, "tests");
    if (existsSync(td)) for (const row of (await import("node:fs")).readdirSync(td, {withFileTypes: true})) {
      if (row.isFile() && row.name.endsWith(".rs")) itests.push(resolve(td, row.name));
      else if (row.isDirectory() && existsSync(resolve(td, row.name, "main.rs"))) itests.push(resolve(td, row.name, "main.rs"));
    }
  }
  const ratioOf = (inputs: any[]) => Math.max(inputs.length / 32, inputs.reduce((s: number, i: any) => s + i.bytes, 0) / 262144, inputs.reduce((s: number, i: any) => s + i.activeLines, 0) / 4000);
  for (const test of mode === "both" ? [false, true] : [mode === "test"]) {
    const cfg = [...hostCfg, "debug_assertions", ...(test ? ["test"] : []), ...feats];
    const entries = test ? [libEntry, ...itests.filter(e => existsSync(e))] : [libEntry];
    let best: any[] | null = null; let bestName = "";
    for (const entry of entries) {
      let got: any[];
      try { got = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined}); } catch (e) { console.log("# ERROR " + String((e as Error).message).slice(0, 160)); continue; }
      if (!best || ratioOf(got) > ratioOf(best)) { best = got; bestName = relative(dir, entry); }
    }
    if (!best) continue;
    const inputs = best;
    console.log(`# ${test ? "test" : "lib"}: ${inputs.length} files ${inputs.reduce((s: number, i: any) => s + i.bytes, 0)} bytes ${inputs.reduce((s: number, i: any) => s + i.activeLines, 0)} lines${test && bestName !== relative(dir, libEntry) ? " (target " + bestName + ")" : ""}`);
    for (const i of inputs as any[]) console.log(`${i.kind}\t${i.bytes}\t${i.activeLines}\t${relative(dir, resolve(ROOT, i.path ?? i.file ?? i.name ?? ""))}`);
  }
}
