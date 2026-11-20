import json,re,shutil,sys,os
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226"
P="🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order"
R="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree"
def rd(p): return open(f"{S}/base/{p}",encoding="utf8").read()
def wr(p,t):
    os.makedirs(os.path.dirname(f"{S}/new/{p}"),exist_ok=True); open(f"{S}/new/{p}","w",encoding="utf8").write(t)
def once(t,old,new):
    assert t.count(old)==1,(old[:60],t.count(old)); return t.replace(old,new)

# ---- schema
sp=P+"/🧬️schema/🔣️.json"; t=rd(sp)
wait_def='''    "Wait": {
      "type": "object",
      "additionalProperties": false,
      "description": "The declared wait of one lease class: how long a request without its own timeout waits (boundMilliseconds), the longest a caller may ask for (maximumMilliseconds), how often a waiting request says who it waits for (heartbeatMilliseconds), how long a registered holder may go without renewing its entry before it is judged silent (silentMilliseconds), and what the refusal tells the person (hint). Bounds are written for the reference machine and scaled by the machine speed factor. A wait that runs out is always the typed refusal `ResourceLeaseTimeoutErrorV1` naming the holder.",
      "required": ["boundMilliseconds", "maximumMilliseconds", "heartbeatMilliseconds", "silentMilliseconds", "hint"],
      "properties": {
        "boundMilliseconds": {"type": "integer", "minimum": 100, "maximum": 86400000},
        "maximumMilliseconds": {"type": "integer", "minimum": 100, "maximum": 86400000},
        "heartbeatMilliseconds": {"type": "integer", "minimum": 100, "maximum": 600000},
        "silentMilliseconds": {"type": "integer", "minimum": 100, "maximum": 600000},
        "hint": {
          "type": "object",
          "additionalProperties": false,
          "required": ["en", "de"],
          "properties": {"en": {"type": "string", "minLength": 1}, "de": {"type": "string", "minLength": 1}}
        }
      }
    },
'''
t=once(t,'    "Class": {\n',wait_def+'    "Class": {\n')
t=once(t,'''        "title",
        "match"
      ],
      "properties": {
        "id": {''','''        "title",
        "match",
        "wait"
      ],
      "properties": {
        "wait": {
          "$ref": "#/definitions/Wait"
        },
        "id": {''')
t=once(t,'''        "classes",
        "allowed"
      ],''','''        "classes",
        "unclassifiedWait",
        "allowed"
      ],''')
t=once(t,'''        "classes": {
          "type": "array",
          "minItems": 1,''','''        "unclassifiedWait": {
          "$ref": "#/definitions/Wait"
        },
        "classes": {
          "type": "array",
          "minItems": 1,''')
msg_old='''        "modeShared": {
          "en": "shared",'''
msg_new='''        "timeoutDeclared": {
          "en": " This is the declared wait of the lease class {class}: at most {limit} s on this machine ({base} s declared, speed factor {factor}); the holder {verdict}. {hint}",
          "de": " Das ist die festgelegte Wartezeit der Lease-Klasse {class}: höchstens {limit} s auf diesem Rechner ({base} s festgelegt, Geschwindigkeitsfaktor {factor}); der Halter {verdict}. {hint}"
        },
        "verdictProgressing": {
          "en": "is alive and renews its registration, so it is working",
          "de": "lebt und erneuert seine Anmeldung, er arbeitet also"
        },
        "verdictSilent": {
          "en": "is alive but has not renewed its registration for {silent} s, so it may be blocked or its process number was reused",
          "de": "lebt, hat seine Anmeldung aber seit {silent} s nicht erneuert, er ist womöglich blockiert oder seine Prozessnummer wurde wiederverwendet"
        },
        "verdictUnregistered": {
          "en": "is not in the lease registry",
          "de": "steht nicht in der Lease-Registrierung"
        },
        "waiting": {
          "en": "Process {pid} ({command}) holds the lease {resource} since {since}{silent}; waiting for it ({seconds} s of at most {limit} s)",
          "de": "Der Prozess {pid} ({command}) hält die Lease {resource} seit {since}{silent}; es wird darauf gewartet ({seconds} s von höchstens {limit} s)"
        },
        "waitingSilent": {
          "en": " and has not renewed its registration for {silent} s, so it may be blocked",
          "de": " und hat seine Anmeldung seit {silent} s nicht erneuert, er ist womöglich blockiert"
        },
        "waitingSelf": {
          "en": "Another task of this very process holds the lease {resource} since {since}; waiting for it ({seconds} s of at most {limit} s)",
          "de": "Eine andere Task desselben Prozesses hält die Lease {resource} seit {since}; es wird darauf gewartet ({seconds} s von höchstens {limit} s)"
        },
        "waitingUnregistered": {
          "en": "Waiting for lease {resource} ({mode}); no holder is registered (a process of older code or a claim queue ahead); waiting for it ({seconds} s of at most {limit} s)",
          "de": "Warten auf die Lease {resource} ({mode}); kein Halter ist angemeldet (ein Prozess mit älterem Code oder eine Warteschlange davor); es wird darauf gewartet ({seconds} s von höchstens {limit} s)"
        },
'''
t=once(t,msg_old,msg_new+msg_old)
wr(sp,t)

