import sys,os,time,contextlib
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__)))+"/F66")
import lib as L
@contextlib.contextmanager
def locked(files,lane="F70"):
    areas={L.area(f) for f in files}
    while True:
        got,held=L.locks(areas,lane)
        if not held:break
        L.release(got);time.sleep(30)
    try:yield
    finally:L.release(got)
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        if s.count(old)!=1:raise SystemExit(f"anchor count {s.count(old)} in {path}: {old[:80]}")
        s=s.replace(old,new)
    L.write(path,s)
