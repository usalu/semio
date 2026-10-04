"""🕰️ Adds the `at` of `start-run` to the Rust unit-test helpers `command_start(_at)` of the quiz crate: each call takes the
decision time of the context it is decided with, so every test keeps its run starting at the decision time it had.
Prints the lines it could not resolve; those are edited by hand.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7] / "🧰️framework" / "🛍️products" / "❓️quiz" / "🔨️modules"
FILES = [ROOT / "🧾️lifecycle" / "🧪️tests" / "🔬️unit" / "🦀️.rs", ROOT / "👁️views" / "🧪️tests" / "🔬️unit" / "🦀️.rs"]
CALL = re.compile(r"command_start(_at)?\(([^()]*?(?:\([^()]*\))?[^()]*)\)(, )(&(\w+)\)|&at\(([\d_]+)\)|&LearnerContext \{ now: ([\d_]+),|&LearnerContext \{ limits: &limits, \.\.(\w+) \}|(\d+)\))")


def stamped(match):
    kind, arguments, comma, tail = match.group(1) or "", match.group(2), match.group(3), match.group(4)
    if arguments.count(",") >= (3 if kind else 2):
        return match.group(0)
    now = f"{match.group(5)}.now" if match.group(5) else match.group(6) or match.group(7) or (f"{match.group(8)}.now" if match.group(8) else match.group(9))
    return f"command_start{kind}({arguments}, {now}){comma}{tail}"


for path in FILES:
    text = path.read_text(encoding="utf-8")
    lines = text.split("\n")
    for number, line in enumerate(lines):
        if "command_start" in line and "fn command_start" not in line:
            changed = CALL.sub(stamped, line)
            if changed == line:
                print(f"[unresolved] {path.parent.parent.parent.name}:{number + 1}: {line.strip()}", file=sys.stderr)
            lines[number] = changed
    path.write_bytes("\n".join(lines).encode("utf-8"))
