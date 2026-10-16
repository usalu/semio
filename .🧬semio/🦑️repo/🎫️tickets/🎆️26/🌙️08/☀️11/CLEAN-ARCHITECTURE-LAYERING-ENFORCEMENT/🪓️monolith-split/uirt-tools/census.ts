// 🧮️ Source-closure rows of the ui-runtime crates for the lib target and the unit-test selection (cfg test, testkit), via the canonical counter.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const U = ROOT + "/🧰️framework/🔨️modules/🖱️ui/🧠️runtime";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const host = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const wasm = ["target_arch=wasm32", "target_os=unknown", "target_env=", "target_pointer_width=32"];
const wasi = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32"];
const read = async (p: string) => { try { return readFileSync(p); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return null; throw e; } };
const crates: [string, string, string][] = [
  ["semio-framework-ui-runtime", U + "/📦️packages/🦀️rust/🦀️.rs", U + "/📦️packages/🦀️rust"],
  ["semio-framework-ui-runtime-exchange", U + "/📡️exchange/🦀️.rs", U + "/📡️exchange/📦️packages/🦀️rust"],
  ["semio-framework-ui-runtime-presentation", U + "/🖼️presentation/🦀️.rs", U + "/🖼️presentation/📦️packages/🦀️rust"],
  ["semio-framework-ui-runtime-reconcile-primitives", U + "/🧱️primitives/🦀️.rs", U + "/🧱️primitives/📦️packages/🦀️rust"],
  ["semio-framework-ui-runtime-reconcile-cursor", U + "/⏭️cursor/🦀️.rs", U + "/⏭️cursor/📦️packages/🦀️rust"],
  ["semio-framework-ui-runtime-reconcile", U + "/♻️reconcile/🦀️.rs", U + "/♻️reconcile/📦️packages/🦀️rust"],
];
const cfgs: [string, string[]][] = [["lib", []], ["wasm", wasm], ["wasi", wasi], ["test", ["test", "feature=testkit"]]];
for (const [name, entry, dir] of crates) {
  for (const [label, extra] of cfgs) {
    const base = label === "wasm" || label === "wasi" ? [] : host;
    try {
      const inputs = await readRustSourceClosureV1([entry], ROOT, [...base, ...(label === "lib" ? ["debug_assertions"] : ["debug_assertions"]), ...extra], read, {manifestDirectory: dir, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0), emb = inputs.filter(i => i.kind === "embedded");
      console.log([name.replace("semio-framework-", ""), label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, emb.reduce((s, i) => s + i.bytes, 0), (inputs.length > 32 || bytes > 262144 || lines > 4000) ? "OVER" : (inputs.length > 28 || bytes > 230000 || lines > 3500) ? "above-target" : "ok"].join("\t"));
    } catch (e) { console.log([name, label, "ERROR", String((e as Error).message).slice(0, 200)].join("\t")); }
  }
}
