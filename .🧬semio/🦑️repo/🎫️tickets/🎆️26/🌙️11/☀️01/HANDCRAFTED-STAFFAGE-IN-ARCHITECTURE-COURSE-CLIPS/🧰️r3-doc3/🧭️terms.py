"""🧭️ Terminology parity: map every short English string to its German renderings and list English strings with more than one German rendering, plus German strings with more than one English source."""
import collections
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')
TSV = sys.argv[1]
MAXLEN = int(sys.argv[2]) if len(sys.argv) > 2 else 60
rows = []
with open(TSV, encoding='utf-8') as handle:
    for line in handle:
        rel, path, en, de = line.rstrip('\n').split('\t')
        rows.append((rel, path, en, de))

to_de = collections.defaultdict(collections.Counter)
to_en = collections.defaultdict(collections.Counter)
where = {}
for rel, path, en, de in rows:
    if len(en) > MAXLEN:
        continue
    key_en = re.sub(r'\s+', ' ', en.strip())
    key_de = re.sub(r'\s+', ' ', de.strip())
    to_de[key_en][key_de] += 1
    to_en[key_de][key_en] += 1
    where.setdefault((key_en, key_de), (rel, path))

print('[DEBUG] === en with multiple de renderings: %d' % sum(1 for k, v in to_de.items() if len(v) > 1))
for en, counter in sorted(to_de.items(), key=lambda kv: kv[0].lower()):
    if len(counter) > 1:
        print('[DEBUG] EN %r -> %s' % (en, ' | '.join('%r x%d' % (d, c) for d, c in counter.most_common(6))))
print('[DEBUG] === de with multiple en sources: %d' % sum(1 for k, v in to_en.items() if len(v) > 1))
for de, counter in sorted(to_en.items(), key=lambda kv: kv[0].lower()):
    if len(counter) > 1:
        print('[DEBUG] DE %r <- %s' % (de, ' | '.join('%r x%d' % (e, c) for e, c in counter.most_common(6))))
