import { load, save } from "../humans/lib.ts";
const doc = load("skateboarder"); const a = doc.asset;
for (const k of ["50", "100"]) { const f = a.representations[k].geometry.front; a.representations[k].geometry.front = f.filter((s: any) => !/^wheel-rear-/.test(s.id)); }
save("skateboarder", doc);
