import re, sys
from pathlib import Path

root = Path(r"C:\git\semio")
energy = root / "🎓️teaching/🏛️architecture/⚡️energy"
script = (energy / "🏗️authoring/📜️script.ts").read_text(encoding="utf-8")
registered = set(re.findall(r'"(🎓️teaching\.[^"]+)"', script))
actual = set()
for p in energy.rglob("*.py"):
    s = p.as_posix()
    if ".venv" in s or "site-packages" in s or "__pycache__" in s or "/.🧬semio/" in s:
        continue
    if "🧪️" not in s:
        continue
    t = p.read_text(encoding="utf-8", errors="replace")
    if "unittest" not in t and "TestCase" not in t:
        continue
    rel = p.relative_to(root).with_suffix("")
    actual.add(".".join(rel.parts))
print("MISSING:")
for m in sorted(actual - registered):
    print(" ", m)
print("REGISTERED BUT NO FILE:")
for m in sorted(r for r in registered - actual if r.startswith("🎓️teaching.🏛️architecture.⚡️energy")):
    print(" ", m)
Path(r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️13\APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS\tests\modules.txt").write_text("\n".join(sorted(actual)) + "\n", encoding="utf-8")
