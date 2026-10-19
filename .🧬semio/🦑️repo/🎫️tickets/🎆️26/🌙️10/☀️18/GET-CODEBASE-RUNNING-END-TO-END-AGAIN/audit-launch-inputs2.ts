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
const L = JSON.parse(strip(readFileSync(".vscode/launch.json", "utf8")));
for (const id of process.argv.slice(2)) {
  const i = L.inputs.find((x: any) => x.id === id);
  console.log(id, JSON.stringify(i ? { type: i.type, n: (i.options ?? []).length, opts: (i.options ?? []).slice(0, 8), default: i.default, desc: String(i.description ?? "").slice(0, 140) } : null));
}
