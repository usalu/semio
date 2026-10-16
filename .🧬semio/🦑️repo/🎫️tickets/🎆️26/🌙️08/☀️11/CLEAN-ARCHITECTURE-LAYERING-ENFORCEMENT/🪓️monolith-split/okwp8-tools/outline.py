"""🗂️ Item outline of a Rust file: line range, kind, visibility, name, test flag, bytes."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
from okrs import items
def outline(path):
    src = open(path, encoding='utf8').read()
    out = []
    for it in items(src):
        a = src.count('\n', 0, it['start']) + 1
        b = src.count('\n', 0, it['end']) + 1
        test = any(('test' in x and 'cfg(test)' not in x.replace(' ', '')) for x in it['attrs']) and it['kind'] == 'fn'
        out.append(dict(kind=it['kind'], name=it['name'], vis=it['vis'], a=a, b=b, test=test, bytes=len(it['text'].encode()), attrs=it['attrs']))
    return out
if __name__ == '__main__':
    for o in outline(sys.argv[1]):
        print(o['a'], o['b'], o['kind'], 'T' if o['test'] else '-', o['vis'], (o['name'] or '')[:90], o['bytes'])
