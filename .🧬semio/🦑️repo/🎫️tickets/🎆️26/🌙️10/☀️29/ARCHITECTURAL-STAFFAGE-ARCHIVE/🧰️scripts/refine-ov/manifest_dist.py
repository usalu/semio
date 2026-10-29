import json
P = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
raw = open(P).read(); m = json.loads(raw)
NEW = {
 "cafe-chair": ("A light chair drawn in elevation with coinciding near and far legs (two leg pairs, four legs in reality), so its load rests on two point contacts at the rear and front feet; a 75 kg occupant load sits on the seat in the sit-down clip and the chair tips back on its rear feet, where the park bench rests on two long end frames.",
                "Leichter Stuhl mit in der Ansicht zusammenfallenden Beinpaaren (zwei Beinpaare, real vier Beine); seine Last ruht auf zwei Punktkontakten an Hinter- und Vorderfuß, im Hinsetz-Clip wirkt eine Personenlast von 75 kg auf dem Sitz, und er kippt auf den hinteren Füßen zurück, während die Parkbank auf zwei langen Endgestellen ruht."),
 "height-adjustable-desk": ("Two identical two-stage telescoping prismatic columns raise the top from 0.72 m to 1.15 m under a drawn monitor and a placed laptop load, so the desk changes height rather than standing fixed.",
                "Zwei identische zweistufige prismatische Teleskopsäulen heben die Platte von 0,72 m auf 1,15 m, unter einem gezeichneten Monitor und einer aufgelegten Laptop-Last, sodass der Tisch seine Höhe ändert statt fest zu stehen."),
 "market-parasol": ("A 2.6 m eight-rib canopy on an aluminium mast with a sliding runner whose stretchers fold the ribs down along the mast, plus a tilt joint on a heavy concrete base, whose structural check applies the wind loads at the crown hub to a fixed socket.",
                "2,6 m achtspeichiger Schirm auf einem Aluminiummast mit Gleitläufer, dessen Spreizen die Speichen am Mast nach unten klappen, und Neigungsgelenk auf einem schweren Betonfuß; die Tragwerksprüfung setzt die Windlasten an der Schirmkrone auf eine feste Aufnahme an."),
 "tied-canopy": ("A 5.5 m steel canopy pinned at the wall and held by a tube strut hinged at both ends, modelled as a bilateral roller along the rod, so the strut carries snow in tension and net wind uplift in compression and the canopy keeps its restraint.",
                "5,5 m langes Stahlvordach, am Wandanschluss gelenkig gelagert und von einem beidseitig gelenkig angeschlossenen Rohrstab gehalten, als beidseitige Rolle entlang des Stabs modelliert; der Stab trägt Schnee auf Zug und Netto-Windsog auf Druck, sodass das Vordach seine Halterung behält."),
 "glass-balustrade": ("A single 1.1 m laminated glass panel cantilevered from a floor clamp with no posts, shown in section: a handrail line load of 1.0 kN/m (taken per metre of run on a 1 m strip), a separate crowd impact point load and wind pressure act at its top.",
                "Eine einzelne 1,1 m hohe Verbundglasfüllung ohne Pfosten, am Bodenklemmhalter eingespannt und im Schnitt gezeigt: Eine Handlauf-Linienlast von 1,0 kN/m (je Meter Länge auf einem 1-m-Streifen), eine getrennte Anprall-Einzellast und Winddruck wirken oben."),
}
n = 0
for e in m["entries"]:
    if e["id"] in NEW:
        old = e["distinction"]
        for lang, new in zip(("en", "de"), NEW[e["id"]]):
            o = json.dumps(old[lang], ensure_ascii=False); nw = json.dumps(new, ensure_ascii=False)
            assert o in raw, (e["id"], lang)
            raw = raw.replace(o, nw, 1); n += 1
json.loads(raw)
open(P, "w").write(raw)
print(n)
