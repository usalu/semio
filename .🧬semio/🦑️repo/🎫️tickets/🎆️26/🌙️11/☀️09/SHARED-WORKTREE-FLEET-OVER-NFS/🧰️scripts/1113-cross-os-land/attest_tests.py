from lib import *
import json
def py(t):
    s=read(src("🩺️attest/🧪️tests/🐍️.py"))
    t=sub(t,'FIELDS=["manifest","head.commit","indexTree","view.digest"]',between(s,'FIELDS=["manifest"','failures=[]').rstrip("\n")+"\n")
    t=sub(t,'''    return attestation["view"]["digest"] if attestation["view"] else None
''',between(s,'    if field=="view.digest": return attestation["view"]["digest"]','def view_of(porcelain):'))
    t=sub(t,'voters=[origin for origin in live if field!="view.digest" or value_of(attestations[origin],field) is not None]','voters=[origin for origin in live if field in ALWAYS or value_of(attestations[origin],field) is not None]')
    t=sub(t,'return {"version":1,"ok":ok,"freshSeconds":60,"rows":ordered,"differing":differing}','return {"version":1,"ok":ok,"freshSeconds":60,"rows":ordered,"differing":differing,"warnings":capabilities(live,attestations)}')
    return t
edit("🩺️attest/🧪️tests/🐍️.py",py)
def ts(t):
    s=read(src("🩺️attest/🧪️tests/🟦️.ts"))
    t=sub(t,'import {attestPathV1,codeDigestV1,','import {attestPathV1,codeDigestV1,type FleetGuardsV1,')
    t=sub(t,'head:null,indexTree:null,view:null,','head:null,indexTree:null,view:null,guards:null,')
    block=between(s,'const guardsOf=','test("an origin without git is exempt from the view agreement"')
    t=sub(t,'test("an origin without git is exempt from the view agreement"',block+'test("an origin without git is exempt from the view agreement"')
    return t
edit("🩺️attest/🧪️tests/🟦️.ts",ts)
def fixture(t):
    d=json.loads(t)
    for case in d["verdicts"]:
        for a in case["attestations"].values(): a["guards"]=None
        case["expected"]["warnings"]=[]
    return json.dumps(d,ensure_ascii=False,indent=1)+"\n"
edit("🩺️attest/🧪️tests/🧫️fixtures/🔣️.json",fixture)
