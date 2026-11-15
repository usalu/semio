import subprocess,sys,re,os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
bun=subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1",shell=True,text=True).strip()
for a in sys.argv[1:]:
    out=subprocess.run([bun,"volumes-check.ts","vegetation/"+a,"--quiet"],capture_output=True,text=True).stdout
    print("==",a)
    for l in out.splitlines():
        if l.startswith("FAIL") and not l.startswith("FAIL:"):
            parts=l.split()
            bad=[p for p in l.split(" ") if p.startswith("!")]
            import re
            bad=re.findall(r"!\S+=\S+<?[=<>]*\S*(?: \[[^\]]*\])?",l)
            print(" ",parts[1],parts[2]," ".join(bad)[:330])
