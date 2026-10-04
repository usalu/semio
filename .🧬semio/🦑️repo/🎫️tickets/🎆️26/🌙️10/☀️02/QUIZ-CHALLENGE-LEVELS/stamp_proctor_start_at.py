"""🕰️ Adds `at` to every `Command::StartRun { … challenge: X }` literal of the proctor's Rust unit tests: the decision time of
the call on the same line (`context(n)`, `}, n)`, `}, n).await`), else the named fallback of the file. Writes in place.
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[7] / "🎓️teaching" / "🛂️proctor" / "🔨️modules"
FILES = {
    ROOT / "🎭️actors" / "🧪️tests" / "🔬️unit" / "🦀️.rs": "0",
    ROOT / "🔭️projections" / "🧪️tests" / "🔬️unit" / "🦀️.rs": "0",
    ROOT / "⌨️cli" / "🧪️tests" / "🔬️unit" / "🦀️.rs": "0",
    ROOT / "🧩️instance" / "🧪️tests" / "🔬️unit" / "🦀️.rs": "0",
}
LITERAL = re.compile(r"(Command::StartRun \{[^{}]*?challenge(?:: [\w:]+)?) \}")
DECIDED = re.compile(r"^\}(?:\)|, |\), )(?:&context\()?([\w_+ ]+?)\)")

for path, fallback in FILES.items():
    lines = path.read_bytes().decode("utf-8").split("\n")
    for number, line in enumerate(lines):
        position = 0
        while (match := LITERAL.search(line, position)) is not None:
            if ", at" in match.group(1):
                position = match.end()
                continue
            rest = line[match.end() - 1 :]
            decided = DECIDED.search(rest)
            at = decided.group(1) if decided else fallback
            replaced = "%s, at: %s }" % (match.group(1), at)
            print("[DEBUG] %s:%d at %s" % (path.parent.parent.parent.name, number + 1, at))
            line = line[: match.start()] + replaced + line[match.end() :]
            position = match.start() + len(replaced)
        lines[number] = line
    payload = "\n".join(lines).encode("utf-8")
    with open(path, "r+b") as handle:
        handle.write(payload)
        handle.truncate(len(payload))
