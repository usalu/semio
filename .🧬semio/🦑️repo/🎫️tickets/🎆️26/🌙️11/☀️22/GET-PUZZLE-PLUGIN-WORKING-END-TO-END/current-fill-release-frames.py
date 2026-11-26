from collections import Counter
from pathlib import Path
import re
import subprocess
import sys
generated = Path(__file__).resolve().parent / "🗑️generated"
name = sys.argv[1] if len(sys.argv) > 1 else "current-2d-fill-release-paid"
chunks = (generated / f"{name}.log").read_text().split("[DEBUG] physical release sample ")[1:]
stacks = [re.findall(r"history-fill-meter\(\+(0x[0-9a-f]+)", chunk) for chunk in chunks]
addresses = list(dict.fromkeys(a for stack in stacks for a in stack))
if not addresses:
    sys.exit()
exe = generated / "current-2d-hist-relink" / "history-fill-meter"
symbols = subprocess.run(["addr2line", "-C", "-f", "-e", str(exe), *addresses], check=True, capture_output=True, text=True).stdout.splitlines()[::2]
names = dict(zip(addresses, symbols))
for stack, hits in Counter(tuple(s[:20]) for s in stacks).most_common(10):
    print(hits, "samples")
    for a in stack[1:]:
        print(names[a][:235])
