"""✂️ Anchored-edit helpers for the flow-flow carve. Every edit asserts its anchor is unique; every write stays inside the work tree."""
import os,re
SCR="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/ff/"
REPOROOT="/Users/ueli/Documents/semio/"
FFREL="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow"
WORK=SCR+"work/"
def guard(path):
    rp=os.path.realpath(path)
    ok=[os.path.realpath(WORK),os.path.realpath(REPOROOT+FFREL)] if os.environ.get("FF_APPLY")=="1" else [os.path.realpath(WORK)]
    assert any(rp==o or rp.startswith(o+os.sep) for o in ok),("write outside work tree",path)
    assert not os.path.islink(path),("symlink",path)
BASE=WORK
def p(rel):return BASE+rel
def read(rel):
    return open(p(rel),encoding="utf-8").read()
def write(rel,text):
    path=p(rel)
    os.makedirs(os.path.dirname(path),exist_ok=True)
    guard(os.path.dirname(path))
    open(path,"w",encoding="utf-8").write(text)
def one(text,anchor):
    assert text.count(anchor)==1,(text.count(anchor),anchor[:80])
    return text.index(anchor)
def cut(text,start,end,include_end=True):
    """removes [start,end] (anchors, both unique); returns (new_text, removed)"""
    i=one(text,start);j=one(text,end)+(len(end) if include_end else 0)
    assert i<j,(start[:40],end[:40])
    return text[:i]+text[j:],text[i:j]
def replace(text,old,new):
    one(text,old)
    return text.replace(old,new)
def lines_ok(text):
    return text if text.endswith("\n") else text+"\n"
