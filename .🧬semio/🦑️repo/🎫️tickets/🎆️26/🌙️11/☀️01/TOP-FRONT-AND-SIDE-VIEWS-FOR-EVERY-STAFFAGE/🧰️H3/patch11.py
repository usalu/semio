s=open("human.py",encoding="utf8").read()
s=s.replace('{"tone": .4, "regions": [h.S(sc, "skin")["region"], h.S(sc, "hair")["region"]]}','{"tone": .4, "regions": regs(h.S(sc, "skin"), h.S(sc, "hair"))}')
s+='''

def regs(*styles):
    out = []
    for d in styles:
        for r in ([d["region"]] if "region" in d else d.get("regions", [])):
            if r not in out: out.append(r)
    return out
'''
open("human.py","w",encoding="utf8").write(s)
