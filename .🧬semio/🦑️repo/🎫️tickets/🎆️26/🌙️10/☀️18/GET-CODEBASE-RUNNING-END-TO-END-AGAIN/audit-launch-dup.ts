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
const name = "🧱️artifact-io-⚖️installed-service🧰️os🟦️";
const hits = L.configurations.filter((c: any) => c.name === name);
for (const h of hits) console.log(JSON.stringify({ command: String(h.command).slice(0, 220), presentation: h.presentation, cwd: h.cwd }));
const S = JSON.parse(strip(readFileSync(".vscode/🧩️launch.seed.jsonc", "utf8")));
const sh = S.configurations.filter((c: any) => c && c.name === name);
for (const h of sh) console.log("SEED", JSON.stringify({ command: String(h.command).slice(0, 220), presentation: h.presentation }));
