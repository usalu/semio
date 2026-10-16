/** 🧪️ Moves the testing-library re-exports out of the react SDK entry into `ui/targets/react/testing`; re-points plugin importers that use only those names. Usage: bun split-react-testing.ts <files.txt> [--write] */
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { ROOT, SDK_DIR, scanSource, readLines } from "./lib.ts";
const write = process.argv.includes("--write");
const TESTING = new Set(["cleanup", "fireEvent", "render", "screen", "waitFor"]);
const OLD = "@semio-tech/framework/ui/targets/react", NEW = "@semio-tech/framework/ui/targets/react/testing";
const files = readLines(process.argv[2]).map(f => f.startsWith("/") ? f.slice(ROOT.length + 1) : f);
let edited = 0, mixed = 0;
for (const f of files) {
  let text: string;
  try { text = readFileSync(ROOT + "/" + f, "utf8"); } catch { continue; }
  if (!text.includes(OLD)) continue;
  const recs = scanSource(f, text).filter(r => r.spec === OLD);
  const patches: { start: number; end: number; text: string }[] = [];
  for (const r of recs) {
    const names = r.named.map(n => n.imported);
    if (!names.length) continue;
    const t = names.filter(n => TESTING.has(n));
    if (t.length === names.length) patches.push({ start: r.litStart, end: r.litEnd, text: `"${NEW}"` });
    else if (t.length) { mixed++; console.log("MIXED", f, names.join(",")); }
  }
  if (!patches.length) continue;
  patches.sort((a, b) => b.start - a.start);
  let next = text;
  for (const p of patches) next = next.slice(0, p.start) + p.text + next.slice(p.end);
  edited++; console.log("EDIT", f.slice(-90));
  if (write) writeFileSync(ROOT + "/" + f, next);
}
console.log({ edited, mixed });
if (write) {
  const dir = join(ROOT, SDK_DIR, "🖱️ui/🎯️targets/⚛️react");
  const facade = join(dir, "🟦️.ts");
  const lines = readFileSync(facade, "utf8").split("\n");
  const testingLine = lines.find(l => /\bcleanup\b/.test(l) && l.startsWith("export {"));
  if (!testingLine) throw Error("testing line not found");
  writeFileSync(facade, lines.filter(l => l !== testingLine).join("\n"));
  mkdirSync(join(dir, "🧪️testing"), { recursive: true });
  writeFileSync(join(dir, "🧪️testing/🟦️.ts"), `/** 🧪️ SDK entry \`@semio-tech/framework/ui/targets/react/testing\`: React testing-library surface kept apart because importing it registers test hooks. */\n${testingLine.replace("../../../../../🔨️modules", "../../../../../../🔨️modules")}\n`);
  const pkgPath = join(ROOT, SDK_DIR, "package.json");
  const raw = readFileSync(pkgPath, "utf8");
  const pkg = JSON.parse(raw);
  pkg.exports["./ui/targets/react/testing"] = "./🖱️ui/🎯️targets/⚛️react/🧪️testing/🟦️.ts";
  const keys = Object.keys(pkg.exports);
  const head = keys.filter(k => !k.startsWith("./2d") || true);
  const sorted = Object.fromEntries([...head.slice(0, head.indexOf("./value/ieee754") + 1), ...head.slice(head.indexOf("./value/ieee754") + 1).sort()].map(k => [k, pkg.exports[k]]));
  pkg.exports = sorted;
  writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + (raw.endsWith("\n") ? "\n" : ""));
}
