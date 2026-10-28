"""🧩 Applies the anchored edits that queued batches (walk-g `walks`, gram3 `churn`) make to the files of walk-views, in a given root (a mirror), so walk-views can be checked on top of them: python3 -I compose.py <root> apply|status"""
import json, os, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
root, verb = sys.argv[1], sys.argv[2]
TARGETS = ("🧭️script-inputs/🟨️.mjs", "📖️texts/🔣️.json")
for source in ("walk-g-tools/staged/walks/edits.json", "gram3-g-tools/staged/churn/edits.json"):
    for edit in json.load(open(os.path.join(C, source), encoding="utf8"))["edits"]:
        rel = edit.get("file") or edit.get("path")
        if not rel.endswith(TARGETS): continue
        path = os.path.join(root, rel); text = open(path, encoding="utf8").read()
        old, new = edit["old"], edit["new"]
        state = "applied" if text.count(new) and (new.startswith(old) and text.count(new) >= 1 and text.count(old) == text.count(new) or not text.count(old)) else "base" if text.count(old) == 1 else f"old x{text.count(old)}"
        if verb == "apply" and state == "base": text = text.replace(old, new); open(path, "w", encoding="utf8").write(text); state = "now applied"
        print(source.split("/")[0], state, rel[-40:], repr(old[:50]))
