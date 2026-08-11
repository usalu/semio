# F1 — CSV (rfc4180) Schema Rework Report

Artifact: `📊️csv` / standard `🔖️rfc4180`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/`.

## Pre-flight check

Per the ownership note, re-checked `git status`/`git diff` on the artifact's files before
starting: **clean, no pending edits.** W0's "mid-edit, −77/+15" observation on the snapshot
file had already settled by another session before this agent started. Proceeded normally.

## What changed

### Snapshot (`🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`)

Old model: `has_header: bool`, `headers: Vec<String>`, `rows: Vec<Vec<String>>` — the header
row was physically split out of the row list at decode time and re-spliced at encode time,
and no field ever recorded whether the SOURCE quoted it (re-encoding always used the
structural minimum, silently losing real information).

New model, per the completeness target:

```rust
pub struct CsvField { pub value: String, pub quoted: bool }
pub struct CsvRecord { pub fields: Vec<CsvField> }
pub struct CsvSnapshot { pub schema: String, pub has_header: bool, pub records: Vec<CsvRecord> }
```

`has_header` is now pure metadata about whether `records[0]` should be read as a header —
RFC 4180 draws no structural distinction between a header record and a data record on the
wire, so decode/encode never drops, relocates, or re-derives the first record; `has_header`
just travels alongside the same uniform row list. `CsvField.quoted` is real per-field
provenance: the tokenizer in `⚙️engine` now tracks whether each field was wrapped in `"..."`
in the source, and the writer re-quotes whenever `quoted || structurally-required`
(comma/quote/newline present) — so a field that didn't NEED quoting but WAS quoted in the
source round-trips quoted, not silently normalized away. Covered by a new engine test
(`quoted_flag_round_trips_even_when_not_structurally_required`).

`CsvField`/`CsvRecord` are plain value/collection structs (no `ArtifactSchema` derive) —
matches the established repo convention of only deriving `ArtifactSchema` on the top-level
snapshot/diff struct, confirmed against `GifFrame`/`GifColorTable` in the gif 89a snapshot
file (nested entity types there carry no `#[artifact_schema]`/`#[state]` either).

### Diff (`🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`)

Fully rewritten from the old `CsvDiff { snapshot: Option<CsvSnapshot> }` full-replace slot to
a handcrafted sparse diff:

```rust
pub struct CsvDiff { pub has_header: Option<bool>, pub records: Option<CsvRecordsDiff> }
pub struct CsvRecordsDiff { pub removed: Vec<usize>, pub modified: Vec<CsvRecordModified>, pub added: Vec<CsvRecordAdded> }
pub struct CsvRecordModified { pub index: usize, pub diff: CsvRecordDiff }
pub struct CsvRecordAdded { pub index: usize, pub record: CsvRecord }
pub struct CsvRecordDiff { pub fields: Option<Vec<Option<CsvFieldDiff>>> }   // positional, see "Design choice" below
pub struct CsvFieldDiff { pub value: Option<String>, pub quoted: Option<bool> }
```

`impl protocol::MutationDiff<CsvSnapshot> for CsvDiff { apply, absorb }` and
`impl protocol::command::DiffAlgebra<CsvSnapshot> for CsvDiff { inverse, between, is_empty }`
(see "Deviation: `DiffAlgebra` import path" below for why `protocol::command::DiffAlgebra`
rather than `protocol::DiffAlgebra`). `inverse` is implemented via
`Self::between(&self.apply(base), base)`, which is legitimate given `between` is itself
handcrafted and `between_roundtrip_law` holds — it directly satisfies the inverse law
(`d.inverse(base).apply(&d.apply(base)) == base` reduces to `between(applied,base).apply(applied) == base`,
exactly the between-roundtrip law applied at `applied`).

