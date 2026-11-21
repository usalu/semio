"""🏗️ Apply the four glass panes' vertical dead load to their actual spans on the IPE 160."""
import json
import os
from pathlib import Path

def text(en,de):return {"en":en,"de":de}

def apply(asset):
    g=asset["structure"]["gravityMS2"]
    strip_mass=0.016*next(m["densityKgM3"] for m in asset["materials"] if m["id"]=="glass")
    strips=[("1",.03,1.3825,1),("2",1.4025,2.755,1),("3a",2.775,3.722,1),("3b",3.722,4.1275,.975),("4",4.1475,5.5,1)]
    glass=sum((hi-lo)*width*strip_mass for _,lo,hi,width in strips)
    asset["dimensions"]["massKg"]=round(asset["rig"]["bones"][0]["mass"]["kg"]+glass,4)
    carried=[{"id":"glass-weight-"+k,"kind":"carried","title":text("Glazing dead load "+k,"Eigenlast der Verglasung "+k),"fxN":0,"fyN":0,"distributed":{"bone":"canopy","fromM":lo,"toM":hi,"wxNPerM":0,"wyNPerM":round(-strip_mass*width*g,6)},"source":"authored-estimate"} for k,lo,hi,width in strips]
    for pose in asset["poses"]:
        pose["loads"]=[l for l in pose["loads"] if not l["id"].startswith("glass-weight-")]+carried
        if pose["id"]=="wind-uplift":pose["description"]=text("At 0.50 kN/m² wind suction the glazing and beam still place the hinged strut in tension.","Bei 0,50 kN/m² Windsog halten Glasdeckung und Träger den gelenkigen Rohrstab noch auf Zug.")
    asset["structure"]["assumptions"][-1]=text("Four panes of 2 × 8 mm glass weigh 215.995 kg. The 20 mm joints and 25 mm fork notch are deducted. The glazing is a permanent line load in all three poses, separate from the IPE 160 steel mass of 86.78 kg. The analytical assembly mass excludes the wall, idealised support strut, interlayer and fittings.","Vier Scheiben aus 2 × 8 mm Glas wiegen 215,995 kg. Die 20-mm-Fugen und die 25-mm-Gabelaussparung sind abgezogen. Die Verglasung wirkt in allen drei Stellungen als ständige Linienlast, getrennt von den 86,78 kg des IPE 160. Wand, idealisierter Stützstab, Zwischenfolie und Beschläge sind in der rechnerischen Baugruppenmasse nicht enthalten.")
    notes=asset["provenance"]["notes"]
    for lang,prior,new in (("en","The glass cover is drawn but its weight is not part of the rig mass.","The four panes transfer their permanent weight to the beam over their actual spans. The steady wind case is 0.50 kN/m² and the animated gust peaks at 0.75 kN/m²."),("de","Die Glasdeckung ist gezeichnet, ihr Gewicht gehört nicht zur Rigmasse.","Die vier Scheiben tragen ihr Eigengewicht über ihre tatsächlichen Spannweiten in den Träger ein. Der stetige Windsog beträgt 0,50 kN/m², die animierte Böe erreicht 0,75 kN/m².")):
        notes[lang]=notes[lang].replace(prior,new)
    gust=next(c for c in asset["clips"] if c["id"]=="wind-gust")
    gust["tracks"][0]["keys"][1]["v"]=1.5
    gust["description"]=text("Suction rises from zero to 0.75 kN/m² in three seconds and falls again. The strut changes from tension to compression and back when wind overcomes the glazing and beam dead load.","Der Sog steigt binnen drei Sekunden auf 0,75 kN/m² und klingt wieder ab. Sobald der Windsog die Eigenlast von Glas und Träger überwindet, wechselt der Rohrstab von Zug zu Druck und beim Abklingen wieder zurück.")
    gust["events"]=[{"t":1.95925,"kind":"motion","title":text("Strut switches to compression","Rohrstab wechselt auf Druck")},{"t":4.04075,"kind":"motion","title":text("Strut returns to tension","Rohrstab kehrt auf Zug zurück")}]

if __name__=="__main__":
    path=next(Path("🎓️teaching").glob("*architecture/*staffage/*catalog/*/tied-canopy/*.json"))
    raw=path.read_bytes();doc=json.loads(raw);apply(doc["asset"]);assert path.read_bytes()==raw
    tmp=path.with_suffix(".partial");tmp.write_text(json.dumps(doc,ensure_ascii=False,indent=2)+"\n");os.replace(tmp,path)
