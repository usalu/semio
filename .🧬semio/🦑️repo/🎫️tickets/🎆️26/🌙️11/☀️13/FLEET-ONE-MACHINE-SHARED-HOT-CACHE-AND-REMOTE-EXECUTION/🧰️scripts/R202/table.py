import re,sys
S='/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R202/samples.txt'
blocks=re.split(r'\n(?=\d\d:\d\d:\d\d \| phase)',open(S,encoding='utf8').read())
rows=[];last=None
for b in blocks:
    m=re.match(r'(\d\d:\d\d:\d\d) \| phase: (.*)',b)
    if not m: continue
    t=m.group(1);ph=m.group(2)
    if 'shared closures' in ph: p='prime'
    elif 'Inferring projects' in ph: p='infer'
    elif 'cargo' in ph or 'dependencies' in ph or 'Listing' in ph or 'repository:command' in ph: p='start'
    else: p='read'
    l=re.search(r'ledger: (\d+) of (\d+) done: (\d+) present, (\d+) fetched from the fleet, (\d+) inferred here \((\d+) running\), (\d+) wanted from the fleet, (\d+) stale',b)
    led=f'{l.group(1)}/{l.group(2)} · {l.group(3)}/{l.group(4)}/{l.group(5)}/{l.group(8)} · run {l.group(6)}' if l else ''
    r=re.search(r'rate ([\d.]+)/min over (\d+)min',b); rate=f'{r.group(1)}' if r else ''
    ch=re.search(r'children\(pid age cpu mem\): (.*)',b); n=len(re.findall(r'\d+ \d\d:\d\d(?::\d\d)? [\d.]+% \d+MB',ch.group(1))) if ch else ''
    sp=re.search(r'(\d+) MB spare',b); rv=re.search(r'show: 1 job\(s\), (\d+) MB reserved, (\d+) MB observed',b)
    bk=f'{rv.group(1)}/{rv.group(2)}' if rv else ''
    po=re.search(r'children \[(\d+), (\d+), (\d+)\]',b); pool=f'{po.group(1)}/{po.group(2)}/{po.group(3)}c' if po else ''
    ld=re.search(r'load \(([\d.]+)',b)
    rows.append(f'| {t} | {p} | {led} | {rate} | {n} | {bk} | {sp.group(1) if sp else ""} | {pool} | {ld.group(1)[:4] if ld else ""} |')
print('| time | phase | done/total · present/fetched/here/stale | per min (10 min) | procs below farm door | show reserved/observed MB | spare MB | fleet wave claimed/published/children | load |')
print('|---|---|---|---|---|---|---|---|---|')
print('\n'.join(rows))
