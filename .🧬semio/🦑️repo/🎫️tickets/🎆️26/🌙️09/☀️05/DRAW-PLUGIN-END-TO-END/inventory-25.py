from pathlib import Path
import json, re, datetime
root=Path.cwd()
ticket=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END')
inputs=['changed-files.md','root-owned-files.md','editor.md','guest.md','release-registry-25.md']
origins={}
for name in inputs:
    for line in (ticket/name).read_text().splitlines():
        if line.startswith('#'): continue
        candidates=re.findall(r'`([^`]+)`',line)
        plain=line.removeprefix('- ').split(' — ',1)[0].strip()
        if plain.startswith(('🧰','✏','🌎','🎓','♻','🏢','👴','.🧬')): candidates.append(plain)
        for p in candidates:
            if not p.startswith(('🧰','✏','🌎','🎓','♻','🏢','👴','.🧬')) or '*' in p or '<' in p or ' ' in p: continue
            if '/🗑️generated/' in p or p.startswith('.cache/'): continue
            if p.endswith(('/','...')): continue
            origins.setdefault(p,set()).add(name)
for p in ticket.iterdir():
    if p.is_file() and datetime.datetime.fromtimestamp(p.stat().st_mtime).date()>=datetime.date(2026,11,25):
        origins.setdefault(str(p),set()).add('ticket-mtime-2026-11-25')
rows=[]
for p,sources in sorted(origins.items()):
    f=root/p
    rows.append({'path':p,'sources':sorted(sources),'state':'present' if f.is_file() else 'missing_or_nonfile','attribution':'ambiguous_mtime_only' if sources=={'ticket-mtime-2026-11-25'} else 'listed'})
(ticket/'close-inventory-25.json').write_text(json.dumps({'scope':'Reported paths and recent ticket records; generated directories excluded. Mtime-only records require review. Missing may be removed or stale paths.','entries':rows},ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'total':len(rows),'missing_or_nonfile':sum(r['state']!='present' for r in rows),'ambiguous_mtime_only':sum(r['attribution']=='ambiguous_mtime_only' for r in rows)},ensure_ascii=False))
