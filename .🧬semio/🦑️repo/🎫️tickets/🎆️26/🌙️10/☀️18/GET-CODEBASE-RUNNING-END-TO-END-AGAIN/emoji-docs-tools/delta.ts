import { statSync } from "node:fs";
const out = process.argv[2];
const since = Number(process.argv[3]);
const all = (await Bun.file(`${out}/files-all.txt`).text()).split("\n").filter(Boolean);
const old = new Set((await Bun.file(`${out}/files.txt`).text()).split("\n").filter(Boolean));
const sel: string[] = [];
for (const p of all) {
  let m = 0;
  try { m = statSync(`/Users/ueli/Documents/semio/${p}`).mtimeMs; } catch { continue; }
  if (m > since || !old.has(p)) sel.push(p);
}
await Bun.write(`${out}/delta/files.txt`, sel.join("\n") + "\n");
console.log("delta files", sel.length);
