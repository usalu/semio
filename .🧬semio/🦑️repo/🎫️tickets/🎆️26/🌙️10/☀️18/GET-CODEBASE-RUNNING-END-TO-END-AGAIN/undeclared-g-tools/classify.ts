// undeclared-g classify: reads a oracle report (learned paths) and prints one class per path. Classes: scheduling (policy domain prefix: excluded from keys by design), presentation (policy resource), test-marked (path has a 🧪 segment: testSourceExclusions), config (package.json/tsconfig.json), data (.json), module (.ts/.mjs/.tsx), other.
import {readFileSync} from "node:fs";
import {buildSchedulingPrefixesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
const prefixes = buildSchedulingPrefixesV1();
const presentation = new Set<string>(JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json", "utf8")).presentationResources);
export function classOf(path: string): string {
  if (presentation.has(path)) return "presentation";
  if (prefixes.some((prefix) => path.startsWith(prefix))) return "scheduling";
  if (/(^|\/)(package|tsconfig)(\.[a-z]+)?\.json$/u.test(path)) return "config";
  if (path.split("/").some((part) => part.startsWith("🧪"))) return "test-marked";
  if (/\.(ts|tsx|mjs|js|cjs|mts)$/u.test(path)) return "module";
  if (/\.json$/u.test(path)) return "data";
  return "other";
}
if (import.meta.main) {
  const report = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
  const counts: Record<string, number> = {};
  for (const path of report.learned as string[]) { const c = classOf(path); counts[c] = (counts[c] ?? 0) + 1; if (process.argv[3] !== "counts") console.log(c.padEnd(12), path); }
  console.log(JSON.stringify(counts));
}
