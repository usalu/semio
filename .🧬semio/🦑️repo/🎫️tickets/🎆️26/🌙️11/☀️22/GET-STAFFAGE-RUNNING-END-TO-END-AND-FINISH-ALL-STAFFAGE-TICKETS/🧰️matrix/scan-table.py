"""📋️ Writes the per-asset table of a scan report: `python3 scan-table.py <scan-after-shared.json> <out.md> <seconds> <started> <ended>`.

The report is the `--report` file of `🔎️scan.ts` (`{ version, rows }`, rows in compact form). The table has one line per asset with the hard findings per code family: synchrony at rest and in other poses (with the extent findings among them), anchors by the place they lie, band copies and in-band derived views by scale and view, the scales without an analysis cue, the paper findings, the other hard codes and the reported group drift. Totals per family and per category precede it.
"""

import json
import sys
from collections import Counter, defaultdict

report, out, seconds, started, ended = sys.argv[1:6]
rows = json.load(open(report, encoding="utf-8"))["rows"]
CATEGORIES = ["animals", "appliances", "components", "furniture", "humans", "machinery", "objects", "vegetation", "vehicles"]


def places(row):
    found = row["summary"]["anchorPlaces"]
    names = {"other-bone": "other bone", "off-shapes": "off"}
    return "; ".join(f"{names[key]} {found[key]}" for key in ("other-bone", "off-shapes") if key in found)


def by_scale(row, family):
    cells = sorted({f"{scale} {'/'.join(group['views'])}" for group in row["groups"] if group["family"] == family for scale in group["scales"]}, key=lambda cell: (-int(cell.split()[0]), cell))
    return ", ".join(cells)


def cue(row):
    cells = sorted({f"{scale} {group['mode']}" for group in row["groups"] if group["family"] == "cue" for scale in group["scales"]})
    return ", ".join(cells)


def sync(row):
    summary = row["summary"]
    if summary["families"]["sync"] == 0:
        return ""
    extent = sum(count for code, count in summary["codes"].items() if code in ("sync-forward", "sync-height", "sync-lateral"))
    return f"{summary['syncRest']} / {summary['syncPosed']}" + (f"; {extent} extent" if extent else "")


def others(row):
    summary = row["summary"]
    return ", ".join(f"{code} {count}" for code, count in sorted(summary["codes"].items()) if not code.startswith(("sync-", "group-drift-", "paper-", "analysis-cue", "derived-view-")) and code != "anchor-landmark")


families = ["sync", "anchor", "band-copy", "in-band", "cue", "paper", "dimension", "view", "error"]
with_family = {family: [row for row in rows if row["summary"]["families"][family] > 0] for family in families}
dirty = [row for row in rows if not row["clean"]]
lines = [
    "# 🔎️ Catalog Scan After the Shared-Code Repairs",
    "",
    f"One process of `🧪️tests/🔭️multiview-coverage/🔎️scan.ts --all`, {started} to {ended}, {int(float(seconds))} s for {len(rows)} assets ({float(seconds) / len(rows):.1f} s per asset). Machine-readable: `🧰️matrix/scan-after-shared.json` (the compact rows: summary and the findings grouped per code, views, mode, bone and anchor with poses, scales, count and worst case). This table: `🧰️matrix/scan-table.py`.",
    "",
    "The scan checks every pose of every authored scale with the geometry checks of the matrix and the default pose with the derived-view, paper, availability and cue checks. Clip instants, transitions, range ends and symbols are left to the matrix, so the counts are lower bounds for it. The catalog is a live worktree: each row is a snapshot of the asset at the moment it was scanned.",
    "",
    "## Totals",
    "",
    f"- Assets: {len(rows)}; with a hard finding: **{len(dirty)}**; clean: **{len(rows) - len(dirty)}**.",
    f"- Hard findings: {sum(row['summary']['hard'] for row in rows)}; reported group drift: {sum(row['summary']['reported'] for row in rows)} in {sum(1 for row in rows if row['summary']['reported'] > 0)} assets.",
    "",
    "| Family | Assets | Findings |",
    "|---|---|---|",
]
for family in families:
    lines.append(f"| {family} | {len(with_family[family])} | {sum(row['summary']['families'][family] for row in rows)} |")
anchors = Counter()
kinds = Counter()
for row in rows:
    anchors.update(row["summary"]["anchorPlaces"])
    kinds.update(row["summary"]["anchorKinds"])
codes = Counter()
for row in rows:
    codes.update(row["summary"]["codes"])
lines += [
    "",
    f"- Synchrony findings at rest {sum(row['summary']['syncRest'] for row in rows)}, in other poses {sum(row['summary']['syncPosed'] for row in rows)}; assets wrong already at rest: {sum(1 for row in rows if row['summary']['syncRest'] > 0)}.",
    f"- Anchor findings by place: on a shape of another bone {anchors.get('other-bone', 0)}, off every drawn shape {anchors.get('off-shapes', 0)}; by anchor kind: {', '.join(f'{kind} {count}' for kind, count in sorted(kinds.items()))}.",
    f"- By code: {', '.join(f'`{code}` {count}' for code, count in sorted(codes.items(), key=lambda item: -item[1]))}.",
    "",
    "| Category | Assets | With a hard finding | Clean | sync | anchor | band copy | in-band | cue | paper |",
    "|---|---|---|---|---|---|---|---|---|---|",
]
per = defaultdict(list)
for row in rows:
    per[row["category"]].append(row)
for category in CATEGORIES:
    group = per.get(category, [])
    has = lambda family: sum(1 for row in group if row["summary"]["families"][family] > 0)
    lines.append(f"| {category} | {len(group)} | {sum(1 for row in group if not row['clean'])} | {sum(1 for row in group if row['clean'])} | {has('sync')} | {has('anchor')} | {has('band-copy')} | {has('in-band')} | {has('cue')} | {has('paper')} |")
lines += [
    "",
    "## Assets with a hard finding",
    "",
    "Sync: per-bone and extent findings at rest / in other poses, and the extent findings among them. Anchors: findings by the place the anchor lies instead. Band copy, in-band and paper: scale and view. Cue: scale and mode. Group drift is reported only.",
    "",
    "| Asset | Category | Sync (rest / posed) | Anchors | Band copy | In-band | Cue | Paper | Other hard codes | Group drift (reported) |",
    "|---|---|---|---|---|---|---|---|---|---|",
]
for row in sorted(dirty, key=lambda row: (row["category"], row["id"])):
    lines.append(f"| {row['id']} | {row['category']} | {sync(row)} | {places(row)} | {by_scale(row, 'band-copy')} | {by_scale(row, 'in-band')} | {cue(row)} | {by_scale(row, 'paper')} | {others(row)} | {row['summary']['reported'] or ''} |")
lines += ["", "## Clean assets", ""]
for category in CATEGORIES:
    clean = sorted(row["id"] for row in per.get(category, []) if row["clean"])
    lines.append(f"- **{category}** ({len(clean)}): {', '.join(clean)}")
open(out, "w", encoding="utf-8").write("\n".join(lines) + "\n")
print(f"{len(rows)} assets, {len(dirty)} with a hard finding")
