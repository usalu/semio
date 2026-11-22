import re,collections,sys
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E19/"
log=sys.argv[1] if len(sys.argv)>1 else "logs.txt"
t=open(G+log).read().split('=== ')[1:]
c=collections.defaultdict(list)
for b in t:
    h,_,body=b.partition('\n')
    if 'rc=0 ' in h: continue
    m=re.findall(r'^\s*(?:error|Error)[: ][^\n]{0,130}',body,re.M)
    k=(m[0] if m else "NOERR "+body.strip()[-80:]).strip()
    k=re.sub(r'/Users/\S+','<p>',k)[:110]
    c[k].append(h.split(' rc=')[0])
for k,v in sorted(c.items(),key=lambda x:-len(x[1])): print(len(v),k)
