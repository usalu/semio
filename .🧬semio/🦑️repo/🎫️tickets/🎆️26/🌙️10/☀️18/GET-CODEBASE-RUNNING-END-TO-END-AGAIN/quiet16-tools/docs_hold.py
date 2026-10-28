"""📝️ quiet16 batch `hold`, guide bullets by anchor (EN + DE, keeping the file mode): a quiet grant grows when the planner or a graph rebuild asks, in any process of the run, or when the run's processes outgrow the quiet reservation; a nested invocation never grows it; a no-op is learned quiet, so the quiet size follows what no-ops hold.
usage: python3 -I docs_hold.py check|apply|revert [--root DIR]   (exit 2 = drift)"""
import os, sys

args = sys.argv[1:]
root = os.path.abspath(args[args.index("--root") + 1]) if "--root" in args else os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
mode = args[0] if args else "check"
PATH = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md")
PAIRS = [
    ("and grows into the full request in the fair queue with the position it arrived with, once the planner, a graph rebuild or the first Bazel action that really runs needs it (`… work has to run, waiting 12s to grow the reservation from 1500 MB to 5632 MB`). A run that executed anything is learned as busy.",
     "and grows into the full request in the fair queue with the position it arrived with, once the planner, a graph rebuild or the first Bazel action that really runs needs it, in whichever process of the run they run (`… work has to run, waiting 12s to grow the reservation from 1500 MB to 5632 MB`; the run waits for that growth, and its Bazel actions then run only as wide as the jobs the grant holds, so a partial growth never runs at the full width Bazel was started with), or once the run's processes hold more than the quiet reservation (the same line; the run goes on meanwhile). A nested invocation that inherits the quiet grant (the door re-entering to start Bazel) starts at once with all its jobs and grows nothing itself. A run that asked for work or whose peak exceeded the quiet size is learned as busy, a no-op as quiet, so the quiet size follows what the no-ops of the class hold (their physical footprint plus 25 %: about 400-700 MB for the course, print and dashboard no-ops instead of the 2048 MB starting value)."),
    ("und wächst in der fairen Warteschlange mit der Position, mit der er ankam, auf die volle Anfrage, sobald der Planer, ein Graph-Neuaufbau oder die erste wirklich laufende Bazel-Aktion es braucht (`… es muss Arbeit laufen, wartet seit 12 s darauf, die Reservierung von 1500 MB auf 5632 MB zu vergroessern`). Ein Lauf, der etwas ausgeführt hat, wird als beschäftigt gelernt.",
     "und wächst in der fairen Warteschlange mit der Position, mit der er ankam, auf die volle Anfrage, sobald der Planer, ein Graph-Neuaufbau oder die erste wirklich laufende Bazel-Aktion es braucht, in welchem Prozess des Laufs sie auch laufen (`… es muss Arbeit laufen, wartet seit 12 s darauf, die Reservierung von 1500 MB auf 5632 MB zu vergroessern`; der Lauf wartet auf dieses Wachstum, und seine Bazel-Aktionen laufen danach nur so breit, wie die Zusage Jobs hält, ein teilweises Wachstum läuft also nie in der vollen Breite, mit der Bazel gestartet wurde), oder sobald die Prozesse des Laufs mehr als die ruhige Reservierung halten (dieselbe Zeile; der Lauf geht dabei weiter). Ein verschachtelter Aufruf, der die ruhige Zusage erbt (die Tür, die zum Start von Bazel erneut eintritt), startet sofort mit allen Jobs und lässt selbst nichts wachsen. Ein Lauf, der Arbeit angefragt hat oder dessen Spitze die ruhige Größe überschritt, wird als beschäftigt gelernt, ein No-op als ruhig; die ruhige Größe folgt also dem, was die No-ops der Klasse halten (ihr physischer Footprint plus 25 %: etwa 400-700 MB für die No-ops von Kurs, Druck und Dashboard statt des Startwerts von 2048 MB)."),
    ("a hot course no-op holds the 2048 MB quiet reservation and finishes in 10 s at 1.1-1.6 GB.",
     "a hot course no-op holds the quiet reservation its class learned (2048 MB before its first quiet run, about 400-700 MB after three) and finishes in 10 s; it never grows, so on a declared 16 GB machine it never waits for memory behind the reservations of other agents' builds."),
    ("ein heißer Kurs-No-op hält die ruhige Reservierung von 2048 MB und endet nach 10 s bei 1,1-1,6 GB.",
     "ein heißer Kurs-No-op hält die ruhige Reservierung, die seine Klasse gelernt hat (2048 MB vor ihrem ersten ruhigen Lauf, etwa 400-700 MB nach dreien), und endet nach 10 s; er wächst nie und wartet daher auf einer erklärten 16-GB-Maschine nie auf Speicher hinter den Reservierungen der Builds anderer Agenten."),
    ("holds only the small quiet reservation (1500 to 2048 MB, not the 5632 to 7168 MB of a busy class)",
     "holds only the small quiet reservation (what its no-ops held plus margin, 400 to 2048 MB, not the 3351 to 7168 MB of a busy class)"),
    ("hält nur die kleine ruhige Reservierung (1500 bis 2048 MB statt der 5632 bis 7168 MB einer beschäftigten Klasse)",
     "hält nur die kleine ruhige Reservierung (was seine No-ops hielten plus Marge, 400 bis 2048 MB, statt der 3351 bis 7168 MB einer beschäftigten Klasse)"),
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
