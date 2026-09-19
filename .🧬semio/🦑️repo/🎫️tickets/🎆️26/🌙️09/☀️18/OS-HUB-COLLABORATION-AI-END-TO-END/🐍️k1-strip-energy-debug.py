"""🧹 Strip the leftover temporary `[DEBUG]` probes from the energy simulation engine.

Two shapes only, both statement-complete so no `if` head can fuse onto the next line:
  1. `{ eprintln!("[DEBUG] ...); return self.begin_fault(); }` -> `{ return self.begin_fault(); }`
  2. a whole line that is exactly one `eprintln!("[DEBUG] ...);` statement -> removed
Anything else aborts, so a peer's concurrent edit cannot be silently mangled.
"""

import re
import sys

PATH = sys.argv[1]
FAULT = re.compile(r'^(\s*)\{ eprintln!\("\[DEBUG\].*?\); (return self\.begin_fault\(\); \})$')
BARE = re.compile(r'^\s*eprintln!\("\[DEBUG\].*\);$')

with open(PATH, encoding="utf-8") as handle:
    lines = handle.readlines()

out, folded, dropped, unmatched = [], 0, 0, []
for index, line in enumerate(lines, start=1):
    body = line.rstrip("\n")
    if "[DEBUG]" not in body:
        out.append(line)
        continue
    fault = FAULT.match(body)
    if fault:
        out.append(f"{fault.group(1)}{{ {fault.group(2)}\n")
        folded += 1
        continue
    if BARE.match(body):
        dropped += 1
        continue
    unmatched.append((index, body))
    out.append(line)

if unmatched:
    for index, body in unmatched:
        print(f"UNMATCHED {index}: {body}")
    raise SystemExit("aborted: unexpected [DEBUG] shape")

with open(PATH, "w", encoding="utf-8") as handle:
    handle.writelines(out)
print(f"folded={folded} dropped={dropped} total={folded + dropped}")
