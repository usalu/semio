import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("fix-symbols.ts", "utf8");
t = t.replace('import { load, save, shapes, sign, poly, rect, text, clone } from "./lib.ts";', 'import { load, save, shapes as rawShapes, sign, poly, rect, text, clone, worldOf } from "./lib.ts";\nlet CURRENT: any = null;\nconst shapes = (a: any, scale: number, view: string): any[] => rawShapes(a, scale, view);\nconst W = (a: any, view: string, list: any[]): any[] => worldOf(a, view, list);');
t = t.replace(/sign\(shapes\(a, 200, "(elevation|front|plan)"\)(, \{[^}]*\})?\)/g, (m, v, o) => `W(a, "${v}", ${m})`);
t = t.replace(/\.\.\.sign\(shapes\(a, 200, "(elevation|front|plan)"\)(, \{[^}]*\})?\)/g, (m, v, o) => `...W(a, "${v}", ${m.slice(3)})`);
t = t.replace("s.geometry.front = sign(front200.map((x: any) => clone(x)));", 's.geometry.front = W(a, "front", sign(front200.map((x: any) => clone(x))));');
writeFileSync("fix-symbols.ts", t);
