from lib2 import *
A="split-ac-indoor-unit"
EPS=5
def main():
    doc=read_doc(A); asset=doc["asset"]
    drop_rise(asset)
    for sc in ("50","100","200"):
        rep=asset["representations"][sc]
        if rep["status"]!="authored": continue
        g=rep["geometry"]["front"]
        for k,s in enumerate(g):
            if s["id"]=="discharge-vane":
                n=ext(asset,int(sc),"vane",-0.39,0.39)
                n["id"]="discharge-vane"; g[k]=n
    for sc,rep in asset["representations"].items():
        if rep["status"]!="authored": continue
        d=rep["decisions"]["silhouette"]
        old_en=" The front-view vane is bound to a helper bone, so it hangs lower with the vane angle."
        old_de=" Die Lamelle der Vorderansicht ist an einen Hilfsknochen gebunden und hängt mit dem Lamellenwinkel tiefer."
        add_en=" The front-view vane is the side-view vane profile extruded over the 0.78 m, so it hangs lower with the vane angle."
        add_de=" Die Lamelle der Vorderansicht ist das Lamellenprofil der Seitenansicht, über 0,78 m ausgezogen, und hängt mit dem Lamellenwinkel tiefer."
        d["en"]=d["en"].replace(old_en,"")
        d["de"]=d["de"].replace(old_de,"")
        if add_en not in d["en"]: d["en"]+=add_en
        if add_de not in d["de"]: d["de"]+=add_de
    asset["presentation"].pop("viewpoints",None)
    write_doc(A,doc)
    print("applied",A)
main()
