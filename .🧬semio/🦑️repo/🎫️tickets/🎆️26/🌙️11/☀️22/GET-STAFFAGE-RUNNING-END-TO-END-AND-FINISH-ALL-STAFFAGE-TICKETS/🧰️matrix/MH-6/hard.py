"""🧮️ The hard findings of a gate log in short: `python3 hard.py <gate.log> [--twin]` prints per scale and code the keys (bone or anchor@view) with count, worst delta and subject, and the last line of the log; `--twin` adds the twin tier."""
import re, sys
tiers = ("HARD", "twin") if "--twin" in sys.argv else ("HARD",)
show = False
for line in open(sys.argv[1]):
    line = line.rstrip("\n")
    m = re.match(r"^(\S+) 1:(\d+) (HARD|twin|reported) (\S+) ×(\d+) worst (\d+) mm", line)
    if m:
        show = m.group(3) in tiers
        if show: print(f"1:{m.group(2)} {m.group(3)} {m.group(4)} ×{m.group(5)} {m.group(6)} mm :: {line.split(': ', 1)[1][:150]}")
    elif line.startswith("    ") and show: print("     ", line.strip()[:110])
    elif line.startswith("hard findings"): print(line)
    else: show = False
