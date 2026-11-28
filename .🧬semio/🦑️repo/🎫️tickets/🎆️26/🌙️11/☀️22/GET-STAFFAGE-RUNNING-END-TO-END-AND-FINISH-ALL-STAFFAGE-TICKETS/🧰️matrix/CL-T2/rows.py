"""🧾️ The table rows of the report of lane CL-T2 from the logs of its runs: `python3 rows.py <GEN>` prints per written asset the digests before and after, the bytes, the number of changes, the time of the write, the triangles of the structure scenes before and after, the proofs after the write and the suites."""
import re
import sys
from pathlib import Path

gen = Path(sys.argv[1])
before = {}
for name in ["q-geo1.log", "scan1.txt", "q-try1.log"]:
    path = gen / "log" / name
    if not path.exists():
        continue
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        m = re.match(r"(\S+) scenes (.*)", line)
        if m and (m.group(1) not in before or name != "q-try1.log"):
            before.setdefault(m.group(1), dict(p.split("=") for p in m.group(2).split()))
said = (gen.parent.parent.parent / "requests.md").read_text(encoding="utf-8")
for proof in sorted((gen / "proof").glob("*.proved")):
    a = proof.stem
    text = (gen / "proof" / f"{a}.txt").read_text(encoding="utf-8")
    grab = lambda pattern: (re.search(pattern, text) or [None, "?"])[1]
    after = dict(p.split("=") for p in grab(rf"{a} scenes (.*)").split())
    was = before.get(a, {})
    scenes = " / ".join(f"{was.get(f's{s}-structure', '?')}→{after.get(f's{s}-structure', '?')}" if was.get(f"s{s}-structure") != after.get(f"s{s}-structure") else after.get(f"s{s}-structure", "?") for s in (50, 100, 200, 500))
    suites = (gen / "proof" / f"{a}-suites.txt")
    lines = suites.read_text(encoding="utf-8").splitlines() if suites.exists() else []
    last = len(lines) - 1 - next((i for i, l in enumerate(reversed(lines)) if l.startswith("suites of")), len(lines) - 1)
    run = lines[last:]
    ok = [re.sub(r" exit 0.*", "", l) for l in run if " exit 0 " in l]
    bad = [l[:160] for l in run if re.search(r" exit [1-9]", l)]
    model = next((re.search(r"findings: .*", l)[0][:120] for l in run if "model-catalog" in l), "not run")
    wrote = re.findall(rf"CL-T2 rewrote {re.escape(a)} at (\S+ \S+)", said)
    was_digest, now_digest, size, changes, cells = grab(r"catalog file before (\w+)"), grab(r"sha-256 (\w+),"), grab(r", (\d+) bytes"), grab(re.escape(a) + r": (\d+) changes"), grab(r"cells: (.*?);")
    tail = ("; " + "; ".join(bad)) if bad else ""
    print(f"| `{a}` | `{was_digest}` → `{now_digest}` ({size} bytes), {changes} changes, written {wrote[-1][11:] if wrote else '?'} | {scenes} | {cells} | {len(ok)} of {len(ok) + len(bad)} suites green{tail}; model-catalog {model} |")
