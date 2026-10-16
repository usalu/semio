"""📊 hr2 cruiser counter: per-rule totals, production edges and rule families. Usage: python3 counts.py <err.txt> [<err2.txt>]"""
import re,sys,collections
def load(p):
    tot=collections.Counter(); prod=collections.Counter(); edges=collections.defaultdict(set)
    for l in open(p,encoding='utf-8',errors='replace'):
        m=re.match(r'^\s*(error|warn) ([^:]+): (.*?)(?: → (.*))?$',l.rstrip('\n'))
        if not m: continue
        rule=m.group(2); src=m.group(3); dst=m.group(4) or ''
        fam=rule
        if rule.startswith('no-cross-package-relative-'): fam='no-cross-package-relative-*'
        if rule.startswith('no-cross-plugin-'): fam='no-cross-plugin-*'
        for k in {rule,fam}:
            tot[k]+=1
            if '/🧪️tests/' not in src and '/🧪️tests/' not in dst and '/🧪️test/' not in src: prod[k]+=1
        edges[rule].add((src,dst))
    return tot,prod,edges
a=load(sys.argv[1])
b=load(sys.argv[2]) if len(sys.argv)>2 else None
keys=['no-core-path','not-to-unlisted','no-circular','no-cross-package-relative-*','no-cross-plugin-*','ui-no-framework-packages','renderer-hosts-only-ui','framework-no-products','plugins-framework-sdk-only','no-state-outside-os']
print('rule\tall_before\tprod_before'+('\tall_after\tprod_after' if b else ''))
for k in keys:
    row=[k,a[0][k],a[1][k]]
    if b: row+= [b[0][k],b[1][k]]
    print('\t'.join(map(str,row)))
print('total error lines', sum(v for k,v in a[0].items() if not k.endswith('*') and '*' not in k), (sum(v for k,v in b[0].items() if '*' not in k) if b else ''))
