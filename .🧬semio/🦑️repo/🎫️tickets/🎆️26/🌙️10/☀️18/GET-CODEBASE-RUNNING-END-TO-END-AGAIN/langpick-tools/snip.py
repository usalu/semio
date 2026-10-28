"""🔎️ Prints context windows around language-resolution identifiers in the given files (streams, line by line)."""
import re, sys
pat = re.compile(sys.argv[1])
for path in sys.argv[2:]:
    with open(path, encoding="utf8", errors="replace") as f:
        for n, line in enumerate(f, 1):
            for m in pat.finditer(line):
                a, b = max(0, m.start() - 110), min(len(line), m.end() + 110)
                print(f"{path.split('modules/')[-1]}:{n}: {line[a:b].strip()}")
