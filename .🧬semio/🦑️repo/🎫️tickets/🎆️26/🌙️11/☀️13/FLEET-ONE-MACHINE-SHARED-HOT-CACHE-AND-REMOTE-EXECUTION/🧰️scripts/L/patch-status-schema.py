p="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json"
s=open(p,encoding='utf8').read()
old=s[s.index('        "fleet": {\n          "description": "The fleet credential generation'):s.index('        "claims": {\n          "description": "The fleet single-flight')]
new='''        "fleet": {
          "description": "The fleet identities this server trusts: how many origins of the group store are usable, retired, in conflict with their pinned key, invalid or not yet registered, and how often requests were refused, by reason (never a key, a name or a secret); null while the server runs without fleet identities.",
          "oneOf": [
            { "const": null },
            {
              "type": "object",
              "additionalProperties": false,
              "required": ["origins", "denied"],
              "properties": {
                "origins": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },
                "denied": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } }
              }
            }
          ]
        },
        "divergences": {
          "description": "Unit keys that two origins stored differently: the count and the latest rows. The first entry stays, the later write was refused; both digests are evidence of nondeterminism or an attack.",
          "type": "object",
          "additionalProperties": false,
          "required": ["count", "recent"],
          "properties": {
            "count": { "type": "integer", "minimum": 0 },
            "recent": {
              "type": "array",
              "maxItems": 20,
              "items": {
                "type": "object",
                "additionalProperties": false,
                "required": ["namespace", "kind", "key", "keptOrigin", "keptSha256", "refusedOrigin", "refusedSha256", "atMs"],
                "properties": {
                  "namespace": { "type": "string" },
                  "kind": { "enum": ["ac", "cas", "nx"] },
                  "key": { "type": "string" },
                  "keptOrigin": { "type": "string" },
                  "keptSha256": { "type": "string", "pattern": "^[0-9a-f]{64}$" },
                  "refusedOrigin": { "type": "string" },
                  "refusedSha256": { "type": "string", "pattern": "^[0-9a-f]{64}$" },
                  "atMs": { "type": "integer", "minimum": 0 }
                }
              }
            }
          }
        },
'''
assert s.count(old)==1
s=s.replace(old,new)
open(p,'w',encoding='utf8').write(s)
print("schema ok")
