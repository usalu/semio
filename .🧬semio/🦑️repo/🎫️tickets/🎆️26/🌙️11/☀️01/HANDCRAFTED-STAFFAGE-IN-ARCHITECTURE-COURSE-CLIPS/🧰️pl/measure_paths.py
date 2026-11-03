"""📏️ Measures full Windows path lengths in UTF-16 code units for the round 2 path audit."""
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")

ROOT = "C:/git/semio"
PREFIX = "C:\\git\\semio\\"
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
STAFFAGE = "🎓️teaching/🏛️architecture/🧍️staffage"
VISUALS = "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals"
LIST = TICKET + "/r2-changed-files.md"
WARN, MAX = 240, 256
OUT = os.path.join(ROOT, TICKET, "🗑️generated", "pl", "paths.tsv")


def units(s):
    return len(s.encode("utf-16-le")) // 2


def rel_of(full):
    return os.path.relpath(full, ROOT).replace("\\", "/")


def walk(sub):
    for dp, dns, fns in os.walk(os.path.join(ROOT, sub)):
        for f in fns:
            yield rel_of(os.path.join(dp, f))


scopes = {}


def add(rel, scope):
    scopes.setdefault(rel, set()).add(scope)


listed = 0
with open(os.path.join(ROOT, LIST), encoding="utf-8") as fh:
    for line in fh:
        m = re.match(r"^- (?:[A-Z] )?`(.+?)`\s*$", line)
        if m:
            listed += 1
            add(m.group(1).replace("\\", "/"), "listed")

for rel in walk(TICKET):
    add(rel, "ticket")
for rel in walk(STAFFAGE):
    add(rel, "staffage")
for rel in walk(VISUALS):
    add(rel, "visuals")

rows = []
for rel, src in scopes.items():
    full = PREFIX + rel.replace("/", "\\")
    rows.append((units(full), rel, ",".join(sorted(src)), os.path.exists(os.path.join(ROOT, rel))))
rows.sort(key=lambda r: (-r[0], r[1]))

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
    fh.write("units\tviolation\texists\tscope\tpath\n")
    for u, rel, src, ex in rows:
        fh.write(f"{u}\t{'VIOLATION' if u > MAX else ('warn' if u > WARN else 'ok')}\t{ex}\t{src}\t{rel}\n")

print(f"listed lines parsed: {listed}, unique paths: {len(rows)}")
for name in ("listed", "ticket", "staffage", "visuals"):
    sub = [r for r in rows if name in r[2].split(",")]
    print(f"{name}: total={len(sub)} >240={sum(r[0] > WARN for r in sub)} >256={sum(r[0] > MAX for r in sub)} max={max((r[0] for r in sub), default=0)}")
print(f"listed but missing on disk (deleted): {sum(1 for r in rows if 'listed' in r[2] and not r[3])}")
print(f"ALL: >240={sum(r[0] > WARN for r in rows)} >256={sum(r[0] > MAX for r in rows)} max={rows[0][0] if rows else 0}")
print("--- worst 15 ---")
for u, rel, src, ex in rows[:15]:
    print(f"{u}\t{'VIOLATION' if u > MAX else 'warn'}\t{src}\t{rel}")
print("--- all over 240 ---")
for u, rel, src, ex in rows:
    if u > WARN:
        print(f"{u}\t{src}\t{rel}")
