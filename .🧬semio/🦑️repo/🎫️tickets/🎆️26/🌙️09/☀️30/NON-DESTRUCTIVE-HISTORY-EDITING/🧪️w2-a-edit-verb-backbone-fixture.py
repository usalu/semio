"""🏷️ Extends the language-neutral document-backbone batch fixture and its schema with the envelope's trailing flags
(bit 0 transaction, bit 1 verb): every expected envelope names its `verb`, the old flag-2 refusal becomes a flag-4
refusal, and verb-only, transaction-plus-verb and truncated-verb vectors join."""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal")
FIXTURE = ROOT / "🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json"
SCHEMA = ROOT / "🧬️schema/🧮️document-backbone-batch-v1/🔣️.json"
PREFIX = "01016d01640161000000017301aa016902bbcc030405"
TX = "1374782d303132333435363738396162636465660a6170702373656c656374"
VERB = "08" + "typeText".encode().hex()
LIMITS = '{ "maximumBytes": 262144, "maximumEnvelopes": 8192, "maximumDependenciesPerEnvelope": 8192, "maximumTotalDependencies": 8192, "maximumTargetSegmentsPerEnvelope": 8192, "maximumTotalTargetSegments": 8192, "maximumIdentifierBytes": 256, "maximumSchemaBytes": 256, "maximumPayloadBytes": 262144 }'
ENVELOPE = '{ "mutationId": "m", "documentId": "d", "actor": "a", "dependencies": [], "observed": null, "target": [], "diff": { "schema": "s", "payloadHex": "aa" }, "inverse": { "schema": "i", "payloadHex": "bbcc" }, "timestamp": { "actor": "3", "physicalMs": "4", "logical": "5" }, '


def case(case_id, raw, expect):
    return f'''    {{
      "id": "{case_id}",
      "rawHex": "{raw}",
      "limits": {LIMITS},
      "expect": {expect}
    }}'''


fixture = FIXTURE.read_text()
if '"verb":' not in fixture:
    fixture = fixture.replace('"transaction": null }', '"transaction": null, "verb": null }')
    fixture = fixture.replace('"transaction": { "id": "tx-0123456789abcdef", "tool": "app#select" } }', '"transaction": { "id": "tx-0123456789abcdef", "tool": "app#select" }, "verb": null }')
    old = case("transaction-flag-invalid", PREFIX + "02", '{ "outcome": "malformed", "reason": "transaction-flag" }')
    assert fixture.count(old) == 1
    added = ",\n".join([
        case("trailing-flags-invalid", PREFIX + "04", '{ "outcome": "malformed", "reason": "trailing-flags" }'),
        case("verb-canonical", PREFIX + "02" + VERB, '{ "outcome": "accepted", "reason": "canonical", "envelopes": [' + ENVELOPE + '"transaction": null, "verb": "typeText" }] }'),
        case("transaction-verb-canonical", PREFIX + "03" + TX + VERB, '{ "outcome": "accepted", "reason": "canonical", "envelopes": [' + ENVELOPE + '"transaction": { "id": "tx-0123456789abcdef", "tool": "app#select" }, "verb": "typeText" }] }'),
        case("verb-truncated", PREFIX + "02" + VERB[:8], '{ "outcome": "malformed", "reason": "truncated" }'),
    ])
    fixture = fixture.replace(old, added)
    FIXTURE.write_text(fixture)

schema = SCHEMA.read_text()
if '"verb"' not in schema:
    pairs = [
        ('"required": ["mutationId", "documentId", "actor", "dependencies", "observed", "target", "diff", "inverse", "timestamp", "transaction"],', '"required": ["mutationId", "documentId", "actor", "dependencies", "observed", "target", "diff", "inverse", "timestamp", "transaction", "verb"],'),
        ('''          "description": "The committed tool transaction that authored the operation: flag 0, or flag 1 then id and tool text, after the HLC."
        }''', '''          "description": "The committed tool transaction that authored the operation: trailing-flags bit 0, then id and tool text, after the HLC."
        },
        "verb": {
          "oneOf": [{ "type": "null" }, { "type": "string" }],
          "description": "The id of the action or command whose edit carried the operation, never display text: trailing-flags bit 1, then its text after the transaction."
        }'''),
    ]
    for old, new in pairs:
        assert schema.count(old) == 1, old[:80]
        schema = schema.replace(old, new)
    SCHEMA.write_text(schema)
print("ok")
