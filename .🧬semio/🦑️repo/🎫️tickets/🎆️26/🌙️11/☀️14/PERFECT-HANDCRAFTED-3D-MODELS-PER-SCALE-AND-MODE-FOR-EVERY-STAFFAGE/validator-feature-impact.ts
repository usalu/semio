/** 🔬️ What every `volumes-feature` finding of the published catalog asks of its author: per asset the findings per scale, split into undeclared thin sheets (declare `sheet: true`), long parts that keep their true section when declared `slender` (structure members from 5:1 and two features of length, any solid from 20:1) and compact solids below the feature (model them for the scale). Usage: `bun validator-feature-impact.ts > validator-feature-impact.md` */
import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { join } from "node:path";
import { VOLUME_LIMITS, featureOf, featureRule, sortedExtents } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const lines: string[] = [];
const total = { assets: 0, findings: 0, sheet: 0, slender: 0, compact: 0 };
for (const category of readdirSync(ROOT).sort()) {
  if (category.startsWith(".") || !statSync(join(ROOT, category)).isDirectory()) continue;
  for (const slug of readdirSync(join(ROOT, category)).sort()) {
    const path = join(ROOT, category, slug, "🔣️.json");
    if (!existsSync(path)) continue;
    try {
      const asset = JSON.parse(readFileSync(path, "utf8")).asset;
      const findings = featureRule(asset).findings;
      if (findings.length === 0) continue;
      const per = new Map<number, { sheet: string[]; slender: string[]; compact: number }>();
      for (const f of findings) {
        const id = (f.data as { solid?: string } | undefined)?.solid ?? "?";
        const solid = (asset.volumes[`${f.scale}`][f.list!] as { id: string }[]).find((s) => s.id === id) as never as Parameters<typeof sortedExtents>[0] & { part?: string; sheet?: boolean };
        const entry = per.get(f.scale!) ?? { sheet: [], slender: [], compact: 0 };
        per.set(f.scale!, entry);
        const [, middle, longest] = solid === undefined ? [0, 1, 0] : sortedExtents(solid);
        const ratio = middle > 0 ? longest / middle : 0;
        const member = f.list === "structure" && solid?.part === "member" && ratio >= VOLUME_LIMITS.slenderMemberRatio && longest >= VOLUME_LIMITS.slenderMemberFeatures * featureOf(f.scale!) - 1e-9;
        if (f.message.en.includes("declare `sheet: true`")) entry.sheet.push(id);
        else if (solid !== undefined && solid.sheet !== true && (member || ratio >= VOLUME_LIMITS.slenderRatio)) entry.slender.push(id);
        else entry.compact++;
      }
      total.assets++;
      total.findings += findings.length;
      const short = (ids: string[]): string => (ids.length <= 8 ? ids.join(", ") : `${ids.slice(0, 8).join(", ")} +${ids.length - 8}`);
      const parts = [...per].sort((a, b) => a[0] - b[0]).map(([scale, e]) => {
        total.sheet += e.sheet.length;
        total.slender += e.slender.length;
        total.compact += e.compact;
        return `1:${scale} ${[e.compact > 0 ? `${e.compact} below the feature` : "", e.slender.length > 0 ? `declare slender: ${short(e.slender)}` : "", e.sheet.length > 0 ? `declare sheet: ${short(e.sheet)}` : ""].filter((x) => x !== "").join("; ")}`;
      });
      lines.push(`- **${category.replace(/[^a-z-]/g, "")}/${slug}** (${findings.length}): ${parts.join(" | ")}`);
    } catch (error) {
      lines.push(`- **${category.replace(/[^a-z-]/g, "")}/${slug}**: could not be judged (${(error as Error).message.slice(0, 80)})`);
    }
  }
}
console.log(`${total.assets} assets, ${total.findings} findings: ${total.compact} solids below the feature of their scale (model them for the scale), ${total.slender} long parts that keep their true section once declared \`slender: true\`, ${total.sheet} undeclared thin sheets (\`sheet: true\` if truly thin and meant at that scale).\n`);
console.log(lines.join("\n"));
