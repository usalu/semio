import { build } from "./b3-registry.mjs";
import { entries } from "./lib.mjs";
import { syntaxErrors } from "./check-docs.mjs";
build();
for (const [p, v] of entries()) { let e=[]; if (p.endsWith(".json")) { try { JSON.parse(v.after);} catch(x){e=[String(x)]} } else e = syntaxErrors(p, v.after); console.log(p.split("/").slice(-2).join("/"), v.before.length, v.after.length, e.join(";")); }
