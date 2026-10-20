import fs from "node:fs";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { stripDebugRs, rustTokens } from "./strip-debug-rs.mjs";
import { stripDebugTs } from "./strip-debug-ts.mjs";
const R = "/Users/ueli/Documents/semio/";
const files = JSON.parse(fs.readFileSync("debug-appendix.json", "utf8"));
const idsRs = (s) => { const m = new Map(); const add = (w) => m.set(w, (m.get(w) ?? 0) + 1); for (const t of rustTokens(s)) { const x = s.slice(t.s, t.e); if (t.k === "id") add(x); else if (t.k === "str") for (const q of x.matchAll(/\{([A-Za-z_][A-Za-z0-9_]*)(?::[^}]*)?\}/g)) add(q[1]); } return m; };
const idsTs = (s, p) => { const sf = ts.createSourceFile(p, s, ts.ScriptTarget.Latest, true); const m = new Map(); const v = (n) => { if (ts.isIdentifier(n)) m.set(n.text, (m.get(n.text) ?? 0) + 1); ts.forEachChild(n, v); }; v(sf); return m; };
for (const [s, p] of files) {
  const t = fs.readFileSync(R + p, "utf8");
  const rs = p.endsWith(".rs");
  const o = (rs ? stripDebugRs : stripDebugTs)(t, p, []);
  const a = rs ? idsRs(t) : idsTs(t, p), b = rs ? idsRs(o) : idsTs(o, p);
  const sus = [];
  for (const [w, c] of a) { const d = b.get(w) ?? 0; if (d < c && d <= 1) sus.push(w + ":" + c + "->" + d); }
  if (sus.length) console.log(p.split("/").slice(-3).join("/"), sus.join(" "));
}
