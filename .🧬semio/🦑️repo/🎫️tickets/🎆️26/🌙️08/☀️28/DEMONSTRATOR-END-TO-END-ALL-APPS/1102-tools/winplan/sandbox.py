import sys,os,shutil,runpy,builtins,pathlib,json
R="/Users/ueli/Documents/semio"
S=sys.argv[1];script=sys.argv[2];sys.argv=[script]+sys.argv[3:]
os.makedirs(S,exist_ok=True);os.chdir(S)
written=set()
_o=builtins.open
_ex=os.path.exists
def rel(p):
    p=os.fspath(p)
    if p.startswith(R+"/"):return p[len(R)+1:]
    if p.startswith("/"):return None
    return os.path.normpath(p)
def prep(p,mode="r"):
    r=rel(p)
    if r is None:return p
    d=os.path.join(S,r)
    if not _ex(d) and _ex(os.path.join(R,r)) and "w" not in mode:
        os.makedirs(os.path.dirname(d),exist_ok=True)
        if os.path.isfile(os.path.join(R,r)):_o(d,'wb').write(_o(os.path.join(R,r),'rb').read())
    if any(c in mode for c in "wax+"):
        os.makedirs(os.path.dirname(d),exist_ok=True);written.add(r)
    return d
def op(p,mode="r",*a,**k):
    if isinstance(p,int):return _o(p,mode,*a,**k)
    return _o(prep(p,mode),mode,*a,**k)
builtins.open=op
os.path.exists=lambda p:_ex(prep(p))
pathlib.Path.read_text=lambda self,*a,**k:op(self,"r",encoding=k.get("encoding")).read()
def wt(self,t,*a,**k):
    with op(self,"w",encoding=k.get("encoding")) as f:return f.write(t)
pathlib.Path.write_text=wt
pathlib.Path.exists=lambda self:_ex(prep(self))
rc=0
try:
    runpy.run_path(script,run_name="__main__")
except SystemExit as e:
    rc=0 if e.code in (None,0) else 1;
    if e.code not in (None,0):print("EXIT:",e.code)
except BaseException as e:
    rc=2;print("ERR:",type(e).__name__,str(e)[:300])
print("WRITTEN:",json.dumps(sorted(written),ensure_ascii=False))
sys.exit(rc)
