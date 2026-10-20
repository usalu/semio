"""Regenerates every group that is still pending, blocked or drifted (an applied group is never regenerated: its anchors are gone from the live tree) from the live tree and the staged files; run it again before an apply when other rounds moved the anchors."""
import os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
TOOL = "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts"
state = subprocess.run([sys.executable, os.path.join(HERE, "apply.py")], capture_output=True, text=True).stdout
rebase = [line.split()[0] for line in state.splitlines() if line.startswith("g") and line.split()[1].startswith(("DRIFT", "pending", "BLOCKED"))]
for name in ["g1", "g2", "g3", "g4", "g5", "g6", "g7", "g10", "g12", "g13", "g14", "g15"]:
    if name in rebase:
        subprocess.run([sys.executable, os.path.join(HERE, "build_" + name + ".py")], check=True, stdout=subprocess.DEVNULL)
for name in ["g1", "g2", "g3", "g4", "g5", "g14", "g15"]:
    if name not in rebase:
        continue
    path = os.path.join(HERE, "groups", name + ".py")
    text = open(path, encoding="utf-8").read()
    marker = "REQUIRES = ["
    start = text.index(marker) + len(marker)
    text = text[:start] + repr(TOOL) + (", " if text[start] != "]" else "") + text[start:]
    open(path, "w", encoding="utf-8").write(text)
folder = TOOL.replace("🟦️.ts", "")
open(os.path.join(HERE, "groups", "g8.py"), "w", encoding="utf-8").write("NAME = 'g8'\nTITLE = 'test toolchain interface: Python 3 (python3, python, py -3), ripgrep and a writable tree for the test suites, with a missing tool reported once in English or German'\nREQUIRES = []\nEDITS = []\nNEW = %r\n" % [TOOL, TOOL.replace("🟦️.ts", "🔣️.json"), TOOL.replace("🟦️.ts", "🧪️tests/🟦️.ts"), TOOL.replace("🟦️.ts", "🧪️tests/🧫️fixtures/🔣️.json")])
print("groups regenerated")
