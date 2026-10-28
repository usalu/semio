"""🧪️ Compare retained broad-run file and test-name observations without mutating source."""
from collections import Counter
from pathlib import Path
import json
import re
import sys

ticket = Path(__file__).parent
generated = ticket / '🗑️generated'

def read(name):
    """🔎️ Retain each actual Bun outcome name beneath its printed source file."""
    result = Counter()
    files = Counter()
    current = ''
    for line in (generated / name).read_text(encoding='utf-8').splitlines():
        if line.endswith('.ts:') and not line.startswith(' '):
            current = line[:-1].replace('\\', '/')
        match = re.match(r'^\((pass|skip|fail)\) (.*?)(?: \[\d+(?:\.\d+)?(?:ms|s)\])?$', line)
        if match:
            result[(current, match[1], match[2])] += 1
            files[current] += 1
    return result, files

before, old_files = read(sys.argv[1] if len(sys.argv)>1 else 'tests104.txt')
after, new_files = read('ifc4x3-and-details-tests.txt')
missing = [{'file': key[0], 'outcome': key[1], 'name': key[2], 'count': count} for key, count in (before-after).items()]
added = [{'file': key[0], 'outcome': key[1], 'name': key[2], 'count': count} for key, count in (after-before).items()]
result = {'missing': missing, 'added': added, 'beforePrinted': sum(before.values()), 'afterPrinted': sum(after.values()), 'newFileCounts': [{'file': file, 'tests': count} for file, count in new_files.items() if file not in old_files]}
(generated / ((sys.argv[2] if len(sys.argv)>2 else 'checkpoint119')+'-test-comparison.json')).write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps({'missing': len(missing), 'added': len(added), 'beforePrinted': result['beforePrinted'], 'afterPrinted': result['afterPrinted'], 'newFileCounts': result['newFileCounts']}))
