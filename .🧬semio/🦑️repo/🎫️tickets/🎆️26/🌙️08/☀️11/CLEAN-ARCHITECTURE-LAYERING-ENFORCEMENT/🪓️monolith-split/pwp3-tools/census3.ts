// 🧮️ Staged census of the plugin crates on the CURRENT tree: for every crate the gate-counter closure of its member files (crate map by directory, lift files to their destination) with every mount of a file another crate owns stripped, per cfg row.
import {readFileSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const input = JSON.parse(readFileSync(import.meta.dir + "/../🗑️generated/" + (process.env.INPUT ?? "pwp3-census-input.json"), "utf8")) as {owner: Record<string, string>; lib: Record<string, string[]>; test: Record<string, string[]>};
const owner = new Map<string, string>(Object.entries(input.owner).map(([p, c]) => [resolve(ROOT, p), c]));
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const MOUNT = /((?:[ \t]*\/\/[^\n]*\n)*(?:[ \t]*#\[cfg[^\n]*\]\s*\n)*)([ \t]*#\[path\s*=\s*"([^"]+)"\]\s*\n)([ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;[ \t]*\n(?:[ \t]*(?:pub(?:\([^)]*\))?\s+)?use\s+\w+::\*;[ \t]*\n)?)/gu;
const CFGS = String.raw`(?:[ \t]*#\[cfg[^\n]*\]\s*\n)*[ \t]*`;
const INLINE = new RegExp("(" + CFGS + String.raw`#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;)`, "gu");
const INCLUDE = new RegExp("(" + CFGS + String.raw`include!\(\s*"([^"]+\.rs)"\s*\);?)`, "gu");
const only = process.env.ONLY?.split(",");
const PLUGDIR = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const SNAPDIR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pwp3-snap/";
const snapOf = (p: string): string | undefined => { if (!p.startsWith(PLUGDIR)) return undefined; const q = SNAPDIR + p.slice(PLUGDIR.length); try { readFileSync(q); return q; } catch { return undefined; } };
const feats = ["artifact-app-testing", "media-turn-testing", "reserved-producer-testing", "original-actor-context-testing"].map(f => "feature=" + f);
const rows: [string, string | undefined, string[], boolean][] = [
  ["lib", undefined, [], false], ["lib+features", undefined, feats, false], ["lib-wasip2", "wasm32-wasip2", [], false], ["lib-wasm", "wasm32-unknown-unknown", [], false],
  ["test", undefined, ["test", ...feats], true], ["test-wasip2", "wasm32-wasip2", ["test"], true],
];
const crates = Object.keys({...input.lib, ...input.test}).filter(c => !c.startsWith("UNMAPPED") && !c.startsWith("TOP")).sort();
const out: Record<string, unknown>[] = [];
for (const crate of crates) {
  if (only && !only.includes(crate)) continue;
  for (const [label, target, extra, withTests] of rows) {
    const members = [...(input.lib[crate] ?? []), ...(withTests ? input.test[crate] ?? [] : [])].map(f => resolve(ROOT, f));
    if (members.length === 0) continue;
    const cfg = [...cfgOf(target), "debug_assertions", ...extra];
    const virtual = resolve(ROOT, "virtual-" + crate + ".rs");
    const text = members.map((m, i) => `#[path = "${m}"]\npub mod m${i};\n`).join("");
    const foreign = (p: string, rel: string) => { const o = owner.get(resolve(dirname(p), rel)); return o !== undefined && o !== crate; };
    const read = async (p: string): Promise<Uint8Array | null> => {
      if (p === virtual) return new TextEncoder().encode(text);
      try {
        let t = readFileSync(process.env.SNAPREAD === "1" && snapOf(p) ? snapOf(p)! : p, "utf8");
        if (p.endsWith(".rs")) {
          t = t.replace(MOUNT, (all, pre, attr, path) => foreign(p, path) ? "" : all);
          t = t.replace(INLINE, (all, whole, path) => foreign(p, path) ? "" : all);
          t = t.replace(INCLUDE, (all, whole, path) => foreign(p, path) ? "" : all);
        }
        return new TextEncoder().encode(t);
      } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") return p.endsWith(".rs") ? null : new Uint8Array(0); throw e; }
    };
    try {
      const inputs = await readRustSourceClosureV1([virtual], ROOT, cfg, read, {manifestDirectory: ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", generated: () => undefined});
      const real = inputs.filter(i => !String((i as any).path ?? "").endsWith("virtual-" + crate + ".rs"));
      out.push({crate, row: label, files: real.length, bytes: real.reduce((s, i) => s + i.bytes, 0), lines: real.reduce((s, i) => s + i.activeLines, 0), embedded: real.filter(i => i.kind === "embedded").length});
    } catch (e) { out.push({crate, row: label, error: String((e as Error).message).slice(0, 200)}); }
  }
}
for (const r of out) console.log(JSON.stringify(r));
