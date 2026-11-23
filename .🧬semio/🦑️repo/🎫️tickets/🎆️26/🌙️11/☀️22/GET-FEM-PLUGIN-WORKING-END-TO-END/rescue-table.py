import os

T = os.path.dirname(os.path.abspath(__file__))
rows = [l.split("\t") for l in open(T + r"\🗑️generated\rescue-decisions.tsv", encoding="utf-8").read().split("\n")]
out = ["| # | File (under `🎓️teaching/🏛️architecture/⚡️energy/`) | Decision |", "|---|---|---|"]
for i, dec, p in rows:
    short = p.split("⚡️energy/")[-1]
    if dec == "old-resolved":
        d = "old side applied: conflict hunks are only the `_din_ref` to `reference_chip` refactor, whitespace, or old refinements containing every `C:\\semio` line"
    else:
        d = f"`C:\\semio` file kept (pre-merge state); old change listed as open item: `git-semio-rescue-backup/open-old-change-{i}.patch`"
    out.append(f"| {i} | `{short}` | {d} |")
open(T + r"\🗑️generated\rescue-table.md", "w", encoding="utf-8").write("\n".join(out))
