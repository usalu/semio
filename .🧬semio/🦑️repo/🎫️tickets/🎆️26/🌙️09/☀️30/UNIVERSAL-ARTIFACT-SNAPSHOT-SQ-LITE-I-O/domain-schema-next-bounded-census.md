# Current Domain Schema And Next Bounded Census

**Correction from deeper audit:** glTF and DWG both concatenate existing split SQL fragments and publish explicit SQLite codec hooks. The root-adjacency comparison below is not a missing-semantic-schema census. See `gltf-dwg-split-semantic-schema-audit.md` for actual schema/mount evidence and the original ownership frontier.

Read-only current census, 2026-10-10. No tests/builds. Compared every tracked `✏️s/🔌️plugins/**/🧬️schema/📸️snapshot/🦀️.rs` to its exact adjacent `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql` using file existence. This is a bounded source census, not universal runtime support or every macro-generated snapshot census.

## Missing Schema Result

There are no two genuinely small plugin snapshots satisfying all requested properties of missing handcrafted SQL, missing actual mount, and existing independent semantic SQL tests. Only three schema-file paths lack adjacent SQL: DWG AC1018 (3 lines, merely reexports AC1024 snapshot), glTF2 any (1279 lines), and DWG AC1024 any (5219 lines). AC1018 is not a distinct small persisted implementation. Do not misreport existing receiving-method gaps as missing handcrafted schemas.

Actual missing implementations:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🦀️.rs:1187`: GltfSnapshot fields exactly `schema:String`, `document:GltfDocument`, `buffers:Vec<Vec<u8>>`, `source_form:GltfSourceForm`. This four-field root contains a large semantic document tree. Artifact root `🦀️.rs:115` mounts `.document_codec_bare::<GltfSnapshot,GltfMutation>` for s.stdio.gltf/2.0/*. No adjacent SQLite schema found. Do not serialize document to opaque JSON to make this appear bounded.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:5047`: DwgSnapshot fields exactly `schema`, `version`, `maintenance_version`, `codepage`, `drawing`, `header`, `classes`, `dependencies`, `summary`, `application`, `template`, `auxiliary_header`, `revision_history`, `preview`, `application_history`. Artifact root `🦀️.rs:115–116` mounts bare document codecs for ac1018 and ac1024. Their nested domains need full field-by-field schema census; these are not a small two-artifact next step.

Framework exact snapshot-schema path comparison also found a 23-line plugin test fixture and 176-line directed Board schema without adjacent SQL. The fixture is excluded; Board nested user_data DslValue and shared child topology need separate semantic review. Neither is evidence of a small genuine plugin artifact missing schema.

## Smallest Useful Bounded Production Seams After CSV/TSV

The bounded recommendation is **Playground then Binary**, with explicit acknowledgment that both already have handcrafted SQL and independent tests; their remaining seam is original ownership/actual declaration qualification, not writing first SQL.

Playground base: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any`.

- Root persisted field is only `schema:String` (`🧬️schema/📸️snapshot/🦀️.rs:13`). No opaque JSON or nested dynamic owner.
- Existing SQL `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql` is `playground_document(id INTEGER PRIMARY KEY,schema TEXT NOT NULL)`.
- Actual artifact root declaration `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🦀️.rs:87` uses document_codec_bare with PLAYGROUND_DIALECT. Verify the resulting provider publication before claiming installed support.
- Existing `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` imports Bun SQLite/Ajv; tests query handwritten SQL and independently edit/renumber schema marker, literal Unicode/native cases and closed schema. Fixture and schema are explicitly imported there. Existing census lists original P/R and VD/VE missing. One-field ownership is the smallest meaningful original-grant/partial-string/installed-provider exemplar.

Binary base: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any`.

- Root fields exactly `schema:String`, `bytes:Vec<u8>` (`🧬️schema/📸️snapshot/🦀️.rs:18`). Intrinsic bytes are the actual domain, not an opaque carrier hiding unrelated semantic fields.
- Existing SQL `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql` owns binary_document id1/schema and binary_byte(id,document_id FK,ordinal>=0,value0..255). This is handcrafted byte-indexed relational meaning.
- Root artifact declaration `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️.rs:82` mounts document_codec_bare BinarySnapshot/BinaryMutation for s.stdio.binary/raw/*.
- SQLite tests `🧪️tests/🟦️.ts:2–18` use Bun SQLite deserialize/edit/serialize against neutral fixture, invalid byte and noncontiguous ordinal checks. Exact source imports fixture `../🧫️fixtures/🔣️.json` and producer implementation `../🟦️.ts`.
- Fresh Rust SQLite impl at `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:11` still declares native decode using bare NativeDecodeControl, unlike current NativeSnapshotDecodeOwner trait contract. Correct original owner methods and all actual receiving/native callers together, then mount and exercise installed provider, avoiding a compatibility overload.

For each bounded candidate: preserve authentic Option until paid admission; implement both receiving directions and both subset direction validators; derive/verify actual RetireOwned; supply explicit independent neutral body and close authority; verify zero short-grant effects and actual UTF8/octet prefix retirement conservation; query edited physical SQL independently through Bun SQLite; exercise actual document declaration factory and public route. Existing semantic tests provide oracle assets, but do not establish current native compile/runtime success. No runtime result is claimed.
