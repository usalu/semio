// usage: bun findings.ts [overlayDir]  -- overlayDir mirrors repository-relative paths and shadows the checkout (staged check)
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
const root = process.cwd(), overlay = process.argv[2];
const m = await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts`);
const disk = m.diskZeroTouchTreeV1(root);
let last = "";
const tree = { packages: disk.packages, read: (p: string) => { last = p; if (overlay && existsSync(join(overlay, p))) return readFileSync(join(overlay, p), "utf8"); return disk.read(p); } };
const roster = m.readZeroTouchRosterV1(tree.read(m.ZERO_TOUCH_CLOSURES_FILE)!);
const commands = m.readZeroTouchCommandsV1(tree.read(m.ZERO_TOUCH_COMMANDS_FILE)!);
let total = 0;
for (const [name, run] of [["closures", () => m.zeroTouchClosureFindingsV1(roster, tree)], ["commands", () => m.zeroTouchCommandFindingsV1(commands, roster.seams.map((s: any) => s.module), tree)]] as const) {
  try {
    const r = run();
    console.log(name, "findings:", r.findings.length);
    total += r.findings.length;
    for (const f of r.findings) console.log(`  ${f.closure}\t${f.kind}\t${f.file}\t${f.specifier}\t${f.detail}`);
  } catch (e) { total++; console.log(name, "THROW", String(e).slice(0, 200), "last read:", last); }
}
process.exit(total ? 1 : 0);
