import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const manifests = ["🌎️hub", "🎓️teaching", "🏢️semio-tech", "♻️mit-bestand", "👴️leutwiler"].flatMap((top) => Bun.spawnSync(["find", top, "(", "-name", "node_modules", "-o", "-name", "target", "-o", "-name", "dist", "-o", "-name", "🗑️generated", ")", "-prune", "-o", "(", "-name", "📋️project.json", "-o", "-name", "project.json", ")", "-print"], { cwd: root }).stdout.toString().trim().split("\n").filter(Boolean));
const walkPattern = /readdirSync|readdir\(|new Glob|globSync|\bglob\(|walk\w*\(|scan\w*\(|ls-files|\bfind\b.*-name|opendir|Bun\.Glob/;
const importPattern = /(?:from\s+|import\(\s*)["'](\.[^"']+)["']/g;
const closure = (entry: string): string[] => {
  const seen = new Set<string>();
  const stack = [entry];
  while (stack.length) {
    const file = stack.pop()!;
    if (seen.has(file) || !existsSync(file)) continue;
    seen.add(file);
    const text = readFileSync(file, "utf8");
    for (const m of text.matchAll(importPattern)) {
      const target = resolve(dirname(file), m[1]!);
      if (target.includes("/🧰️framework/")) continue;
      stack.push(target);
    }
  }
  return [...seen];
};
const rows: string[] = [];
for (const rel of manifests) {
  const file = join(root, rel);
  const project = JSON.parse(readFileSync(file, "utf8"));
  const projectDir = dirname(file);
  for (const [name, raw] of Object.entries<any>(project.targets ?? {})) {
    if (raw.cache === false || raw.continuous) continue;
    if (!/check|wiring|^verify|-verify|^test-.*(source|native)$/.test(name)) continue;
    const command: string = raw.options?.command ?? "";
    const m = /^bun\s+(\S*📜️script\.ts)/u.exec(command);
    if (!m) continue;
    const cwdOpt: string = raw.options?.cwd ?? "{projectRoot}";
    const cwd = cwdOpt.startsWith("{projectRoot}") ? projectDir : resolve(root, cwdOpt);
    const script = resolve(cwd, m[1]!);
    const files = closure(script);
    const hits: string[] = [];
    for (const f of files) {
      const lines = readFileSync(f, "utf8").split("\n");
      lines.forEach((line, i) => { if (walkPattern.test(line)) hits.push(`${f.replace(root + "/", "")}:${i + 1}`); });
    }
    if (!hits.length) continue;
    rows.push(JSON.stringify({ manifest: rel, target: name, cache: raw.cache ?? "absent", inputs: raw.inputs ?? "(default)", defaultInput: project.namedInputs?.default ?? "(workspace)", files: files.length, hits: hits.slice(0, 6), hitCount: hits.length }));
  }
}
console.log(rows.join("\n"));
console.error(`flagged=${rows.length}`);
