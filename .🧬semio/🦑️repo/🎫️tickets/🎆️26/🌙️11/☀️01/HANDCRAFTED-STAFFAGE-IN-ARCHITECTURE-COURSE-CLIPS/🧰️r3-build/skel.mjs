import { readFileSync } from "node:fs";
const f = process.argv[2];
const depth = Number(process.argv[3] || 3);
const d = JSON.parse(readFileSync(f, "utf8")).asset;
const show = (v, lvl, key) => {
  const pad = "  ".repeat(lvl);
  if (Array.isArray(v)) {
    console.log(`${pad}${key}: [${v.length}]`);
    if (lvl < depth && v.length && typeof v[0] === "object") show(v[0], lvl + 1, "[0]");
  } else if (v && typeof v === "object") {
    console.log(`${pad}${key}:`);
    if (lvl < depth) for (const [k, x] of Object.entries(v)) show(x, lvl + 1, k);
  } else console.log(`${pad}${key}: ${JSON.stringify(v)?.slice(0, 90)}`);
};
show(d, 0, "asset");
