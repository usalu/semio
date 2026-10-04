"""🕰️ Adds `at` to every `{ type: "start-run", … challenge: … }` object literal of the React suites: in the deputy suite the
decision time passed beside it (`}, n)`), in the journey suite the proctor double's clock (`proctor.now`), elsewhere a
fixed instant of the suite. A literal that already carries `at`, or whose challenge is deliberately broken, is left alone.
Prints every stamp; writes through the existing file handle.
"""
import pathlib
import re

TESTS = pathlib.Path(__file__).resolve().parents[7] / "🧰️framework" / "🛍️products" / "❓️quiz" / "🧪️tests"
FILES = {"🫡️deputy-decisions": None, "📬️outbox-delivery": "1_000", "🚶️learner-journey": "proctor.now"}
LITERAL = re.compile(r'(\{ type: "start-run", [^{}]*?challenge: (?:"[a-z]+"|\w+)) \}')
DECIDED = re.compile(r"^\}, ([\d_]+)\)")

for case, fallback in FILES.items():
    path = TESTS / case / "🟦️.tsx"
    lines = path.read_bytes().decode("utf-8").split("\n")
    for number, line in enumerate(lines):
        position = 0
        while (match := LITERAL.search(line, position)) is not None:
            if " at: " in match.group(1) or '"lenient"' in match.group(1):
                position = match.end()
                continue
            decided = DECIDED.search(line[match.end() - 1 :])
            at = decided.group(1) if decided else fallback
            replaced = "%s, at: %s }" % (match.group(1), at)
            print("[DEBUG] %s:%d at %s" % (case, number + 1, at))
            line = line[: match.start()] + replaced + line[match.end() :]
            position = match.start() + len(replaced)
        lines[number] = line
    payload = "\n".join(lines).encode("utf-8")
    with open(path, "r+b") as handle:
        handle.write(payload)
        handle.truncate(len(payload))
