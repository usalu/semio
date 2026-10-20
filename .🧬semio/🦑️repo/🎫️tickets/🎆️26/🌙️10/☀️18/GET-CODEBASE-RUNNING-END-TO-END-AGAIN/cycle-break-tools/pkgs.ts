import { loadPackages } from "../exports-stage-tools/lib.ts";
const m = loadPackages();
const q = process.argv.slice(2);
for (const p of m.all) if (p.name && q.some(s => p.name.includes(s))) console.log(p.name, p.dir, Object.keys(p.exports).length);
