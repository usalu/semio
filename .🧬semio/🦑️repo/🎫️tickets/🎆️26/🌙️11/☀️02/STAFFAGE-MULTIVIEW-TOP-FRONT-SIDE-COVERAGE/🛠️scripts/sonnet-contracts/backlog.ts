import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

type Issue = { code: string; path: string; message: string; data?: Record<string, unknown> };
type Entry = { failures: number; codes: Record<string, number>; issues: Issue[] };
type Report = { assetsChecked: number; assetsFailing: number; codeTotals: Record<string, number>; assets: Record<string, Entry> };

const generated = resolve(process.argv[2]!);
const out = resolve(process.argv[3]!);
const main = JSON.parse(readFileSync(`${generated}/failures.json`, "utf8")) as Report;
const rangesFile = `${generated}/failures-ranges.json`;
const ranges = existsSync(rangesFile) ? (JSON.parse(readFileSync(rangesFile, "utf8")) as Report) : null;

const lines: string[] = [];
const say = (line = ""): number => lines.push(line);
const cell = (value: unknown): string => String(value ?? "").replace(/\|/gu, "/").replace(/\n/gu, " ");
const table = (header: readonly string[], rows: readonly (readonly unknown[])[]): void => {
  say(`| ${header.join(" | ")} |`);
  say(`|${header.map(() => "---").join("|")}|`);
  for (const row of rows) say(`| ${row.map(cell).join(" | ")} |`);
  say();
};

say("# Staffage multiview backlog (owner: sonnet-contracts)");
say();
say(`Generated ${new Date().toISOString().slice(0, 10)} by \`T/🛠️scripts/sonnet-contracts/failures.ts\` and \`backlog.ts\` from the current catalog. Machine-readable: \`T/🗑️generated/contracts/failures.json\`. ${main.assetsChecked} assets checked, ${main.assetsFailing} failing.`);
say();
say("## Totals per code");
say();
table(["code", "issues"], Object.entries(main.codeTotals).sort((a, b) => b[1] - a[1]));
say("Rules and their meaning: `T/🔗️interfaces.md` sections 4 and 6. Tolerance of every extent rule: |measured - declared| <= max(2 % of declared, 15 mm). `lateral-ambiguous` is one issue per bone and lists the shapes, the bands (`1:<scale> <view>`), `thetaMax` (the largest out-of-plane rotation of that bone over every pose, every clip and 9 clip samples) and the worst paper error `extent * sin(thetaMax)` in mm, flagged when it exceeds the 0.2 mm print minimum feature.");
say();
say("## About `elevation-lateral` (answer to the inventory question)");
say();
say("The predicate reads (1) the abduction and twist offsets (`abductionDeg`, `twistDeg`) in every pose and every clip key, taking the bones that carry one and all their descendants, and (2) the lateral width of the shape's own front twin (same id and bone). It does not read rest-pose bone z, and a crown silhouette on a bone that never leaves the elevation plane (no ancestor with abduction or twist anywhere) cannot trigger it. So it fired on the 11 trees only because a crosswind sway key rotated those bones out of plane; D29 applies (vegetation reverted the crosswind) and the rule no longer fires in the current catalog.");
say();

const byCategory = new Map<string, [string, Entry][]>();
for (const [key, entry] of Object.entries(main.assets)) {
  const category = key.split("/")[0]!;
  (byCategory.get(category) ?? byCategory.set(category, []).get(category)!).push([key, entry]);
}
for (const [category, assets] of [...byCategory].sort((a, b) => a[0].localeCompare(b[0]))) {
  say(`## ${category}`);
  say();
  const lateral: (readonly unknown[])[] = [];
  const other: (readonly unknown[])[] = [];
  for (const [key, entry] of assets.sort((a, b) => a[0].localeCompare(b[0]))) {
    const slug = key.split("/")[1]!;
    for (const issue of entry.issues) {
      if (issue.code === "lateral-ambiguous" && issue.data !== undefined) {
        const d = issue.data as { bone: string; shapes: string[]; bands: string[]; thetaMaxDeg: number; errorMm: number };
        lateral.push([slug, d.bone, d.shapes.slice(0, 8).join(", ") + (d.shapes.length > 8 ? `, +${d.shapes.length - 8}` : ""), d.bands.join(", "), d.thetaMaxDeg, d.errorMm]);
      } else other.push([slug, issue.code, issue.path, issue.message]);
    }
  }
  if (lateral.length > 0) {
    say("### lateral-ambiguous");
    say();
    table(["asset", "bone", "shapes", "bands", "thetaMax deg", "error mm"], lateral);
  }
  if (other.length > 0) {
    say("### other findings");
    say();
    table(["asset", "code", "path", "message"], other);
  }
}

say("## Range and nominal suitability (D10, D30)");
say();
if (ranges === null) say("Not yet complete: the `--ranges` run is in progress (it measures every band at its nominal scale and at both ends of its declared `validRange`); this section is filled when it finishes.");
else {
  say("Split by `data.at`: `nominal` is the band's own preset scale, `range-end` the smallest and largest denominator of its declared range.");
  say();
  const rows: (readonly unknown[])[] = [];
  for (const [key, entry] of Object.entries(ranges.assets)) for (const issue of entry.issues) if (issue.code.startsWith("nominal-") || issue.code.startsWith("range-end-")) {
    const d = issue.data as { at: string; denominator: number; scale: number; check: string; shapes: string[]; measuredMm: number; requiredMm: number };
    rows.push([key, d.at, `1:${d.denominator}`, `1:${d.scale}`, d.check, d.shapes.join("+"), d.measuredMm, d.requiredMm]);
  }
  table(["asset", "at", "band", "measured at", "check", "shapes", "measured mm", "required mm"], rows);
}
writeFileSync(out, lines.join("\n"));
