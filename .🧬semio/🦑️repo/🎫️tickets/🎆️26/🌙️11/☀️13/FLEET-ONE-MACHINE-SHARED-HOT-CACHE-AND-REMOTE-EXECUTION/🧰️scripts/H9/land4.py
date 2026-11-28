import os,sys
R="/Users/ueli/Documents/semio/"
G=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,s):
    t=p+".h9tmp"; open(t,"w",encoding="utf8").write(s); os.replace(t,p)
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
files={}
p=G+"🟨️.mjs"; s=rd(p)
s=sub(s,"before:owners0,previous:spec.previousEnv});","before:owners0,previous:spec.previousEnv,spec});")
files[p]=s
p=G+"🪡️edge-heal/🟨️.mjs"; s=rd(p)
s=sub(s,"cold,repair,healed,resumed,before,previous}){","cold,repair,healed,resumed,before,previous,spec}){")
s=sub(s,"before[name]?.edgesEnv?.slice(0,8),previous?.slice(0,8)].join()","before[name]?.edgesEnv?.slice(0,8),previous?.slice(0,8),spec?.stubs?.[name]?.kind,spec?.stubs?.[name]?.tag,spec?.edgeStale?.includes(name),spec?.owners?.includes(name),(spec?.discover??[]).some(path=>String(path).startsWith(row.root)),(spec?.discover??[]).length,Object.keys(spec?.stubs??{}).length].join()")
s=sub(s,"`previous` the environment it was read for)","`previous` the environment it was read for, `spec` the compute spec: the diagnosis ends with the stub kind and tag of the owner, whether the spec named it stale, in the wave, or under a discovered path, and the counts of discovered paths and stubs)")
files[p]=s
p=G+"🪡️edge-heal/🟨️.d.mts"; s=rd(p)
s=sub(s,"previous:string|null|undefined}):void;","previous:string|null|undefined;spec?:{stubs?:Record<string,{kind?:string;tag?:string}>;edgeStale?:readonly string[];owners?:readonly string[];discover?:readonly string[]}}):void;")
files[p]=s
if "--check" in sys.argv: print("anchors ok",len(files)); sys.exit(0)
for p,s in files.items(): wr(p,s)
print("landed",len(files))
