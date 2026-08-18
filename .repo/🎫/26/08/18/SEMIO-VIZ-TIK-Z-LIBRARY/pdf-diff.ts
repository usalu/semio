import { readFileSync } from "node:fs";
import { join } from "node:path";

const dir = import.meta.dir;
const a = readFileSync(join(dir, "viz-api-a.pdf")).toString("latin1");
const b = readFileSync(join(dir, "viz-api-b.pdf")).toString("latin1");
const re = /(\d+) 0 obj\n/g;
const objs: { n: string; i: number }[] = [];
for (const match of a.matchAll(re)) objs.push({ n: match[1]!, i: match.index });
console.log("objects", objs.slice(-10));
for (const obj of objs.slice(-8)) {
  console.log(`OBJ ${obj.n} @ ${obj.i}`);
  console.log(JSON.stringify(a.slice(obj.i, obj.i + 240)));
}
console.log("A@15280", JSON.stringify(a.slice(15200, 15400)));
console.log("B@15280", JSON.stringify(b.slice(15200, 15400)));
const startxref = a.lastIndexOf("startxref");
console.log("tail A", JSON.stringify(a.slice(startxref - 80)));
console.log("tail B", JSON.stringify(b.slice(b.lastIndexOf("startxref") - 80)));
