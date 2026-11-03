import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("fix-500.ts", "utf8");
const rep = (a: string, b: string) => { if (!t.includes(a)) throw new Error("missing " + a); t = t.replace(a, b); };
rep('.filter((s: any) => keep(s.id) && s.role !== "glazing" && s.role !== "texture")', '.filter((s: any) => keep(s.id) && s.role !== "texture")');
rep('        if (o.role === "detail") o.role = "body";', '        if (o.role === "detail") o.role = "body";\n        if (/^guide-/.test(o.id) && o.kind === "rect") {\n          const edge = o.x > 0 ? o.x + o.w : o.x;\n          o.w = 0.06;\n          o.x = o.x > 0 ? edge - 0.06 : edge;\n        }');
rep('pens: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 }', 'pens: { body: 0.13, detail: null, texture: null, glazing: 0.1, ground: 0.18 }');
rep('if (o.id === "feet") o.h = 0.1;', 'if (o.id === "feet") o.h = 0.12;');
writeFileSync("fix-500.ts", t);
