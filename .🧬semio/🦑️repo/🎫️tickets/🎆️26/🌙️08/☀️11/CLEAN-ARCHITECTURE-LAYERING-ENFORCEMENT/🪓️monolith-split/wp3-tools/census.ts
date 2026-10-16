// 🧮️ WP3 staged census: gate-counter closure of every WP3 crate (and lift destination) built from the crate map, with mounts of files other crates own stripped, per cfg row.
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const map: Record<string, string[]> = JSON.parse(readFileSync(import.meta.dir + "/crate-map.json", "utf8"));
const owner = new Map<string, string>();
for (const [crate, files] of Object.entries(map)) for (const f of files) owner.set(resolve(ROOT, f), crate);
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const MOUNT = /((?:[ \t]*\/\/[^\n]*\n)*(?:[ \t]*#\[cfg[^\n]*\]\s*\n)*)([ \t]*#\[path\s*=\s*"([^"]+)"\]\s*\n)([ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;[ \t]*\n(?:[ \t]*(?:pub(?:\([^)]*\))?\s+)?use\s+\w+::\*;[ \t]*\n)?)/gu;
const only = process.env.ONLY?.split(",");
const rows: [string, string | undefined, string[]][] = [["lib", undefined, []], ["lib-wasm", "wasm32-unknown-unknown", []], ["test", undefined, ["test"]], ["test-wasm", "wasm32-unknown-unknown", ["test"]]];
const out: Record<string, unknown>[] = [];
for (const crate of Object.keys(map)) {
  if (only && !only.includes(crate)) continue;
  const members = map[crate].map(f => resolve(ROOT, f));
  for (const [label, target, extra] of rows) {
    const cfg = [...cfgOf(target), "debug_assertions", ...extra];
    const virtual = resolve(ROOT, "virtual-" + crate + ".rs");
    const text = members.map((m, i) => `#[path = "${m}"]\npub mod m${i};\n`).join("");
    const read = async (p: string): Promise<Uint8Array | null> => {
      if (p === virtual) return new TextEncoder().encode(text);
      try {
        let t = readFileSync(p, "utf8");
        if (p.endsWith(".rs")) t = t.replace(MOUNT, (all, pre, attr, path, mod) => { const t2 = resolve(dirname(p), path); const o = owner.get(t2); return o !== undefined && o !== crate ? "" : all; });
        return new TextEncoder().encode(t);
      } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return p.endsWith(".rs") ? null : new Uint8Array(0); throw e; }
    };
    try {
      const inputs = await readRustSourceClosureV1([virtual], ROOT, cfg, read, {manifestDirectory: process.env.PKG ?? ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust", generated: () => undefined});
      const real = inputs.filter(i => (i as any).path !== virtual && !String((i as any).path ?? "").endsWith("virtual-" + crate + ".rs"));
      out.push({crate, row: label, files: real.length, bytes: real.reduce((s, i) => s + i.bytes, 0), lines: real.reduce((s, i) => s + i.activeLines, 0), embedded: real.filter(i => i.kind === "embedded").length});
    } catch (e) { out.push({crate, row: label, error: String((e as Error).message).slice(0, 160)}); }
  }
}
for (const r of out) console.log(JSON.stringify(r));
