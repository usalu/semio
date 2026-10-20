import { build } from "./b6-legacy.mjs";
import { entries, load } from "./lib.mjs";
import { syntaxErrors } from "./check-docs.mjs";
build();
for (const [p, v] of entries()) { const e = syntaxErrors(p, v.after); console.log(p.split("/").slice(-3).join("/"), v.before.length, v.after.length, e.join(";")); }
