# 📸️ W2b — mutate-remodeling-1 harness: runtime fixture resolution

Finishes the harness rewrite the first W2 shard started (mounts + oracle names + feature, auto-committed
`5e03e56997`) and closes the two halves it never reached: the Rust adapter's 107 compile-time
`include_str!` literals and the Python reference's hard-coded `VECTORS` path table.

## 1. Final harness shape

**One statement of where a vector lives, and it is the FEATURE file's.** Every scenario carries a doc
string naming its documents as fixture URIs; both halves resolve them through the test context at RUN
time, so the plan pins their digests and neither implementation carries a transcribed path that can
drift away from the directory it names. That drift is exactly what the 2026-09-05 path-shortening pass
caused here.

| | before | after |
|---|---|---|
| `🥒️.feature` | 34 `<vector>` rows + 3 scenarios addressing fixtures by prose/step text | 35 kinds × 2 outlines by doc string, `commit-reconstruction` by three `local://` URIs, `identity-round-trip` by a `carrier` URI |
| `🦀️.rs` | 107 `include_str!`, `fixture_text(kind)` match with 35 arms, oracle **and** subject registration | `subject::vector(ctx)` → `ctx.doc_json()` → `ctx.fixture_bytes(spec.str("before"))…`; **0** `include_str!`; subject role only |
| `🐍️.py` | `VECTORS = {kind: (dir, fixture, tag)}` + `_leaf_root()` | `TAGS = {kind: wireTag}` + `doc_json(ctx)` + `spec_of(ctx, kind)`; no path fragment anywhere |

Role split now matches the `@oracle-remodeling-1-python-independent` tag the feature declares: **Rust is
the subject** (gated on `sut`, which the runner enables for that role alone — `📜️script.ts:474`),
**Python is the oracle**. The old Rust oracle registrations were dead: `oracleDecision` maps the
registered oracle's ecosystem to `python`, so Rust never runs that role.

Invariants kept, all of them still asserted in the Rust subject handler: `law::divergence` (apply ==
committed after), `law::mutation_is_observable` (the vector moved the document), `law::inverse_restores`
(apply + every computed inverse step == committed before, member positions included),
`law::round_trip_preserves` + `law::carrier_is_exact` (identity). The refusal check is no longer a
`DECLARED_CODE` table in Rust — the code comes from the vector's own doc string, and a vector that
declares one must additionally leave the document untouched.

## 2. Two scenarios that could not have run, and now do

`validateRegistration` (`🟦️.ts:2452`) and the Rust runner (`🏃️runner/🦀️.rs:266`) plan **every**
scenario for a role and error on a gap; the plan is filtered by `@level-` only (`🟦️.ts:1129`), never by
implementation. The committed Python therefore had three unregistered scenario ids
(`mutate-commit-reconstruction`, `inverse-commit-reconstruction`, `identity-round-trip`) and the oracle
role could only have errored on them. Both are now answered honestly:

- **commit-reconstruction** — the reference derives the refusal from the verb's meaning (a commit
  publishes what a staged run produced, so a `sparse` argument carrying an inline `points` buffer with
  no staging handle names no staged run) and independently requires the committed after-document to be
  the before-document unchanged. It does not adopt production's diagnostic text; the code is the
  feature's declaration and the subject asserts it was raised.
- **identity-round-trip** — the two halves are now compared on the **printed carrier**, not on a parsed
  document, because the reference cannot parse `.dsl.semio` (this subset's committed text grammar is the
  repo-wide placeholder `payload = OCTET+`, already reported by the registry's own
  `remodeling-mutation-semantics` entry). Python answers with the committed bytes — what a faithful
  parse-and-reprint must produce — and Rust's projection changed from the parsed snapshot to the printed
  text so the comparison is meaningful. Rust still asserts the reparse half inside its handler, where a
  byte comparison cannot reach.

## 3. `t038` / `t039` renamed

Both placeholders were replaced using the convention `🐍️resync-fixture-names.py` documents, whose source
is `.🧬semio/…/🎆️26/🌙️04/☀️08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY/🪟️shorten-long-paths.ts`:
`truncateWithHash(caseName, max(20, len − (maxPathLen − 190)), salt = repo-relative dirPath)` with
`sha1(salt).slice(0,6)`. The formula was verified against two sibling dirs it had already produced
(`🎥️adds-stream-c-458900`, `📋️records-a-qc-f5caf4` — exact matches; a third,
`🎞️appends-a-third-8ac259`, disagrees on the hash, so the long name recorded in the stale `include_str!`
for that kind was already not the name the pass hashed).

| kind | was | now | old long name (salt) |
|---|---|---|---|
| `change-stream-sync` | `t038` | `⏱️shifts-stream-a-5b442c` | `⏱️shifts-stream-a-sync-offset-to-minus-seven-and-a-half` |
| `update-mesh-params` | `t039` | `🔳️doubles-the-c245d5` | `🔳️doubles-the-texture-size-and-drops-the-watertight-guarantee` |

Repointed together with the rename: the two `#[path]` mounts and their module idents in
`📦️packages/🦀️rust/🦀️.rs` (targeted edits — that file is shared with W4), the four `directoryName`/`id`
fields in `🔮️oracle/🔣️.json`, and four `Examples` rows in the feature.

