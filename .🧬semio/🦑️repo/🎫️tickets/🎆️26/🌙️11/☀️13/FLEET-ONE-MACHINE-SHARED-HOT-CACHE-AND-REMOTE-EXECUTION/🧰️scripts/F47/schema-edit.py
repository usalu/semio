import os,tempfile
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🧬️schema/🔣️.json"
s=open(P,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:70],s.count(old))
    s=s.replace(old,new)
rep('''"description": "The identity of the cache server that published these facts: its data directory, its listening port and its host name. A publisher with another identity never replaces recent facts and never lowers the generation, so a scratch or test server is detectable and harmless.",
          "type": "object",
          "additionalProperties": false,
          "required": ["directory", "port", "host"],
          "properties": {
            "directory": { "type": "string", "minLength": 1 },
            "port": { "type": "integer", "minimum": 1, "maximum": 65535 },
            "host": { "type": "string", "minLength": 1, "maxLength": 253 }
          }''','''"description": "The identity of the cache server that published these facts: its origin id (the machine is reached by this id, never by an address or an operating system host name: its addresses are in <store>/v1/origins/<origin>/addresses.json and it advertises itself as a DNS-SD service of that name), its data directory and its listening port, which is the cache port. A publisher with another identity never replaces recent facts and never lowers the generation, so a scratch or test server is detectable and harmless.",
          "type": "object",
          "additionalProperties": false,
          "required": ["origin", "directory", "port"],
          "properties": {
            "origin": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,63}$" },
            "directory": { "type": "string", "minLength": 1 },
            "port": { "type": "integer", "minimum": 1, "maximum": 65535 }
          }''')
rep('"required": ["endpoints", "instance", "access", "protocols", "protocolVersion"],','"required": ["instance", "access", "protocols", "protocolVersion"],')
a=s.index('            "endpoints": {');b=s.index('            "instance": { "$ref"')
s=s[:a]+s[b:]
rep('''"description": "The remote execution scheduler of the group (reserved; absent until published).",
          "type": "object",
          "additionalProperties": false,
          "required": ["endpoint", "protocolVersion"],
          "properties": {
            "endpoint": { "$ref": "#/definitions/Endpoint" },''','''"description": "The remote execution scheduler of the group (absent until published): the origin that runs it and the port of its Remote Execution API; the addresses are those of the origin.",
          "type": "object",
          "additionalProperties": false,
          "required": ["origin", "port", "protocolVersion"],
          "properties": {
            "origin": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,63}$" },
            "port": { "type": "integer", "minimum": 1, "maximum": 65535 },''')
rep('"required": ["version", "fetchedAtMs", "checkedAtMs", "selectedEndpoint", "facts"],','"required": ["version", "fetchedAtMs", "checkedAtMs", "selectedEndpoint", "candidates", "selectedExecution", "facts"],')
rep('''"selectedEndpoint": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/Endpoint" }] },''','''"selectedEndpoint": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/Endpoint" }] },
        "candidates": { "description": "The endpoints of the cache server that its published addresses give at the last refresh; derived on this machine, never published.", "type": "array", "maxItems": 40, "items": { "$ref": "#/definitions/Endpoint" } },
        "selectedExecution": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/Endpoint" }] },''')
t=tempfile.NamedTemporaryFile("w",dir=os.path.dirname(P),delete=False,encoding="utf8",suffix=".tmp");t.write(s);t.close();os.rename(t.name,P);print("ok")
