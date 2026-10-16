import os,time,shutil,json,sys
ROOT='/Users/ueli/Documents/semio/'
F=ROOT+'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/'
SNAP=F+'🗑️generated/tt2-snap/draw/'
STATE=F+'🗑️generated/tt2-edited.json'
DRAW=ROOT+'✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/'
ANY=DRAW+'✳️any/'
def _state():
    try: return json.load(open(STATE,encoding='utf8'))
    except Exception: return {}
def edit(path,pairs,min_age=3600):
    """pairs: list of (old,new[,count]). Asserts anchors, snapshots once, refuses files touched by others within min_age."""
    assert os.path.isfile(path),path
    st=_state()
    mine=path in st
    if not mine: assert time.time()-os.stat(path).st_mtime>min_age,('recently modified by someone else',path)
    os.makedirs(SNAP,exist_ok=True)
    snap=SNAP+str(abs(hash(path)))+'.pre'
    if not mine:
        shutil.copy(path,snap); st[path]=snap
    s=open(path,encoding='utf8').read()
    n0=s.count('\n')
    for p in pairs:
        old,new=p[0],p[1]; count=p[2] if len(p)>2 else 1
        assert s.count(old)==count,('anchor count',s.count(old),count,old[:160])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
    json.dump(st,open(STATE,'w',encoding='utf8'),ensure_ascii=False,indent=1)
    print('edited',path.replace(ROOT,'')[-90:],n0,'->',s.count('\n'))
