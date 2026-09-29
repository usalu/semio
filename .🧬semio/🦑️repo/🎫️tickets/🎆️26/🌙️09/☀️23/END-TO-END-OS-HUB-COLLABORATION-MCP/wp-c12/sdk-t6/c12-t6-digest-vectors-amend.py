"""🔗️ C12: one-shot, idempotent amendment of `c12-sdk-composition-patch.py` (coordinator 11:3x): the revision digest rule is
language-neutral. Adds `🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` (single-operation, grown-to-two and
grown-to-ten-thousand edits with their expected digests, derived by a THIRD implementation — `digest/make-edit-digest-chains.py`,
Python hashlib), its `EditDigestChains` schema, the TS oracle (node:crypto, inside `testCanonicalEditFixtures`) and the Rust
law (store `edit_digest` + incremental extension equals from scratch). Inventory: the digest is computed only by the Rust store
(and, for single-operation edits, the Rust one-item sealer with its TS oracle — both unchanged); the hub (Go), the TS worker and
the replication crate never compute or verify it, they carry revisions as opaque tokens. Usage: python3 <this>"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PATCH = os.path.join(HERE, "c12-sdk-composition-patch.py")
text = open(PATCH, encoding="utf-8").read()
if "CHAINS_FIXTURE = " in text:
    print("already amended")
    raise SystemExit(0)
fixture_text = open(os.path.join(HERE, "digest", "edit-digest-chains.json"), encoding="utf-8").read()
json.loads(fixture_text)
assert '"""' not in fixture_text and "\\" not in fixture_text


def swap(old, new):
    global text
    assert text.count(old) == 1, (old[:90], text.count(old))
    text = text.replace(old, new)


swap("""   row is re-printed in O(preview). SDK law `a_long_coalesced_gesture_previews_its_newest_operations_only`.
""", """   row is re-printed in O(preview). SDK law `a_long_coalesced_gesture_previews_its_newest_operations_only`.
   (e) The digest rule is language-neutral: `🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` (single-operation,
   grown-to-two, grown-to-ten-thousand → expected digests from a third implementation, Python hashlib), schema def
   `EditDigestChains`, replayed by the TS oracle (node:crypto) and the Rust store law. Only the Rust store computes this
   digest (the single-operation sealer path is unchanged); the hub, the TS worker and replication carry revisions opaquely.
""")
swap('''PREVIEW_EVAL_LAWS = "''', '''CANONICAL_FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json"
CANONICAL_SCHEMA = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json"
CANONICAL_TS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts"
CANONICAL_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs"
PREVIEW_EVAL_LAWS = "''')

