import sys,json,glob
base=sys.argv[1]
p=glob.glob(base+"/*replica/\U0001F9EC️schema/\U0001F523️.json")[0]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('"origins", "retired", "latency"],','"origins", "retired", "latency", "health"],')
old='''        "git": {
          "type": "object",
          "additionalProperties": false,
          "required": ["shared", "reason", "indexTree"],'''
new='''        "health": {
          "description": "Null, or what the running daemon wrote to .cache/f/health.json: the state of its store (`ok`, `stalled` when a store call did not return within its deadline, `unreachable`), the refused new code and the recurring failure kinds with their lifetime count and last time.",
          "oneOf": [
            { "type": "null" },
            {
              "type": "object",
              "additionalProperties": false,
              "required": ["wallMs", "store", "storeSinceMs", "code", "pending", "failures"],
              "properties": {
                "wallMs": { "$ref": "#/definitions/Count" },
                "store": { "enum": ["ok", "stalled", "unreachable"] },
                "storeSinceMs": { "$ref": "#/definitions/Count" },
                "code": { "$ref": "#/definitions/Sha256" },
                "pending": {
                  "oneOf": [
                    { "type": "null" },
                    { "type": "object", "additionalProperties": false, "required": ["code", "ok", "reason"], "properties": { "code": { "$ref": "#/definitions/Sha256" }, "ok": { "const": false }, "reason": { "type": "string", "maxLength": 512 } } }
                  ]
                },
                "failures": {
                  "type": "array",
                  "maxItems": 8,
                  "items": { "type": "object", "additionalProperties": false, "required": ["kind", "count", "lastMs"], "properties": { "kind": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,39}$" }, "count": { "$ref": "#/definitions/Count" }, "lastMs": { "$ref": "#/definitions/Count" } } }
                }
              }
            }
          ]
        },
'''
rep(old,new+old)
open(p,"w",encoding="utf8").write(s)
json.loads(s)
# texts
t=glob.glob(base+"/*replica/\U0001F4D6️texts/\U0001F523️.json")[0]
s=open(t,encoding="utf8").read()
new={
"selfcheck.ok":("Self-check passed: {modules} modules load, {schemas} schemas compile, {texts} text catalogues are complete in all languages.","Selbstprüfung bestanden: {modules} Module laden, {schemas} Schemas übersetzen sich, {texts} Textkataloge sind in allen Sprachen vollständig."),
"selfcheck.failed":("Self-check failed: {error}","Selbstprüfung fehlgeschlagen: {error}"),
"event.restart-refused":("The new code {digest} does not start ({reason}); the daemon keeps running the old code until the code on disk changes again.","Der neue Code {digest} startet nicht ({reason}); der Daemon läuft mit dem alten Code weiter, bis sich der Code auf der Platte erneut ändert."),
"status.health.store":("The store has not answered since {seconds} s ({state}); local work continues and the daemon resumes by itself when the store answers again.","Der Speicher antwortet seit {seconds} s nicht ({state}); lokale Arbeit läuft weiter und der Daemon macht von selbst weiter, sobald der Speicher wieder antwortet."),
"status.health.pending":("New code {digest} was refused: {reason}","Neuer Code {digest} wurde abgelehnt: {reason}"),
"status.health.failure":("Recurring failures of kind {kind}: {count} times, last {seconds} s ago.","Wiederkehrende Fehler der Art {kind}: {count} Mal, zuletzt vor {seconds} s."),
"error.origin.taken":("The origin {origin} already belongs to another machine (its home.json carries another key). Use an unused origin id, or retire the origin and join with a new one.","Der Ursprung {origin} gehört bereits einer anderen Maschine (seine home.json trägt einen anderen Schlüssel). Verwende eine unbenutzte Ursprungs-Kennung oder lege den Ursprung still und tritt mit einer neuen bei."),
"error.retire.unreadable":("The attestation of origin {origin} exists but cannot be read, so its state is unknown. Pass --force to retire it anyway.","Die Bescheinigung des Ursprungs {origin} existiert, ist aber nicht lesbar, sein Zustand ist daher unbekannt. Gib --force an, um ihn trotzdem stillzulegen."),
}
block="".join('    %s: {\n      "en": %s,\n      "de": %s\n    },\n'%(json.dumps(k),json.dumps(v[0],ensure_ascii=False),json.dumps(v[1],ensure_ascii=False)) for k,v in new.items())
anchor='  "messages": {\n'
assert s.count(anchor)==1
s=s.replace(anchor,anchor+block)
import re
m=re.search(r'"error.syntax": \{\s*"en": "([^"]*)",\s*"de": "([^"]*)"',s)
print(m.group(1))
en=m.group(1);de=m.group(2)
en2=en.replace("retire|touch.","retire|touch|self-check.") if "retire|touch." in en else en
assert en2!=en, en
s=s.replace(en,en2).replace(de,de.replace("retire|touch.","retire|touch|self-check."))
open(t,"w",encoding="utf8").write(s)
json.loads(s)
