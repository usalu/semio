// 🌳️ The TS module-tree view of the renderer root before (baseline sandbox) and after the mount hubs: file sets and concatenated text.
import {listModuleTree, readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const ENG = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const A = "/Users/ueli/Documents/semio/" + ENG, B = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/rwp0-ts-B/" + ENG;
const roots = ["🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs", "🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/🦀️.rs", "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"];
for (const r of roots) for (const tests of [false, true]) {
  const a = listModuleTree(A + "/" + r, {tests}).map(p => p.slice(A.length + 1)), b = listModuleTree(B + "/" + r, {tests}).map(p => p.slice(B.length + 1));
  const sa = new Set(a), sb = new Set(b);
  const onlyA = a.filter(x => !sb.has(x)), onlyB = b.filter(x => !sa.has(x));
  const ta = readModuleTreeSource(A + "/" + r, {tests}).split("\n"), tb = readModuleTreeSource(B + "/" + r, {tests}).split("\n");
  const count = (l: string[]) => { const m = new Map<string, number>(); for (const x of l) if (x.trim()) m.set(x, (m.get(x) ?? 0) + 1); return m; };
  const ca = count(ta), cb = count(tb);
  let extra = 0, lost = 0;
  const lostLines: string[] = [], extraLines: string[] = [];
  for (const [k, v] of ca) { const d = v - (cb.get(k) ?? 0); if (d > 0) { extra += d; extraLines.push(k); } }
  for (const [k, v] of cb) { const d = v - (ca.get(k) ?? 0); if (d > 0) { lost += d; lostLines.push(k); } }
  console.log(`${r.slice(-60)} tests=${tests} files A=${a.length} B=${b.length} onlyA=${JSON.stringify(onlyA.map(x => x.slice(-70)))} onlyB=${onlyB.length} textLines extra=${extra} lost=${lost}`);
  if (process.env.SHOW) { for (const l of lostLines.slice(0, 12)) console.log("   LOST", l.slice(0, 140)); for (const l of extraLines.slice(0, 12)) console.log("   EXTRA", l.slice(0, 140)); }
}
