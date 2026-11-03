from lib import *
H={"room-thermostat":1.5,"thermostatic-radiator-valve":0.5,"shower-head":0.9}
for a,h in H.items():
    doc=read_doc(a)
    for p in doc["asset"]["poses"]: p["root"]={"y":h}
    write_doc(a,doc); print("root",a,h)
doc=read_doc("laptop"); d=doc["asset"]["representations"]["50"]["decisions"]["silhouette"]
oe="Front view: the 0.313 m wide base with its two front feet and chamfered front edge, and the lid as a slab behind it whose top edge rises with the lid angle; the dark display fills its face. Top view: the base with hinge shoulder, five key rows and trackpad, covered by the lid, which draws back over the hinge as it opens."
od="Vorderansicht: die 0,313 m breite Basis mit ihren zwei vorderen Füßen und der abgeschrägten Vorderkante und der Deckel als Platte dahinter, deren Oberkante mit dem Deckelwinkel steigt; das dunkle Display füllt seine Fläche. Draufsicht: die Basis mit Scharnierschulter, fünf Tastenreihen und Touchpad, vom Deckel bedeckt, der sich beim Öffnen über das Scharnier zurückzieht."
assert oe in d["en"] and od in d["de"]
d["en"]=d["en"].replace(" "+oe,""); d["de"]=d["de"].replace(" "+od,"")
write_doc("laptop",doc)
