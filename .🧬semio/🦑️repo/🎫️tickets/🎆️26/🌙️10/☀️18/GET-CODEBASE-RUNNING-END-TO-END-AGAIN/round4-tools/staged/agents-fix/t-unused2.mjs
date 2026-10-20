import { build } from "./b5-debug.mjs";
import { entries } from "./lib.mjs";
import { rustTokens } from "./strip-debug-rs.mjs";
import fs from "node:fs";
build();
const idsOf = (s) => { const m = new Map(); const add = (w) => m.set(w, (m.get(w) ?? 0) + 1); for (const t of rustTokens(s)) { const x = s.slice(t.s, t.e); if (t.k === "id") add(x); else if (t.k === "str") for (const q of x.matchAll(/\{([A-Za-z_][A-Za-z0-9_]*)(?::[^}]*)?\}/g)) add(q[1]); } return m; };
for (const [p, v] of entries()) {
  if (!p.endsWith(".rs")) continue;
  const a = idsOf(v.before), b = idsOf(v.after);
  const out = [];
  for (const [w, c] of a) { const d = b.get(w) ?? 0; if (d < c && d >= 1 && d <= 3 && !["eprintln","println","DEBUG"].includes(w) && /^[a-z_][a-z0-9_]*$/.test(w) && w.length>1) out.push(w + ":" + c + "->" + d); }
  if (out.length) console.log(p.split("/").slice(-3).join("/"), out.join(" "));
}
