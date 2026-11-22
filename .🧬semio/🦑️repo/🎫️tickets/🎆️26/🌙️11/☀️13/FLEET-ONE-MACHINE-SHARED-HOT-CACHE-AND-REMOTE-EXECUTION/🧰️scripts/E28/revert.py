#!/usr/bin/env python3
"""usage: revert.py <live root> <base dir> <landing table>   restores originals only for files that still hold the bytes the landing wrote"""
import os,sys,hashlib,shutil
root,base,table=sys.argv[1:4]
def sha(p):return hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
for line in open(table):
    rel,before,after=line.rstrip("\n").rsplit(" ",2)
    p=os.path.join(root,rel)
    if not os.path.exists(p):print("gone",rel);continue
    if sha(p)!=after:print("CHANGED SINCE LANDING, left alone:",rel);continue
    if before=="new":os.remove(p);print("removed",rel)
    else:
        tmp=p+".e28.tmp";shutil.copyfile(os.path.join(base,rel),tmp);shutil.copymode(p,tmp);os.replace(tmp,p);print("restored",rel)
