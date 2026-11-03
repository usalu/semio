import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("fix-500.ts", "utf8");
const a = t.indexOf('      if (["side-l", "side-r", "hot-out"]');
const b = t.indexOf("      return o;", a);
const repl = `      if (["side-l", "side-r", "hot-out"].includes(o.id) && o.kind === "rect") {
        const S = 0.11;
        const cx = o.x + o.w / 2;
        const cy = o.y + o.h / 2;
        const lateral = view === "plan" || (view === "front" && o.id === "side-r");
        o.x = o.id === "side-l" && view !== "front" ? o.x : o.id === "side-r" && view !== "front" ? o.x + o.w - S : cx - S / 2;
        o.y = o.id === "hot-out" && view !== "plan" ? o.y + o.h - S : cy - S / 2;
        void lateral;
        o.w = S;
        o.h = S;
      }
`;
t = t.slice(0, a) + repl + t.slice(b);
const c = t.indexOf("  if (out.plan.length) {");
const d = t.indexOf("  a.representations[\"500\"]", c);
t = t.slice(0, c) + t.slice(d);
t = t.replace("        void lateral;\n", "").replace('        const lateral = view === "plan" || (view === "front" && o.id === "side-r");\n', "");
writeFileSync("fix-500.ts", t);
