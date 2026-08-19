#!/usr/bin/env python3
"""Append .expect("valid artifact store fixture") to ArtifactStore::new(...) calls missing Result handling."""

import re
import sys
from pathlib import Path

EXPECT = '.expect("valid artifact store fixture")'
SKIP_MARKERS = (
    '.expect(',
    '.unwrap(',
    '.is_ok()',
    '.is_err()',
    '.map_err(',
    'match ',
    'assert!(matches!',
)


def needs_fix(chunk: str) -> bool:
    if '::new(' not in chunk or 'ArtifactStore' not in chunk:
        return False
    if any(marker in chunk for marker in SKIP_MARKERS):
        return False
    if re.search(r'::new\([^;]*\)\?', chunk):
        return False
    return True


def fix_chunk(chunk: str) -> str:
    if chunk.rstrip().endswith(';'):
        return re.sub(r'\)(\s*);$', f'){EXPECT}\\1;', chunk.rstrip(), count=1) + (
            '\n' if chunk.endswith('\n') else ''
        )
    stripped = chunk.rstrip()
    return stripped + EXPECT + '\n'


def process_file(path: Path) -> int:
    text = path.read_text()
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    fixes = 0
    while i < len(lines):
        if 'ArtifactStore' in lines[i] and '::new(' in lines[i]:
            j = i
            depth = lines[i].count('(') - lines[i].count(')')
            while depth > 0 and j + 1 < len(lines):
                j += 1
                depth += lines[j].count('(') - lines[j].count(')')
            chunk = ''.join(lines[i:j + 1])
            if needs_fix(chunk):
                fixed = fix_chunk(chunk)
                fixes += 1
                out.append(fixed)
            else:
                out.extend(lines[i:j + 1])
            i = j + 1
            continue
        out.append(lines[i])
        i += 1
    if fixes:
        path.write_text(''.join(out))
    return fixes


if __name__ == '__main__':
    root = Path(sys.argv[1])
    total = 0
    for rel in sys.argv[2:]:
        count = process_file(root / rel)
        print(f'{count:4d} {rel}')
        total += count
    print(f'total fixes: {total}')
