// 🌳️ TS module-tree view before/after the hubs: the original root snapshot is placed beside the live root under a temporary name, listed, compared, and removed again.
import {listModuleTree, readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import {copyFileSync, rmSync} from "node:fs";
const PLUG = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const SNAP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/orig/plugin-root.rs";
const tmp = PLUG + "🦀️.pwp0-orig-tmp.rs";
copyFileSync(SNAP, tmp);
try {
  for (const tests of [false, true]) {
    const a = listModuleTree(PLUG + "🦀️.rs", {tests}).map((p: string) => p.slice(PLUG.length)), b = listModuleTree(tmp, {tests}).map((p: string) => p.slice(PLUG.length));
    const sa = new Set(a), sb = new Set(b);
    const onlyA = a.filter((x: string) => !sb.has(x)), onlyB = b.filter((x: string) => !sa.has(x));
    const cnt = (t: string) => { const m = new Map<string, number>(); for (const l of t.split("\n")) if (l.trim()) m.set(l, (m.get(l) ?? 0) + 1); return m; };
    const ca = cnt(readModuleTreeSource(PLUG + "🦀️.rs", {tests})), cb = cnt(readModuleTreeSource(tmp, {tests}));
    let extra = 0, lost = 0; const lostL: string[] = [];
    for (const [k, v] of ca) { const d = v - (cb.get(k) ?? 0); if (d > 0) extra += d; }
    for (const [k, v] of cb) { const d = v - (ca.get(k) ?? 0); if (d > 0) { lost += d; lostL.push(k); } }
    console.log(`tests=${tests}: files live=${a.length} original=${b.length} onlyLive=${JSON.stringify(onlyA.map((x: string) => x.slice(0, 40)))} onlyOriginal=${onlyB.length}; text lines extra=${extra} lost=${lost}`);
    for (const l of lostL.slice(0, 5)) console.log("   LOST", l.slice(0, 120));
  }
} finally { rmSync(tmp, {force: true}); }
