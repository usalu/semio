import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:90],s.count(old))
    s=s.replace(old,new)
rep('import {readFile} from "node:fs/promises";','import {readFile,stat} from "node:fs/promises";')
rep('JoinRefusedV1,relayOnceV1,','JoinRefusedV1,OriginTakenV1,relayOnceV1,')
rep('import {fleetReportV1,freshSecondsV1,readAttestationV1,','import {attestPathV1,codeDigestV1,fleetReportV1,freshSecondsV1,readAttestationV1,')
rep('import {formatHlcV1,listOriginsV1,parseHlcV1} from "../../📒️journal/🟦️.ts";','import {formatHlcV1,listOriginsV1,parseHlcV1} from "../../📒️journal/🟦️.ts";\nimport {selfCheckV1} from "../../🛡️resilience/🟦️.ts";')
rep('|"rebase"|"repair"|"retire"','|"rebase"|"repair"|"retire"|"self-check"') if '|"rebase"|"repair"|"retire"' in s else None
open(p,"w",encoding="utf8").write(s)
print([l for l in s.split("\n") if l.startswith("type Operation")][0][:300])
print([l for l in s.split("\n") if "retire:{valued" in l])