## 4. Files changed

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📸️mutate-remodeling-1/🥒️.feature`
- `…/📸️mutate-remodeling-1/🦀️.rs` (rewritten)
- `…/📸️mutate-remodeling-1/🐍️.py`
- `…/🔮️oracle/🔣️.json`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/🦀️.rs` (2 targeted edits only)
- `…/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/t038` → `⏱️shifts-stream-a-5b442c` (`mv`)
- `…/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/t039` → `🔳️doubles-the-c245d5` (`mv`)
- ticket: `🐍️fixture-audit.py` (rewritten for the doc-string pattern), `🐍️python-oracle-dryrun.py` (new)

## 5. Verification actually run

```
python3 -m py_compile 🐍️.py                     → OK
python3 🐍️fixture-audit.py                      → 106 distinct fixture uri(s), 0 missing, 106 resolved, 0 unexpanded
python3 🐍️mount-check.py                        → 387 #[path], 1 dangling — NOT mine (see §6)
python3 🐍️python-oracle-dryrun.py               → 71 planned, 71 registered, 71 passed, 0 failed, 0 unregistered, 0 unknown
grep -c 'include_str!' 🦀️.rs                    → 1 (a docstring mention, no fixture include)
```

`🐍️fixture-audit.py` was rewritten because the URIs no longer live only in `Examples` cells: it now
scans the feature description, step text **and** doc strings (exactly the three carriers `fixtureUrisIn`
reads, `🟦️.ts:985`), substitutes each Outline's row, and resolves `asset://` against the owner root,
`local://` against the case's `🧫️fixtures/` and `shared://` against the owner's — matching
`resolveFixtures`. It also fails on a surviving `<placeholder>`, which the old row-only version could
not detect.

`🐍️python-oracle-dryrun.py` is new and is the strongest evidence here: it stubs the `semio_repo_test`
host surface, expands the feature the way `materializeScenario` does, and **executes all 71 oracle
handlers against the committed bytes**. All 71 pass — meaning every one of the 34 kinds' Python appliers
and inverse rules still reach their committed after/before documents through the new doc-string path,
and the registration now covers the whole plan.

**Cargo: not launched from this lane, and the crate does not build.** The central
`cargo check -p semio-s-plugin-remodel --lib` (pid 47704, started 04:38, i.e. against the tree *before*
this shard's renames) finished at ~05:26 with `error: could not compile semio-s-plugin-remodel (lib) due
to 128 previous errors; 39 warnings emitted`. The host was at load 63 and swap 25.8/26.6 GB at that
point — over both gate thresholds — so no lane-local `cargo test` was started, and per the shard's
instructions the errors are recorded rather than retried.

**All 128 errors are in files this shard does not own.** Error classes: E0277 ×61, E0283 ×15, E0433 ×13,
E0053 ×11, E0425 ×7, E0432 ×5, E0063 ×3, E0308 ×2, E0046 ×2, E0599 ×1, E0080 ×1. Top files (all under
`🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

| count | file | lane |
|---|---|---|
| 14 | `✏️editor/📌️panels/✅️quality/🦀️.rs` | W4 |
| 14 | `✏️editor/⚙️engine/🥽️mesh/🦀️.rs` | W4 |
| 13 | `🗿️artifacts/📸️remodeling/🦀️.rs` (plugin root) | W4 |
| 9 | `✏️editor/📌️panels/⚙️parameters/🦀️.rs` | W4 |
| 8 | `🧬️schema/🧬️mutations/🏁commit-reconstruction/🔺️diff/🦀️.rs` | W1 |
| 6 | `✏️editor/📌️panels/🧵️results`, `🎯️calibration` | W4 |
| 6 | `🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️.rs` | W1 |
| 3 | `🚪️io/🦀️.rs` | W6 |
| 1 | `📚️examples/🛰️synthetic-orbit/🦀️.rs` | W7a (in flight) |

**Zero** errors in any of the 34 fixture-leaf test modules, including the two this shard renamed. That is
the part of the lib build this shard is responsible for, and the rename was verified structurally
instead: `🐍️mount-check.py` resolves both new `#[path]` targets, and the 34 `mod tests_*` idents in the
wiring file are still unique. Note also that `cargo test --lib` could not have exercised the case adapter
in any case — `📸️mutate-remodeling-1/🦀️.rs` is compiled only by the generated test host, never mounted
into the crate — which is why the Python dry-run above was built as the executable evidence.

## 6. Findings left for other lanes

1. **`🚪️io/🦀️.rs:566` has a dangling `#[path = "🧪️tests/🦀️.rs"]`** — the only dangling mount in the
   whole remodel tree. That file is W6's scope; not touched.
2. **The Rust subject never sets `production_dispatch`.** `Outcome::with_raw` leaves it `None`, and the
   protocol's own docstring says its absence is how a vector-replay adapter is distinguished from one
   that reached production dispatch. The committed adapter had the same gap; it is pre-existing, not
   introduced here, and closing it needs the bridges to report the dispatch.
3. **No fixture disagrees with the Rust/Python semantics** as far as this shard could measure: the
   Python half reproduces all 34 committed after-documents and restores all 34 before-documents.
   `create-asset` remains the one narrowed comparison — the reference adopts the committed `childId`
   because production mints it through `DefaultHasher`, which the Rust standard library documents as
   unspecified, and checks only its shape. That limit is stated in the reference's own docstring and in
   the feature's prose, and it is unchanged by this shard.
