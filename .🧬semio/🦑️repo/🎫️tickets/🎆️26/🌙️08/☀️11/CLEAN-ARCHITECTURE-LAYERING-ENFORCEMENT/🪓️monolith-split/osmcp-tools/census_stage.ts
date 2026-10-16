// 🧮️ Census of staged os-mcp crates through the canonical closure counter, reading staged files from a JSON map before the repo is touched.
import {readFileSync,existsSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const [stageFile, cratesFile] = process.argv.slice(2);
const staged: Record<string,string> = JSON.parse(readFileSync(stageFile!, "utf8"));
const crates: {name:string; entry:string; manifestDir:string; features:string[]}[] = JSON.parse(readFileSync(cratesFile!, "utf8"));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { if (p in staged) return new TextEncoder().encode(staged[p]!); try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const out: any[] = [];
for (const c of crates) {
  for (const mode of ["lib", "test"]) {
    const cfg = [...hostCfg, "debug_assertions", ...c.features.map(f => "feature=" + f), ...(mode === "test" ? ["test"] : [])];
    try {
      const inputs = await readRustSourceClosureV1([c.entry], ROOT, cfg, read, {manifestDirectory: c.manifestDir, generated: () => undefined});
      const emb = inputs.filter(i => i.kind === "embedded");
      out.push({name: c.name, mode, files: inputs.length, bytes: inputs.reduce((s, i) => s + i.bytes, 0), lines: inputs.reduce((s, i) => s + i.activeLines, 0), embeddedFiles: emb.length, embeddedBytes: emb.reduce((s, i) => s + i.bytes, 0)});
    } catch (e) { out.push({name: c.name, mode, error: String((e as Error).message).slice(0, 200)}); }
  }
}
for (const r of out) console.log(r.error ? `${r.name} ${r.mode} ERROR ${r.error}` : `${r.name.padEnd(46)} ${r.mode.padEnd(4)} files=${String(r.files).padStart(3)} lines=${String(r.lines).padStart(5)} bytes=${String(r.bytes).padStart(7)} emb=${r.embeddedFiles}/${r.embeddedBytes}`);
