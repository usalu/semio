import os,re,sys,subprocess
ROOT="/Users/ueli/Documents/semio"
needles=[l.rstrip("\n") for l in sys.stdin if l.strip()]
roots=sys.argv[1:] or ["🧰️framework"]
for n in needles:
    p=subprocess.run(["/usr/bin/grep","-rlF","--include=*.rs",n]+roots,cwd=ROOT,capture_output=True,text=True)
    hits=[h for h in p.stdout.split("\n") if h]
    prod=[h for h in hits if "/🧪️tests/" not in h and "🧪️testing" not in h]
    print(("OK  " if prod else ("TEST" if hits else "MISS")),repr(n[:80]),"->",[h.replace("🧰️framework/","")[-110:] for h in (prod or hits)[:3]])
