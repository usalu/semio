import json
L="🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/"
FE="️"
path=L+"\U0001F9EC"+FE+"schema/\U0001F523"+FE+".json"
integer=lambda lo,hi:{"type":"integer","minimum":lo,"maximum":hi}
text=lambda mx:{"type":"string","minLength":1,"maxLength":mx}
priorityEnum={"enum":["interactive","agent","background"]}
policy={
 "version":1,
 "priority":{"environment":"SEMIO_LEASE_PRIORITY","default":"agent","classes":[
   {"id":"interactive","rank":0,"advanceMilliseconds":3600000},
   {"id":"agent","rank":1,"advanceMilliseconds":0},
   {"id":"background","rank":2,"advanceMilliseconds":-3600000}]},
 "progress":{"publishIntervalMilliseconds":5000,"maximumPhaseCharacters":96,
   "families":[{"prefix":"build:graph-phase:","pattern":"^build:graph-phase:([a-z]+):([0-9]+):([0-9]+)"}],
   "counter":"^(.{1,80}?):([0-9]+)$","ignoredPrefixes":["lease-queue:","lease-sweep:","claim-retirement-scan:"]},
 "eta":{"minimumFraction":0.02,"minimumSamples":2,"maximumSamples":32,"minimumHoldMilliseconds":5000,"overdueFactor":1.5},
 "freshness":{"silentAfterMilliseconds":60000,
   "stuckAfterMilliseconds":[
     {"prefix":"repository-graph-facts:","milliseconds":1200000},
     {"prefix":"build-graph-state:","milliseconds":1200000},
     {"prefix":"","milliseconds":3600000}]},
 "yield":{"pollMilliseconds":5000,"requestMaximumAgeMilliseconds":600000,"directory":"yield","stuckReason":"stuck","staleReason":"stale"},
 "reclaim":{"mode":"request","environment":"SEMIO_LEASE_RECLAIM","graceMilliseconds":120000,"killGraceMilliseconds":15000,"identityToleranceMilliseconds":3000,"legacyAfterMilliseconds":5400000,"logFile":"reclaim.jsonl"},
 "history":{"file":"history.jsonl","maximumRows":512,"minimumHoldMilliseconds":5000}}
policy_schema={"type":"object","additionalProperties":False,"required":list(policy),"properties":{
 "version":{"const":1},
 "priority":{"type":"object","additionalProperties":False,"required":["environment","default","classes"],"properties":{
   "environment":text(64),"default":priorityEnum,
   "classes":{"type":"array","minItems":3,"maxItems":3,"items":{"type":"object","additionalProperties":False,"required":["id","rank","advanceMilliseconds"],"properties":{"id":priorityEnum,"rank":integer(0,2),"advanceMilliseconds":integer(-86400000,86400000)}}}}},
 "progress":{"type":"object","additionalProperties":False,"required":["publishIntervalMilliseconds","maximumPhaseCharacters","families","counter","ignoredPrefixes"],"properties":{
   "publishIntervalMilliseconds":integer(250,600000),"maximumPhaseCharacters":integer(16,256),
   "families":{"type":"array","items":{"type":"object","additionalProperties":False,"required":["prefix","pattern"],"properties":{"prefix":text(128),"pattern":text(256)}}},
   "counter":text(256),"ignoredPrefixes":{"type":"array","items":text(64)}}},
 "eta":{"type":"object","additionalProperties":False,"required":["minimumFraction","minimumSamples","maximumSamples","minimumHoldMilliseconds","overdueFactor"],"properties":{
   "minimumFraction":{"type":"number","exclusiveMinimum":0,"maximum":1},"minimumSamples":integer(1,64),"maximumSamples":integer(1,512),"minimumHoldMilliseconds":integer(0,3600000),"overdueFactor":{"type":"number","minimum":1,"maximum":100}}},
 "freshness":{"type":"object","additionalProperties":False,"required":["silentAfterMilliseconds","stuckAfterMilliseconds"],"properties":{
   "silentAfterMilliseconds":integer(1000,86400000),
   "stuckAfterMilliseconds":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["prefix","milliseconds"],"properties":{"prefix":{"type":"string","maxLength":128},"milliseconds":integer(60000,604800000)}}}}},
 "yield":{"type":"object","additionalProperties":False,"required":["pollMilliseconds","requestMaximumAgeMilliseconds","directory","stuckReason","staleReason"],"properties":{
   "pollMilliseconds":integer(250,600000),"requestMaximumAgeMilliseconds":integer(1000,86400000),"directory":text(32),"stuckReason":{"const":"stuck"},"staleReason":{"const":"stale"}}},
 "reclaim":{"type":"object","additionalProperties":False,"required":["mode","environment","graceMilliseconds","killGraceMilliseconds","identityToleranceMilliseconds","legacyAfterMilliseconds","logFile"],"properties":{
   "mode":{"enum":["report","request","terminate"]},"environment":text(64),"graceMilliseconds":integer(1000,3600000),"killGraceMilliseconds":integer(1000,600000),"identityToleranceMilliseconds":integer(0,60000),"legacyAfterMilliseconds":integer(60000,604800000),"logFile":text(64)}},
 "history":{"type":"object","additionalProperties":False,"required":["file","maximumRows","minimumHoldMilliseconds"],"properties":{"file":text(64),"maximumRows":integer(16,100000),"minimumHoldMilliseconds":integer(0,3600000)}}}}
