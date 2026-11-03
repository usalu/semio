import json
p="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
nl="\r\n" if "\r\n" in s else "\n"
def sub(s,old,new):
    old=old.replace("\n",nl); new=new.replace("\n",nl)
    assert s.count(old)==1,(old[:60],s.count(old)); return s.replace(old,new)
s=sub(s,'''"material": "epdm-hose",
              "penMm": 0.3
''','''"material": "epdm-hose",
              "penMm": 0.13
''')
s=sub(s,'the hose is a 0.3 mm line.','the hose is a 0.13 mm line, the finest pen, for its true 0.12 mm.')
s=sub(s,'der Schlauch ist eine 0,3-mm-Linie.','der Schlauch ist eine 0,13-mm-Linie, der feinste Stift, für wahre 0,12 mm.')
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
              "penMm": 0.25
            },
'''
s=sub(s,hose,"")
s=sub(s,'''"r1": 0.026,
              "region": "handle",
              "material": "abs-chrome"
            },
            {
              "id": "head",
              "bone": "shower",
              "role": "body",
              "z": 3.6,
              "filled": true,
              "tone": 0.9,
              "kind": "capsule",
              "x0": 0.02921,''','''"r1": 0.026,
              "regions": [
                "handle",
                "hose"
              ],
              "material": "abs-chrome"
            },
            {
              "id": "head",
              "bone": "shower",
              "role": "body",
              "z": 3.6,
              "filled": true,
              "tone": 0.9,
              "kind": "capsule",
              "x0": 0.02921,''')
s=sub(s,'"en": "A 5.5 mm by 1.1 mm set: a pole with a short handle and a head at the top and a hose line."','"en": "A 5.5 mm by 1.1 mm set: a pole with a short handle and a head at the top."')
s=sub(s,'eine Stange mit kurzem Griff und einem Kopf oben und eine Schlauchlinie."','eine Stange mit kurzem Griff und einem Kopf oben."')
s=sub(s,'"en": "Rail pole, handle, head and hose loop."','"en": "Rail pole, handle and head."')
s=sub(s,'"de": "Stange, Griff, Kopf und Schlauchbogen."','"de": "Stange, Griff und Kopf."')
s=sub(s,'"en": "Brackets, holder, elbow and lever join the pole (regions rail and valve merged).",','"en": "Brackets, holder, elbow and lever join the pole (regions rail and valve merged); the hose region joins the handle (regions handle and hose merged).",')
s=sub(s,'(Regionen Stange und Ventil zusammengeführt)."','(Regionen Stange und Ventil zusammengeführt); die Schlauchregion geht im Griff auf (Regionen Griff und Schlauch zusammengeführt)."')
s=sub(s,'"en": "All fittings and the nozzle line.",','"en": "All fittings, the nozzle line and the hose, whose true width of 12 mm is 0.06 mm on paper, below the finest pen.",')
s=sub(s,'"de": "Alle Beschläge und die Düsenlinie."','"de": "Alle Beschläge, die Düsenlinie und der Schlauch, dessen wahre Breite von 12 mm auf Papier 0,06 mm misst, unter dem feinsten Stift."')
s=sub(s,'handle 52 mm and head 48 mm; the hose is a 0.25 mm line.','handle 52 mm and head 48 mm; the hose is dropped, not thickened.')
s=sub(s,'der Griff 52 mm und der Kopf 48 mm; der Schlauch ist eine 0,25-mm-Linie.','der Griff 52 mm und der Kopf 48 mm; der Schlauch entfällt, statt verdickt zu werden.')
s=sub(s,'"en": "Pole 0.8, handle 0.82, head 0.9, hose 0.55; one body pen of 0.13 mm."','"en": "Pole 0.8, handle 0.82, head 0.9; one body pen of 0.13 mm."')
s=sub(s,'Stange 0,8, Griff 0,82, Kopf 0,9, Schlauch 0,55; ein Körperstift von 0,13 mm.','Stange 0,8, Griff 0,82, Kopf 0,9; ein Körperstift von 0,13 mm.')
s=sub(s,'"en": "The pole lists rail and valve as merged regions."','"en": "The pole lists rail and valve as merged regions; the handle lists handle and hose."')
s=sub(s,'"de": "Die Stange führt Stange und Ventil als zusammengeführte Regionen."','"de": "Die Stange führt Stange und Ventil als zusammengeführte Regionen; der Griff führt Griff und Schlauch."')
open(p,"w",encoding="utf-8",newline="").write(s); json.loads(s); print("ok")
