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
const inputs = j.inputs ?? [];
const kinds: Record<string, number> = {};
for (const i of inputs) kinds[i.type] = (kinds[i.type] ?? 0) + 1;
console.log("inputTypes", JSON.stringify(kinds));
for (const i of inputs.slice(0, 3)) console.log(JSON.stringify(i).slice(0, 400));
const names = inputs.map((i: any) => i.id);
console.log("inputIds", names.length, JSON.stringify(names.slice(0, 80)));
const cmds = (j.configurations ?? []).map((c: any) => c.command ?? "");
const forms: Record<string, number> = {};
for (const c of cmds) { const k = c.replace(/"[^"]*"/g, '"…"').split(/\s+/).slice(0, 4).join(" ").slice(0, 70); forms[k] = (forms[k] ?? 0) + 1; }
console.log("commandForms", JSON.stringify(Object.entries(forms).sort((a, b) => b[1] - a[1]).slice(0, 40), null, 0));
