#!/usr/bin/env python3
"""suite.py TESTFILE... : run each bun test file in ovref and in ov3 (cwd = overlay) and print pass/fail counts and the failing test names that differ."""
import os,re,subprocess,sys
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def run(root,t):
    r=subprocess.run(["nice","-n","10","bun","test","./"+t],cwd=root,capture_output=True,text=True,timeout=600,env={**os.environ,"SEMIO_TEST_ARTIFACT_DIR":"/private/tmp/boot-fast/art"})
    o=r.stdout+r.stderr
    p=re.findall(r"^\s*(\d+) pass",o,re.M);f=re.findall(r"^\s*(\d+) fail",o,re.M)
    names=sorted(set(re.findall(r"^\(fail\) (.*?)(?: \[[\d.]+ms\])?$",o,re.M)))
    return (int(p[-1]) if p else None,int(f[-1]) if f else None,names,o)
for t in sys.argv[1:]:
    path=B+t
    a=run("/private/tmp/boot-fast/ovref",path);b=run("/private/tmp/boot-fast/ov3",path)
    flag="SAME" if (a[0],a[1])==(b[0],b[1]) and a[2]==b[2] else "DIFF"
    print(f"{flag} {t}: ref pass={a[0]} fail={a[1]} | new pass={b[0]} fail={b[1]}")
    if flag=="DIFF":
        print("  only new fails:",[n[:90] for n in b[2] if n not in a[2]]);print("  only ref fails:",[n[:90] for n in a[2] if n not in b[2]])
