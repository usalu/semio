# W1b Close Report

Agent: W1b (Close), serial, sole writer this wave for the 4 hot single-writer files:
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`,
`✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`, `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`, `📜️script.ts`.
Also lightly edited 21 scaffolded `.rs` files under the new semio/format artifact trees to make
the crate compile (explicitly in-scope per the task brief) — see Task 1 §"Compile fixes" below.

---

## Task 1 — glue.rs mounts

Wrote a generator (`generators/gen_glue.py`, copied into this ticket's scratch reference —
actual run artifacts are the `.txt` files below) that walks the real scaffolded files on disk for
`🧿️semio` + the 7 new format artifacts and emits the exact `#[path]` nested-mod tree, mirroring
gif/step's convention 1:1 (verified: every `#[path]` this wave inserted resolves to a real file on
disk — 431/431 checked programmatically). Spliced as 8 new `pub mod <artifact> { ... }` blocks
into `pub mod artifacts { ... }`, after the existing `bcf` block. Brace-balance verified
(open==close both for the inserted block alone and the whole file after insertion).

### Compile fixes (in-scope scaffold-file repairs, design/shape unchanged)

`cargo check -p semio-s-plugin-stdio --lib` went from 69 real errors (after an initial run that
also showed 2 errors in a concurrently-being-fixed foreign dependency crate,
`semio-framework-plugin` — confirmed via `git status` MM and resolved on its own mid-wave by the
other session, matching W1/W1b's own prior reports of this exact breakage) down to **0 errors**:

1. **19 missing companion-type imports** in `🧬️schema/🦀️component.rs` files (semio's 13 subsets +
   `✳️any` minus brep which needed the same fix separately, plus mp4/avi/mp3/wav/epw/tsv — html was
   already fine): each file imported only its top-level `*Snapshot` type but referenced a second
   type from the same `snapshot` module (e.g. `Vec<BrepSolid>`) without importing it. Added the
   missing names to each existing `use …::snapshot::{...}` import.
2. **`sniff_real_bytes` missing from mp3's engine**: mp3's analyzer and doc comment both reference
   `engine::sniff_real_bytes`, but only `detect_id3v2`/`find_frame_sync` existed. Added a
   `pub fn sniff_real_bytes(bytes: &[u8]) -> bool` wrapper (ID3v2 header OR frame sync), matching
   the other 6 formats' identical function signature — no other engine files needed this.
3. **All 21 schema-owning mutation enums missing `OpText`/`OpBinary`** (`ArtifactCodec::of` in
   `register_document_codec` requires `Mutation: OpText + OpBinary`): tried the "real" path first
   (`#[derive(dsl::DslOps)]` + `#[dsl(block)]` on the embedded snapshot field, copying gif/binary's
   exact pattern) on `semio/brep` as a trial — it surfaced `SemioBrepSnapshot: DslField` not
   satisfied, which would require deriving `dsl::DslRecord` on every nested type in the snapshot
   tree (real, substantial W2 work, not a wiring fix). Switched to a hand-rolled `serde_json`
   round-trip for `OpText`/`OpBinary` instead — same "JSON-pack passthrough" honesty boundary the
   scaffold's own `ArtifactPack` impl already documents for these types, no snapshot-tree changes
   needed. Also added the missing `NoMutation` default variant (needed for `Default`, which
   `ArtifactCodec::of` likely also wants transitively via other bounds already satisfied elsewhere)
   and a `#[cfg(test)] use protocol::{OpBinary, OpText};` import (present in gif's file, missing
   from every scaffold file — caused a second round of test-only compile errors after the first
   `cargo check` pass went green). Added an `op_text_binary_roundtrip_law` test per file (21 new
   tests), matching the existing `mutation_diff_law`/`inverse_law` test naming convention.
   Documented the design deviation with a doc comment on every touched `OpCodecs` region explaining
   exactly why the derive path wasn't used and that W2 replaces this whole region anyway.

**Final state**: `cargo check -p semio-s-plugin-stdio --lib` → 0 errors (267 pre-existing-pattern
warnings only). `cargo test -p semio-s-plugin-stdio --lib` → **1231 passed; 0 failed** (up from
the W1 baseline's 1075 — 156 new, all from the newly-reachable semio/format trees: 92 under
`artifacts::semio::`, 9–10 each under the 7 format artifacts).

## Task 2 — component.rs plugin() registration

Added `crate::artifacts::semio::standards::v1::engine::register();` (per the scaffold's own doc
comment, this single call registers all 14 subsets' schema descriptors + document codecs +
SubsetValidators) and `crate::artifacts::{mp4,avi,mp3,wav,epw,tsv,html}::standards::<slug>::engine::register();`
to the existing chain, plus 8 new `.artifact_kind(crate::artifacts::<x>::artifact_kind())` calls,
following the exact existing pattern for all 28 prior artifacts (pdf's own
`standards::v1_7::engine::register()` extra call is the direct precedent for semio/formats needing
the standard-qualified path rather than a top-level `engine` shim).

## Task 3 — catalog.json

Byte-identical round-trip verified before editing (`json.load` → `json.dumps(indent=2,
ensure_ascii=False)` reproduces the original file byte-for-byte when nothing changes — confirmed
via `diff` before making any real edit). Pre-edit snapshot: `w1b-catalog-before.json`. Diff of
every change: `w1b-catalog-diff.txt`.

- **+8 `stdio_roster` rows**: `semio` (dir `🧿️semio`, mime `application/vnd.semio`, ext `.semio`,
  `depends` = the 28 distinct bridged-format ids from the master plan's lattice table — every
  format any semio subset imports/exports from, in existing-roster order + the 4 new binary
  formats appended); the 7 formats (mp4→`video/mp4`/binary dep, avi→`video/x-msvideo`/binary,
  mp3→`audio/mpeg`/binary, wav→`audio/wav`/binary, epw→`text/plain`/txt dep, tsv→`text/tab-separated-values`
  (the real IANA-registered TSV mime, matching the `iana` standard slug)/txt dep,
  html→`text/html`/txt dep — binary vs. txt dependency chosen to mirror how the existing roster
  already splits binary-container vs. text-line formats, e.g. gif/dwg/png→binary,
  csv/md/json/xml→txt).
- **+35 `stdio_dag_edges`** mirroring the new rows' `depends` exactly like every existing row.
  Verified acyclic: 28 edges point *from* semio *to* formats, zero edges point the other way.
- **`neutral` field retired from all 36 rows** (0 occurrences left, confirmed via grep) — W1
  confirmed zero script.ts readers.
- **`counts.stdio_artifacts`: 28 → 36**.
- **`curated_io_pairs` left at 273** (hand-maintained, not mechanically recomputed anywhere in
  script.ts) with a new sibling `curated_io_pairs_note` explaining why and pointing at W4.
- **+1 `owners` row for `s.stdio.semio`** (`plugin: "🗄️stdio"`, `kind_id: "s.stdio.semio"`,
  `stdio_artifacts`: the same 28-id capability-statement list). **`import`/`export` deliberately
  left `[]`** — discovered mid-wave that `policyIoMatrixMigratedBreaches` checks exactly these two
  fields against *physical* io leaf existence for any owner whose `path` is already migrated to
  the `🏅️standards/` shape (semio is the very first such owner; none of the 54 pre-existing domain
  owners are migrated yet, so this interaction never fired before). A populated 28-id import/export
  list would have produced 56 new breaches with **no allowlist mechanism available** for that rule
  (verified: no `POLICY_*_ALLOWLIST` const backs it). Since `owner.stdio_artifacts` itself is never
  read anywhere in `script.ts` (grep-confirmed — only `table.counts.stdio_artifacts` is), keeping
  it populated costs nothing and still documents the design intent; W4 populates `import`/`export`
  together with the real io leaves in one atomic step, exactly when this rule should start
  legitimately checking them.

## Task 4 — script.ts (3 edits)

1. `policyStdioCatalogBreaches`'s `?? 29` fallback → `?? 36` (plus the doc comment and error-text
   "Twenty-nine"/"exactly 29" strings sitting directly next to it, also stale, fixed alongside for
   the same reason — matching the CLAUDE.md refactor-inconsistencies rule).
