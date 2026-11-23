import os

T = os.path.dirname(os.path.abspath(__file__))
fn = T + r"\git-semio-rescue.md"
lines = open(fn, encoding="utf-8").read().split("\n")
for i, l in enumerate(lines):
    if l.startswith("- 36 files with real conflicts"):
        lines[i] = ("- 36 files with real conflicts (162 hunks), revised after the coordinator's review (see table below). "
                    "Evidence on age: `C:\\semio` git history is shallow (root 3d62b2e, 2026-11-18; 35 files last touched there, `WindowChrome`-adjacent `browser/🟦️.tsx` also by 5db034f 2026-11-20), so it cannot show whether the `C:\\semio` side changed after the old 11-14 finals; file mtimes were overwritten by the first pass. "
                    "Rule applied per file: a conflict hunk may take the old side only if it is the `_din_ref` to `reference_chip` refactor, whitespace-only, or the old side contains every `C:\\semio` line that is not in the 746 base; if any hunk has `C:\\semio`-only lines, the whole `C:\\semio` file is kept and the old change is an open item (patch in `git-semio-rescue-backup/open-old-change-NN.patch`, base 746 to old, LF-normalised). "
                    "Result: 14 files old side, 22 files `C:\\semio` kept. `git-semio-rescue-overridden-hunks.txt` is from the first pass and only valid for the 14 old-side files.")
    if l.startswith("- 36 conflict files were resolved"):
        lines[i] = "- 22 conflict files keep the `C:\\semio` version and their old-side change is an open item (patches in `git-semio-rescue-backup/`); the 14 old-side files are listed in the table. Re-run of the running-example tests after the revision: 11 passed; all merged `.py`/`.json` still compile/parse."
txt = "\n".join(lines).replace(
    "## 4. Open items for the owner",
    "### Per-file decision for the 36 conflict files\n\n" + open(T + r"\🗑️generated\rescue-table.md", encoding="utf-8").read() + "\n\n## 4. Open items for the owner")
txt = txt.replace("`C:\\git\\semio` is safe to delete: every commit", "`C:\\git\\semio` is safe to delete (the 22 open old-side energy changes are preserved as patches in `git-semio-rescue-backup/`): every commit")
open(fn, "w", encoding="utf-8").write(txt)
