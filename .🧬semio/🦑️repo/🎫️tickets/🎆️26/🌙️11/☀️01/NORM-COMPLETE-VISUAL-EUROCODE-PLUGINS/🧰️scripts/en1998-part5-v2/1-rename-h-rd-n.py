"""Renames the foundation field hRdN (declared sliding resistance H_Rd, an input) to passiveResistanceN everywhere: tokens in every text file, the mutation leaf folders and their labels. The wall field hRdNPerM is untouched. Usage: /home/deck/semio/.venv/bin/python -I 1-rename-h-rd-n.py [--dry]"""
import os, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
DRY = "--dry" in sys.argv
SUBS = [
    (re.compile(r"(?<![A-Za-z])hRdN(?![A-Za-z])"), "passiveResistanceN"),
    (re.compile(r"HRdN(?![A-Za-z])"), "PassiveResistanceN"),
    (re.compile(r"h_rd_n(?!_per_m)"), "passive_resistance_n"),
    (re.compile(r"h-rd-n(?!-per-m)"), "passive-resistance-n"),
    (re.compile(r"Sliding resistance H_Rd"), "Passive resistance at the sides"),
    (re.compile(r"Gleitwiderstand H_Rd"), "Passiver Widerstand an den Seiten"),
    (re.compile(r"Change Sliding resistance"), "Change passive resistance"),
    (re.compile(r"Gleitwiderstand ändern"), "Passiven Widerstand ändern"),
    (re.compile(r'"Sliding resistance"'), '"Passive resistance"'),
    (re.compile(r'"Gleitwiderstand"'), '"Passiver Widerstand"'),
]
changed = 0
for dp, dn, fn in os.walk(ROOT):
    dn[:] = [d for d in dn if d not in ("target", "node_modules", "__pycache__")]
    for f in fn:
        p = os.path.join(dp, f)
        try:
            s = open(p, encoding="utf-8").read()
        except (UnicodeDecodeError, OSError):
            continue
        t = s
        for rx, rep in SUBS:
            t = rx.sub(rep, t)
        if t != s:
            changed += 1
            if not DRY:
                open(p, "w", encoding="utf-8").write(t)
print("files changed", changed)
moved = 0
for dp, dn, fn in os.walk(ROOT, topdown=False):
    for d in dn:
        if "h-rd-n" in d and "per-m" not in d:
            new = os.path.join(dp, d.replace("h-rd-n", "passive-resistance-n"))
            assert len(new.encode()) - len("/home/deck/semio/") < 256, new
            moved += 1
            if not DRY:
                os.rename(os.path.join(dp, d), new)
print("folders renamed", moved)
