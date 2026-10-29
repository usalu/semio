import { readFileSync, existsSync, readdirSync, statSync } from "node:fs";
import { join, dirname, resolve } from "node:path";
const { readRustSourceClosureV1 } = await import(process.argv[2]);
const ws = "/Users/ueli/Documents/semio";
const walk = d => readdirSync(d).flatMap(n => { const p = join(d, n); if (["target","node_modules",".git"].includes(n)) return []; let s; try{s=statSync(p)}catch{return[]} return s.isDirectory() ? walk(p) : n === "Cargo.toml" ? [p] : []; });
for (const root of process.argv.slice(3)) for (const m of walk(root)) {
  const t = readFileSync(m, "utf8"); if (!t.includes("rust-source-boundary") || !/^\[lib\]/m.test(t)) continue;
  const num = k => +(new RegExp("^" + k + "\\s*=\\s*(\\d+)", "m").exec(t)?.[1] ?? 0);
  const lib = /\[lib\][^\[]*?path\s*=\s*"([^"]+)"/s.exec(t)?.[1]; if (!lib) continue;
  const entry = resolve(dirname(m), lib); if (!existsSync(entry)) continue;
  try {
    const r = await readRustSourceClosureV1([entry], ws, [], async p => existsSync(p) ? new Uint8Array(readFileSync(p)) : null, { manifestDirectory: dirname(m), generated: () => undefined });
    const b = r.reduce((s, x) => s + x.bytes, 0), l = r.reduce((s, x) => s + x.activeLines, 0);
    const lim = { b: num("maxSourceBytes"), f: num("maxSourceFiles"), l: num("maxActiveLines") };
    if (b > lim.b || r.length > lim.f || l > lim.l) console.log("OVER", m.slice(ws.length + 1), `bytes ${b}/${lim.b} files ${r.length}/${lim.f} lines ${l}/${lim.l}`);
  } catch (e) { console.log("ERR", m.slice(ws.length + 1), String(e.message).slice(0, 120)); }
}
console.log("done");
