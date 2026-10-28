"""usage: table.py <outfile> <log>... — per caching test file the verdict of the LAST given log that ran it (official run-many logs; the logs print every file twice, so counts come from the last block)."""
import re,sys
mod="⚡️caching/"
rows={}
for log in sys.argv[2:]:
    cur=None;local={}
    for l in open(log,encoding='utf8',errors='replace').read().split('\n'):
        if l.startswith('(pass)') or l.startswith('[capacity]') or l.startswith('[script]'):continue
        m=re.match(r'^(?:\S*?)'+re.escape(mod)+r'(.+?\.ts):\s*$',l)
        if m:cur=m.group(1);local[cur]={'pass':0,'fail':0,'err':''};continue
        if cur:
            m=re.match(r'^\s*(\d+) (pass|fail)$',l)
            if m:local[cur][m.group(2)]=int(m.group(1))
            if l.startswith('error:') and not local[cur]['err']:local[cur]['err']=l[7:150]
    for f,r in local.items():rows[f]=(log.split('/')[-1],r)
g=sum(1 for _,(lg,r) in rows.items() if r['fail']==0 and r['pass']>0)
out=["| file | verdict | pass | fail | first error | log |","|---|---|---|---|---|---|"]
for f,(lg,r) in sorted(rows.items(),key=lambda x:(x[1][1]['fail']==0,x[0])):
    out.append(f"| `{f}` | {'GREEN' if r['fail']==0 and r['pass']>0 else 'RED'} | {r['pass']} | {r['fail']} | {r['err'].replace('|','/')} | {lg} |")
open(sys.argv[1],'w').write(f"{len(rows)} files, {g} green, {len(rows)-g} red\n\n"+"\n".join(out)+"\n")
print(len(rows),g,len(rows)-g)