**Design choice — record field-count changes** (documented per the brief's instruction to
pick a shape and justify it): the mutation vocabulary has `SetField{record_index,
field_index, value, quoted}` but deliberately NO `InsertField`/`RemoveField` mutation — a
record's field count only ever changes via a whole-record add/remove at the `records`
collection level. So `CsvRecordDiff` uses the simpler positional
`fields: Option<Vec<Option<CsvFieldDiff>>>` shape (not another nested removed/modified/added
triple) — position `i` patches `base.fields[i]` in place, `None` means unchanged, and the
vector only needs to be as long as the highest patched index. `CsvDiff::between` still
handles the general case honestly: if two records at the same index have DIFFERENT field
counts (possible on arbitrary/hand-built snapshots even though no mutation alone can cause
it), the record is expressed as a same-index remove+add pair at the `records` level instead
of forcing it through the positional patch shape — verified by a `between_roundtrip_law`
synthetic case with mismatched field counts.

**Absorb** (`absorb_records`): structural, total, base-free per the recipe. Builds a
base→mid index transport by literally simulating the SAME removed(descending)/added
(ascending, clamped) sequence `CsvDiff::apply` performs, over a virtual index array bounded
tightly by whichever indices the two diffs being composed actually reference (not by real
snapshot length, since absorb is base-free by contract). A ψ transport (mid→after) does the
same for surviving d1-added items that outlive d2. See "Non-obvious bug found & fixed during
verification" below — the naive tight-bound-from-d1-alone version had a real correctness gap
that the scratch-crate law tests caught.

### Mutations (`🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`)

Grew from `{NoMutation, SetSnapshot}` to the full target vocabulary:
`NoMutation, SetSnapshot{snapshot}, SetHasHeader{has_header}, InsertRecord{index,record},
RemoveRecord{index}, SetField{record_index,field_index,value,quoted}`. Every variant's
`diff()` is handcrafted (constructs the sparse `CsvDiff` directly, no apply-and-capture);
`inverse()` is handcrafted per variant, reading whatever pre-state it needs from `base`
(e.g. `RemoveRecord`'s inverse recovers the removed record's exact value+quoted content from
`base.records[index]`). `apply_csv_mutation` follows the exact recipe body: `let d =
mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`.

The pre-existing `📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}` triad (the only triad
directory that already existed for this artifact) was kept and updated to match the new
`diff_set_snapshot(base, next)` signature — it's a thin delegate to the top-level functions,
so no logic duplication. Per the brief, per-variant triad dirs are OPTIONAL scaffolding; the
4 new variants (`SetHasHeader`/`InsertRecord`/`RemoveRecord`/`SetField`) were added directly
inline in the top-level `🧬️mutations/🦀️component.rs` file rather than creating new triad
directories.

### Engine (`⚙️engine/🦀️component.rs`)

Tokenizer (`parse_csv_records`) now returns `Vec<CsvRecord>` and tracks `quoted` per field
(set the moment a `"` opens a field, independent of whether the CONTENT strictly needed
quoting). Writer (`escape_field`) quotes when `field.quoted || structurally-required`.
`decode_csv_with`/`encode_csv_with` no longer splice the header row in/out — `has_header` is
now pure metadata as described above. Existing tests updated to the new shape (field-value
extraction helper `field_values`), plus two new tests: quoted-provenance assertions on the
already-existing quoting test, and the new `quoted_flag_round_trips_even_when_not_structurally_required`
test.

### Analyzer (`🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs`)

`looks_like_csv`'s consistency-sniffing logic updated from `.rows`/`.headers` to
`.records[i].fields.len()` — same sniffing heuristic (consistent field count across records,
width > 1, comma present), just reading the new shape.

### `CsvArtifact` (`🪆️subsets/✳️any/🧬️schema/🦀️component.rs`)

Not explicitly listed in the ownership brief's mounted-file set, but it's the full-artifact
mirror of `CsvSnapshot` (`to_snapshot`/`from_snapshot`/`set_snapshot` conversions field-for-
field) and would not compile without updating in lockstep — updated `headers`/`rows` →
`records: Vec<CsvRecord>` to match, mechanically, no behavior change beyond the shape.

