import { readFileSync } from "node:fs";
import { execSync } from "node:child_process";

const files = execSync(
  "find . -name '*component.feature' -not -path './compose/*'",
  { cwd: "/Users/ueli/Documents/semio", encoding: "utf8", maxBuffer: 64 << 20 },
)
  .split("\n")
  .filter(Boolean);

const descOf = (text: string): string => {
  const lines = text.split("\n");
  const start = lines.findIndex((l) => /^\s*Feature:/.test(l));
  if (start < 0) return "";
  const out: string[] = [];
  for (let i = start + 1; i < lines.length; i++) {
    const l = lines[i];
    if (/^\s*(@|Scenario|Background|Rule|Example)/.test(l)) break;
    out.push(l);
  }
  return out.join("\n");
};

const sentencesOf = (desc: string): string[] => {
  const flat = desc.replace(/\s+/g, " ").trim();
  const parts = flat.split(/(?<=[.!?])\s+(?=[A-Z`“⚠️📄️🧬️⚖️🔬️])/);
  return parts.map((s) => s.trim()).filter((s) => s.length > 70);
};

const bySentence = new Map<string, string[]>();
for (const f of files) {
  const text = readFileSync("/Users/ueli/Documents/semio/" + f.slice(2), "utf8");
  const seen = new Set<string>();
  for (const s of sentencesOf(descOf(text))) {
    if (seen.has(s)) continue;
    seen.add(s);
    if (!bySentence.has(s)) bySentence.set(s, []);
    bySentence.get(s)!.push(f);
  }
}

const shared = [...bySentence.entries()]
  .filter(([, fs]) => fs.length >= 3)
  .sort((a, b) => b[1].length - a[1].length);

const touched = new Set<string>();
for (const [, fs] of shared) for (const f of fs) touched.add(f);

console.log(`files=${files.length}`);
console.log(`shared-sentences(>=3 files, >70 chars)=${shared.length}`);
console.log(`features touched=${touched.size}`);
console.log("");
for (const [s, fs] of shared) {
  console.log(`### ${fs.length}  ${s}`);
  for (const f of fs) console.log(`      ${f}`);
}
