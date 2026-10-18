import { readStoreSourceText } from "@semio-tech/framework/test/source-text";
const t = readStoreSourceText();
for (const n of process.argv.slice(2)) {
  let from = 0, k = 0;
  while (k < 6) { const s = t.indexOf(n, from); if (s < 0) break; console.log("FOUND", JSON.stringify(n.slice(0, 70)), s, JSON.stringify(t.slice(Math.max(0, s - 60), s).split("\n").pop())); from = s + 1; k++; }
  if (k === 0) console.log("MISSING", JSON.stringify(n.slice(0, 90)));
}
