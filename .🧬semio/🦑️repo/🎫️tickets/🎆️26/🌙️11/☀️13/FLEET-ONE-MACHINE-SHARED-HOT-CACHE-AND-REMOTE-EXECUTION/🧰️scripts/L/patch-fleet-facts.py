import json,re
B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/"
s=open(B+'🧬️schema/🔣️.json',encoding='utf8').read()
def rep(old,new):
    global s
    assert s.count(old)==1,old[:60]
    s=s.replace(old,new)
rep('''    { "$ref": "#/definitions/Facts" },
    { "$ref": "#/definitions/Credential" },
    { "$ref": "#/definitions/LocalCopy" }''','''    { "$ref": "#/definitions/Facts" },
    { "$ref": "#/definitions/LocalCopy" }''')
rep('''The store is the trust boundary of the fleet (whoever mounts it already changes the code every machine runs), so the credential travels in the same file; it is a dedicated fleet-scope credential, never an administrative secret, it rotates, and it is never logged.''','''The store is the trust boundary of the fleet (whoever mounts it already changes the code every machine runs), but the file carries no secret: every machine proves who it is with signed requests (see the identity schema), so a file that every member can read hands out no capability.''')
# remove Token, Identifier
i=s.index('    "Token": {'); j=s.index('    "Milliseconds": {'); s=s[:i]+s[j:]
rep('generation counts credential rotations and only grows.','generation counts content changes and only grows.')
rep('"required": ["version", "group", "generation", "publishedAtMs", "publisher", "cache", "credential"],','"required": ["version", "group", "generation", "publishedAtMs", "publisher", "cache", "auth"],')
i=s.index('        "credential": {\n          "type": "object",'); j=s.index('        "execution": {')
s=s[:i]+'''        "auth": {
          "description": "How clients prove who they are: signed requests of the origins (scheme semio-fleet-v1, see 🪪️identity). A client that does not know the scheme stays local.",
          "type": "object",
          "additionalProperties": false,
          "required": ["scheme", "roles"],
          "properties": {
            "scheme": { "const": "semio-fleet-v1" },
            "roles": { "const": ["cache-read", "cache-write", "submit", "worker", "admin"] }
          }
        },
'''+s[j:]
i=s.index('    "Credential": {'); j=s.index('    "LocalCopy": {'); s=s[:i]+s[j:]
open(B+'🧬️schema/🔣️.json','w',encoding='utf8').write(s)
json.loads(s)
t=open(B+'🟦️.ts',encoding='utf8').read()
def rept(old,new):
    global t
    assert t.count(old)==1,old[:60]
    t=t.replace(old,new)
rept('credential:Readonly<{scope:"fleet";id:string;token:string;rotatedAtMs:number}>;execution?','auth:Readonly<{scheme:"semio-fleet-v1";roles:readonly ["cache-read","cache-write","submit","worker","admin"]}>;execution?')
rept('the fleet-scope credential, and (reserved)','the authentication scheme, and (reserved)') if 'the fleet-scope credential, and (reserved)' in t else None
rept('definition:"Facts"|"Credential"|"LocalCopy"','definition:"Facts"|"LocalCopy"')
open(B+'🟦️.ts','w',encoding='utf8').write(t)
print("ok")
