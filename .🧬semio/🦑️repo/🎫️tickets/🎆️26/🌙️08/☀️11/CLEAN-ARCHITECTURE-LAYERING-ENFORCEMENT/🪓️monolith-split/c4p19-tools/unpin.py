"""📌 Removes the lib-wiring ratchet pins of the given roots (each must now be at most the 150-line budget). usage: unpin.py <repo-relative root>..."""
import sys, json
R = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json'
text = open(R, encoding='utf-8').read()
lines = text.split('\n')
for root in sys.argv[1:]:
    n = len(open('/Users/ueli/Documents/semio/' + root, encoding='utf-8').read().split('\n'))
    assert n <= 150, (root, n)
    key = '    "%s": ' % root
    hit = [i for i, l in enumerate(lines) if l.startswith(key)]
    assert len(hit) == 1, (root, hit)
    del lines[hit[0]]
    print('unpinned', root[-60:], n)
out = '\n'.join(lines)
json.loads(out)
open(R, 'w', encoding='utf-8').write(out)
