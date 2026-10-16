"""🔗 Crate-level cycle check of the CURRENT assignment on the pristine item graph (all split modules together)."""
import sys, os, collections
sys.path.insert(0,'.')
from groups import *
from assign import *
from sources import load_all
srcs=load_all()
A=assign_all(srcs)
mods=['db_storage','db_engine','db_artifact','db_wal','db_fault_testing']
files={}
fidmap={}
for fid,s in srcs.items():
    if s.module in mods and s.role in('main','sub'):
        key=fid
        files[key]=PRISTINE+'/'+fid
def assign(key,it):
    idx=next(i for i,x in enumerate(srcs[key].items) if x.header==it.header)
    return A.get(key,{}).get(idx)
nodes,G,g=check(files,assign,show=4)
