#!/usr/bin/env python3
"""flags `artifact_app_laws::NAME(` calls whose top-level argument count differs from the pub fn definition (self excluded)."""
import re,subprocess,sys,collections
root="/Users/ueli/Documents/semio"
D=root+"/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws"
def split_top(text,start,angle=False):
    depth=0;cur=start;args=[];i=start;ins=False
    while i<len(text):
        c=text[i]
        if ins:
            if c=="\\": i+=2; continue
            if c=='"': ins=False
        elif c=='"': ins=True
        elif c in "([{": depth+=1
        elif c in ")]}":
            if depth==0:
                a=text[cur:i].strip()
                if a: args.append(a)
                return args,i
            depth-=1
        elif angle and c=="<": depth+=1
        elif angle and c==">" and text[i-1]!="-": depth-=1
        elif c=="," and depth==0:
            args.append(text[cur:i].strip());cur=i+1
        i+=1
    return None,None
defs={}
files=subprocess.run(["rg","-l","--glob","*.rs","pub (async )?fn ",D],capture_output=True,text=True).stdout.split()
for f in files:
    t=open(f).read()
    for m in re.finditer(r'pub (?:async )?fn (\w+)\s*(?:<[^()]*?>)?\(',t):
        # generic params may contain parens in bounds; handle by finding first '(' after name that starts params
        name=m.group(1)
        # re-find paren after generics
        j=m.end()-1
        args,_=split_top(t,j+1,True)
        if args is None: continue
        args=[a for a in args if a and not a.startswith("&self") and not a.startswith("&mut self") and a!="self"]
        defs.setdefault(name,set()).add(len(args))
bad=collections.Counter();ex={}
for f in subprocess.run("rg -l --glob '*.rs' 'artifact_app_laws::' -- ✏️s 🌎️hub",shell=True,cwd=root,capture_output=True,text=True).stdout.split("\n"):
    if not f: continue
    t=open(root+"/"+f).read()
    for m in re.finditer(r'artifact_app_laws::(\w+)\s*(?:::<(?:[^<>]|<[^<>]*>)*>)?\(',t):
        name=m.group(1)
        if name not in defs: continue
        args,_=split_top(t,m.end())
        if args is None: continue
        if len(args) not in defs[name]:
            bad[(name,len(args),tuple(sorted(defs[name])))]+=1; print("SITE",f,t.count("\n",0,m.start())+1,name,len(args))
            ex.setdefault((name,len(args)),f)
for k,v in sorted(bad.items(),key=lambda x:-x[1]): print(v,k,ex[(k[0],k[1])][-80:])
