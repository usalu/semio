"""🧷️ Carries a field that another worker changed in the catalog over to the copies of this batch, so that a later write does not undo it: rebase.py <id>… splices the text of the member `"tags"` of the asset (changed catalog-wide on 2026-11-25 19:09) from the catalog document into `$GEN/{backup,loft,nest,keep,written,was}/<id>*.json`, byte for byte; every other byte of a copy stays. The first run keeps the backup as it was in `$GEN/backup-1124`. Prints per asset the copies changed and whether the backup now equals the catalog document."""
import json, pathlib, shutil, sys
W = pathlib.Path(__file__).resolve().parent
GEN = W.parent.parent / "🗑️generated" / "matrix" / "MA"
CATALOG = W.parents[8] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🗂️catalog"


def span(text: str) -> tuple[int, int]:
    start = text.index('"tags": [')
    depth, at = 0, text.index("[", start)
    for i in range(at, len(text)):
        depth += {"[": 1, "]": -1}.get(text[i], 0)
        if depth == 0:
            return start, i + 1
    raise ValueError("unbalanced tags")


for a in sys.argv[1:]:
    source = next(CATALOG.glob(f"*/{a}/🔣️.json")).read_text()
    s0, s1 = span(source)
    (GEN / "backup-1124").mkdir(exist_ok=True)
    if not (GEN / "backup-1124" / f"{a}.json").exists():
        shutil.copy2(GEN / "backup" / f"{a}.json", GEN / "backup-1124" / f"{a}.json")
    changed = []
    for folder in ("backup", "loft", "nest", "keep", "written", "was"):
        for path in sorted((GEN / folder).glob(f"{a}*.json")):
            text = path.read_text()
            if '"tags": [' not in text[:20000]:
                continue
            t0, t1 = span(text)
            if text[t0:t1] != source[s0:s1]:
                next_text = text[:t0] + source[s0:s1] + text[t1:]
                json.loads(next_text)
                path.write_text(next_text)
                changed.append(f"{folder}/{path.name}")
    same = (GEN / "backup" / f"{a}.json").read_text() == source
    print(a, len(changed), "copies rebased;", "backup equals the catalog" if same else "backup differs from the catalog (the catalog carries a repair)")
