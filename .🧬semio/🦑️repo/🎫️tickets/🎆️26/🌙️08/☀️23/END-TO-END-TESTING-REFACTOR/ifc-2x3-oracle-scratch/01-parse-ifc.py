import re, sys, json

PATH = 'temp/wellness-center-sama.ifc'

def strip_strings(s):
    out = []
    i = 0
    n = len(s)
    in_str = False
    while i < n:
        c = s[i]
        if in_str:
            if c == "'":
                # check doubled '' escape
                if i + 1 < n and s[i+1] == "'":
                    i += 2
                    continue
                in_str = False
                i += 1
                continue
            i += 1
            continue
        else:
            if c == "'":
                in_str = True
                i += 1
                continue
            out.append(c)
            i += 1
    return ''.join(out)

REF_RE = re.compile(r'#(\d+)')
LINE_RE = re.compile(r'^#(\d+)\s*=\s*([A-Za-z0-9_]+)\s*\((.*)\)\s*;\s*$')

id2name = {}
id2line = {}
id2refs = {}

with open(PATH, 'r', encoding='utf-8', errors='replace') as f:
    in_data = False
    for line in f:
        st = line.rstrip('\n')
        stripped = st.strip()
        if stripped == 'DATA;':
            in_data = True
            continue
        if stripped == 'ENDSEC;' and in_data:
            break
        if not in_data or not stripped:
            continue
        m = LINE_RE.match(stripped)
        if not m:
            print('UNMATCHED LINE:', stripped[:200])
            continue
        eid = int(m.group(1))
        name = m.group(2)
        args = m.group(3)
        id2name[eid] = name
        id2line[eid] = stripped
        masked = strip_strings(args)
        refs = set(int(x) for x in REF_RE.findall(masked))
        id2refs[eid] = refs

print('total entities', len(id2name))

from collections import Counter
namecounts = Counter(id2name.values())
print('IFCBUILDINGSTOREY count', namecounts.get('IFCBUILDINGSTOREY', 0))
print('IFCBUILDING count', namecounts.get('IFCBUILDING', 0))
print('IFCSITE count', namecounts.get('IFCSITE', 0))
print('IFCPROJECT count', namecounts.get('IFCPROJECT', 0))
print('IFCRELCONTAINEDINSPATIALSTRUCTURE count', namecounts.get('IFCRELCONTAINEDINSPATIALSTRUCTURE', 0))
print('IFCRELAGGREGATES count', namecounts.get('IFCRELAGGREGATES', 0))

storeys = [i for i, n in id2name.items() if n == 'IFCBUILDINGSTOREY']
for sid in storeys:
    print('storey', sid, id2line[sid][:150])

# save intermediate data for next script via pickle
import pickle
with open('/private/tmp/claude-501/-Users-ueli-Documents-semio/34f3999f-e145-4d4e-ab13-c3c2aef22ddf/scratchpad/ifc_parsed.pkl', 'wb') as f:
    pickle.dump({'id2name': id2name, 'id2line': id2line, 'id2refs': id2refs}, f)
print('saved pickle')
