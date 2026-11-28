import os,sys,tempfile
R="/Users/ueli/Documents/semio/"
def edit(path,pairs):
    p=R+path if not path.startswith("/") else path
    s=open(p,encoding="utf8").read()
    for a,b in pairs:
        if b in s and a not in s:
            print("already applied:",a[:50]);continue
        assert s.count(a)==1,("anchor count",s.count(a),a[:80])
        s=s.replace(a,b)
    if os.environ.get('DRY'):print('dry ok',p[-50:]);return
    d=os.path.dirname(p);fd,t=tempfile.mkstemp(dir=d,prefix=".h45-");os.write(fd,s.encode("utf8"));os.close(fd)
    os.chmod(t,os.stat(p).st_mode&0o777);os.replace(t,p);print("edited",p[-60:])
