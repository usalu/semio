#!/usr/bin/env python3
"""Appends the guard verdict cases to the attest fixture. Inputs are written here; the expected reports are computed by the independent oracle `judge` of the attest oracle (never by the TypeScript under test) and reviewed by hand."""
import copy,importlib.util,json,os,sys
HERE=sys.argv[1]
spec=importlib.util.spec_from_file_location("oracle",os.path.join(HERE,"🐍️.py"))
src=open(os.path.join(HERE,"🐍️.py"),encoding="utf8").read().split("try:\n    import jsonschema")[0]
ns={"__file__":os.path.join(HERE,"🐍️.py")}
exec(compile(src,"oracle","exec"),ns)
judge=ns["judge"]
path=os.path.join(HERE,"🧫️fixtures","🔣️.json")
fixture=json.load(open(path,encoding="utf8"))
fixture["verdicts"]=[case for case in fixture["verdicts"] if not case["name"].startswith("guards:")]
base=next(case for case in fixture["verdicts"] if case["name"]=="three origins agree")
LINUX={"version":"2.47.3","pins":"4c7bc38fc560","volume":{"caseInsensitive":False,"normalisationInsensitive":False,"symlinks":True,"longPaths":None},"infoExclude":0,"caseIgnore":0,"caseMismatch":0,"trackedIgnored":2,"trackedRefused":0,"links":0,"overBudget":0,"modeMismatch":0}
def case(name,changes):
    result=copy.deepcopy(base)
    result["name"]="guards: "+name
    for origin,change in changes.items():
        guards=copy.deepcopy(LINUX) if change is not None else None
        if change is not None:
            for key,value in change.items():
                if key=="volume": guards["volume"].update(value)
                else: guards[key]=value
        result["attestations"][origin]["guards"]=guards
    result["expected"]=judge(result)
    return result
mac={"volume":{"caseInsensitive":True,"normalisationInsensitive":True},"version":"2.54.0"}
windows={"volume":{"caseInsensitive":True,"longPaths":True},"modeMismatch":None,"version":"2.47.3"}
cases=[
 case("identical guards agree",{"a":{},"b":{},"c":{}}),
 case("a different pins hash is named",{"a":{},"b":{},"c":{"pins":"0123456789ab"}}),
 case("private exclude rules on one machine are named",{"a":{},"b":{"infoExclude":2},"c":{}}),
 case("a case mismatch on one machine is named",{"a":{},"b":{},"c":{"caseMismatch":1}}),
 case("case-dependent ignore rules, tracked-but-ignored and refused tracked paths are named",{"a":{},"b":{"caseIgnore":1,"trackedIgnored":3},"c":{"trackedRefused":1}}),
 case("capability differences only warn: git version, volume behaviour, a Windows machine that cannot read the executable bit",{"a":{},"b":mac,"c":windows}),
 case("paths over the budget and links without link support warn and the verdict stays ok",{"a":{},"b":{"overBudget":2,"links":0},"c":{"links":3,"volume":{"symlinks":False}}}),
 case("a machine without guards is exempt",{"a":{},"b":None,"c":{}}),
 case("a mode mismatch is compared among the machines that can read it",{"a":{"modeMismatch":1},"b":{"modeMismatch":1},"c":{"modeMismatch":None}}),
 case("a mode mismatch on one POSIX machine is named",{"a":{},"b":{},"c":{"modeMismatch":4}}),
]
fixture["verdicts"].extend(cases)
open(path,"w",encoding="utf8").write(json.dumps(fixture,ensure_ascii=False,indent=1)+"\n")
for item in cases: print(item["name"],"->",[(r["originId"],r["verdict"],r["differs"]) for r in item["expected"]["rows"]],item["expected"]["warnings"],item["expected"]["ok"])
