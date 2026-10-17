"""🆚️ Compares two oracle result files of the same family (before/after the spill): multiset of (code, normalised message) over all diagnostics, with the non-stub rows listed separately. Moves between crates keep the key. usage: d3_diff.py <pre-result.json> <post-result.json> [cfg]"""
import json, re, sys, collections
pre=json.load(open(sys.argv[1])); post=json.load(open(sys.argv[2])); cfg=sys.argv[3] if len(sys.argv)>3 else "lib"
def nrm(s):
    s=re.sub(r'__m_[A-Za-z0-9_]+','__m_X',s)
    s=re.sub(r'crate::__m_X::','',s)
    s=re.sub(r'semio_framework_os_store_[a-z_]+','SF',s)
    return re.sub(r'\s+',' ',s)[:240]
def keys(r,nonstub):
    c=collections.Counter(); ex={}
    for d in r["summary"][cfg]["new_list"]:
        if nonstub and (d.get("stub") or d.get("inherited")): continue
        k=(d["code"],nrm(d["msg"])); c[k]+=1; ex.setdefault(k,[]).append((d["file"][-70:],d["line"]))
    return c,ex
for nonstub in (True,False):
    a,ea=keys(pre,nonstub); b,eb=keys(post,nonstub)
    print("== nonstub only" if nonstub else "== all listed rows","pre",sum(a.values()),"post",sum(b.values()))
    added=[(k,b[k]-a.get(k,0)) for k in b if b[k]>a.get(k,0)]
    gone=[(k,a[k]-b.get(k,0)) for k in a if a[k]>b.get(k,0)]
    print(" added keys",len(added),"gone keys",len(gone))
    for k,n in sorted(added,key=lambda x:-x[1])[:25]: print("  +",n,k[0],k[1][:150],eb[k][:2])
    for k,n in sorted(gone,key=lambda x:-x[1])[:25]: print("  -",n,k[0],k[1][:150],ea[k][:2])
