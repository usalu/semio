"""Rewrite self.go("key", ..., seconds=N) into self.step("key", *self.stage(...), seconds=N) so the static step-key scan sees every clause."""
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
out, index = [], 0
pattern = re.compile(r'self\.go\("(\w+)", ')
while True:
    match = pattern.search(text, index)
    if not match:
        out.append(text[index:])
        break
    out.append(text[index:match.start()])
    depth, i = 1, match.end()
    quote = None
    while depth:
        ch = text[i]
        if quote:
            if ch == "\\":
                i += 1
            elif ch == quote:
                quote = None
        elif ch in "\"'":
            quote = ch
        elif ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        i += 1
    args = text[match.end():i - 1]
    seconds = re.search(r",\s*seconds=([\d.]+)\s*$", args)
    inner = args[:seconds.start()] if seconds else args
    tail = f", seconds={seconds.group(1)}" if seconds else ""
    out.append(f'self.step("{match.group(1)}", *self.stage({inner}){tail})')
    index = i
path.write_text("".join(out), encoding="utf-8")
print("[DEBUG] rewrote", len(pattern.findall(text)), "calls")
