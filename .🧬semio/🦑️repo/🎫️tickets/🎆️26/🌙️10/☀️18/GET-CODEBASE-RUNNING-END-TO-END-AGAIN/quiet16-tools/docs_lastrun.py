"""📝️ quiet16 batch `lastrun` (after docs_hold.py), guide bullets by anchor (EN + DE, keeping the file mode): the quiet forecast follows the class's own last outcome; a class whose last run executed nothing stays quiet however long ago it ran, one whose last run executed work only within the follow-up window.
usage: python3 -I docs_lastrun.py check|apply|revert [--root DIR]   (exit 2 = drift)"""
import os, sys

args = sys.argv[1:]
root = os.path.abspath(args[args.index("--root") + 1]) if "--root" in args else os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
mode = args[0] if args else "check"
PATH = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md")
PAIRS = [
    ("whose class mostly executed nothing holds only a small reservation (`[capacity] run: forecast quiet from 5 earlier run(s) of its class (100% of them executed nothing): 1500 MB reserved now, the full 5632 MB only if work has to run`)",
     "whose class's last run executed nothing, however long ago, holds only a small reservation (`[capacity] run: forecast quiet because the last run of its class executed nothing (3 quiet run(s) behind its size, 100% of the recent runs): 640 MB reserved now, the full 5632 MB only if work has to run`)"),
    ("dessen Klasse meist nichts ausgeführt hat, hält nur eine kleine Reservierung (`[capacity] run: ruhig prognostiziert aus 5 frueheren Laeufen seiner Klasse (100% fuehrten nichts aus): jetzt 1500 MB reserviert, die vollen 5632 MB nur, wenn Arbeit laufen muss`)",
     "dessen letzter Lauf seiner Klasse nichts ausführte, gleich wie lange her, hält nur eine kleine Reservierung (`[capacity] run: ruhig prognostiziert, weil der letzte Lauf seiner Klasse nichts ausführte (3 ruhige Läufe hinter ihrer Größe, 100% der jüngsten Läufe): jetzt 640 MB reserviert, die vollen 5632 MB nur, wenn Arbeit laufen muss`)"),
    ("a finished run of its class within 30 minutes is forecast quiet even when the class has little quiet history, because the finished run left its results cached",
     "a finished run of its class that executed work (an edit) within 30 minutes is forecast quiet too, because the finished run left its results cached"),
    ("der Lauf, der innerhalb von 30 Minuten auf einen beendeten Lauf seiner Klasse folgt, gilt auch bei wenig ruhiger Vorgeschichte als ruhig, weil der beendete Lauf seine Ergebnisse zwischengespeichert hat",
     "der Lauf, der innerhalb von 30 Minuten auf einen beendeten Lauf seiner Klasse folgt, der Arbeit ausführte (eine Änderung), gilt ebenfalls als ruhig, weil der beendete Lauf seine Ergebnisse zwischengespeichert hat"),
    ("A class whose last run ended more than 30 minutes ago is asked for the full grant again: run the build twice.",
     "A class whose last run executed work and ended more than 30 minutes ago is asked for the full grant again (one whose last run executed nothing stays quiet however long ago it ran)."),
    ("Endete der letzte Lauf einer Klasse vor mehr als 30 Minuten, wird wieder die volle Zusage verlangt: den Build zweimal starten.",
     "Führte der letzte Lauf einer Klasse Arbeit aus und endete vor mehr als 30 Minuten, wird wieder die volle Zusage verlangt (führte er nichts aus, bleibt die Klasse ruhig, gleich wie lange her)."),
]


def write(value, fmode):
    temporary = PATH + ".quiet16.tmp"
    open(temporary, "w", encoding="utf8").write(value)
    os.chmod(temporary, fmode)
    os.replace(temporary, PATH)


text = open(PATH, encoding="utf8").read()
fmode = os.stat(PATH).st_mode & 0o7777
applied = all(new in text for _, new in PAIRS)
pending = all(text.count(old) == 1 for old, _ in PAIRS)
if mode == "check":
    print("applied" if applied else "pending" if pending else "drift")
    sys.exit(0 if applied or pending else 2)
if mode == "apply":
    if applied: print("already applied"); sys.exit(0)
    if not pending: print("drift"); sys.exit(2)
    for old, new in PAIRS: text = text.replace(old, new)
    write(text, fmode); print("applied")
elif mode == "revert":
    if not applied: print("not applied"); sys.exit(0)
    for old, new in PAIRS: text = text.replace(new, old)
    write(text, fmode); print("reverted")
else:
    sys.exit("verb: check|apply|revert")
