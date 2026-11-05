# Canonical Puzzle2d Declared JSON Contract

The real declared JSON export previously routed native ToValue floats directly to JSON numbers; its paired import routed JSON through FromValue, which accepts numeric f64 values rather than word objects. The actual owned JSON formatter emits nonfinite floats as null. That does not implement the newly authored declared IO contract: every typed f64 must preserve all 64 bits, including signed zero, subnormal, infinity and NaN payloads.

This was a source-demonstrated contract mismatch, not an inferred native failure. The existing native semantic snapshot schema still correctly describes numeric state and native fixture DTOs; it was not already a bits-object wire schema. Those semantic schemas and numeric mutation corpora remain unchanged. Parent’s external caller/asset audit is json-asset-callers.md: scoped examples are deferred DSL, metadata JSON is not a snapshot, and native callback/WASM numeric DTO routes do not use the declared JSON leaves. No checked external declared-wire asset needed a rewrite.

## One Actual Owner and One Wire Schema

The implementation lives at canonical any/io/json. The existing inferences package lib directly mounts one internal puzzle2d_json module in its logical IO shell. Actual import/export leaves call this owner. The text-inverse composition already reexports those inferences JSON modules; no duplicate converter, new crate or compatibility wrapper was added.

The new IO json schema is the authoritative external wire contract. Export’s public JSON_SCHEMA constant points to that exact included asset. The portable law imports the same schema and its FloatWord definition; the duplicate floatWordSchema fixture definition was removed. Native semantic-state schemas remain separate.

The converter explicitly visits typed roles, never arbitrary objects containing bits:

| Row | Fields |
|---|---|
| Camera | x, y, zoom |
| Node | x, y, radius, width, height, scale |
| Node handle | angle, radius, scale |
| Edge | gap, shift, rise, rotation, turn, tilt, x, y |
| Target region | x, y, width, height |
| Catalogue node handle template | angle, t, radius |

Every actual row is visited; no specimen index is hardcoded. These type roles cover all 43 floating instances in the complete neutral fixture. All nonfloat members, integer values, identifiers, labels, ordering and metadata travel through the existing typed ToValue/FromValue boundary unchanged. Unknown semantic objects are not recursively interpreted.

Encoded fields are closed objects containing only bits with exactly 16 lowercase hexadecimal characters. Decoding rejects primitive numeric fallback, extra/duplicate word members, nontext bits, wrong length, uppercase and nonhex input. The global f64 codec, native snapshot representation and runtime dependencies are unchanged.

## Typed Presence and Custody

The wire input schema’s required sets were reviewed against every actual record’s value defaults and Option fields. Root requires schema; camera, node/edge/region rosters and meta retain their typed defaults on omission. Nested nondefault required members remain required. Defaulted node anchor/handles, edge geometry, region fields, compatibility flags, representation tags, template angle, catalogue record rosters and meta defaults remain available on omission. Present null is accepted only for actual optional fields, not required/defaulted scalar/record/roster fields.

The converter preserves missing/null members before final typed construction. Final FromValue remains the presence/shape/default authority; no parallel production presence validator or schema-based runtime dependency was introduced. Canonical output remains compared as a complete strict golden, including every original field.

DslValue and the intermediate Stdio JsonSnapshot are ordinary owned String/Vec trees, as established in json-acceptance.md. No fabricated Original, unrelated SQLite control or retirement loan was added to these synchronous APIs. The new native tests own genuine Puzzle snapshots in Option guards and retire them through the actual ArtifactSqliteSnapshotHost trait. The initially selected base trait was corrected before native execution; no default snapshot is constructed merely for cleanup.

## Authored and Independent Controls

Existing shared neutral fixture now contains ten word inputs, 45 presence/default/null cases spanning root and every record kind, and two unrelated semantic-object specimens. The complete original 43-pointer/all-nine-word positive native law remains unchanged.

Two actual inferences-owned native laws exercise public import for every word/presence case, check the resulting typed defaults/optional absence, and verify unrelated semantic objects remain untouched in both conversion directions. The fixture is a literal include_str input of that test owner, so its full bytes/digest are captured by the actual Rust Source reader; it is not omitted from admission.

Exact additional native selection (not executed by the root physical prefix):

