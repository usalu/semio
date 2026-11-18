import os,re,hashlib,sys
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
T="3a9fba88"
lit=re.compile(r'"((?:[^"\\\n]|\\.){6,400})"|\'((?:[^\'\\\n]|\\.){6,400})\'|`((?:[^`\\]|\\.){6,400})`')
prefixes=["","Error: ","RepoError: ","TypeError: ","RangeError: ","AbortError: ","SyntaxError: ","ReferenceError: ","Error: Error: "]
n=0;seen=set()
for dp,dn,fn in os.walk(R):
    dn[:]=[d for d in dn if d not in("node_modules",".git","target")]
    for f in fn:
        if not f.endswith((".mjs",".ts",".js",".json")):continue
        p=os.path.join(dp,f)
        try:s=open(p,encoding="utf8").read()
        except:continue
        if len(s)>3_000_000:continue
        for m in lit.finditer(s):
            t=m.group(1) or m.group(2) or m.group(3)
            if t in seen:continue
            seen.add(t)
            for v in (t,t.replace('\\"','"').replace("\\n","\n")):
              for pre in prefixes:
                n+=1
                if hashlib.sha256((pre+v).encode()).hexdigest()[:8]==T:print("HIT",p,repr(pre+v));sys.exit()
print("no hit",n,len(seen))
