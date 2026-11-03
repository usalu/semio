import { readFileSync, writeFileSync } from "node:fs";

type Failure = { count: number; first: { subject: string; message: string }[]; worst?: { deltaMm: number; subject: string; pair: string }; byKey?: Record<string, { deltaMm: number; subject: string }> };
type Cell = { asset: string; category: string; scale: string; scaleKind: string; view: string; mode: string; status: string; failures: Record<string, Failure>; rangeEnd?: Record<string, Failure>; groupDrift?: Record<string, Failure> };
type Matrix = { assets: number; coverage: { exhaustive: string[]; sampled: string[] }; summary: { total: Record<string, number>; byCategory: Record<string, { total: Record<string, number>; byView: Record<string, Record<string, number>>; assets: number; failingAssets: number }> }; cells: Cell[] };

const [matrixDir, outFile] = process.argv.slice(2) as [string, string];
const m = JSON.parse(readFileSync(`${matrixDir}/matrix.json`, "utf8")) as Matrix;
const owner = (asset: string, category: string): string => {
  if (category === "humans") return "sonnet-humans";
  if (category === "animals") return "sonnet-animals";
  if (category === "vegetation") return "sonnet-vegetation";
  if (category === "objects" || category === "furniture") return "sonnet-objects";
  if (category === "machinery" || category === "vehicles") return "sonnet-machines";
  if (category === "components") return "sonnet-buildings";
  return asset <= "hot-water-cylinder" ? "sonnet-buildings" : "sonnet-animals";
};
const nominal = (k: string): boolean => k === "preset" || k === "declared";
const lines: string[] = [];
const t = m.summary.total;
lines.push(`Run: ${m.cells.length} cells over ${m.assets} assets: ${t.ok} ok, ${t.unavailable} declared unavailable, ${t["range-end"]} range-end (print-profile violation at a validRange end, own column), ${t["group-drift"]} group-drift (own column, not FAIL), ${t.fail} FAIL.`, "");
lines.push("### Per category (cells)", "", "| category | assets | failing assets | ok | unavailable | range-end | group-drift | fail | elevation fail | front fail | plan fail |", "|---|---|---|---|---|---|---|---|---|---|---|");
for (const [c, g] of Object.entries(m.summary.byCategory)) lines.push(`| ${c} | ${g.assets} | ${g.failingAssets} | ${g.total.ok} | ${g.total.unavailable} | ${g.total["range-end"]} | ${g.total["group-drift"]} | ${g.total.fail} | ${g.byView.elevation!.fail} | ${g.byView.front!.fail} | ${g.byView.plan!.fail} |`);

const cellsPerCheck = new Map<string, number>();
const example = new Map<string, string>();
for (const cell of m.cells) for (const [check, f] of Object.entries(cell.failures)) {
  cellsPerCheck.set(check, (cellsPerCheck.get(check) ?? 0) + 1);
  if (!example.has(check)) example.set(check, `${cell.asset} 1:${cell.scale} ${cell.view} ${cell.mode}: ${f.first[0]?.message.replace(/\|/gu, "/").slice(0, 150)}`);
}
lines.push("", "### Failing checks (cells)", "", "| check | cells | assets | example |", "|---|---|---|---|");
const assetsPerCheck = new Map<string, Set<string>>();
for (const cell of m.cells) for (const check of Object.keys(cell.failures)) (assetsPerCheck.get(check) ?? assetsPerCheck.set(check, new Set()).get(check)!).add(cell.asset);
for (const [check, n] of [...cellsPerCheck].sort((a, b) => b[1] - a[1])) lines.push(`| ${check} | ${n} | ${assetsPerCheck.get(check)!.size} | ${example.get(check)} |`);

