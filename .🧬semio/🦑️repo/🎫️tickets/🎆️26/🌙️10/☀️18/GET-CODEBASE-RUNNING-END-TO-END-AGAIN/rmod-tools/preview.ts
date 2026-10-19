import { readFileSync, writeFileSync } from "node:fs";
import { transform } from "./crates.ts";
import { root, C } from "./lib.ts";
const f = process.argv[2]!;
const p = f.startsWith("/") ? f : root + "/" + f;
const t = readFileSync(p, "utf8");
const r = transform(p, t);
writeFileSync(C + "/🗑️generated/rmod-preview.out", r ?? t);
console.log(r === null ? "untouched" : r === "MANUAL" ? "manual" : "ok");
