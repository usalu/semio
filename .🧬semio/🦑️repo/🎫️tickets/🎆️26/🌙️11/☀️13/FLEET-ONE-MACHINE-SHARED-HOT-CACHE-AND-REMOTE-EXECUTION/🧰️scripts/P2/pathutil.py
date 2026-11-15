import os
LIVE=os.environ.get("P2_LIVE","/Users/Shared/semio")
def strip(s): return s.replace("️","")
def resolve(*parts):
    cur=LIVE
    for part in "/".join(parts).split("/"):
        if not part: continue
        names=os.listdir(cur)
        hit=[n for n in names if strip(n)==strip(part)]
        if len(hit)!=1: raise SystemExit(f"cannot resolve {part!r} in {cur}: {hit}")
        cur=os.path.join(cur,hit[0])
    return os.path.relpath(cur,LIVE)