## Deviation: `DiffAlgebra` import path

`protocol::DiffAlgebra` (the path the recipe and every other in-flight sibling agent's diff
file — txt, xml, deflate, zip, json, binary, all observed with the identical import at some
point during this session — used) does **not currently resolve**: the `protocol` facade
(`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs`, a shared/spine file this
agent must not touch) re-exports `MutationDiff` from `os_spr::command` but was never updated
to also re-export the new `DiffAlgebra` trait (`command/component.rs`, per spine change S-1,
is itself already committed with `DiffAlgebra` defined — confirmed via `git log`/`git diff`
showing both files clean, not mid-edit — this is a genuine, already-committed gap in the
curated re-export list, not concurrent churn).

Fix applied **entirely within this artifact's own files**, no shared file touched: `protocol`
and `dsl` are both `extern crate semio_framework_os_kernel as …` aliases for the SAME crate
(`📦️glue.rs:5-7`), and that crate's root does `pub use crate::os_spr::*;` which brings the
already-public `os_spr::command` submodule (where `DiffAlgebra` lives) into scope at the
crate root regardless of the facade's curated list. So `protocol::command::DiffAlgebra`
resolves today, with zero shared-file edits. Used in `🔺️diff/🦀️component.rs` and the
`🧬️mutations/🦀️component.rs` test module.

