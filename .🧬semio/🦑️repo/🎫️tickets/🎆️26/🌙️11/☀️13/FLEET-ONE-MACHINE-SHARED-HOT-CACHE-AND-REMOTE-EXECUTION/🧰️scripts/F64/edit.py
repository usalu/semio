"""Atomic anchored edit helper: edit.py FILE ANCHOR_FILE REPLACEMENT_FILE (anchor must be unique)."""
import os,sys
f,a,r=sys.argv[1:4]
t=open(f,encoding='utf8').read();A=open(a,encoding='utf8').read().rstrip('\n');R=open(r,encoding='utf8').read().rstrip('\n')
if t.count(A)!=1: sys.exit(f"anchor count {t.count(A)}")
tmp=f+'.f64.tmp';open(tmp,'w',encoding='utf8').write(t.replace(A,R));os.replace(tmp,f);print("ok")
