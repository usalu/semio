"""🧪️ Session-2 W1-E (G6): ends every wgpu `UiSliderNode`/`UiNumberStepperNode`/`UiInputNode` struct literal with
`..Default::default()` so the numeric facets added to those nodes (appearance, scale, precision, display unit and factor,
limits, stepper detents and unit) never ripple into call sites again. Patterns already end in `..`; a literal that already
has a base is left alone. Prints every site it touches.

Run from the repo root: `python3 .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-w1e-wgpu-node-literals.py`.
"""
import re
import subprocess

NAMES = ("UiSliderNode", "UiNumberStepperNode", "UiInputNode")


def literal_end(text, start):
    depth = 0
    index = start
    while index < len(text):
        char = text[index]
        if char in "{([":
            depth += 1
        elif char in "})]":
            depth -= 1
            if depth == 0:
                return index
        elif char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        index += 1
    raise ValueError("unbalanced literal")


def top_level(body):
    depth, out, index = 0, [], 0
    while index < len(body):
        char = body[index]
        if char == '"':
            index += 1
            while body[index] != '"':
                index += 2 if body[index] == "\\" else 1
        elif char in "{([":
            depth += 1
        elif char in "})]":
            depth -= 1
        elif depth == 0:
            out.append(char)
        index += 1
    return "".join(out)


def main():
    files = subprocess.run(["git", "grep", "-l", "--untracked", "-E", "(UiSliderNode|UiNumberStepperNode|UiInputNode) [{]", "--", "*.rs"], capture_output=True, text=True, check=True).stdout.splitlines()
    for path in files:
        if path.startswith(".🧬semio"):
            continue
        text = open(path, encoding="utf-8").read()
        edits = []
        for match in re.finditer(r"\b(UiSliderNode|UiNumberStepperNode|UiInputNode) \{", text):
            if text[max(0, match.start() - 11):match.start()].endswith("pub struct "):
                continue
            open_brace = match.end() - 1
            close = literal_end(text, open_brace)
            body = text[open_brace + 1:close]
            if ".." in top_level(body):
                continue
            stripped = body.rstrip()
            insertion = ", ..Default::default()" if not stripped.endswith(",") else " ..Default::default()"
            if "\n" in body:
                indent = re.search(r"\n([ \t]*)\S", body).group(1)
                insertion = ("," if not stripped.endswith(",") else "") + "\n" + indent + "..Default::default()"
            edits.append((open_brace + 1 + len(stripped), insertion))
            print(f"{path}:{text.count(chr(10), 0, match.start()) + 1}: {match.group(1)}")
        for position, insertion in sorted(edits, reverse=True):
            text = text[:position] + insertion + text[position:]
        if edits:
            open(path, "w", encoding="utf-8").write(text)


main()
