import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { apply } from "./members.ts";
const doc = JSON.parse(readFileSync(fileOf("tower-crane"), "utf8")); apply(doc.asset);
writeFileSync(process.argv[2], JSON.stringify(doc, null, 1));
