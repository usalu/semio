/** 🩻️ The authors' scan of the suite (`T/🔎️scan.ts`, `scanAsset`) on documents that are not in the catalog (a backup, a dry run): `bun scan-file.ts <file.json>…` prints per document the hard findings by code with count, worst delta and one message, among them the codes the fixer's gate does not measure (`paper-width`, `paper-height`, `derived-view-*`, `analysis-cue-*`). */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { scanAsset } = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🔎️scan.ts"));
for (const arg of process.argv.slice(2)) {
  const row = scanAsset(readDoc(arg).doc.asset);
  const by = new Map<string, { n: number; mm: number; message: string }>();
  for (const finding of row.findings as any[]) {
    if (finding.tier !== "hard") continue;
    const key = `${finding.code} 1:${finding.scale} ${finding.views.join("/")}${finding.bone ? ` ${finding.bone}` : ""}${finding.anchor ? ` ${finding.anchor}` : ""}`;
    const entry = by.get(key) ?? by.set(key, { n: 0, mm: -1, message: "" }).get(key)!;
    entry.n++;
    if ((finding.deltaMm ?? 0) > entry.mm) Object.assign(entry, { mm: finding.deltaMm ?? 0, message: `${finding.pose}: ${finding.message}` });
  }
  console.log(`${arg.split("/").slice(-2).join("/")}: ${row.summary.hard} hard`);
  for (const [key, entry] of by) console.log(`   ${key} ×${entry.n} ${entry.mm.toFixed(0)} mm :: ${entry.message.slice(0, 170)}`);
}
