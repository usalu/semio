from pathlib import Path
import re
for path in Path('✏️s').rglob('*.rs'):
    text=path.read_text(encoding='utf-8-sig')
    for match in re.finditer(r'\bmatch\s+[^\n{]+\{', text):
        start=match.end()-1
        depth=1
        end=start+1
        while end<len(text) and depth:
            depth+=(text[end]=='{')-(text[end]=='}')
            end+=1
        body=text[start:end]
        for name in ['IfcValue','Part21Value','StepValue']:
            variants=set(re.findall(name+r'::(\w+)',body))
            if len(variants)>5 and 'Binary' not in variants and not re.search(r'\b_\s*=>',body):
                print(f'{path}:{text[:start].count(chr(10))+1}: {sorted(variants)}')
