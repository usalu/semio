#!/usr/bin/env python3
"""Fix section summaries that were placed BEFORE #region instead of AFTER."""

import re
from pathlib import Path

FILE = Path("/workspaces/semio/semio-repo/cli/main.go")

lines = FILE.read_text().split('\n')
new_lines = []
i = 0
fixes = 0

while i < len(lines):
    if (i + 1 < len(lines) and 
        lines[i].startswith('// ') and 
        not lines[i].startswith('// #') and
        not lines[i].startswith('// Spec') and
        '// #region' in lines[i + 1]):
        summary = lines[i]
        region = lines[i + 1]
        new_lines.append(region)
        new_lines.append(summary)
        fixes += 1
        i += 2
    else:
        new_lines.append(lines[i])
        i += 1

FILE.write_text('\n'.join(new_lines))
print(f"Fixed {fixes} section summaries (moved after #region).")
