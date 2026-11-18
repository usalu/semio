import sys,json
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old,s.count(old))
    s=s.replace(old,new)
rep('"attachMaximumPollMilliseconds": 120000\n','"attachMaximumPollMilliseconds": 120000,\n        "stallMilliseconds": 600000,\n        "askGraceMilliseconds": 300000,\n        "progressWriteMilliseconds": 5000,\n        "maximumHandovers": 3,\n        "commandCharacters": 200\n')
rep('"required": ["version", "token", "ticket", "owners", "structural", "state", "queuedMs"],\n      "properties": {','"required": ["version", "token", "ticket", "owners", "structural", "state", "queuedMs"],\n      "properties": {\n        "holder": {"$ref": "#/definitions/Holder"},')
rep('"blocked": "graph: {peers} refresh(es) of the same projects go first ({seconds} s; shared: {sample})"','"blocked": "graph: {peers} refresh(es) of the same projects go first ({seconds} s; shared: {sample}){holder}"')
rep('"blockedAll": "graph: {peers} refresh(es) go first, one of them rebuilds the whole workspace ({seconds} s)"','"blockedAll": "graph: {peers} refresh(es) go first, one of them rebuilds the whole workspace ({seconds} s){holder}"')
rep('"blocked": "Graph: {peers} Erneuerung(en) derselben Projekte sind zuerst dran ({seconds} s; gemeinsam: {sample})"','"blocked": "Graph: {peers} Erneuerung(en) derselben Projekte sind zuerst dran ({seconds} s; gemeinsam: {sample}){holder}"')
rep('"blockedAll": "Graph: {peers} Erneuerung(en) sind zuerst dran, eine davon baut den ganzen Arbeitsbereich neu ({seconds} s)"','"blockedAll": "Graph: {peers} Erneuerung(en) sind zuerst dran, eine davon baut den ganzen Arbeitsbereich neu ({seconds} s){holder}"')
en={
"holderSuffix":"; holder: {holder}",
"holder":"pid {pid} ({command}), started {started} UTC, code {digest}, {phase} {done} of {total}, {idle} s without progress",
"askedStalled":"graph: pid {pid} made no progress for {seconds} s; it is asked to hand over at its next unit, what it finished stays in the store",
"askedStale":"graph: pid {pid} runs code {digest} but the library is at {current}; it is asked to hand over at its next unit, what it finished stays in the store",
"deposed":"graph: pid {pid} did not answer the request to hand over for {seconds} s; the door passes on",
"handoverStalled":"graph: this read made no progress for {seconds} s while others wait; it hands over after {done} of {total} owners, what it finished is kept and the request starts again",
"handoverStale":"graph: this read runs code {digest} but the library is at {current}; it hands over after {done} of {total} owners, what it finished is kept and the request starts again on the current code"}
de={
"holderSuffix":"; Halter: {holder}",
"holder":"pid {pid} ({command}), gestartet {started} UTC, Code {digest}, {phase} {done} von {total}, seit {idle} s ohne Fortschritt",
"askedStalled":"Graph: pid {pid} machte {seconds} s keinen Fortschritt; es wird gebeten, an der nächsten Einheit zu übergeben, Fertiges bleibt im Speicher",
"askedStale":"Graph: pid {pid} führt Code {digest} aus, die Bibliothek steht auf {current}; es wird gebeten, an der nächsten Einheit zu übergeben, Fertiges bleibt im Speicher",
"deposed":"Graph: pid {pid} hat die Bitte zur Übergabe {seconds} s lang nicht beantwortet; die Tür geht weiter",
"handoverStalled":"Graph: dieses Lesen machte {seconds} s keinen Fortschritt, während andere warten; es übergibt nach {done} von {total} Eigentümern, Fertiges bleibt erhalten und die Anfrage startet neu",
"handoverStale":"Graph: dieses Lesen führt Code {digest} aus, die Bibliothek steht auf {current}; es übergibt nach {done} von {total} Eigentümern, Fertiges bleibt erhalten und die Anfrage startet auf dem aktuellen Code neu"}
def lines(d,ind): return ",\n".join(ind+json.dumps(k,ensure_ascii=False)+": "+json.dumps(v,ensure_ascii=False) for k,v in d.items())
a='"closureRepaired": "graph: the build input closure {key}... is gone from this machine; {removed} record(s) that named it were removed and are computed again"'
rep(a,a+",\n"+lines(en,"          "))
b='"closureRepaired": "Graph: der Eingabe-Abschluss {key}... fehlt auf diesem Rechner; {removed} Datensatz/Datensätze, die ihn nannten, wurden entfernt und werden neu berechnet"'
rep(b,b+",\n"+lines(de,"          "))
defs='''    "Holder": {
      "description": "Who builds: the process that holds or queues a claim, written once with the claim so every waiter and the front door can say who is ahead. `digest` is the code identity the process runs (empty when unknown).",
      "type": "object",
      "additionalProperties": false,
      "required": ["pid", "command", "startedMs", "digest"],
      "properties": {
        "pid": {"type": "integer", "minimum": 1, "maximum": 4294967295},
        "command": {"type": "string", "maxLength": 200},
        "startedMs": {"type": "integer", "minimum": 0, "maximum": 9007199254740991},
        "digest": {"type": "string", "pattern": "^(?:[a-f0-9]{64})?$"}
      }
    },
    "Progress": {
      "description": "Sidecar `<token>.progress.json` of a held claim: the phase and owner counters of its build and the time of its last ADVANCE (the phase changed or `done` grew). A holder that does not advance for `stallMilliseconds` is asked to hand over; progress, not elapsed time, decides.",
      "type": "object",
      "additionalProperties": false,
      "required": ["phase", "done", "total", "advancedMs"],
      "properties": {
        "phase": {"type": "string", "pattern": "^[a-z]{1,40}$"},
        "done": {"type": "integer", "minimum": 0, "maximum": 9007199254740991},
        "total": {"type": "integer", "minimum": 0, "maximum": 9007199254740991},
        "advancedMs": {"type": "integer", "minimum": 0, "maximum": 9007199254740991}
      }
    },
    "Ask": {
      "description": "Sidecar `<token>.ask.json` written by a waiter: the holder is asked to stop at its next unit boundary, keep what it finished and release. `stalled`: no advance for `stallMilliseconds`; `stale-code`: the code digest of the holder differs from the current library. A holder that has not answered `askGraceMilliseconds` after `atMs` no longer blocks the door (it is deposed).",
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "reason", "atMs", "by"],
      "properties": {
        "version": {"const": 1},
        "reason": {"enum": ["stalled", "stale-code"]},
        "atMs": {"type": "integer", "minimum": 0, "maximum": 9007199254740991},
        "by": {
          "type": "object",
          "additionalProperties": false,
          "required": ["pid", "digest"],
          "properties": {"pid": {"type": "integer", "minimum": 0}, "digest": {"type": "string"}}
        }
      }
    },
    "DoorCase": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "claim", "now", "current", "reason", "idleMs", "deposed"],
      "properties": {
        "id": {"type": "string"},
        "claim": {"type": "object"},
        "now": {"type": "integer"},
        "current": {"type": ["string", "null"]},
        "reason": {"enum": ["stalled", "stale-code", null]},
        "idleMs": {"type": ["integer", "null"]},
        "deposed": {"type": "boolean"}
      }
    },
'''
rep('    "AdmissionCase": {',defs+'    "AdmissionCase": {')
open(p,"w",encoding="utf8").write(s)
json.loads(s)
