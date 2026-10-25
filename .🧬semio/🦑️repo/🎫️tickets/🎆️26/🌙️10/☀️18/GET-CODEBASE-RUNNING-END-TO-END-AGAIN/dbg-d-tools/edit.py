import sys,subprocess,os,time
def find(suffix):
    out=subprocess.run(["rg","--files","--hidden","-g","!node_modules","-g","!.cache","-g","!🗑️generated","-g","!bazel-*","-g","!.🧬semio","-g","!dist","-g","!target","-g","*"+suffix.split("/")[-1],"."],capture_output=True,text=True,cwd="/Users/ueli/Documents/semio").stdout.split("\n")
    m=[o for o in out if o.endswith(suffix)]
    assert len(m)==1,(suffix,m)
    return "/Users/ueli/Documents/semio/"+m[0][2:]
def edit(suffix,pairs):
    p=find(suffix)
    t=open(p,encoding="utf8").read()
    for old,new in pairs:
        assert t.count(old)==1,(suffix,old[:80],t.count(old))
        t=t.replace(old,new)
    open(p,"w",encoding="utf8").write(t)
    print("edited",suffix)
import importlib.util
_s=importlib.util.spec_from_file_location("sl","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/dbg-d-tools/strip_logs.py");sl=importlib.util.module_from_spec(_s);_s.loader.exec_module(sl)
def remove_if_log(suffix,head,expect_tag="[DEBUG]"):
    p=find(suffix);t=open(p,encoding="utf8").read()
    assert t.count(head)==1,(suffix,head,t.count(head))
    s=t.index(head);o=s+head.index("(")
    assert t[o]=="(",t[o]
    e=sl.match_paren(t,o,False);k=e+1
    while t[k] in " \t":k+=1
    m=sl.CALL_TS.match(t,k);assert m,t[k:k+50]
    o2=m.end()-1;e2=sl.match_paren(t,o2,False);assert expect_tag in t[o2:o2+40],t[o2:o2+40]
    end=e2+1
    if t[end]==";":end+=1
    print("removing:",t[s:end][:120],"...",t[end-40:end])
    t=t[:s]+t[end:]
    open(p,"w",encoding="utf8").write(t)
