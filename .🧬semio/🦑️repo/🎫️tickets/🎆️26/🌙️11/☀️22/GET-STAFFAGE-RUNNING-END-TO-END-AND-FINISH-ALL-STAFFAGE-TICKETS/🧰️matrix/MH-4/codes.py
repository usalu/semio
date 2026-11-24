"""🧾️ A gate log in short: `python3 codes.py <gate log>…` prints per asset and scale the hard codes with their count, worst delta and the bones or anchors concerned (count and worst delta each)."""
import re, sys
for path in sys.argv[1:]:
    head = None
    for line in open(path):
        if line.startswith("    "):
            if head: head[1].append(re.sub(r" @ .*", "", line.strip()))
            continue
        if head: print(head[0], "|", "; ".join(head[1])[:int(sys.argv[0] and 400)])
        head = None
        m = re.match(r"(\S+) 1:(\d+) HARD (\S+) (×\d+) worst (\d+) mm", line)
        if m: head = [f"{m.group(1)} 1:{m.group(2)} {m.group(3)} {m.group(4)} {m.group(5)}mm", []]
    if head: print(head[0], "|", "; ".join(head[1])[:400])
