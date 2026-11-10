#!/usr/bin/env python3
"""Rewrites committed component descriptors to the typed localized shape (port labels and reuse breadcrumbs), identical to what the Rust declarations emit. Dry run by default; --write applies."""
import json,subprocess,sys
LABELS={"Text":"Text","Result":"Ergebnis","Parameters":"Parameter","Parameter":"Parameter","Drawing":"Zeichnung","Geometry":"Geometrie","Map":"Karte","Scene":"Szene","Features":"Objekte","Frames":"Einzelbilder","Photos":"Fotos","Steps":"Schritte","Results":"Ergebnisse","Brep":"B-Rep","Mesh":"Netz","Dictionary":"Wörterbuch","Fields":"Felder","Layout":"Layout","Chapters":"Kapitel","Graph":"Graph","Vector":"Vektor","Image":"Bild","Playback":"Wiedergabe","Kit Catalog":"Kit-Katalog","Puzzle Design":"Puzzle-Entwurf","5D Puzzle Design":"5D-Puzzle-Entwurf","Catalog":"Katalog","Artifact":"Artefakt","Snapshot":"Momentaufnahme","Out":"Ausgang","In":"Eingang","Port":"Anschluss"}
SEG={"Entwerfen mit Bestand":("Designing with Existing Stock","Entwerfen mit Bestand"),"Bearbeiten":("Edit","Bearbeiten"),"Koordinator":("Coordinator","Koordinator"),"Generator":("Generator","Generator"),"Verfolgen":("Trace","Verfolgen"),"Aussuchen":("Select","Aussuchen"),"Aggregator":("Aggregator","Aggregator"),"Statik":("Statics","Statik"),"Energie":("Energy","Energie")}
def cell(en,de): return {t:{"en":en,"de":de} for t in("native","reuse")}
bad=[]
def port(o,f):
    t=o["label"]
    pair=tuple(t.split(" / ")) if t in("Model / Modell","Report / Bericht") else (t,LABELS[t]) if t in LABELS else None
    if pair is None: bad.append((f,t)); return
    o["label"]=cell(*pair)
def walk(o,f):
    if isinstance(o,dict):
        if isinstance(o.get("label"),str) and "direction" in o and "mediaType" in o and "id" in o: port(o,f)
        for k,v in list(o.items()):
            if k=="terminologyBreadcrumbs" and v:
                for name,segs in v.items(): v[name]=[cell(*SEG[s]) if s in SEG else cell(s,s) for s in segs]
            else: walk(v,f)
    elif isinstance(o,list):
        for v in o: walk(v,f)
files=[f for f in subprocess.check_output(["git","ls-files","*component/\U0001F523\uFE0F.json"],text=True).split("\n") if f and "tests" not in f]
n=0
for f in files:
    s=open(f).read(); o=json.loads(s); walk(o,f)
    out=json.dumps(o,indent=2,ensure_ascii=False)+("\n" if s.endswith("\n") else "")
    if out!=s:
        n+=1; print("EDIT",f[-70:])
        if "--write" in sys.argv: open(f,"w").write(out)
for b in bad: print("UNMAPPED",b)
print("files",n,"unmapped",len(bad)); sys.exit(1 if bad else 0)
