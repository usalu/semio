#!/usr/bin/env python3
"""🔎 Diagnostic-only scanner (never edits) for silently-dropped `()`-returning async-fn calls used
in statement position inside 🧰️framework/🔨️modules/🎭️actor/🦀️component.rs. Complements
insert-await.py: rustc emits no diagnostic for a dropped Future<Output=()>, so these bugs are
invisible to a diagnostic-driven tool and must be hand-reviewed. This script only lists candidates;
every fix is applied by hand via Edit, per R10."""
import re
import sys

path = sys.argv[1] if len(sys.argv) > 1 else "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️component.rs"
text = open(path, encoding="utf-8").read()
lines = text.split("\n")

# Collect names of async fns defined in this file (free fns and methods) whose declared return type
# is empty (i.e. `)  {` immediately, no `->`), which is the unit-returning, silently-droppable class.
unit_async_names = set()
non_unit_async_names = set()
fn_re = re.compile(r'(?:pub(?:\([^)]*\))?\s+)?async\s+fn\s+(\w+)\s*\(')
for m in fn_re.finditer(text):
    name = m.group(1)
    # find the matching closing paren for the fn's parameter list, then check what follows
    start = m.end() - 1  # position of the '('
    depth = 0
    i = start
    while i < len(text):
        if text[i] == '(':
            depth += 1
        elif text[i] == ')':
            depth -= 1
            if depth == 0:
                break
        i += 1
    rest = text[i+1:i+40].lstrip()
    if rest.startswith('->'):
        non_unit_async_names.add(name)
    else:
        unit_async_names.add(name)

print(f"# unit-returning async fn names ({len(unit_async_names)}):", sorted(unit_async_names))
print(f"# non-unit async fn names ({len(non_unit_async_names)}) [not scanned — already surfaced as type errors]")
print()

# Now scan line-by-line for statement-position calls to unit_async_names lacking `.await` before `;`
call_re = re.compile(r'(?:\b(\w+)::)?\.?\b(' + '|'.join(re.escape(n) for n in unit_async_names) + r')\s*\(')
for lineno, line in enumerate(lines, start=1):
    stripped = line.strip()
    # crude statement-position heuristic: line ends with ");" (possibly with trailing stuff we ignore)
    if not stripped.endswith(');'):
        continue
    for m in re.finditer(r'\b(' + '|'.join(re.escape(n) for n in unit_async_names) + r')\s*\(', line):
        name = m.group(1)
        # find matching close paren from this '('
        open_idx = m.end() - 1
        depth = 0
        j = open_idx
        while j < len(line):
            if line[j] == '(':
                depth += 1
            elif line[j] == ')':
                depth -= 1
                if depth == 0:
                    break
            j += 1
        after = line[j+1:j+8]
        if after.strip().startswith('.await'):
            continue
        # skip if this call is itself nested as an argument inside another call already flagged
        print(f"{lineno}:{m.start()+1}: {stripped}")
