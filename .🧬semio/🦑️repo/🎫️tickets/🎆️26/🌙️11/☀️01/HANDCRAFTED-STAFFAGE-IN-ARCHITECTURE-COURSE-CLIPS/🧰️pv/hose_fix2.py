import json
p="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
def sub(s,old,new):
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
hose='''            {
              "id": "hose",
              "bone": "elbow",
              "role": "body",
              "z": 3,
              "filled": false,
              "tone": 0.55,
              "kind": "path",
              "d": "M 0.01 -0.004 C 0.07 -0.01 0.22 -0.14 0.2 -0.26 C 0.18 -0.38 0.07 -0.36 0.09 -0.22 C 0.11 -0.06 0.14 0.26 0.02196 0.51843",
              "region": "hose",
              "material": "epdm-hose",
              "penMm": 0.1
            },
'''
s=sub(s,'''            {
              "id": "tube",
              "bone": "rail",
              "role": "body",
              "z": 3.2,
              "filled": true,
              "tone": 0.8,
              "kind": "rect",
              "x": 0.04,
              "y": 0,
              "w": 0.04,''',hose+'''            {
              "id": "tube",
              "bone": "rail",
              "role": "body",
              "z": 3.2,
              "filled": true,
              "tone": 0.8,
              "kind": "rect",
              "x": 0.04,
              "y": 0,
              "w": 0.04,''')
s=sub(s,'''"regions": [
                "handle",
                "hose"
              ],''','"region": "handle",')
s=sub(s,'a head at the top."','a head at the top and the hose loop as a line."')
s=sub(s,'und einem Kopf oben."','und einem Kopf oben und die Schlauchschleife als Linie."')
s=sub(s,'"en": "Rail pole, handle and head."','"en": "Rail pole, handle, head and hose loop."')
s=sub(s,'"de": "Stange, Griff und Kopf."','"de": "Stange, Griff, Kopf und Schlauchbogen."')
s=sub(s,'; the hose region joins the handle (regions handle and hose merged).",','.",')
s=sub(s,'; die Schlauchregion geht im Griff auf (Regionen Griff und Schlauch zusammengeführt)."','."')
s=sub(s,'"en": "All fittings, the nozzle line and the hose, whose true width of 12 mm is 0.06 mm on paper, below the finest pen.",','"en": "All fittings and the nozzle line.",')
s=sub(s,'"de": "Alle Beschläge, die Düsenlinie und der Schlauch, dessen wahre Breite von 12 mm auf Papier 0,06 mm misst, unter dem feinsten Stift."','"de": "Alle Beschläge und die Düsenlinie."')
s=sub(s,'the hose is dropped, not thickened.','the hose loop (it spans the extent of the set) is a 0.1 mm line, the finest pen that prints, for a true 0.06 mm; it is not thickened further.')
s=sub(s,'der Schlauch entfällt, statt verdickt zu werden.','die Schlauchschleife (sie bestimmt die Ausdehnung der Garnitur) ist eine 0,1-mm-Linie, der feinste druckbare Stift, für wahre 0,06 mm; sie wird nicht weiter verdickt.')
s=sub(s,'"en": "Pole 0.8, handle 0.82, head 0.9; one body pen of 0.13 mm."','"en": "Pole 0.8, handle 0.82, head 0.9, hose 0.55; body pen 0.13 mm, hose line 0.1 mm."')
s=sub(s,'Stange 0,8, Griff 0,82, Kopf 0,9; ein Körperstift von 0,13 mm.','Stange 0,8, Griff 0,82, Kopf 0,9, Schlauch 0,55; Körperstift 0,13 mm, Schlauchlinie 0,1 mm.')
s=sub(s,'; the handle lists handle and hose."','."')
s=sub(s,'; der Griff führt Griff und Schlauch."','."')
open(p,"w",encoding="utf-8",newline="").write(s); json.loads(s); print("ok")
