import sys
p=sys.argv[1];t=open(p,encoding='utf8').read()
def once(s,o,n):
    assert s.count(o)==1,(s.count(o),o[:80]);return s.replace(o,n)
t=once(t,'import{expandRootV1,neutralizeRootV1}from"../../🛰️exchange/🟦️.ts";','import{expandRootV1,neutralizeRootV1}from"../../🛰️exchange/🟦️.ts";\nimport{indexTextRewriterV1}from"../../../🕸️graph/🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts";')
anchor='const bundleParts=(bundle:ClosureBundleV1)'
add=open(sys.argv[2],encoding='utf8').read()
t=once(t,anchor,add+anchor)
old='moved=JSON.parse(neutralTextV1(JSON.stringify(rest),host,policy)) as Record<string,unknown>'
assert t.count(old)==2
t=t.replace(old,'moved=movedRestV1(rest,"neutral",host,policy)')
t=once(t,'moved=JSON.parse(localTextV1(JSON.stringify(rest),host,policy)) as Record<string,unknown>','moved=movedRestV1(rest,"local",host,policy)')
open(p,'w',encoding='utf8').write(t)
