import { readFileSync } from "node:fs";
import { ROOT } from "./patch.ts";
const t = readFileSync(`${ROOT}/🛋️furniture/bookshelf/🔣️.json`, "utf8");
const reps = t.indexOf('"representations"');
const s = t.slice(reps);
console.log(/\n( *)"500": \{/.test(s), /\{\s*"status": "authored"/.test(s), /\n( *)"500": \{\s*"status"/.test(s), /\n( *)"500": \{\s*"status": "authored"/.test(s));