2. **New `policyStdioCodecIdUniquenessBreaches` rule** (kind `stdio-artifacts/codec-id-uniqueness`),
   addressing W1's blocking finding that `store::register_document_codec` silently overwrites on a
   duplicate schema id (empirically confirmed by W1's own regression test — plain `HashMap::insert`,
   no panic). Two-pass grep: (1) crate-wide map of every `const NAME: &str = "value";` in stdio,
   keyed by identifier's last path segment (handles both bare and fully-`crate::…`-qualified
   references, both forms are used in the wild); (2) every
   `register_document_codec(store::ArtifactCodec::of::<…>(<id-expr>))` call site's id expression
   resolved through that map (or taken directly if it's a string literal), grouped by resolved
   value, flagged if >1 call site claims the same id. Wired into `policyStdioArtifactsBreaches`
   right after its sibling `policyDialectLiteralPathBreaches`.
   - **Round-trip verified for real**: ran it clean first (0 breaches at the time — before glue.rs
     was mounted the rule couldn't see any of the new files' call sites at all); after mounting, it
     **immediately found a genuine pre-existing bug**, not a synthetic one — `📐️step`'s sibling
     artifact `🖊️dwg`'s two standards (`ac1018`/`ac1024`) both register the exact same id
     `"stdio.dwg"` via the exact same shared `STDIO_DWG_DOCUMENT_SCHEMA` constant (harmless today
     only because both sites also register the identical `(DwgSnapshot, DwgMutation)` type pair —
     but exactly the "two codecs, one id" shape the rule exists to catch, and dwg is outside this
     wave's write-scope so **left unfixed and reported**, not silently patched). Then did the
     requested synthetic probe: temporarily duplicated html's real call site under tsv's id
     (cross-artifact, fully-qualified-path form) inside
     `🌐️html/…/🎹️composer/🦀️component.rs`, confirmed the rule fired (4 breaches, +2 from the
     injection), reverted the file to a byte-identical `diff`-confirmed clean state, confirmed back
     to exactly the 2 pre-existing dwg breaches. Confirmed **zero duplicate ids among all 21 new
     semio/format schema descriptors** (the scaffold agent did its job).
3. **Shrink-only allowlist seeding**, keys computed programmatically (temporarily exported
   `policyNormalizeRelPath`/`policyDiffCompletenessBreaches` to call them from a scratch script and
   read back the real canonical keys; both exports reverted afterward — zero net diff there):
   - `POLICY_DIFF_COMPLETENESS_ALLOWLIST`: +21 keys (13 semio subsets + `✳️any` + 7 formats — every
     schema-owning unit's full-replace `Diff` has no `protocol::DiffCodec` impl yet, exactly the
     documented W2 scope).
   - `POLICY_ROUND_TRIP_TEST_ALLOWLIST`: +7 keys (the 7 format engines + semio's own v1 engine —
     `tsv` correctly excluded, its engine is genuinely complete per the scaffold manifest and
     already passes the rule without allowlisting).
   - **No allowlist exists** for `mutation-migration/triad-completeness`,
     `mutation-migration/artifact-engine`, `artifact-schema/facet-completeness`,
     `stdio-artifacts/composer`, `os-state-authority/item-scope-global`, `taxonomy/emoji-prefix`, or
     `artifact-schema/type-name-parity` — all 8/8/24/29/29/21/8 = 127 remaining new breaches are in
     these categories, and every one matches an already-large pre-existing baseline pattern
     (confirmed by inspection: these rules check the *pre-migration* artifact-root shape and
     haven't been taught about `🏅️standards/` yet — the same generalization W1's Task 1 already did
     for `schema-representation`, out of this wave's 3-item Task 4 scope to repeat for 7 more
     rules). Left as documented, non-blocking, unfixed gaps for the orchestrator's future call —
     fabricating allowlist entries for rules with no allowlist mechanism was not done.

## Task 5 — type-ownership doc

Written to `w1b-type-ownership.md`: one row per subset (owned types + reserved-for-W2 names +
spec-mandated cross-reuse notes for model↔brep/mesh and presentation↔document), shared
🧮️geometry/🧰️triples infra section, and a summary of what's genuinely real vs. scaffolded
placeholder (cross-checked against disk, not just copied from the scaffold manifest).

---

## Verification (full gate, real output)

```
cargo test -p semio-s-plugin-stdio --lib
```
**`test result: ok. 1231 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`**
(`w1b-closer-cargo-test-final.txt`) — up from the W1 baseline's 1075/0, all new tests pass,
growing. Scoped checks: `artifacts::semio::` → 92/0, each of the 7 formats → 9–10/0.

```
bun ./📜️script.ts policy
```
**`21513 high-priority breach(es) across 25 rule(s)`** (`w1b-closer-policy-final.txt`), vs. W1's
baseline `21384` (+129 net, +2 of which is the new codec-id-uniqueness rule's pre-existing real dwg
finding — unrelated to this wave's new files). My new artifact trees' own breach total: **127**
(`w1b-closer-my-new-breaches-final.txt`), down from an initial post-mount 211 after allowlist
seeding (-21 diff-completeness, -7 round-trip-test) and the owners-row `import`/`export` fix (-56
io-matrix-migrated). Every remaining one of the 127 is a documented pre-migration-shape rule
limitation shared with pre-existing artifacts, not a genuine scaffold defect.

```
git check-ignore -v ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🎥️mp4,📼️avi,🎵️mp3,🔊️wav,🌦️epw,📑️tsv,🌐️html}
```
Exit 1, empty output (`w1b-closer-gitignore-check.txt`) — **zero gitignore traps**, confirmed also
via `git status --porcelain` showing all 8 as plain `??` untracked.

## Files changed this wave

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — 8 new artifact mount blocks (~1465 lines).
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` — 8 new `engine::register()` + 8 new `.artifact_kind(...)` calls.
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` — +8 roster rows, +35 DAG edges, +1 owner row,
  `neutral` retired repo-wide (36 rows), counts updated.
- `📜️script.ts` — fallback bump + stale-string fix, new `policyStdioCodecIdUniquenessBreaches` rule
  (+ wired into `policyStdioArtifactsBreaches`), 2 allowlists seeded (+21, +7 keys).
- 21 scaffolded `.rs` files under `🧿️semio/…` and the 7 format artifact trees — import fixes, one
  missing `sniff_real_bytes` fn, `OpText`/`OpBinary` impls + `NoMutation` variant + test on every
  mutation file (compile-blocking, in-scope per the task brief; design/shape otherwise unchanged).
- New: `.🦑️repo/🎫️tickets/…/w1b-type-ownership.md` (Task 5 deliverable).
- Ticket scratch (this report + raw command outputs, all `.txt`): `w1b-catalog-before.json`,
  `w1b-catalog-diff.txt`, `w1b-closer-cargo-check-final.txt`, `w1b-closer-cargo-test-final.txt`,
  `w1b-closer-policy-after-glue.txt`, `w1b-closer-policy-final.txt`,
  `w1b-closer-my-new-breaches-final.txt`, `w1b-closer-gitignore-check.txt`.

## Open items for the orchestrator / next waves

1. **Pre-existing dwg codec-id collision** (`stdio.dwg` registered twice, `ac1018` + `ac1024`,
   harmless today only because both sites share the same type pair) — surfaced by the new policy
   rule, outside this wave's write-scope, not fixed here.
2. **7 rule categories with no allowlist mechanism** (127 breaches total) all stem from the same
   root cause W1's Task 1 already generalized once for `schema-representation`: policy functions
   that check the pre-migration artifact-root shape and don't yet understand `🏅️standards/`. Same
   fix shape as W1 Task 1, not repeated here (out of this wave's 3-item Task 4 scope).
3. **W2's actual job is unchanged and now unblocked**: replace the full-replace
   `Diff`/`SetSnapshot`-only `Mutation` shape with real per-field vocabularies per subset (this
   wave only made the placeholder shape *compile* with real `OpText`/`OpBinary`, it did not attempt
   the real per-field diff work — that's explicitly still W2's).
