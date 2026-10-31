import json,sys,os,re
from collections import Counter, defaultdict
G=sys.argv[1]; ROOT="/Users/ueli/Documents/semio"
OUT=sys.argv[2]
d=json.load(open(os.path.join(G,"policy29-findings.json")))
F=d['findings']
untr=set(x for x in open(os.path.join(G,"policy29-untracked.txt"),encoding="utf-8").read().split("\n") if x)
for f in F:
    if f['file'] in untr: f['new']=True
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
def area(p):
    if "text-splice" in p: return "reactor, rtb (keyword 'splice')"
    if "🔋️energy" in p or "energy" in p: return "energy"
    if "🧊️3d" in p or "task-resume" in p or "action-producer" in p: return "puzzlefin, resume"
    if "publication" in p or "native-build" in p: return "hostinit, policy, puzzlefin (keyword 'publication')"
    if "typing/edit" in p or "tool-machine/⌨️typing" in p: return "typedops, extfix (keyword 'typing')"
    if "tool-machine/🔗️read" in p: return "policy (keyword 'transfer')"
    if "typed-operations" in p: return "typedops, reactor, resume"
    if "render/📥️staging" in p: return "bridge, reactor, resume (keyword 'render')"
    if "op-binary" in p or "producer" in p or "command/🎟️typed" in p: return "dispatch, puzzlefin (keyword 'producer'/'op-binary')"
    if "job/" in p or "ownership" in p or "/👷️job" in p: return "dispatch, policy, reactor (keyword 'ownership')"
    if "writer" in p: return "puzzlefin, rtb (keyword 'writer')"
    if "layout" in p: return "cachegc, dispatch, rta (keyword 'layout')"
    return "unmapped"
def snippet(text, line_text_key=None, pat=None, width=110):
    m = re.search(pat, text) if pat else None
    if not m: return text[:width]
    a=max(0,m.start()-40); b=min(len(text), m.end()+60)
    return text[a:b]
lines=[]
new=[f for f in F if f['new'] and f['cat']!='9-ticket-copy-of-tracked']
cnt_new=Counter(f['cat'] for f in new)
cnt_all=Counter(f['cat'] for f in F if f['cat']!='9-ticket-copy-of-tracked')
lines.append(cnt_new)
pickle_out={"new":cnt_new,"all":cnt_all}
json.dump({"new":[{k:v for k,v in f.items()} for f in new]}, open(os.path.join(G,"policy29-new.json"),"w",encoding="utf-8"), ensure_ascii=False)
print(json.dumps(dict(cnt_new), ensure_ascii=False))
print(json.dumps(dict(cnt_all), ensure_ascii=False))
