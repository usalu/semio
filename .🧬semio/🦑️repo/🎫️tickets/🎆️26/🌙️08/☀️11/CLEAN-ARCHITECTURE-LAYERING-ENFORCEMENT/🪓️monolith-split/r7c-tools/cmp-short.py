# 🔬️ Compares two rustc --error-format=short logs as multisets of (file, code+message) with line numbers stripped: usage cmp-short.py <base.short> <cur.short> [--warn]
import re,sys,collections
def load(p, warn):
    c=collections.Counter()
    for l in open(p,encoding='utf8',errors='replace'):
        m=re.match(r'(.*?):\d+:\d+: (error|warning)(\[[^\]]*\])?: (.*)$',l.rstrip('\n'))
        if not m: continue
        if m.group(2)=='warning' and not warn: continue
        if 'unlinked crate' in m.group(4) and not ('--keep-unlinked' in sys.argv): continue
        f=m.group(1); f=f[f.find('✏️s/'):] if '✏️s/' in f else f
        c[(f,m.group(2),m.group(3) or '',m.group(4))]+=1
    return c
warn='--warn' in sys.argv
a=load(sys.argv[1],warn); b=load(sys.argv[2],warn)
only_b=b-a; only_a=a-b
print('base',sum(a.values()),'cur',sum(b.values()),'onlyCandidate',sum(only_b.values()),'onlyBase',sum(only_a.values()))
for k,v in sorted(only_b.items())[:int(40)]:
    print('  +',v,k[0][-70:],k[1],k[2],k[3][:140])
for k,v in sorted(only_a.items())[:15]:
    print('  -',v,k[0][-70:],k[1],k[2],k[3][:140])
