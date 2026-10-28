"""🪟 Skim a product log: drop rustc warning blocks, progress JSON and heartbeat lines, keep line numbers."""
import re, sys

drop = re.compile(r'^\s*(\||=|help:|-->|\d+\s*[-+|]|\.\.\.)|running elapsedMs=|^\[script\] \{|Checking source configuration|Reading sources|Source admitted|^\s*$')
inwarn = False
for n, line in enumerate(open(sys.argv[1], errors='replace'), 1):
    line = line.rstrip('\n')
    if re.match(r'^(warning|error)(\[\w+\])?: ', line):
        if re.match(r'^warning: unnecessary qualification|^warning: unused', line):
            inwarn = True
            continue
        inwarn = False
    if inwarn:
        if line == '' or re.match(r'^\s', line) or drop.search(line):
            continue
        inwarn = False
    if drop.search(line):
        continue
    print(f"{n}: {line[:int(sys.argv[2]) if len(sys.argv) > 2 else 300]}")