# ---- policy
pp=P+"/🔣️policy.json"; t=rd(pp)
H=lambda en,de:{"en":en,"de":de}
W={ # id: (bound,max,heartbeat,hint)
"pin":(120000,600000,30000,H("Nobody waits for a custody pin: a publication holds it. Run the command again.","Auf einen Obhuts-Pin wartet niemand: eine Veröffentlichung hält ihn. Den Befehl erneut ausführen.")),
"toolchain":(1800000,21600000,30000,H("A provisioner or a whole build holds the Rust toolchain. Run the command again once it has finished, or run `repo tools setup all` first.","Ein Bereitsteller oder ein ganzer Build hält die Rust-Toolchain. Den Befehl erneut ausführen, sobald er fertig ist, oder zuerst `repo tools setup all` ausführen.")),
"compiler-capacity":(600000,3600000,30000,H("The machine's compiler capacity is taken. Run the command again when a running build has finished.","Die Compiler-Kapazität des Rechners ist vergeben. Den Befehl erneut ausführen, wenn ein laufender Build fertig ist.")),
"tool-singleton":(1800000,21600000,30000,H("Another process sets up or downloads this tool. Run the command again once it has finished.","Ein anderer Prozess richtet dieses Werkzeug ein oder lädt es herunter. Den Befehl erneut ausführen, sobald er fertig ist.")),
"image-mapping":(1800000,1800000,30000,H("The native image is mapped by another process. Run the command again once it has finished.","Das native Abbild ist von einem anderen Prozess eingebunden. Den Befehl erneut ausführen, sobald er fertig ist.")),
"bazel-picker":(600000,3600000,30000,H("Another process picks the Bazel slot. Run the command again.","Ein anderer Prozess wählt den Bazel-Platz. Den Befehl erneut ausführen.")),
"descriptor-custody":(600000,3600000,30000,H("Another process owns the Rust descriptor namespace. Run the command again once it has finished.","Ein anderer Prozess besitzt den Rust-Deskriptor-Namensraum. Den Befehl erneut ausführen, sobald er fertig ist.")),
"cargo-coordination":(1800000,21600000,30000,H("Another Cargo run holds this crate. Run the command again once it has finished.","Ein anderer Cargo-Lauf hält diese Crate. Den Befehl erneut ausführen, sobald er fertig ist.")),
"graph":(1800000,21600000,30000,H("A graph read or build holds the graph. Run the command again once it has finished; a read that renews its claim is working.","Ein Graph-Lesevorgang oder Build hält den Graphen. Den Befehl erneut ausführen, sobald er fertig ist; ein Lesevorgang, der seinen Anspruch erneuert, arbeitet.")),
"daemon-slot":(120000,600000,30000,H("The daemon slot is taken. Run the command again.","Der Daemon-Platz ist belegt. Den Befehl erneut ausführen.")),
"namespace":(1800000,21600000,30000,H("Another process works in this code namespace. Run the command again once it has finished.","Ein anderer Prozess arbeitet in diesem Code-Namensraum. Den Befehl erneut ausführen, sobald er fertig ist.")),
"artifact":(1800000,21600000,30000,H("Another process publishes or captures this artifact. Run the command again once it has finished.","Ein anderer Prozess veröffentlicht oder erfasst dieses Artefakt. Den Befehl erneut ausführen, sobald er fertig ist.")),
"native-whole-retirement":(1800000,21600000,30000,H("A retirement of native generations is running. Run the command again once it has finished.","Eine Ausmusterung nativer Generationen läuft. Den Befehl erneut ausführen, sobald sie fertig ist.")),
"image":(1800000,21600000,30000,H("Readers or a maintenance hold the dependency store or its journal. Run the command again once the holder has finished; a holder that renews its claim is working.","Leser oder eine Wartung halten den Abhängigkeitsspeicher oder sein Journal. Den Befehl erneut ausführen, sobald der Halter fertig ist; ein Halter, der seinen Anspruch erneuert, arbeitet.")),
"registry-fetch":(1800000,21600000,30000,H("Another process fetches the Cargo registry. Run the command again once it has finished.","Ein anderer Prozess lädt die Cargo-Registry. Den Befehl erneut ausführen, sobald er fertig ist.")),
"cargo-custody":(600000,3600000,30000,H("Another process holds the Cargo custody of this crate. Run the command again once it has finished.","Ein anderer Prozess hält die Cargo-Obhut dieser Crate. Den Befehl erneut ausführen, sobald er fertig ist.")),
"daemon-startup":(600000,3600000,30000,H("A daemon is starting. Run the command again in a moment.","Ein Daemon startet gerade. Den Befehl in einem Moment erneut ausführen.")),
}
def block(b,m,h,hint,ind):
    return (f'{ind}"wait": {{\n{ind}  "boundMilliseconds": {b},\n{ind}  "maximumMilliseconds": {m},\n{ind}  "heartbeatMilliseconds": {h},\n{ind}  "silentMilliseconds": 300000,\n{ind}  "hint": {{\n{ind}    "en": {json.dumps(hint["en"],ensure_ascii=False)},\n{ind}    "de": {json.dumps(hint["de"],ensure_ascii=False)}\n{ind}  }}\n{ind}}},\n')
