// 🧷️ Ensures a consumer crate enables a feature of an owner crate in [dev-dependencies]: bun ctc-devfeat.ts CONSUMER_MANIFEST OWNER_PACKAGE FEATURE[,FEATURE] [--dry]
import {readFileSync, writeFileSync} from "node:fs";
const [manifestArg, ownerName, featureArg] = process.argv.slice(2);
const dry = process.argv.includes("--dry");
const features = featureArg!.split(",");
const ROOT = "/Users/ueli/Documents/semio/";
const manifest = manifestArg!.startsWith("/") ? manifestArg! : ROOT + manifestArg!;
const text = readFileSync(manifest, "utf8");
const lines = text.split("\n");
const sectionAt = (name: string) => lines.findIndex(l => l.trim() === `[${name}]`);
const sectionEnd = (start: number) => { let i = start + 1; while (i < lines.length && !/^\s*\[/.test(lines[i]!)) i++; return i; };
const findEntry = (section: string): number => {
  const s = sectionAt(section);
  if (s < 0) return -1;
  const e = sectionEnd(s);
  for (let i = s + 1; i < e; i++) if (new RegExp(`^\\s*${ownerName!.replace(/[-]/g, "[-]")}\\s*=`).test(lines[i]!)) return i;
  return -1;
};
const addFeatures = (line: string): string => {
  const m = /features\s*=\s*\[([^\]]*)\]/.exec(line);
  if (m) {
    const have = m[1]!.split(",").map(x => x.trim().replace(/^"|"$/g, "")).filter(Boolean);
    const merged = [...have, ...features.filter(f => !have.includes(f))];
    return line.replace(m[0], `features = [${merged.map(f => `"${f}"`).join(", ")}]`);
  }
  if (/\}\s*$/.test(line)) return line.replace(/\s*\}\s*$/, `, features = [${features.map(f => `"${f}"`).join(", ")}] }`);
  throw new Error("unsupported dependency spec: " + line);
};
const targetSections = lines.map(l => l.trim()).filter(l => /^\[target\..*\.dependencies\]$/.test(l)).map(l => l.slice(1, -1));
const devSection = (dep: string) => dep.replace(/\.dependencies$/, ".dev-dependencies");
const insertEntry = (section: string, entry: string) => {
  const dev = sectionAt(section);
  if (dev >= 0) {
    const e = sectionEnd(dev);
    let insertAt = e;
    while (insertAt > dev + 1 && lines[insertAt - 1]!.trim() === "") insertAt--;
    lines.splice(insertAt, 0, entry);
  } else {
    while (lines.length && lines[lines.length - 1]!.trim() === "") lines.pop();
    lines.push("", `[${section}]`, entry, "");
  }
};
let handled = false;
for (const section of ["dev-dependencies", ...targetSections.map(devSection)]) {
  const idx = findEntry(section);
  if (idx >= 0) {
    const updated = addFeatures(lines[idx]!);
    if (updated === lines[idx]) { console.log("unchanged " + manifest); process.exit(0); }
    lines[idx] = updated;
    handled = true;
    break;
  }
}
if (!handled) {
  let found: {section: string; idx: number} | undefined;
  for (const section of ["dependencies", ...targetSections]) { const idx = findEntry(section); if (idx >= 0) { found = {section, idx}; break; } }
  if (!found) throw new Error(`no dependency entry for ${ownerName} in ${manifest}`);
  let entry = lines[found.idx]!;
  if (/^\s*[\w-]+\s*=\s*"/.test(entry)) throw new Error("string spec unsupported: " + entry);
  entry = entry.replace(/,?\s*optional\s*=\s*true/, "");
  const updated = addFeatures(entry);
  insertEntry(found.section === "dependencies" ? "dev-dependencies" : devSection(found.section), updated);
}
const out = lines.join("\n");
if (dry) console.log(out.split("\n").filter((l, i) => l !== text.split("\n")[i]).slice(0, 4).join("\n"));
else writeFileSync(manifest, out);
console.log((dry ? "would update " : "updated ") + manifest);
