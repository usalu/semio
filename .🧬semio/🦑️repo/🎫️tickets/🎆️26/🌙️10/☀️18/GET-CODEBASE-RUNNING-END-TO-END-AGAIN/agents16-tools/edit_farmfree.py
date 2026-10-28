"""🏊️ agents16 batch `farmfree`: a project farm pool that is disabled (workers refused for a differing identity, or degraded) gives its host capacity grant back at once instead of holding it until the graph construction closes (anchored, idempotent; every anchor exactly once).
Usage: python3 -I edit_farmfree.py <root> [--check]
"""
import os, sys

FARM = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs"
EDITS = [
 (FARM,
  "      report: (event, fields) => {\n",
  "      report: (event, fields) => {\n        if (event === \"disabled\") pool.release();\n"),
 (FARM,
  " * The farm then starts the worker the capacity grant of this process already pays for",
  " * A pool that is disabled (its workers refused for a differing identity, or degraded) gives its host grant back at once, so the memory and cores reserved for workers that never run are free for the other runs of the machine at once, not when the graph construction ends.\n * The farm then starts the worker the capacity grant of this process already pays for"),
]


def main():
    root, check = sys.argv[1], "--check" in sys.argv
    files = {}
    for rel, old, new in EDITS:
        path = os.path.join(root, rel)
        text = files.get(rel) or open(path, encoding="utf8").read()
        if new in text and text.count(new) == 1 and old not in text.replace(new, ""):
            files[rel] = text
            continue
        if text.count(old) != 1:
            print("ANCHOR", text.count(old), rel, old[:80].replace("\n", "\\n"))
            sys.exit(1)
        files[rel] = text.replace(old, new)
    if check:
        print("all %d anchors resolve" % len(EDITS))
        return
    for rel, text in files.items():
        path = os.path.join(root, rel)
        if open(path, encoding="utf8").read() != text:
            mode = os.stat(path).st_mode & 0o777
            tmp = path + ".agents16.tmp"
            open(tmp, "w", encoding="utf8").write(text)
            os.chmod(tmp, mode)
            os.replace(tmp, path)
            print("edited", rel)


if __name__ == "__main__":
    main()
