# 🧽️ Cleanup after the moves: empty impl blocks and empty region pairs disappear, files without any item are removed.
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ctx, parent
from rsedit import Doc, REG_OPEN, REG_CLOSE

def run(root, report):
    removed_files = 0; removed_impls = 0
    for d, c, slug, p in parent.mods_of(root):
        if slug == ctx.BUNDLE_SLUG: continue
        doc = Doc(open(p, encoding="utf-8").read())
        while True:
            hit = next((i for i in doc.items() if i["kind"] == "impl" and re.match(r"impl(?:<[^>]*>)?\s+(?:ShellState|\w+Ctx)", i["head"]) and not doc.members(i)), None)
            if hit is None: break
            doc.cut(hit["s"], hit["e"]); removed_impls += 1
        while True:
            L = doc.lines; found = None
            for k, l in enumerate(L):
                if REG_OPEN.match(l):
                    j = k + 1
                    while j < len(L) and L[j].strip() == "": j += 1
                    if j < len(L) and REG_CLOSE.match(L[j]): found = (k, j); break
            if not found: break
            doc.cut(found[0], found[1])
        left = [i for i in doc.items() if i["kind"] != "use"]
        if not left:
            os.remove(p); os.rmdir(os.path.dirname(p)); removed_files += 1
        else:
            open(p, "w", encoding="utf-8").write(doc.text())
    report.append(f"cleanup: {removed_impls} empty impl blocks, {removed_files} empty files removed")
