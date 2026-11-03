import sys

p = sys.argv[1]
s = open(p, encoding="utf8").read()


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


rep("rangeEnd?: Record<string, Failure> };", "rangeEnd?: Record<string, Failure>; groupDrift?: Record<string, Failure> };")
rep("type Failure = { count: number; first: { subject: string; message: string }[]; worst?: { deltaMm: number; subject: string; pair: string } };", "type Failure = { count: number; first: { subject: string; message: string }[]; worst?: { deltaMm: number; subject: string; pair: string }; byKey?: Record<string, { deltaMm: number; subject: string }> };")
rep(
    "${t[\"range-end\"]} range-end (print-profile violation at a validRange end, reported in its own column), ${t.fail} FAIL.`",
    "${t[\"range-end\"]} range-end (print-profile violation at a validRange end, own column), ${t[\"group-drift\"]} group-drift (own column, not FAIL), ${t.fail} FAIL.`",
)
rep('| ok | unavailable | range-end | fail | elevation fail', '| ok | unavailable | range-end | group-drift | fail | elevation fail')
rep('|---|---|---|---|---|---|---|---|---|---|");', '|---|---|---|---|---|---|---|---|---|---|---|");')
rep('${g.total["range-end"]} | ${g.total.fail}', '${g.total["range-end"]} | ${g.total["group-drift"]} | ${g.total.fail}')
rep(
    'lines.push("### Exhaustive versus sampled"',
    '''type Land = { asset: string; category: string; check: string; id: string; subject: string; scale: string; pair: string; deltaMm: number };
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
    lines.push("| asset | owner | check | bone or anchor and view pair | instant | scale | delta mm |", "|---|---|---|---|---|---|---|", ...list.map((r) => `| ${r.asset} | ${owner(r.asset, r.category)} | ${r.check} | ${r.id} | ${r.subject.replace(/\\|/gu, "/")} | ${r.scale} | ${r.deltaMm.toFixed(0)} |`), "");
  }
};
section("Tier one: per-bone and anchor-landmark FAIL list (sorted by delta)", (r) => r.check.startsWith("sync-bone-") || r.check === "anchor-landmark");
section("Group-drift (own status, not FAIL), listed per asset for the owners", (r) => r.check.startsWith("group-drift-"));
lines.push("### Exhaustive versus sampled"''',
)
open(p, "w", encoding="utf8").write(s)
