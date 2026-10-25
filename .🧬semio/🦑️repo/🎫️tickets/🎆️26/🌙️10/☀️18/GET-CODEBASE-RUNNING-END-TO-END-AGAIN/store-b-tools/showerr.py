import sys,re,os
errs=open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/store-b/census-store-errors.txt').read().split('\n')
sub=sys.argv[1];w=int(sys.argv[2]) if len(sys.argv)>2 else 120
for e in errs:
    if sub not in e:continue
    m=re.match(r'^(.*?):(\d+):(\d+): (error(?:\[E\d+\])?: .*)$',e)
    if not m:print(e[:300]);continue
    path,l,c,msg=m.group(1),int(m.group(2)),int(m.group(3)),m.group(4)
    p=os.path.normpath(path) if os.path.exists(os.path.normpath(path)) else path
    try:
        line=open(p).read().split('\n')[l-1]
        # col is in chars? rustc cols are 1-based char offsets
        ex=line[max(0,c-1-20):c-1+w]
    except Exception as x:ex='<unreadable %s>'%x
    print(f'{l}:{c} {msg[:150]}\n    >> {ex}')
