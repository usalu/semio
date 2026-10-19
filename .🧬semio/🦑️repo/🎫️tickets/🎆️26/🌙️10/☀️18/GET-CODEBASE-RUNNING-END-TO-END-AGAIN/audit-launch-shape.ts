import { readFileSync } from "node:fs";
const strip = (s: string) => {
  let out = "", i = 0, inStr = false;
  while (i < s.length) {
    const c = s[i];
    if (inStr) { out += c; if (c === "\\") { out += s[i + 1]; i += 2; continue; } if (c === '"') inStr = false; i++; continue; }
    if (c === '"') { inStr = true; out += c; i++; continue; }
    if (c === "/" && s[i + 1] === "/") { while (i < s.length && s[i] !== "\n") i++; continue; }
    if (c === "/" && s[i + 1] === "*") { i += 2; while (i < s.length && !(s[i] === "*" && s[i + 1] === "/")) i++; i += 2; continue; }
    out += c; i++;
  }
  return out.replace(/,(\s*[}\]])/g, "$1");
};
const j = JSON.parse(strip(readFileSync(process.argv[2], "utf8")));
const summ = (v: any) => Array.isArray(v) ? `array(${v.length})` : v && typeof v === "object" ? `object(${Object.keys(v).length}: ${Object.keys(v).slice(0, 12).join(",")})` : typeof v;
for (const k of Object.keys(j)) console.log(k, summ(j[k]));
const cfg = j.configurations ?? [];
const keyCount: Record<string, number> = {};
for (const c of cfg) for (const k of Object.keys(c)) keyCount[k] = (keyCount[k] ?? 0) + 1;
console.log("configKeys", JSON.stringify(keyCount));
const nullName = cfg.filter((c: any) => !c.name);
console.log("nullNameSample", JSON.stringify(nullName.slice(0, 2)).slice(0, 800));
if (j.devLaunchers) console.log("devLaunchers sample", JSON.stringify(Array.isArray(j.devLaunchers) ? j.devLaunchers.slice(0,2) : Object.entries(j.devLaunchers).slice(0,2)).slice(0, 1200));
if (j.projectLaunchers) console.log("projectLaunchers sample", JSON.stringify(Array.isArray(j.projectLaunchers) ? j.projectLaunchers.slice(0,2) : Object.entries(j.projectLaunchers).slice(0,2)).slice(0, 1200));
console.log("compounds", Array.isArray(j.compounds) ? j.compounds.length : typeof j.compounds, "inputs", Array.isArray(j.inputs) ? j.inputs.length : typeof j.inputs);
