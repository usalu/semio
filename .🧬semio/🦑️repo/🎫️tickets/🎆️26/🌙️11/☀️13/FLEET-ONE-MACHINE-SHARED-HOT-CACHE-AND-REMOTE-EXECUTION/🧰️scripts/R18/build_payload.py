"""R18: builds payload.tar (mirror/<rel> = the R18 version, base/<rel> = the live file R18 started from) from the scratch overlay and base: every overlay file that differs from its base. usage: build_payload.py <scratch dir> <out.tar>"""
import os,sys,tarfile,filecmp
SC=sys.argv[1];OUT=sys.argv[2];n=0
with tarfile.open(OUT,"w") as t:
    for d,_,fs in sorted(os.walk(SC+"/overlay")):
        for f in sorted(fs):
            mine=os.path.join(d,f);rel=os.path.relpath(mine,SC+"/overlay");base=SC+"/base/"+rel
            if not os.path.exists(base):print("no base, skipped",rel);continue
            if filecmp.cmp(mine,base,shallow=False):print("unchanged",rel[-80:]);continue
            t.add(mine,arcname="mirror/"+rel);t.add(base,arcname="base/"+rel);n+=1
print(n,"files in",OUT)
