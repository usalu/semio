import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from restructure import *

for lang in ['go', 'ts', 'py', 'cs', 'rs']:
    path, start_re, end_re, _ = FILES[lang]
    with open(path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    if lang == 'rs':
        sections = parse_top_level_sections_rs(lines)
    else:
        sections = parse_top_level_sections_generic(lines, start_re, end_re)

    indices = []
    for item in sections:
        if item[0] == 'section':
            idx = get_order_key(item[1])
            indices.append((idx, strip_emoji(item[1])))

    ok = True
    for i in range(1, len(indices)):
        if indices[i][0] < indices[i-1][0]:
            ok = False
            print(f'{lang.upper()}: ORDER VIOLATION: [{indices[i-1][0]}] {indices[i-1][1]} -> [{indices[i][0]}] {indices[i][1]}')

    if ok:
        print(f'{lang.upper()}: All sections in canonical order')
