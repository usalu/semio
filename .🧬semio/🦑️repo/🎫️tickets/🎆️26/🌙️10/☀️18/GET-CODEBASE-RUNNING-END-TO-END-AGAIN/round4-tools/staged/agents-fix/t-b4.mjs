import { build } from "./b4-pointers.mjs";
import { entries } from "./lib.mjs";
import { syntaxErrors } from "./check-docs.mjs";
build();
for (const [p, v] of entries()) { let e=[]; if (p.endsWith(".json")) { try { JSON.parse(v.after);} catch(x){e=[String(x)]} } else e = syntaxErrors(p, v.after); console.log(p.split("/").slice(-3).join("/"), v.before.length, v.after.length, e.join(";")); }