NEW_HUNKS = r'''
CHAINS_FIXTURE = """''' + fixture_text + r'''"""
CHAINS_SCHEMA_OLD = """        "label": { "type": "string" },
        "group_id": { "type": "string" }
      }
    }
  }
}
"""
CHAINS_SCHEMA_NEW = """        "label": { "type": "string" },
        "group_id": { "type": "string" }
      }
    },
    "EditDigestChains": {
      "type": "object",
      "additionalProperties": false,
      "required": ["schema", "cases"],
      "properties": {
        "schema": { "const": "semio.store.edit-digest-chains.v1" },
        "cases": { "type": "array", "minItems": 3, "items": { "$ref": "#/$defs/EditDigestChainsCase" } }
      }
    },
    "EditDigestChainsCase": {
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "expectedDigest"],
      "properties": {
        "name": { "type": "string", "minLength": 1 },
        "edit": { "type": "object" },
        "header": { "type": "object" },
        "generatedOperations": { "type": "integer", "minimum": 2 },
        "expectedDigest": { "$ref": "#/$defs/CanonicalEditDigest" }
      },
      "oneOf": [{ "required": ["edit"] }, { "required": ["header", "generatedOperations"] }]
    }
  }
}
"""
CHAINS_TS_OLD = """  for (const [name, hostile] of hostiles) assert.equal(exported(name)(hostile), false, `${name} hostile is refused`);

}
//#endregion 🧵️CanonicalEditOracle"""
CHAINS_TS_NEW = """  for (const [name, hostile] of hostiles) assert.equal(exported(name)(hostile), false, `${name} hostile is refused`);

  const chains = read("./🧫️fixtures/🔗️edit-digest-chains.json");
  const validateChains = exported("EditDigestChains");
  assert(validateChains(chains), JSON.stringify(validateChains.errors));
  assert.equal(validateChains({ ...chains, cases: chains.cases.map((row: Record<string, unknown>) => ({ ...row, unexpected: true })) }), false, "EditDigestChains hostile is refused");
  const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const record = (domain: string, parts: Buffer[]) => {
    const hash = createHash("sha256").update("semio.artifact.cursor.v2").update(u64(Buffer.byteLength(domain))).update(domain);
    for (const part of parts) hash.update(u64(part.length)).update(part);
    return hash.digest();
  };
  const chain = (domain: string, items: unknown[]) => items.reduce<Buffer>((state, item) => record(domain, [state, Buffer.from(JSON.stringify(item))]), Buffer.alloc(32));
  const text = (edit: Record<string, unknown>, key: string) => [Buffer.from([key in edit ? 1 : 0]), Buffer.from(String(edit[key] ?? ""))];
  const editDigest = (edit: Record<string, unknown>) => {
    const [forwards, inverse, meta] = [edit.forwards as unknown[], edit.inverse as unknown[], (edit.mutationMeta ?? []) as unknown[]];
    if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(edit))]);
    const sequence = Buffer.alloc(4);
    sequence.writeInt32BE(edit.sequenceNumber as number);
    return record("edit-chained", [
      Buffer.from(String(edit.id)),
      ...text(edit, "actor"),
      ...text(edit, "description"),
      ...text(edit, "coalesceKey"),
      sequence,
      Buffer.from(String(edit.startedAt)),
      ...text(edit, "finishedAt"),
      u64(forwards.length), chain("edit-forward", forwards),
      u64(inverse.length), chain("edit-inverse", inverse),
      u64(meta.length), chain("edit-meta", meta),
    ]);
  };
  for (const row of chains.cases) {
    const edit = row.edit ?? {
      ...row.header,
      forwards: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index + 1 } })),
      inverse: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index } })),
    };
    assert.equal(editDigest(edit).toString("hex"), row.expectedDigest, `edit digest vector ${row.name}`);
  }
}
//#endregion 🧵️CanonicalEditOracle"""
CHAINS_LAW_OLD = """            SnapshotRetirementStep::Blocked => panic!("positive retirement grant blocked"),
        }
    }
    panic!("final authority strings did not retire");
}
"""
CHAINS_LAW_NEW = CHAINS_LAW_OLD + """
/// 🔗️ LAW (ticket 26/09/23 C12): the revision digest rule — a single-operation edit hashes its canonical JSON, an edit grown
/// past one operation hashes its header fields and one running chain per operation list — matches the language-neutral
/// vectors (derived by a third implementation, replayed by the TS oracle too), and extending the chains of a grown edit by the
/// operations an amend appended equals its from-scratch digest.
#[test]
fn edit_digest_chains_match_the_neutral_vectors_and_extend_incrementally() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️edit-digest-chains.json")).unwrap();
    let edit_of = |case: &serde_json::Value| match case.get("edit") {
        Some(edit) => Edit::<DslValue>::from_value(edit.clone().into()).unwrap(),
        None => {
            let mut header = case["header"].clone();
            let count = case["generatedOperations"].as_u64().unwrap();
            header["forwards"] = (0..count).map(|index| serde_json::json!({ "SetN": { "n": index + 1 } })).collect();
            header["inverse"] = (0..count).map(|index| serde_json::json!({ "SetN": { "n": index } })).collect();
            Edit::<DslValue>::from_value(header.into()).unwrap()
        }
    };
    for case in fixture["cases"].as_array().unwrap() {
        let digest = super::super::CursorRevisionAccumulator::edit_digest(&edit_of(case));
        assert_eq!(semio_framework_hash::hex_lower(&digest), case["expectedDigest"].as_str().unwrap(), "edit digest vector {}", case["name"]);
    }
    let mut grown = edit_of(&fixture["cases"][1]);
    let (_, chains) = super::super::CursorRevisionAccumulator::edit_digest_extending(&grown, None);
    grown.forwards.push(serde_json::json!({ "SetN": { "n": 3 } }).into());
    grown.inverse.push(serde_json::json!({ "SetN": { "n": 2 } }).into());
    let (extended, _) = super::super::CursorRevisionAccumulator::edit_digest_extending(&grown, chains);
    assert_eq!(extended, super::super::CursorRevisionAccumulator::edit_digest(&grown), "an amend's incremental digest equals the from-scratch digest");
}
"""
'''
swap('\nDAG_OLD = ', NEW_HUNKS + '\nDAG_OLD = ')
swap('''    PREVIEW_EVAL_LAWS: [''', '''    CANONICAL_SCHEMA: [(CHAINS_SCHEMA_OLD, CHAINS_SCHEMA_NEW)],
    CANONICAL_TS: [(CHAINS_TS_OLD, CHAINS_TS_NEW)],
    CANONICAL_LAWS: [(CHAINS_LAW_OLD, CHAINS_LAW_NEW)],
    PREVIEW_EVAL_LAWS: [''')

swap('''problems, writes = [], {}
for relative, hunks in HUNKS.items():
    path = os.path.join(REPO, relative)
    text = open(path, encoding="utf-8").read()''', '''problems, writes = [], {}
CREATES = {CANONICAL_FIXTURE: CHAINS_FIXTURE}
for relative, content in CREATES.items():
    path = os.path.join(REPO, relative)
    if os.path.exists(path) and open(path, encoding="utf-8").read() != content:
        problems.append((relative.rsplit("/", 1)[-1], "exists with other content", 0))
    elif not os.path.exists(path):
        writes[path] = content
    print(relative.rsplit("/", 1)[-1], ["present" if os.path.exists(path) else "create"])
for relative, hunks in HUNKS.items():
    path = os.path.join(REPO, relative)
    text = open(path, encoding="utf-8").read()''')

open(PATCH, "w", encoding="utf-8").write(text)
print("amended")
