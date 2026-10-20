import fs from "node:fs";
import { stripDebugTs } from "./strip-debug-ts.mjs";
const p = process.argv[2];
const t = fs.readFileSync("/Users/ueli/Documents/semio/" + p, "utf8");
const o = stripDebugTs(t, p, []);
fs.writeFileSync("/private/tmp/agents-fix-o.txt", o);
