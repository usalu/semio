import os,sys,shutil
MOD="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
SRC=os.environ["SRC"]   # my finished temp module root (tree root)
DST=os.environ["DST"]   # tree root to patch (copy or live)
def src(p): return os.path.join(SRC,MOD,p)
def dst(p): return os.path.join(DST,MOD,p)
def read(path): return open(path,encoding="utf-8",newline="").read()
def write(path,text): open(path,"w",encoding="utf-8",newline="").write(text)
def sub(text,old,new,count=1):
    assert text.count(old)==count,"anchor count %d (want %d): %r"%(text.count(old),count,old[:100])
    return text.replace(old,new)
def between(text,start,end,after=0):
    i=text.index(start,after); j=text.index(end,i+len(start)); return text[i:j]
def edit(path,fn):
    t=read(dst(path)); n=fn(t); 
    if n!=t: write(dst(path),n)
def copy_new(path):
    os.makedirs(os.path.dirname(dst(path)),exist_ok=True)
    shutil.copyfile(src(path),dst(path))
