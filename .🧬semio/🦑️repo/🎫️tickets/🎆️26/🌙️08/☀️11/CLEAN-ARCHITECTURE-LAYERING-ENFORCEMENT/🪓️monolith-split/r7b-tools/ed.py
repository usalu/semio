# ✂️ Anchored text edits with uniqueness asserts for r7b scripted refactors.
import os
ROOT='/Users/ueli/Documents/semio'
JACK=ROOT+'/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack'
ANY=JACK+'/🏅️standards/🔖️1/🪆️subsets/✳️any'
def rd(p): return open(p,encoding='utf8').read()
def wr(p,t): open(p,'w',encoding='utf8').write(t)
def edit(p,old,new,count=1):
    t=rd(p); n=t.count(old)
    assert n==count,f'{p}: expected {count} of {old[:60]!r}, found {n}'
    wr(p,t.replace(old,new))
def cut(p,start,end,keep_end=False):
    t=rd(p); a=t.index(start); assert t.count(start)==1,f'{p}: start not unique {start[:50]!r}'
    b=t.index(end,a)+(0 if keep_end else len(end))
    return t[:a]+t[b:], t[a:b]