components={"type":"object","maxProperties":64,"propertyNames":{"maxLength":64},"additionalProperties":{"type":"string","pattern":"^[0-9a-f]{12}$"}}
implementation={"type":"object","additionalProperties":False,"required":["digest","loadedAtMs","components"],"properties":{"digest":{"type":"string","pattern":"^[0-9a-f]{16}$"},"loadedAtMs":integer(0,281474976710655),"components":components}}
standing={"type":"object","additionalProperties":False,"required":["version","priority","implementation","progressAtMs"],"properties":{
 "version":{"const":1},"priority":priorityEnum,"implementation":{"$ref":"#/definitions/Implementation"},
 "phase":text(256),"done":integer(0,9007199254740991),"total":integer(0,9007199254740991),"phaseStartedAtMs":integer(0,281474976710655),
 "progressAtMs":integer(0,281474976710655),
 "parentPid":integer(1,9999999999),"chain":{"type":"array","maxItems":16,"items":integer(1,9999999999)}}}
request={"type":"object","additionalProperties":False,"required":["version","holderPid","holderStartedAtMs","requesterPid","requesterStartedAtMs","resource","reason","atMs","requesterComponents"],"properties":{
 "version":{"const":1},"holderPid":integer(1,9999999999),"holderStartedAtMs":integer(0,281474976710655),"requesterPid":integer(1,9999999999),"requesterStartedAtMs":integer(0,281474976710655),
 "resource":text(4096),"reason":{"enum":["stale","stuck"]},"atMs":integer(0,281474976710655),"requesterComponents":components}}
def msg(en,de):return {"en":en,"de":de}
messages={
 "holderProgress":msg("{phase}: {done} of {total}","{phase}: {done} von {total}"),
 "holderPhase":msg("{phase}","{phase}"),
 "holderIdle":msg("no phase reported","keine Phase gemeldet"),
 "eta":msg("about {minutes} min left","noch etwa {minutes} min"),
 "etaOverdue":msg("longer than usual","länger als üblich"),
 "state.progressing":msg("progressing","macht Fortschritt"),
 "state.silent":msg("silent for {seconds} s","seit {seconds} s ohne Fortschritt"),
 "state.stuck":msg("STUCK: no progress for {minutes} min","FESTGEFAHREN: seit {minutes} min kein Fortschritt"),
 "state.stale":msg("STALE: runs older code ({ids}); its result would be discarded","VERALTET: läuft mit älterem Code ({ids}); sein Ergebnis würde verworfen"),
 "state.legacy":msg("no standing published (older implementation)","kein Zustand veröffentlicht (ältere Implementierung)"),
 "status.standing":msg("        priority {priority} · code loaded {age} s ago · {progress}{eta} · {state}{chain}{request}","        Priorität {priority} · Code vor {age} s geladen · {progress}{eta} · {state}{chain}{request}"),
 "status.legacy":msg("        {state}","        {state}"),
 "status.chain":msg(" · started by {chain}"," · gestartet von {chain}"),
 "status.request":msg(" · yield requested by process {pid} ({reason})"," · Freigabe angefordert von Prozess {pid} ({reason})"),
 "yield.stale":msg("Yielding {resource}: process {pid} runs newer code ({ids}); this result would be discarded.","Gebe {resource} frei: Prozess {pid} läuft mit neuerem Code ({ids}); dieses Ergebnis würde verworfen."),
 "yield.stuck":msg("Yielding {resource}: no progress for {minutes} min and process {pid} is waiting.","Gebe {resource} frei: seit {minutes} min kein Fortschritt und Prozess {pid} wartet."),
 "request.sent":msg("Asked process {pid} to yield {resource} ({reason}).","Prozess {pid} gebeten, {resource} freizugeben ({reason})."),
 "reclaim.terminated":msg("Stopped process {pid}: it held {resource} for {minutes} min without progress and ignored the yield request.","Prozess {pid} beendet: hielt {resource} seit {minutes} min ohne Fortschritt und ignorierte die Freigabebitte."),
 "reclaim.refused":msg("Not stopping process {pid}: {reason}.","Prozess {pid} wird nicht beendet: {reason}."),
 "refusal.identity":msg("its start time does not match the registered one","seine Startzeit passt nicht zur registrierten"),
 "refusal.progressed":msg("it made progress since the request","er hat seit der Bitte Fortschritt gemacht"),
 "refusal.owner":msg("it belongs to another user","er gehört einem anderen Benutzer"),
 "refusal.unknown":msg("its start time cannot be read on this platform","seine Startzeit ist auf dieser Plattform nicht lesbar"),
 "refusal.grace":msg("the grace period after the request has not ended","die Schonfrist nach der Bitte läuft noch"),
}
corpus={"type":"object","additionalProperties":False,"required":["version","priorityCases","simulations","stateCases","etaCases","progressCases"],"properties":{
 "version":{"const":1},
 "priorityCases":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","environment","agentSession","interactiveTerminal","expected"],"properties":{"id":text(64),"environment":{"type":"object","additionalProperties":{"type":"string"}},"agentSession":{"type":"boolean"},"interactiveTerminal":{"type":"boolean"},"expected":priorityEnum}}},
 "simulations":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","jobs"],"properties":{"id":text(64),"source":text(512),"resource":text(256),"spanSeconds":integer(0,10000000),"jobs":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","class","arrival","service"],"properties":{"id":text(64),"class":priorityEnum,"arrival":integer(0,10000000),"service":integer(0,10000000),"lane":text(64),"observedWait":integer(0,10000000)}}}}}},
 "stateCases":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","nowMs","holdSinceMs","resource","expected"],"properties":{"id":text(64),"nowMs":integer(0,281474976710655),"holdSinceMs":integer(0,281474976710655),"resource":text(256),"standing":{"$ref":"#/definitions/Standing"},"waiter":{"$ref":"#/definitions/Implementation"},"bound":{"type":"array","items":text(64)},"expected":{"type":"object","additionalProperties":False,"required":["state"],"properties":{"state":{"enum":["legacy","progressing","silent","stuck","stale"]},"staleIds":{"type":"array","items":text(64)}}}}}},
 "progressCases":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","resource","expected"],"properties":{"id":text(64),"resource":text(256),"expected":{"oneOf":[{"type":"null"},{"type":"object","additionalProperties":False,"required":["phase"],"properties":{"phase":text(96),"done":integer(0,9007199254740991),"total":integer(0,9007199254740991)}}]}}}},
 "etaCases":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","elapsedMs","phaseElapsedMs","historyMs","expected"],"properties":{"id":text(64),"elapsedMs":integer(0,86400000000),"phaseElapsedMs":integer(0,86400000000),"done":integer(0,9007199254740991),"total":integer(0,9007199254740991),"historyMs":{"type":"array","items":integer(0,86400000000)},"expected":{"oneOf":[{"type":"null"},{"type":"object","additionalProperties":False,"required":["seconds","source","overdue"],"properties":{"seconds":integer(0,31536000),"source":{"enum":["progress","history"]},"overdue":{"type":"boolean"}}}]}}}}}}
