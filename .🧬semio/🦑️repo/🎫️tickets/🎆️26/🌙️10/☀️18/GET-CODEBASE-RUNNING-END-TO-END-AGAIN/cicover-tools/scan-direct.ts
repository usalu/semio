/** 🧭 Runs the direct-tool-declarations law (live library code, read only) over the `.github` files of a root: `bun scan-direct.ts <root> <live-root>`; exit 1 on a finding that no allow rule covers. */
import {readdirSync, statSync} from "node:fs";
import {join, relative} from "node:path";
const [root, live] = [process.argv[2]!, process.argv[3]!];
const law = await import(join(live, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧰️direct-tool-declarations/🟦️.ts"));
const walk = (directory: string): string[] => readdirSync(directory).flatMap(name => statSync(join(directory, name)).isDirectory() ? walk(join(directory, name)) : [relative(root, join(directory, name))]);
const files = walk(join(root, ".github"));
const result = law.scanDirectToolDeclarationsV1(root, files);
const open = result.findings.filter((finding: {allowed: string | null}) => finding.allowed === null);
console.log(`scanned ${result.scanned} files, ${result.findings.length} findings, ${open.length} without an allow rule`);
for (const finding of open) console.log(JSON.stringify(finding));
process.exit(open.length === 0 ? 0 : 1);