type Worst = { asset: string; category: string; check: string; scale: string; view: string; subject: string; pair: string; deltaMm: number; message: string };
const worst = new Map<string, Worst>();
for (const cell of m.cells) for (const [check, f] of Object.entries(cell.failures)) {
  const key = `${cell.asset}|${check}`;
  const delta = f.worst?.deltaMm ?? -1;
  const prev = worst.get(key);
  if (prev === undefined || delta > prev.deltaMm || (delta === prev.deltaMm && delta < 0 && nominal(cell.scaleKind) && prev.scale.includes(":")))
    worst.set(key, { asset: cell.asset, category: cell.category, check, scale: `1:${cell.scale}`, view: cell.view, subject: f.worst?.subject ?? f.first[0]?.subject ?? "", pair: f.worst?.pair ?? cell.view, deltaMm: delta, message: f.first[0]?.message ?? "" });
}
const rows = [...worst.values()];
const top = (check: string) => rows.filter((r) => r.check === check).sort((a, b) => b.deltaMm - a.deltaMm).slice(0, 10);
lines.push("", "### Top 10 assets per measured check (worst delta in mm)", "");
for (const check of ["sync-height", "sync-forward", "sync-lateral", "dimension-elevation-width", "dimension-elevation-height", "dimension-front-width", "dimension-front-height", "dimension-plan-length", "dimension-plan-width", "paper-height", "paper-width"]) {
  const list = top(check);
  if (list.length === 0) continue;
  lines.push(`**${check}** (${rows.filter((r) => r.check === check).length} assets)`, "", "| asset | delta mm | view pair | scale | instant |", "|---|---|---|---|---|", ...list.map((r) => `| ${r.asset} | ${r.deltaMm.toFixed(0)} | ${r.pair} | ${r.scale} | ${r.subject} |`), "");
}

const key3 = (c: Cell): string => `${c.asset}|${c.scale}|${c.view}`;
const dimFail = new Set(m.cells.filter((c) => Object.keys(c.failures).some((k) => k.startsWith("dimension"))).map(key3));
let paperDimension = 0;
let paperScene = 0;
const paperSceneAssets = new Set<string>();
for (const c of m.cells) {
  if (c.mode !== "dark-line" || !(c.failures["paper-width"] || c.failures["paper-height"])) continue;
  if (dimFail.has(key3(c))) paperDimension++;
  else (paperScene++, paperSceneAssets.add(c.asset));
}
lines.push("### paper-width and paper-height: what they check and why they fail", "", "They compare the bounding box of the drawn `shape:` paths of the scene (millimetres on paper, default pose) with `dimension * 1000 / denominator` under the D18 tolerance, so they catch a scene that is not the evaluated geometry.", `- same (asset, scale, view) also fails dimension-* (the model extent itself is wrong, the paper only repeats it): ${paperDimension} (dark-line cells)`, `- dimension-* passes but the paper box differs (the scene draws other geometry than the evaluated shapes, for example the representation of another band or hidden or merged shapes): ${paperScene} (dark-line cells) in ${paperSceneAssets.size} assets: ${[...paperSceneAssets].slice(0, 20).join(", ")}`, "");

lines.push("### derived-view: reuse across bands versus derivation within a band", "");
for (const check of ["derived-view-band-copy", "derived-view-in-band"]) {
  const per = new Map<string, Set<string>>();
  for (const c of m.cells) if (c.failures[check]) (per.get(c.asset) ?? per.set(c.asset, new Set()).get(c.asset)!).add(`${c.view}@1:${c.scale}`);
  lines.push(`**${check}**: ${per.size} assets`, "", "| asset | owner | views and scales |", "|---|---|---|", ...[...per].sort((a, b) => a[0].localeCompare(b[0])).map(([a, set]) => `| ${a} | ${owner(a, m.cells.find((c) => c.asset === a)!.category)} | ${[...set].slice(0, 8).join(", ")} |`), "");
}

lines.push("### blank-shapes: every cell", "", "| asset | band | view | mode | instant | shapes, pen role and reason |", "|---|---|---|---|---|---|");
for (const c of m.cells) {
  const f = c.failures["blank-shapes"];
  if (f) lines.push(`| ${c.asset} | 1:${c.scale} | ${c.view} | ${c.mode} | ${f.first[0]?.subject} | ${f.first[0]?.message.replace(/\|/gu, "/").replace(/^no \w+ shape of the representation is drawn: /u, "")} |`);
}