corpus["required"]+=["yieldCases","terminateCases"]
corpus["properties"]["yieldCases"]={"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","request","pid","startedAtMs","nowMs","holdsExclusive","holdSinceMs","standing","bound","expected"],"properties":{"id":text(64),"request":{"$ref":"#/definitions/YieldRequest"},"pid":integer(1,9999999999),"startedAtMs":integer(0,281474976710655),"nowMs":integer(0,281474976710655),"holdsExclusive":{"type":"boolean"},"holdSinceMs":integer(0,281474976710655),"standing":{"$ref":"#/definitions/Standing"},"bound":{"type":"array","items":text(64)},"expected":{"type":"object","additionalProperties":False,"required":["accept"],"properties":{"accept":{"type":"boolean"},"refusal":text(32),"ids":{"type":"array","items":text(64)}}}}}}
corpus["properties"]["terminateCases"]={"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","input","expected"],"properties":{"id":text(64),"input":{"type":"object","additionalProperties":False,"required":["mode","state","nowMs","progressAtMs","ticketArrivalMs","signalable","self"],"properties":{"mode":{"enum":["report","request","terminate"]},"state":{"enum":["legacy","progressing","silent","stuck","stale"]},"nowMs":integer(0,281474976710655),"requestAtMs":integer(0,281474976710655),"progressAtMs":integer(0,281474976710655),"registeredStartedAtMs":integer(0,281474976710655),"osStartedAtMs":integer(0,281474976710655),"ticketArrivalMs":integer(0,281474976710655),"signalable":{"type":"boolean"},"self":{"type":"boolean"}}},"expected":{"type":"object","additionalProperties":False,"required":["terminate"],"properties":{"terminate":{"type":"boolean"},"refusal":text(32)}}}}}
doc={"$schema":"http://json-schema.org/draft-07/schema#","$id":"schema://framework.process/ResourceLeaseStandingV1","definitions":{
 "PolicySchema":policy_schema,"Policy":{"const":policy},"Implementation":implementation,"Standing":standing,"YieldRequest":request,"Messages":{"const":messages},"Corpus":corpus}}
with open(path,"w") as handle:
    json.dump(doc,handle,indent=1,ensure_ascii=False);handle.write("\n")
print(path,len(open(path).read()))
