#!/usr/bin/env python3
"""F68: the fleet-fetch setup test loads its JSON fixture through the typed loader against a new `Fixture` definition of the setup schema (the rows are no longer a union of `x?: undefined` shapes)."""
import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from f68lib import *
D=LIB+"/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup"
t=D+"/🧪️tests/🟦️.ts";sc=D+"/🧬️schema/🔣️.json"
defn='''    "Host": {
      "type": "object",
      "additionalProperties": false,
      "required": ["platform", "arch"],
      "properties": {
        "platform": { "type": "string" },
        "arch": { "type": "string" },
        "libc": { "enum": ["gnu", "musl"] }
      }
    },
    "Fixture": {
      "type": "object",
      "additionalProperties": false,
      "required": ["classes", "keys"],
      "properties": {
        "classes": {
          "type": "array",
          "items": {
            "type": "object",
            "additionalProperties": false,
            "required": ["name", "kind", "host", "tag"],
            "properties": {
              "name": { "type": "string" },
              "kind": { "$ref": "#/definitions/Kind" },
              "host": { "$ref": "#/definitions/Host" },
              "tag": { "anyOf": [{ "type": "string" }, { "type": "null" }] }
            }
          }
        },
        "keys": {
          "type": "array",
          "items": {
            "type": "object",
            "additionalProperties": false,
            "required": ["task", "inputs", "kind", "roots", "host"],
            "properties": {
              "task": { "type": "string" },
              "inputs": { "type": "object", "additionalProperties": { "type": "string" } },
              "kind": { "$ref": "#/definitions/Kind" },
              "roots": { "type": "array", "items": { "type": "string" } },
              "host": { "$ref": "#/definitions/Host" }
            }
          }
        }
      }
    },
'''
got,held=take({"caching-bootstrap"},8)
print("held",held)
try:
    if not held:
        s=open(sc,encoding="utf8").read()
        if '"Fixture"' not in s:print(edit(sc,'    "Kind": { "enum": ["neutral", "posix", "host"] },\n','    "Kind": { "enum": ["neutral", "posix", "host"] },\n'+defn))
        s=open(t,encoding="utf8").read()
        if "typedFixtureV1" not in s:
            ok=[edit(t,'import fixture from "./🔣️.json" with {type:"json"};\n','import fixtureJson from "./🔣️.json" with {type:"json"};\nimport {typedFixtureV1} from "../../../../../🧪️testing/🧾️typed-fixture/🟦️.ts";\ntype FixtureHostV1=SetupHostV1;\ntype SetupFixtureV1=Readonly<{classes:readonly Readonly<{name:string;kind:SetupKindV1;host:FixtureHostV1;tag:string|null}>[];keys:readonly Readonly<{task:string;inputs:Readonly<Record<string,string>>;kind:SetupKindV1;roots:readonly string[];host:FixtureHostV1}>[]}>;\nconst fixture=typedFixtureV1<SetupFixtureV1>(fixtureJson,{$ref:"#/definitions/Fixture"},{root:schema,name:"fleet-fetch setup fixture"});\n')]
            print(ok)
finally:release(got)
