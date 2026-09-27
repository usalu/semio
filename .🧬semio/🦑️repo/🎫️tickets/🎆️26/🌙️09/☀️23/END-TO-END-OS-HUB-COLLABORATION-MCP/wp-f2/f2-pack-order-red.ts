import { readFileSync } from "node:fs";
const f = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/🔤️pack-key-order/🔣️.json", "utf8"));
const oracle = (a: string, b: string) => Math.sign(Buffer.compare(Buffer.from(a), Buffer.from(b)));
const naive = (a: string, b: string) => (a < b ? -1 : a > b ? 1 : 0);
const unitsNoFallback = (a: string, b: string) => { const n = Math.min(a.length, b.length); for (let i = 0; i < n; i++) { const x = a.charCodeAt(i), y = b.charCodeAt(i); if (x !== y) return Math.sign(x - y); } return Math.sign(a.length - b.length); };
console.log("naive UTF-16 disagreements:", f.pairs.filter((p: any) => naive(p.left, p.right) !== oracle(p.left, p.right)).length, "of", f.pairs.length);
console.log("units without surrogate fallback:", f.pairs.filter((p: any) => unitsNoFallback(p.left, p.right) !== oracle(p.left, p.right)).length);
