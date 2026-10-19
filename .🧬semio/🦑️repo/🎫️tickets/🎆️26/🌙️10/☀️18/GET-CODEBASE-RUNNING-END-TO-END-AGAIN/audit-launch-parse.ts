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
for (const f of process.argv.slice(2)) {
  const raw = readFileSync(f, "utf8");
  try {
    const j = JSON.parse(strip(raw));
    const confs = j.configurations ?? [];
    const names = confs.map((c: any) => c.name);
    const dup = names.filter((n: string, i: number) => names.indexOf(n) !== i);
    console.log(JSON.stringify({ file: f, parsed: true, configurations: confs.length, duplicateNames: [...new Set(dup)].length, dupSample: [...new Set(dup)].slice(0, 10), topKeys: Object.keys(j) }));
  } catch (e: any) { console.log(JSON.stringify({ file: f, parsed: false, error: String(e.message).slice(0, 300) })); }
}