```sh
repo cargo --store=shared --configuration=prototype -- test --manifest-path '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml' -p semio-s-artifact-puzzle-2d-inferences --lib declared_json_
```

This selects the two new declared_json_ laws. The root physical sqlite_snapshot_puzzle2d_ prefix separately covers the complete positive declared JSON law.

Read-only third-party Ajv, fast-json-patch and DataView evaluation passed all ten complete word specimens and all 45 presence cases against the actual canonical IO schema. The existing normal-capacity portable test was requested again as session62512, with unique generated/json-portable-2.log; its terminal result is pending at this write. Earlier portable49553 covered the old standalone word fixture schema and is not claimed as final full-schema proof.

## Final Actual Source Extents

| Owner | Cfg | Bytes | Inputs | Active Lines |
|---|---|---:|---:|---:|
| Inferences | ordinary | 160297 | 23 | 672 |
| Inferences | test | 208608 | 31 | 1194 |
| Text inverse | ordinary | 119550 | 11 | 1752 |
| Text inverse | test | 211047 | 23 | 2662 |

Every physical probe equals admitted input paths and bytes. The canonical schema and complete authored native fixture are included. The unchanged 262144-byte, 32-input and 4000-active-line limits remain intact. Full temporary inventory is generated/puzzle2d-json-final-extents.json.

Low’s final source review is pending. No native compiler/producer was launched by this lane: parent11101 owns the actual physical2d/JSON run. Its real result determines what source was captured; no stale-source explanation or runtime pass is assumed. New negative laws, four material IO laws, fresh WASM publication and final browser acceptance remain required.

## Final Independent Review and Portable Queue

Low independently reviewed the final strict word policy, every record required/default set, 45 presence vectors, actual host-trait snapshot cleanup, sole inferences module binding, canonical JSON_SCHEMA data and 31-input test closure. Review is green; no compiler or runtime result is inferred. The exact native inferences --lib declared_json_ selection remains two intended laws.

Authoritative portable session62512 remains live at normal outer capacity, two gigabytes requested, twelve requests ahead at its current 83-second observation. The same handle/log is retained without duplicate or restart. Root11101 independently owns the sole actual native gate. The read-only full-schema checks already passed all10 complete specimens and45 presence cases; they are not substituted for the queued existing portable law. A separate terminal-newline word probe was rejected by the actual Ajv schema as required, so no speculative schema alteration was introduced.

The authored fixture now identifies its canonical wire schema by the schema’s exact logical $id, and the existing portable law asserts that identity. This removes ambiguous relative-file-origin metadata; it adds no schema copy or native behavior. Final fresh Source measurement is inferences test208608/31/1194 (48 metadata bytes above the previous observation); all other rows and physical-probe equality remain unchanged. The TypeScript presence-case access is explicitly narrowed for strict imported-JSON typing. No additional native or portable request was launched.

The material-format board fixture identity was corrected to the original helper’s actual retained default schema, board.ports.directed.v1. The old helper discarded the extra input schema while copying nodes/edges/regions; direct construction must author the canonical retained identity explicitly. Both case and its schema now do so, and strict Ajv validation passed. Geometry, labels, four-law selection and code are unchanged. Final text-inverse test extent is211047/23/2662; all other final census rows and physical-probe equality remain unchanged.

## Actual Final Portable Result

Owned session62512 is authoritatively terminal exit0. The existing portable forty-three-field declared JSON law passed against the final canonical IO schema and its exact logical ID: one passed, zero failed, 63 unrelated tests filtered, 1364 assertions, 708 milliseconds total (353.22 milliseconds intended law). The request used normal Repo Bun capacity and was retained through its 725-second queue; no duplicate, light-mode runtime test or capacity bypass was used. Temporary actual evidence: generated/json-portable-2.log.

This result includes the canonical full-wire schema and all45 presence/default/null cases, rather than the earlier standalone word-only schema. It remains a portable independent contract proof, not native codec execution. Parent76254 owns the next actual physical2d/JSON run after the preceding shared history-index trait binding repair. This lane has no live or queued native/portable/producer process. The two inferences-owned negative laws, material IO four-law gate, fresh WASM and browser acceptance remain outstanding.
