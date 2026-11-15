import sys, json
from author_08_person import *
slug="person-with-white-cane"
seed=seed_of(slug)
def mat(sid): return seed_part_attr(seed,sid)
cfg={"is_person":lambda b: is_person(b),
 "parts":{
  "pelvis":[dict(id="hip",mat="denim-jacket",tone=0.4,region="jacket",exclude=["bag","bag-flap"])],
  "head":[dict(id="head",mat="skin-medium",tone=0.82,region="face-skin",include=["head"]),dict(id="hair",mat="hair-brown",tone=0.28,region="hair",include=["hair"],maxn=6)],
 }}
def build(scale):
    sol,reg=person_body(slug,scale,cfg)
    return sol
if __name__=="__main__":
    d=load_src("humans",slug)
    for sc in SCALES:
        d["volumes"][sc]["body"]=build(sc)
    json.dump(d,open("🗑️generated/author-08/try.json","w"))