**Recommended for the wave's closer** (`glue_followup`): add `DiffAlgebra` to the facade's
existing `pub use crate::os_spr::command::{ ... MutationDiff, ... };` list in
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs` so every other F1 agent (and
any future code) can use the shorter `protocol::DiffAlgebra` path the recipe assumes — purely
cosmetic/ergonomic at this point since the `command::` path already works, not a hard
blocker.

## Non-obvious bug found & fixed during verification

The first version of `absorb_records`'s base→mid index transport used a "tight bound from
d1's own removed/modified/added indices" for the virtual base-array length. This is wrong
whenever d2 (the diff being absorbed IN) references a mid position d1 never itself touched —
concretely, `d1 = InsertRecord{index:2}` alone (no removed/modified at all) produces a
base_len_hint of 0 under the naive tight-bound formula, so the simulated mid array had length
1 instead of the true 4, and `d2 = RemoveRecord{index:0}` (a position d1 never referenced)
silently became a graceful-no-op instead of removing the real first record. Caught by 4 of
the 6 curated absorb test cases (`Insert+Remove-before`, `Insert+Insert-same-index`,
`Add+SetField`, the associativity triple) all failing identically in a standalone scratch
crate before being ported to the real file. Fixed by widening `base_len` (and, symmetric-
ally, the ψ mid→after bound) to also cover whatever mid positions the OTHER diff being
composed needs to query — `base_len = max(own tight bound, needed_mid_len + d1.removed.len() - d1.added.len())`.
Re-verified: 9/9 scratch-crate tests green after the fix, same fix ported to the real file.

## Verification

**Compile**: `cargo check -p semio-s-plugin-stdio` — CSV's own files compile with zero
errors (only the two pre-existing cosmetic warnings: an unused `CsvDiff` import in
`⚙️engine` that predates this session, and an "unnecessary qualification" lint on
`protocol::Mutation` in `apply_csv_mutation`'s fully-qualified call, both harmless).

**`cargo test -p semio-s-plugin-stdio --lib "artifacts::csv"` could not complete** — the
whole-crate build is currently blocked by the SAME `protocol::DiffAlgebra` import gap in
OTHER artifacts' files that are mid-edit by other concurrent sessions right now. Polled
repeatedly over several minutes (20s-interval retry loop): the blocking set shrank from 5
files (`💾️binary`, `📄txt`, `📰xml`, `🗜️deflate`, `🎒️zip`) down to 2 (`🔣️json`'s diff+
mutations files) as those other sessions progressively fixed their own imports, but `🔣️json`
remained stuck at the same 2 errors for the whole polling window and never resolved before
this agent's time budget ran out. This matches the documented "repo-wide cargo build failures
can be another session's in-progress work" pattern — confirmed via `git status` that none of
the blocking files are CSV's own, and CSV's own `cargo check` is clean in isolation
throughout.

**Runtime law verification done via the ticket's recommended standalone-scratch-crate
technique** instead: `.🦑️repo/…/ARTIFACT-SYSTEM-OVERHAUL…/f1-csv-scratch/` — a
dependency-free crate reimplementing the exact same snapshot/diff/mutation/absorb algorithm
(byte-for-byte the same logic as the real files, only stripped of `serde`/`protocol`/`schema`
plumbing). **9/9 tests pass**: `mutation_diff_law`, `inverse_law`,
`absorb_law_insert_remove_before`, `absorb_law_insert_insert_same_index_both_survive`,
`absorb_law_add_setfield_patches_into_added`, `absorb_law_modify_remove_collapses`,
`absorb_law_associative_triple`, `between_roundtrip_law`,
`field_sweep_every_mutable_field_changes`. The scratch crate's engine codec logic (tokenizer/
writer quoted-tracking) was ALSO separately verified byte-for-byte via a standalone `rustc`
compile of the exact same `parse_csv_records`/`escape_field`/`write_csv_records` functions
against the `codec_retention_law` fixture (comma-forced quoting, pure-retention quoting, an
empty field, an embedded newline, all in one document) — confirmed `decode→encode` round-trips
to the identical bytes and `decode→encode→decode` round-trips to the identical snapshot.

The real crate's test module (`🧬️mutations/🦀️component.rs`) contains the equivalent tests
(plus `mutation_diff_law`/`inverse_law` sweeping ALL 6 mutation variants including
`NoMutation`/`SetSnapshot`, which the scratch crate — stripped of the `Mutation` trait
machinery — didn't model) and `⚙️engine/🦀️component.rs` contains the exact tokenizer/writer
verified standalone above, both believed green on that basis, but NEITHER has been executed
end-to-end inside the real crate: polled `cargo test -p semio-s-plugin-stdio --lib
"artifacts::csv"` repeatedly for several minutes via a retry loop; the blocking set of OTHER
artifacts (unrelated to csv) shrank from 5 files down to 2 (`🔣️json`) but never fully cleared
within this session's time budget. **Action needed from the next agent/closer touching this
ticket**: re-run
`cargo test -p semio-s-plugin-stdio --lib "artifacts::csv::standards::v_rfc4180::subsets::any::schema::mutations::tests"`
(covers the diff/mutation/absorb laws) and
`cargo test -p semio-s-plugin-stdio --lib "artifacts::csv::standards::v_rfc4180::engine::tests"`
(covers the codec/retention laws) once `🔣️json`'s own `protocol::DiffAlgebra` import is fixed
(or apply the same `protocol::command::DiffAlgebra` workaround to `🔣️json`'s two files, or land
the `glue_followup` facade fix below, either unblocks it) — no further CSV-side changes are
expected to be needed.

**Static checks** (per the verification checklist): `grep "snapshot: Option<" 🔺️diff/🦀️component.rs`
→ zero struct-field occurrences (only a doc-comment explicitly saying there ISN'T one).
`grep "impl DiffAlgebra" 🔺️diff/🦀️component.rs` → present (`impl DiffAlgebra<CsvSnapshot> for CsvDiff`).
`grep ".headers\|.rows" 🗿️artifacts/📊️csv/` → zero occurrences anywhere in the artifact
(old field names fully retired, including in the io serializer/deserializer, composer, and
builder files, none of which needed changes since none referenced the old fields directly).

## Facet leaves handcrafted

Every snapshot/diff/mutations facet leaf was a placeholder stub (`Placeholder { headers:
string[] }`, `*OCTET`/`size-eos` grammars) before this session — all handcrafted:
- `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json` (JSON Schema), `🛰️component.proto`
  for the artifact/snapshot/diff/mutations levels — real interfaces/types matching the Rust
  serde shapes, discriminated union on `CsvMutation`'s `mutation` tag.
- `📝️text/{🅰️.g4,🔤️.ebnf,📖️.grammar.semio}` for snapshot: real RFC 4180 grammar
  (`file = record (CRLF record)* CRLF?`, quoted/non-escaped field alternation, TEXTDATA
  widened to include the full UTF-8 range our codec actually accepts, documented as a
  deliberate superset of the RFC's ASCII-only TEXTDATA).
- `📝️text/{🅰️,🔤️,📖️}` for diff/mutations: honest grammars naming the REAL JSON field/tag
  names (`"hasHeader"`, `"records"`, `"noMutation"`/`"setSnapshot"`/…) rather than restating
  RFC 8259's JSON grammar in full — diff/mutations have no bespoke textual DSL, their wire
  text IS `serde_json`'s output via `OpText`.
- `💾️binary/{🥋️.ksy,🌶️.spicy,🔠️.abnf,📡️.protocol.semio}` for all three levels: snapshot's
  binary facet documents the real shared `.semio` envelope (8-byte magic, u32 LE token
  length, UTF-8 token, UTF-8 RFC 4180 payload) verified against
  `store::semio_format::wrap_binary`/`unwrap_binary`'s actual byte layout in
  `🧬️semio/🦀️component.rs`; diff/mutations document that `OpBinary` is raw un-enveloped
  JSON bytes (verified against `CsvMutation`'s own `encode_op`/`decode_op` impls, which call
  `serde_json::to_vec`/`from_slice` directly with no `wrap_binary` call).

No `*OCTET`/`size-eos` leaf reduces the WHOLE document to an opaque blob anymore — remaining
`*OCTET`/`size-eos` occurrences are terminal-level (a genuinely variable-length UTF-8 token or
payload span within an otherwise fully-structured envelope), which is normal ABNF/Kaitai
style, not a placeholder.

## Files touched

- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` (rewritten)
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (rewritten)
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (rewritten, tests added)
- `🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (`CsvArtifact`, mechanical field-shape sync)
- `⚙️engine/🦀️component.rs` (tokenizer/writer quoted-tracking, tests updated)
- `🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs` (`.rows` → `.records[i].fields`)
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (signature sync)
- All snapshot/diff/mutations/(artifact) facet leaves: `🟦️component.ts`, `🔗️component.graphql`,
  `🔣️component.json`, `🛰️component.proto`, and grammar leaves under `📝️text/` and `💾️binary/`
  in each of `🧬️schema/`, `🧬️schema/📸️snapshot/`, `🧬️schema/🔺️diff/`, `🧬️schema/🧬️mutations/`.
- Scratch verification crate: `.🦑️repo/…/ARTIFACT-SYSTEM-OVERHAUL…/f1-csv-scratch/` (kept in
  ticket folder per the scratch-file policy, not deleted).

## Deviations summary

1. `protocol::command::DiffAlgebra` instead of `protocol::DiffAlgebra` — shared-facade
   re-export gap, worked around locally, see above; recorded as a `glue_followup`.
2. `CsvRecordDiff` uses a positional `Vec<Option<CsvFieldDiff>>` shape rather than a nested
   removed/modified/added triple — justified above by the mutation vocabulary having no
   field-insert/remove op; `CsvDiff::between` still handles arbitrary field-count mismatches
   honestly via a remove+add pair at the records level.
3. `CsvArtifact` (`🧬️schema/🦀️component.rs`) updated even though not explicitly listed in the
   mounted-file set — necessary for compilation, purely mechanical field-shape sync, no new
   logic.
4. `cargo test` not run to completion in the real crate — blocked by OTHER artifacts'
   in-progress concurrent edits hitting the same `protocol::DiffAlgebra` gap (not caused by
   this session); runtime correctness instead verified via an isolated scratch crate
   implementing the identical algorithm, 9/9 green.
