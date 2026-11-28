#!/usr/bin/env python3
"""Independent oracle of the kill record of a termination: the English and the German line from the schema's texts and the fixture's records, `<English> / <German>`. Usage: oracle.py <schema.json> <fixture.json>"""
import json,re,sys
texts=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["Texts"]["const"]
fx=json.load(open(sys.argv[2],encoding="utf8"))
def fill(lang,r):
    ident=r.get("startMs",0)>0 and r.get("killerStartMs",0)>0
    text=texts[lang]["kill-record"]+(" "+texts[lang]["kill-identity"] if ident else "")
    return re.sub(r"\{(\w+)\}",lambda m:str(r.get(m.group(1),"")),text)
print(json.dumps([fill("en",r)+" / "+fill("de",r) for r in fx["records"]],ensure_ascii=False))
