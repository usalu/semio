"""🔎 Dumps top-level use items and mod items per mounted file (analysis only)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from scan0 import crawl, R
from rs import items

if __name__ == "__main__":
    files = crawl()
    for rel, t in sorted(files.items()):
        its = items(t)
        out = []
        for it in its:
            if it["kind"] == "use":
                out.append(("pub " if it["vis"] else "") + "use " + it["name"] + (" #cfg" if any("cfg" in a for a in it["attrs"]) else ""))
            elif it["kind"] == "mod":
                out.append(("pub " if it["vis"] else "") + "mod " + it["name"] + " " + " ".join(a.replace("\n", " ") for a in it["attrs"]))
        print("##", rel.replace(R, ""), len(t.splitlines()))
        for o in out: print("   ", o)
