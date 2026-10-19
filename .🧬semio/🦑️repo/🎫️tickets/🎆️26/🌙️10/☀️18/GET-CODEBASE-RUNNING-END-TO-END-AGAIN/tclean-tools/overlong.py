import subprocess,sys,os,collections
root="C:\\Users\\username\\source\\repos\\semio"
lim=260
def ls(*a):
    out=subprocess.run(["git","ls-files","-z",*a,"--","."+"🧬semio".join(["",""])[:0]],capture_output=True).stdout
    return out
base=sys.argv[1:] or [".🧬semio/🦑️repo/🎫️tickets"]
paths=set()
for args in (["ls-files","-z"],["ls-files","-z","--others","--exclude-standard"]):
    out=subprocess.run(["git",*args,"--",*base],capture_output=True).stdout.decode()
    paths|={p for p in out.split("\0") if p}
over=[p for p in paths if len((root+"\\"+p.replace("/","\\")).encode("utf-16-le"))//2>lim and os.path.exists(p)]
print(len(paths),"paths;",len(over),"overlong existing")
c=collections.Counter()
for p in over:
    parts=p.split("/")
    c["/".join(parts[:9])]+=1
for k,v in c.most_common(80): print(v,k)
open(os.environ.get("OUT","/dev/null"),"w").write("\n".join(sorted(over)))
