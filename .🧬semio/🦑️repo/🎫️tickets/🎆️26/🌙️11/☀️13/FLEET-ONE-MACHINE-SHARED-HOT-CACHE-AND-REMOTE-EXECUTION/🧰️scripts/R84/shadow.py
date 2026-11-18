#!/usr/bin/env python3
"""R84 shadow: writes ONE test file as an absolute-import copy into scratch (never into the tree) so an edited test can run through the genuine-birth harness without touching the live file. usage: shadow.py <live test rel> [edits.json: [[old,new,count],...]] -> prints the scratch path."""
import os,re,sys,json,urllib.parse
R="/Users/ueli/Documents/semio"
S=R+"/.cache/test-sandbox/r84-shadow"
def shadow(rel,text=None,tag=None):
    live=os.path.join(R,rel);d=os.path.dirname(live)
    t=open(live,encoding="utf8").read() if text is None else text
    def ab(m):
        p=os.path.normpath(os.path.join(d,m.group(2)));return m.group(1)+m.group(3)+p+m.group(3)
    t=re.sub(r'(?m)^(\s*(?:import|export)\b[^\n]*?\bfrom\s*|\s*import\s*)(["\x27])(\.{1,2}/[^"\x27]*)\2',lambda m:m.group(1)+json.dumps(os.path.normpath(os.path.join(d,m.group(3))),ensure_ascii=False),t)
    t=re.sub(r'(\bawait import\()(["\x27])(\.{1,2}/[^"\x27]*)\2(\))',lambda m:m.group(1)+json.dumps(os.path.normpath(os.path.join(d,m.group(3))),ensure_ascii=False)+m.group(4),t)
    url="file://"+urllib.parse.quote(live)
    for k,v in (("physicalSourceOriginV1(import.meta.url","@@A@@"),("artifact=import.meta.url","@@B@@"),("capturedSources=import.meta.url","@@C@@")):t=t.replace(k,v)
    t=t.replace("import.meta.url",json.dumps(url,ensure_ascii=False)).replace("import.meta.dir",json.dumps(d,ensure_ascii=False)).replace("import.meta.path",json.dumps(live,ensure_ascii=False)).replace("@@A@@","physicalSourceOriginV1(import.meta.url").replace("@@B@@","artifact=import.meta.url").replace("@@C@@","capturedSources=import.meta.url")
    out=os.path.join(S,tag or re.sub(r"\W","_",rel[-40:]));os.makedirs(out,exist_ok=True);o=os.path.join(out,"shadow.test.ts");open(o,"w",encoding="utf8").write(t);return o
if __name__=="__main__":
    rel=sys.argv[1];t=open(os.path.join(R,rel),encoding="utf8").read()
    if len(sys.argv)>2:
        for old,new,c in json.load(open(sys.argv[2])):
            assert t.count(old)==c,(old[:60],t.count(old));t=t.replace(old,new)
    print(shadow(rel,t))
