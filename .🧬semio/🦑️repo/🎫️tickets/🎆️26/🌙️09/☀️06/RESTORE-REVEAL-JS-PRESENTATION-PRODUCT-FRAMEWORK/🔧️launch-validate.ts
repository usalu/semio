import { readFileSync } from "node:fs";
const strip = (text: string): string => {
  let out = "";
  let inString = false;
  let escaped = false;
  for (let i = 0; i < text.length; i += 1) {
    const c = text[i]!;
    if (inString) {
      out += c;
      if (escaped) escaped = false;
      else if (c === "\\") escaped = true;
      else if (c === '"') inString = false;
      continue;
    }
    if (c === '"') { inString = true; out += c; continue; }
    if (c === "/" && text[i + 1] === "/") { while (i < text.length && text[i] !== "\n") i += 1; out += "\n"; continue; }
    if (c === "/" && text[i + 1] === "*") { i += 2; while (i < text.length && !(text[i] === "*" && text[i + 1] === "/")) i += 1; i += 1; continue; }
    out += c;
  }
  return out.replace(/,(\s*[}\]])/g, "$1");
};
for (const path of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
  const parsed = JSON.parse(strip(readFileSync(path, "utf8"))) as { configurations: { name: string; command?: string }[] };
  const names = parsed.configurations.map((c) => c.name);
  const dupes = names.filter((n, i) => n !== undefined && names.indexOf(n) !== i);
  console.log(`  nameless=${names.filter((n) => n === undefined).length}`);
  const ours = names.filter((n) => (n ?? "").includes("presentation") || (n ?? "").includes("projektetage"));
  console.log(`${path}: OK, ${names.length} configurations, duplicates=[${[...new Set(dupes)].join(", ")}]`);
  console.log(`  ours: ${JSON.stringify(ours)}`);
}
const seed = JSON.parse(strip(readFileSync(".vscode/🧩️launch.seed.jsonc", "utf8"))) as { configurations: Record<string, unknown>[] };
console.log("nameless sample:", JSON.stringify(seed.configurations.filter((c) => c.name === undefined).slice(0, 2)));
