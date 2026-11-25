"""🔢️ Counts test attributes and test-generating macro calls under a tree at HEAD (git show) and in the working tree."""
import re,subprocess,sys
from pathlib import Path
ROOT=Path("/Users/ueli/Documents/semio")
pat=re.compile(r'#\[(?:[a-z_:]*::)?(?:test|async_test|rstest)\b[^\]]*\]|\b(?:incremental_laws|entry_tests|norm_live_acceptance|norm_latency_bench)!')
def count(text): return len(pat.findall(text))
def head(prefix):
    out=subprocess.run(["git","-c","core.quotepath=off","ls-tree","-r","--name-only","HEAD","--",prefix],cwd=ROOT,capture_output=True,text=True).stdout.splitlines()
    total=0;files=0
    for f in out:
        if not f.endswith(".rs"): continue
        t=subprocess.run(["git","show",f"HEAD:{f}"],cwd=ROOT,capture_output=True).stdout.decode("utf8","ignore")
        c=count(t)
        if c: total+=c;files+=1
    return total,files
def now(prefix):
    total=0;files=0
    for f in (ROOT/prefix).rglob("*.rs"):
        c=count(f.read_text(encoding="utf8",errors="ignore"))
        if c: total+=c;files+=1
    return total,files
if __name__=="__main__":
    for p in sys.argv[1:]:
        print(p.split("/")[-1] if False else p[-90:],"HEAD",head(p),"NOW",now(p))
