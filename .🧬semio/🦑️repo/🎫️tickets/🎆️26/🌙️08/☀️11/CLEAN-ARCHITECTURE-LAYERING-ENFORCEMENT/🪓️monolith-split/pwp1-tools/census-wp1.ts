// 🧮️ Source-closure rows (canonical counter) of the WP1 crates of the plugin partition: lib, lib + testing features, test; native, wasm32-wasip2, wasm32-unknown-unknown.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const crates: Record<string, {root: string; pkg: string; cls: string}> = JSON.parse(readFileSync(process.env.CRATES!, "utf8"));
const only = process.env.ONLY ? new Set(process.env.ONLY.split(",")) : undefined;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const feats = ["artifact-app-testing", "media-turn-testing", "reserved-producer-testing", "original-actor-context-testing"].map(f => "feature=" + f);
const targets: [string, string | undefined][] = [["native", undefined], ["wasip2", "wasm32-wasip2"], ["wasm", "wasm32-unknown-unknown"]];
const variants: [string, string[]][] = [["lib", []], ["lib+features", feats], ["test", ["test"]], ["test+features", ["test", ...feats]]];
const rows: string[] = ["crate\ttarget\tvariant\tfiles\tbytes\tlines\tembedded\tverdict"];
for (const [name, c] of Object.entries(crates)) {
  if (only && !only.has(name)) continue;
  const entry = ROOT + "/" + c.root + (c.cls === "A" ? "/🦀️.rs" : "/🦀️.rs");
  const pkg = ROOT + "/" + c.pkg;
  for (const [tname, target] of targets) {
    for (const [vname, extra] of variants) {
      try {
        const inputs = await readRustSourceClosureV1([entry], ROOT, [...cfgOf(target), "debug_assertions", ...extra], read, {manifestDirectory: pkg, generated: () => undefined});
        const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
        const over = inputs.length > 32 || bytes > 262144 || lines > 4000 ? "OVER-GATE" : inputs.length > 28 || bytes > 230000 || lines > 3500 ? "over-target" : "ok";
        rows.push([name, tname, vname, inputs.length, bytes, lines, emb.length, over].join("\t"));
      } catch (e) { rows.push([name, tname, vname, "ERROR", String((e as Error).message).slice(0, 160).replaceAll("\n", " ")].join("\t")); }
    }
  }
}
console.log(rows.join("\n"));