const groups: Record<string, Map<string, Set<string>>> = { nominal: new Map(), "range-end": new Map(), alternative: new Map() };
const re = /(stroke|gap|feature|hatch) [0-9.]+ mm/gu;
for (const cell of m.cells) {
  const f = cell.failures["unavailable-scale-unsuitable"] ?? cell.rangeEnd?.["range-end-unsuitable"];
  if (!f) continue;
  const g = groups[nominal(cell.scaleKind) ? "nominal" : cell.scaleKind === "alternative" ? "alternative" : "range-end"]!;
  const types = g.get(`${cell.asset}|${cell.scale}`) ?? g.set(`${cell.asset}|${cell.scale}`, new Set()).get(`${cell.asset}|${cell.scale}`)!;
  for (const s of f.first) for (const v of s.message.matchAll(re)) types.add(v[1]!);
}
const bucket = (g: Map<string, Set<string>>): string => {
  const types: Record<string, number> = {};
  for (const set of g.values()) for (const type of set) types[type] = (types[type] ?? 0) + 1;
  return `${g.size} (asset, scale) pairs; per violation type (min line = stroke, min gap = gap, min feature = feature, hatch): ${JSON.stringify(types)}`;
};
lines.push("", "### Print-profile violations (D30)", "", `- nominal scales (FAIL): ${bucket(groups.nominal!)}`, `- ends of validRange (status range-end, not FAIL): ${bucket(groups["range-end"]!)}`, `- alternative cells: ${bucket(groups.alternative!)}`, "");

lines.push("### FAIL list per category (worst case per asset and check)", "");
for (const category of Object.keys(m.summary.byCategory)) {
  const list = rows.filter((r) => r.category === category).sort((a, b) => a.asset.localeCompare(b.asset) || a.check.localeCompare(b.check));
  lines.push(`#### ${category} (${new Set(list.map((r) => r.asset)).size} assets)`, "", "| asset | owner | check | instant | scale | view pair | delta mm |", "|---|---|---|---|---|---|---|", ...list.map((r) => `| ${r.asset} | ${owner(r.asset, r.category)} | ${r.check} | ${r.subject.replace(/\|/gu, "/")} | ${r.scale} | ${r.pair} | ${r.deltaMm >= 0 ? r.deltaMm.toFixed(0) : ""} |`), "");
}
type Land = { asset: string; category: string; check: string; id: string; subject: string; scale: string; pair: string; deltaMm: number };
const land = new Map<string, Land>();
const take = (cell: Cell, group: Record<string, Failure> | undefined): void => {
  for (const [check, f] of Object.entries(group ?? {})) {
    if (!(check.startsWith("sync-bone-") || check === "anchor-landmark" || check.startsWith("group-drift-"))) continue;
    for (const [pair, v] of Object.entries(f.byKey ?? {})) {
      const key = `${cell.asset}|${check}|${pair}`;
      const prev = land.get(key);
      if (prev === undefined || v.deltaMm > prev.deltaMm) land.set(key, { asset: cell.asset, category: cell.category, check, id: pair, subject: v.subject, scale: `1:${cell.scale}`, pair, deltaMm: v.deltaMm });
    }
  }
};
for (const cell of m.cells) if (cell.mode === "light-line") (take(cell, cell.failures), take(cell, cell.groupDrift));
const landRows = [...land.values()];
const section = (title: string, pred: (r: Land) => boolean): void => {
  lines.push(`### ${title}`, "");
  for (const category of Object.keys(m.summary.byCategory)) {
    const list = landRows.filter((r) => r.category === category && pred(r)).sort((a, b) => b.deltaMm - a.deltaMm);
    lines.push(`#### ${category} (${list.length} cases in ${new Set(list.map((r) => r.asset)).size} assets)`, "");
    if (list.length === 0) continue;
    lines.push("| asset | owner | check | bone or anchor and view pair | instant | scale | delta mm |", "|---|---|---|---|---|---|---|", ...list.map((r) => `| ${r.asset} | ${owner(r.asset, r.category)} | ${r.check} | ${r.id} | ${r.subject.replace(/\|/gu, "/")} | ${r.scale} | ${(r.deltaMm ?? Number.NaN).toFixed(0)} |`), "");
  }
};
section("Tier one: per-bone and anchor-landmark FAIL list (sorted by delta)", (r) => r.check.startsWith("sync-bone-") || r.check === "anchor-landmark");
section("Group-drift (own status, not FAIL), listed per asset for the owners", (r) => r.check.startsWith("group-drift-"));
lines.push("### Exhaustive versus sampled", "", "Exhaustive:", ...m.coverage.exhaustive.map((e) => `- ${e}`), "", "Sampled:", ...m.coverage.sampled.map((e) => `- ${e}`));
writeFileSync(outFile, `${lines.join("\n")}\n`);
