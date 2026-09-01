# 🚪️ The last 14 are EXPORT defects, not oracle gaps — and one of them is now fixed

`externalOracleCoverage` and `oracleEvidenceCoverage` now BOTH read **600/614 (97.72%)**. The two
dimensions coinciding is itself the result worth stating: there is no longer a single mutation that has
a qualifying oracle registered but nothing to run it against.

The remaining 14 are the same set in both dimensions — 9 `mathematical`, 4 `sequence`, 1 `jpg`.

## They are not blocked on oracles or on fixtures

Splitting the gap by dimension made the cause visible. Four `sequence` kinds (`create-step`,
`delete-step`, `duplicate-step`, `edit-step-params`) were missing EVIDENCE but not an oracle — they
already had `csv-rfc4180-reader`, and only lacked fixtures. Those four are now closed with an authored
`(before.csv, after.csv)` pair each, written by the `csv` crate in a standalone `[workspace]` engine
that mirrors `SequenceIntoCsv`'s exact row shape (`has_header: false`, `[id, kind, JSON-encoded
params]`, params quoted).

The other 13 are blocked one layer further back: **no carrier records the state they change**, which is
a defect in our EXPORT, not a missing third-party library.

## `sequence` — a serializer that declared `Exact` and dropped the whole document

`SequenceIntoJson` declared `const FIDELITY: IoFidelity = IoFidelity::Exact` and then called
`serde_json::to_value(from)` on the snapshot. But `SequenceSnapshot` is
`{schema, content: ArtifactChild<..>}`, and `ArtifactChild` keeps its materialized scene in a
`#[serde(skip)]` `local_owner` (`🏪️store/🦀️component.rs:2567`). So the "exact" JSON export emitted:

```json
{ "schema": "…", "content": { "childId": "…", "target": { … } } }
```

— no steps, no edges. The sibling `SequenceIntoCsv` in the same tree already went through
`to_fixture()`; this one did not.

**Fixed**: it now serializes `from.to_fixture()`, and `SequenceFixture { schema, steps, edges }` carries
every field the four remaining kinds touch — `edges` (`connect-steps`, `disconnect-steps`), `x`/`y`
(`move-step`) and `collapsed` (`change-step-collapsed`).

**The matching oracle was deliberately NOT registered.** The fix cannot be executed while
`semio-s-plugin-stdio` does not build, and an oracle registered against an export nobody has run is
precisely the fiction the sibling `mathematical` catalog already refuses to commit ("a reader cannot
witness what no carrier records, and registering one against them would be a fiction"). Those four stay
counted as missing until the export can actually be run.

## `mathematical` — the same false `Exact`, but not a one-line fix

`MathematicalIntoJson` has the identical shape: `IoFidelity::Exact` with `serde_json::to_value(from)`
over a snapshot whose `notation`, `results` and `computed` are all `ArtifactChild` handles. Same
consequence, same false declaration.

It is NOT the same fix. `sequence` had a `to_fixture()` materialization to serialize; `mathematical`
has none, so defining the projection — which of the three children reach the carrier, and in what
shape — is a schema decision for that artifact, not a mechanical substitution. Reported rather than
guessed.

Its nine kinds (`connect-nodes`, `disconnect-nodes`, `change-graph-directed`, `update-graph-algorithm`,
`replace-graph`, `insert-point`, `move-point`, `remove-point`, `replace-points`) stay blocked, exactly
as that subset's own oracle rationale already records.

## `jpg::remove-huffman-table`

Unchanged and unrelated to the above: no writer emits a JPEG whose DHT list can be varied while the
result stays decodable.

## Corpora added this pass

| subset | kinds | engine | third-party writer |
|---|---|---|---|
| `🧿️semio@v1/✳️drawing` | 17 | `🏭️generator/🦀️svg-engine` | quick-xml 0.37.5 |
| `🔣️json@rfc8259/✳️base` | 5 | `🏭️generator/🦀️serde-json-engine` | serde_json 1 |
| `🎬️sequence@1/✳️any` | 4 | `🏭️generator/🦀️csv-engine` | csv 1.4.0 |

All three are standalone `[workspace]` crates with exactly one dependency, so they build while
`semio-s-plugin-stdio` does not. Every engine `assert_ne!`s each pair: a fixture whose two halves are
equal proves nothing.
