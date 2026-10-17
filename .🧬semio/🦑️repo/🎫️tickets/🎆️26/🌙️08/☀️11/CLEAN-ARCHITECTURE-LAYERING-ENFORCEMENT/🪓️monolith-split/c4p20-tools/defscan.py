import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p20 as pfa

tag = sys.argv[1]
root = sys.argv[2] if len(sys.argv) > 2 else os.path.join(pfa.SCR, 'stage', tag)
sp = importlib.import_module('specs.' + tag).SPEC
files = {'__parent__': os.path.join(root, '🦀️.rs')}
for c in sp['children']:
    files[c['folder']] = os.path.join(root, c['folder'], '🦀️.rs')
texts = {k: open(v, encoding='utf-8').read() for k, v in files.items()}
ext = ''.join(open(os.path.join(pfa.REPO, p), encoding='utf-8').read() for p in sp.get('external', []))
ids = {k: pfa.ident_set(t) for k, t in texts.items()}
extids = pfa.ident_set(ext)
hits = []
count = 0
RX_TOP = re.compile(r'^(?:(?:async|unsafe|const|extern(?: "[^"]*")?)\s+)*(fn|const|static|struct|enum|trait|type|union|mod)\s+(\w+)')
RX_FIELD = re.compile(r'^ {4}(\w+)\s*:\s')
RX_MEM = re.compile(r'^ {4}(?:(?:async|unsafe|const|default)\s+)*(fn|const|type)\s+(\w+)')
for k, t in texts.items():
    if k == '__parent__':
        continue
    others = set(extids)
    for g, s in ids.items():
        if g != k:
            others |= s
    mode = None
    for ln, line in enumerate(t.split('\n'), 1):
        if re.match(r'^\S', line) and not line.startswith('//') and not line.startswith('#['):
            mode = None
            m = RX_TOP.match(line)
            if m:
                count += 1
                if m.group(2) in others and not line.startswith('pub'):
                    hits.append((k, ln, 'item', m.group(2)))
            if re.match(r'^struct\s+\w+[^;]*\{\s*$', line) or re.match(r'^struct\s+\w+<.*\{\s*$', line):
                mode = 'struct'
            elif re.match(r'^impl\b', line) and ' for ' not in line.split('{')[0] and line.rstrip().endswith('{'):
                mode = 'impl'
            elif re.match(r'^impl\b', line) and ' for ' not in line and not line.rstrip().endswith('{'):
                mode = 'impl-open'
            continue
        if mode == 'impl-open':
            if line.rstrip().endswith('{'):
                mode = 'impl'
            continue
        if mode == 'struct':
            m = RX_FIELD.match(line)
            if m:
                count += 1
                if m.group(1) in others:
                    hits.append((k, ln, 'field', m.group(1)))
        elif mode == 'impl':
            m = RX_MEM.match(line)
            if m:
                count += 1
                if m.group(2) in others:
                    hits.append((k, ln, 'member', m.group(2)))
print(tag, 'private declarations scanned:', count, 'cross-file leftovers:', len(hits))
for h in hits[:30]:
    print('  LEFTOVER', h)
sys.exit(1 if hits else 0)