seen=set()
def add(m):
    cid=m.group(2)
    return m.group(0)
# insert after each class's "scope" line inside classes: find '"id": "X",\n      "rank"'
for cid,(b,m,h,hint) in W.items():
    pat=re.compile(r'(      "id": "%s",\n)'%re.escape(cid))
    assert len(pat.findall(t))==1,cid
    t=pat.sub(lambda mo:mo.group(1)+block(b,m,h,hint,"      "),t); seen.add(cid)
classes=set(re.findall(r'\n      "id": "([a-z-]+)",\n      "wait"',t))
allc=set(re.findall(r'\n      "id": "([a-z-]+)",\n',t))
print("classes",len(W),"missing",allc-classes)
t=once(t,'  "classes": [\n','  "unclassifiedWait": {\n    "boundMilliseconds": 1800000,\n    "maximumMilliseconds": 21600000,\n    "heartbeatMilliseconds": 30000,\n    "silentMilliseconds": 300000,\n    "hint": {\n      "en": "Another process holds this resource. Run the command again once the holder has finished.",\n      "de": "Ein anderer Prozess hält diese Ressource. Den Befehl erneut ausführen, sobald der Halter fertig ist."\n    }\n  },\n  "classes": [\n')
wr(pp,t)
json.loads(t)
# ---- contract type
cp=P+"/📋️contract/🟦️.ts"; t=rd(cp)
t=once(t,'title:Readonly<{en:string;de:string}>;match:readonly Readonly<{kind:"prefix"|"exact";value:string}>[]}>;','title:Readonly<{en:string;de:string}>;match:readonly Readonly<{kind:"prefix"|"exact";value:string}>[];wait:LeaseWaitV1}>;')
t=once(t,'/** 🏷️ One lock class:','/** ⏱️ The declared wait of one lock class (`Wait` of the schema): the bound of a request without its own timeout, the ceiling of a caller\'s timeout, the heartbeat of a waiting request, how long a registered holder may stay silent, and the refusal hint in both languages. */\nexport type LeaseWaitV1=Readonly<{boundMilliseconds:number;maximumMilliseconds:number;heartbeatMilliseconds:number;silentMilliseconds:number;hint:Readonly<{en:string;de:string}>}>;\n/** 🏷️ One lock class:')
t=once(t,'classes:readonly LeaseClassV1[];','classes:readonly LeaseClassV1[];unclassifiedWait:LeaseWaitV1;')
print(re.findall(r'classes:[^;]*;',t))
wr(cp,t)
