"""usage: summarize.py <log>... -> per test file of the caching module: pass/fail counts and first error line, parsed from official run logs"""
import re,sys
mod="⚡️caching/"
for log in sys.argv[1:]:
    cur=None;rows={};text=open(log,encoding='utf8',errors='replace').read().split('\n')
    for i,l in enumerate(text):
        if l.startswith('(pass)') or l.startswith('[capacity]') or l.startswith('[script]'):continue
        m=re.match(r'^(?:\S*?)'+re.escape(mod)+r'(.+?\.ts):\s*$',l)
        if m:cur=m.group(1);rows.setdefault(cur,{'pass':0,'fail':0,'err':''});continue
        if cur:
            m=re.match(r'^\s*(\d+) (pass|fail)$',l)
            if m:rows[cur][m.group(2)]=int(m.group(1))
            if l.startswith('error:') and not rows[cur]['err']:rows[cur]['err']=l[:160]
    for f,r in rows.items():
        print(("GREEN" if r['fail']==0 and r['pass']>0 else "RED  "),r['pass'],r['fail'],f,r['err'],sep='\t')
