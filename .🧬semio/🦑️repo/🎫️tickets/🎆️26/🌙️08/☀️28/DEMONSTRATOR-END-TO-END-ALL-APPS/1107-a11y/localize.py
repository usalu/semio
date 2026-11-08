#!/usr/bin/env python3
"""Staged codemod for audit items 17 (MediaPortSpec.label) and 18 (terminology_document).
Dry run by default: python3 -I localize.py ; apply with --write ONLY after the framework schema edit
(see ../1107-result-a11y.md "Core edits") compiles. Stdlib only. Exits 1 on any unmatched site."""
import re,sys,os,glob
import os as _o
R=_o.environ.get("A11Y_ROOT","/Users/ueli/Documents/semio/")
WRITE="--write" in sys.argv
LABELS={"Text":"Text","Result":"Ergebnis","Parameters":"Parameter","Parameter":"Parameter","Drawing":"Zeichnung","Geometry":"Geometrie","Map":"Karte","Scene":"Szene","Features":"Objekte","Frames":"Einzelbilder","Photos":"Fotos","Steps":"Schritte","Results":"Ergebnisse","Brep":"B-Rep","Mesh":"Netz","Dictionary":"Wörterbuch","Fields":"Felder","Layout":"Layout","Chapters":"Kapitel","Graph":"Graph","Vector":"Vektor","Image":"Bild","Playback":"Wiedergabe","Kit Catalog":"Kit-Katalog","Puzzle Design":"Puzzle-Entwurf","5D Puzzle Design":"5D-Puzzle-Entwurf","Catalog":"Katalog","Artifact":"Artefakt","Snapshot":"Momentaufnahme","Out":"Ausgang","In":"Eingang","Port":"Anschluss"}
SEG={"Entwerfen mit Bestand":("Designing with Existing Stock","Entwerfen mit Bestand"),"Bearbeiten":("Edit","Bearbeiten"),"Koordinator":("Coordinator","Koordinator"),"Generator":("Generator","Generator"),"Verfolgen":("Trace","Verfolgen"),"Aussuchen":("Select","Aussuchen"),"Aggregator":("Aggregator","Aggregator"),"Statik":("Statics","Statik"),"Energie":("Energy","Energie")}
DATA={"puzzle","5d","2d","Bad"}
def ctor(text,src):
    p="LocalizedLabel" if "LocalizedLabel" in src else "semio_plugin::LocalizedLabel" if "semio_plugin" in src else "semio_ui_locale::LocalizedLabel" if "semio_ui_locale" in src else None
    return p
files=[l.split(":")[0] for l in os.popen(f"cd {R!r} && command grep -rl 'MediaPortSpec\\|terminology_document(' --include='*.rs' 🧰️framework ✏️s 2>/dev/null").read().split("\n") if l]
bad=0
for f in sorted(set(files)):
    path=R+f; src=open(path).read(); out=src
    def lab(m):
        global bad
        t=m.group(2)
        if t not in LABELS: print("UNMAPPED label",f,t); bad+=1; return m.group(0)
        p=ctor(t,src)
        if p is None and f.startswith("🧰️framework"): p="semio_ui_locale::LocalizedLabel"; print("FALLBACK-IMPORT (confirm crate depends on semio-ui-locale)",f[-50:])
        if p is None: print("NEEDS-IMPORT",f); bad+=1; return m.group(0)
        return f'{m.group(1)}{p}::native("{t}", "{LABELS[t]}")'
    out=re.sub(r'(MediaPortSpec\s*\{[^{}]*?\blabel:\s*)"([^"]+)"\.into\(\)',lab,out)
    def bc(m):
        global bad
        parts=[]
        for s in re.findall(r'"([^"]*)"',m.group(2)):
            p=ctor(s,src)
            if p is None: print("NEEDS-IMPORT",f); bad+=1; return m.group(0)
            parts.append(f'{p}::native("{SEG[s][0]}", "{SEG[s][1]}")' if s in SEG else f'{p}::data("{s}")')
        return f'{m.group(1)}[{", ".join(parts)}])'
    out=re.sub(r'(\.terminology_document\("[^"]+",\s*)\[([^\]]*)\]\)',bc,out)
    if out!=src:
        print("EDIT",f.replace("🏅️standards/🔖️1/🪆️subsets/",""))
        if WRITE: open(path,"w").write(out)
print("unmatched/blocked:",bad); sys.exit(1 if bad else 0)
