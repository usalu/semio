import sys
from rsitems import items,name_kind
f,a,b=sys.argv[1],int(sys.argv[2]),int(sys.argv[3])
L=open(f,encoding='utf-8').read().split('\n')
body='\n'.join(L[a-1:b])
for it in items(body):
    v,k,nm=name_kind(it['toks'])
    print(f"{it['first_tok_line']+a}\t{it['hdr_line']+a}\t{it['end_line']+a}\t{v}\t{k}\t{nm}\t{' '.join(it['attrs'])[:40]}")
