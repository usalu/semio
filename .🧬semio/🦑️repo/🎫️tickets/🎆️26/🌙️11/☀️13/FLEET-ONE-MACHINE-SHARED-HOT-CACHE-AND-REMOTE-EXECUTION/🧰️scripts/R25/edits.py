import os
HERE=os.path.dirname(os.path.abspath(__file__))
PUB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"
FUND=PUB+"/⚙️setup/💰️funding/🟦️.ts"
SCHEMA=PUB+"/⚙️setup/🧬️schema.json"
TEXTS="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🔣️.json"
CODE=open(HERE+"/code-window.ts",encoding="utf8").read()
SCHEMA_NEW='''    "FundingPublicationWindowProtocol": {
      "const": {
        "version": 1,
        "maximumExclusiveMilliseconds": 20000,
        "exclusiveWaitMilliseconds": 120000,
        "retireGraceMilliseconds": 60000,
        "maximumSnapshotAttempts": 8,
        "startToleranceMilliseconds": 5000,
        "stagingDirectory": "staging",
        "generationDirectory": "generations",
        "pinDirectory": "pins",
        "retiringDirectory": "retiring",
        "pointerName": "current.json",
        "completeName": "complete.json"
      }
    },
    "FundingPointer": {
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "sequence", "scopes", "superseded"],
      "properties": {
        "version": { "const": 1 },
        "sequence": { "type": "integer", "minimum": 1, "maximum": 9007199254740991 },
        "scopes": {
          "type": "object",
          "propertyNames": { "pattern": "^[a-z0-9][a-z0-9._-]{0,63}$" },
          "additionalProperties": { "type": "string", "pattern": "^[a-f0-9]{20}$" }
        },
        "superseded": {
          "type": "object",
          "propertyNames": { "pattern": "^[a-f0-9]{20}$" },
          "additionalProperties": { "type": "integer", "minimum": 0, "maximum": 9007199254740991 }
        }
      }
    },
    "FundingStagingOwner": {
      "type": "object",
      "additionalProperties": false,
      "required": ["pid", "startedAtMs"],
      "properties": {
        "pid": { "type": "integer", "minimum": 1 },
        "startedAtMs": { "type": "integer", "minimum": 0, "maximum": 9007199254740991 }
      }
    },
'''
TEXT_OLD='''    "dependency-store.funding-moving": {'''
TEXT_NEW='''    "dependency-store.exclusive-window": {
      "en": "Exclusive access to the dependency store for {ms} ms (limit {limit} ms): published {what}",
      "de": "Exklusiver Zugriff auf den Abhängigkeitsspeicher für {ms} ms (Grenze {limit} ms): veröffentlicht {what}"
    },
    "dependency-store.staging-recovered": {
      "en": "Removed {count} unfinished staging leftovers of ended processes from the dependency store",
      "de": "{count} unfertige Reste beendeter Prozesse wurden aus dem Abhängigkeitsspeicher entfernt"
    },
    "dependency-store.funding-moving": {'''
E=[
 (FUND,'import{readdirSync,readFileSync,rmSync}from"node:fs";','import{closeSync,existsSync,fsyncSync,mkdirSync,openSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";',1),
 (FUND,'/** 🌊️ How a funding is re-run when the shared tree moved under it',CODE+'/** 🌊️ How a funding is re-run when the shared tree moved under it',1),
 (SCHEMA,'    "PreparationRequest": {',SCHEMA_NEW+'    "PreparationRequest": {',1),
 (TEXTS,TEXT_OLD,TEXT_NEW,1),
]

REHEARSAL_COPIES=["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🟦️.ts"]
