import sys,os,subprocess,re,shutil
D=sys.argv[1];idx=[int(x) for x in sys.argv[2:]]
files=[l.strip() for l in open(D+"/🗑️generated/dbg-d/testfiles3.txt") if l.strip()]
mp={l.split(" ",1)[1].strip():l.split(" ",1)[0] for l in open(D+"/🗑️generated/dbg-d/orig/map.txt")}
res=[l for l in open(D+"/🗑️generated/dbg-d/testres.txt")]
for i in idx:
    f=files[i-1];n=mp[f]
    tmp=f[:-3]+".dbgdorig.ts"
    shutil.copy(f"{D}/🗑️generated/dbg-d/orig/{n}.bak",tmp)
    try:
        r=subprocess.run(["nice","-n","10","perl","-e","alarm 200; exec @ARGV","bun","test","./"+tmp],capture_output=True,text=True,cwd="/Users/ueli/Documents/semio")
        out=r.stdout+r.stderr
        sm=" ".join(re.findall(r"^ *\d+ (?:pass|fail)",out,re.M))
        cur=[l for l in res if l.startswith(str(i)+" ")][0].strip()
        print(f"{i} BASE rc={r.returncode} {sm} || NOW {cur[:60]}")
    finally:
        os.remove(tmp)
