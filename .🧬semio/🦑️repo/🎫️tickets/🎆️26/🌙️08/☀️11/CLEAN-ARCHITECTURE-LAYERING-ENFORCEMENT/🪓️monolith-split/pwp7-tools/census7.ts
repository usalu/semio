import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, relative, resolve, normalize} from "node:path";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const PLUG = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const PKG = PLUG + "/📦️packages/🦀️rust";
const GEN = process.env.GEN!;
const map = JSON.parse(readFileSync(GEN + "/pwp7-crates7.json", "utf8"));
const OWN: Record<string, string[]> = map.own;
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
const feats = ["artifact-app-testing", "media-turn-testing", "reserved-producer-testing", "original-actor-context-testing"].map(f => "feature=" + f);
const ROWS: [string, string | undefined, string[]][] = [
  ["lib native", undefined, []], ["lib wasm32-wasip2", "wasm32-wasip2", []], ["lib wasm32-unknown-unknown", "wasm32-unknown-unknown", []],
  ["lib native features", undefined, feats], ["test native", undefined, ["test"]], ["test wasm32-wasip2", "wasm32-wasip2", ["test"]], ["test native features", undefined, ["test", ...feats]],
];
const ownerOf = (rel: string): string | null => {
  let best: [string, number] | null = null;
  for (const [c, ps] of Object.entries(OWN)) for (const p of ps) if ((p.endsWith("/") ? rel.startsWith(p) : rel === p) && (!best || p.length > best[1])) best = [c, p.length];
  return best ? best[0] : null;
};
const only = process.env.ONLY ? process.env.ONLY.split(",") : Object.keys(map.entry);
const rowsOnly = process.env.ROWS ? process.env.ROWS.split(",") : null;
const GROUP = /((?:[ \t]*(?:#\[[^\]]*\]|\/\/\/[^\n]*)[ \t]*\n)*)([ \t]*)((?:pub(?:\([^)]*\))?\s+)?mod\s+\w+;|include!\s*\([^)]*\)\s*;)/g;
for (const crate of only) {
  const [entryRel, cls] = map.entry[crate] as [string, string];
  const entryAbs = PLUG + "/" + entryRel;
  const mounts: string[] = [...(map.mounts[crate] ?? []), ...(map.extra[crate] ?? [])];
  const readFor = (cur: string) => async (p: string): Promise<Uint8Array | null> => {
    let text: string;
    try { text = readFileSync(p, "utf8"); } catch (e) { if ((e as NodeJS.ErrnoException).code === "ENOENT") { if (!p.endsWith(".rs")) return new Uint8Array(0); return null; } throw e; }
    if (!p.endsWith(".rs")) return Buffer.from(text);
    if (p === entryAbs) {
      const extra = mounts.map((m, i) => `#[path = "${relative(dirname(entryAbs), PLUG + "/" + m)}"]\nmod virtual_${i};\n`).join("");
      text = (cls === "A" ? "//! virtual root\n" : text) + "\n" + extra;
    }
    const dir = dirname(p);
    text = text.replace(GROUP, (whole: string, attrs: string, _ind: string, item: string) => {
      const pm = /#\[\s*path\s*=\s*"([^"]*)"\s*\]/.exec(attrs) ?? /include!\s*\(\s*"([^"]*)"\s*\)/.exec(item);
      if (!pm) return whole;
      const target = normalize(resolve(dir, pm[1]));
      if (!target.endsWith(".rs")) return whole;
      const o = ownerOf(relative(PLUG, target));
      return o === cur ? whole : "";
    });
    return Buffer.from(text);
  };
  for (const [label, target, extra] of ROWS) {
    if (rowsOnly && !rowsOnly.includes(label)) continue;
    const cfg = [...cfgOf(target), "debug_assertions", ...extra];
    try {
      const inputs = await readRustSourceClosureV1([entryAbs], ROOT, cfg, readFor(crate), {manifestDirectory: PKG, generated: () => undefined});
      const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
      const emb = inputs.filter(i => i.kind === "embedded").length;
      console.log([crate, label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb].join("\t"));
    } catch (e) { console.log(crate, label, "ERROR", String((e as Error).message).slice(0, 200)); }
  }
}
