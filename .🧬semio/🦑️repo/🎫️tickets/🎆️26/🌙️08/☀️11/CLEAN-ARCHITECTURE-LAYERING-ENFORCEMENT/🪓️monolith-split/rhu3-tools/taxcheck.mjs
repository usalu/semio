import fs from "node:fs";
const p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const t = JSON.parse(fs.readFileSync(p, "utf8"));
let bad = 0, arrays = 0, missing = 0;
const R = "/Users/ueli/Documents/semio/";
const walk = (o, path) => { if (Array.isArray(o)) { if (o.length && o.every(x => typeof x === "string")) { const key = path.at(-1); if (/sourceModulePaths|inputPatterns/.test(key)) { arrays++; for (let i = 1; i < o.length; i++) if (!(o[i - 1] < o[i])) { bad++; console.log("NOT SORTED/UNIQUE", path.join("."), o[i - 1].slice(-70), "|", o[i].slice(-70)); break; } if (key === "sourceModulePaths") for (const x of o) if (!fs.existsSync(R + x)) { missing++; console.log("MISSING FILE", x.slice(-100)); } } } else o.forEach((x, i) => walk(x, [...path, i])); } else if (o && typeof o === "object") for (const [k, v] of Object.entries(o)) walk(v, [...path, k]); };
walk(t, []);
console.log({ arrays, bad, missing });
