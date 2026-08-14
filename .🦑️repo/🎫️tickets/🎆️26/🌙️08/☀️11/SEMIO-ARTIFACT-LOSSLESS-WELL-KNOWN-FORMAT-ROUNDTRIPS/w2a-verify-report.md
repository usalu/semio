# W2a Independent Verification Report (brep / mesh / model / object / cad / drawing)

Verifier: W2a-verify. Re-read every diff/mutations/composer file on disk (not just the reports),
and independently ran the crate's own test suite and policy tool — did not trust any agent's
self-reported numbers.

## Headline

**3 of 6 subsets have real, reproducible test failures right now** (brep, mesh, model). All three
of those subsets' own reports admitted they never got a green `cargo test` run and asked the
closer to re-verify — that re-verification is what this report does, and it found genuine bugs,
not phantom foreign breakage. object, cad, and drawing are clean. **cad has no report file at all**
(`w2a-cad-report.md` does not exist) despite the subset itself being fully implemented and passing.

## Per-subset check table

| Subset | 1. No apply-and-capture in diff()/between() | 2. No `snapshot: Option<>` escape hatch / no illegitimate catch-alls | 3. DIALECT/WRITES matches dir | 4. Real registered SubsetValidator | 5. Own tests actually pass | Report exists & numbers honest |
|---|---|---|---|---|---|---|
| brep | PASS | PASS | PASS (`SubsetId("brep")` under `✳️brep/`) | PASS (`check_brep_referential_integrity`: dangling vertex/loop/face/shell refs) | **FAIL** — `field_sweep_every_field_present_in_diff` fails for real | PASS (report explicitly says "pending verification," did not fabricate a pass) |
| mesh | PASS | PASS | PASS (`SubsetId("mesh")`) | PASS (`check_mesh_referential_invariants`: dangling `materialId`, duplicate ids) | **FAIL** — 3 real failures (`field_sweep`, `between_roundtrip_law`, `inverse_law`) | PASS (report says "unverified... treat claims as unverified until closer runs it" — honest) |
| model | PASS | PASS | PASS (`SubsetId("model")`) | PASS (`semio_model_referential_diagnostics`: dangling parent_id/spatial_id/relation refs, self-parenting) | **FAIL** — `op_text_binary_roundtrip_law` fails for real | PASS (report says "could NOT be run to a pass/fail number," honest) |
| object | PASS | PASS | PASS (`SubsetId("object")`) | PASS (dangling `Ref`, duplicate `objects` id) | PASS — 32/32 in-crate | PASS (report's 33/33 standalone-harness claim is a proxy, clearly labeled as such, and the real in-crate run now confirms it) |
| cad | PASS | PASS | PASS (`SubsetId("cad")`) | PASS (`cad_referential_diagnostics`: dangling layer/block-insert refs) | PASS — 13/13 in-crate | **FAIL — no report file exists at all** (`w2a-cad-report.md` missing; only found via directory listing + `git status`) |
| drawing | PASS | PASS | PASS (`SubsetId("drawing")`) | PASS (`check_drawing_invariants`: dangling style ref, duplicate layer id) | PASS — 13/13 in-crate, matches report's own 13-test enumeration exactly | PASS (report's "1478/19" full-crate snapshot is stale relative to now, but its own subset claim — "13 tests, 0 of the failures are mine" — still holds) |

## Detail on the 3 real bugs found

### brep — `field_sweep_every_field_present_in_diff` (test-fixture bug, not an algorithm bug)
`between_edge` (diff.rs:262-266) is correctly hand-written (field-by-field `!=` comparison). The
failure is in the test's own `sweep_a()`/`sweep_b()` fixtures (diff.rs:763-820): edge `e1`'s
`end_vertex` is `"v1"` in both snapshots (only `start_vertex` and `curve` actually change), so
`between_edge` correctly returns `end_vertex: None` — but the assertion at line 849 wrongly demands
`e1.diff.end_vertex.is_some()`. This is exactly the kind of "self-caught, self-fixed" fixture bug
`drawing`'s own report documents catching in its `layers` sweep — brep just never ran the test to
catch it, because it was blocked on concurrent compile the whole session (its report says so
explicitly). Diff algorithm itself is correct; the test fixture needs `end_vertex` changed too.

### mesh — 3 failures, all one root cause: `NamedTripleDiff.added` has no positional fidelity
`object`'s own report ("Shared infra gaps #2") independently discovered and fixed this exact bug in
its own subset: the shared `engine::triples::NamedTripleDiff<K,D,T>.added: Vec<T>` records no
target position, so `apply_named` (mesh diff.rs:50-64) can only ever append new items at the *end*
of the collection — silently reordering the reconstructed snapshot whenever a remove+add happens in
the same `between()` (mesh's fixtures rename/replace `meshes`/`primitives`/`materials`/`textures`
members, so ordering flips). `object` fixed this locally with a `NamedAdded<T>{index,item}` wrapper;
`mesh` never applied the equivalent fix, so its `field_sweep`/`between_roundtrip_law`/`inverse_law`
tests fail on exact snapshot-equality checks that are order-sensitive. This is a genuine,
reproducible correctness bug in `mesh`'s diff, not concurrent-wave noise.

### model — `op_text_binary_roundtrip_law` (real double-`Option` serde bug)
`SemioModelMutation::SetElement.spatial_id: Option<Option<String>>` with `#[serde(default)]`
(mutations.rs:42). Serializing `Some(None)` produces `"spatial_id":null`; deserializing `null` back
through serde's default `Option` handling collapses to the *outer* `None` instead of `Some(None)` —
the classic double-Option serde footgun (needs a custom `deserialize_with`, which was not added).
Confirmed live: `SetElement{..., spatial_id: Some(None), ...}` round-trips to
`SetElement{..., spatial_id: None, ...}` through `print_op`/`parse_op`. Real bug, not test noise.

## Grep sweep across all 6 (apply-and-capture / catch-all / full-replace)

- No subset's `diff()`/`between()` calls `.apply(` on itself to derive a diff (`object`'s `inverse()`
  legitimately derives via `mid = self.apply(base); Self::between(&mid, base)` — that's the
  accepted generic inverse-from-between technique, not a diff/between shortcut, and `between()`
  itself is hand-written).
- No subset has a `snapshot: Option<...>` full-replace field on its `Diff` struct — every occurrence
  found is a doc comment explicitly documenting the *absence* of that anti-pattern.
- Every bare `other =>`/`_ =>` match arm found is a legitimate decode-error fallback
  (`Err(format!("... unknown tag {other:?}"))`) or a variant-kind-mismatch fallback
  (`DrawNodeDiff::Replace`/`SemioValueDiff::Replace`, mirroring `svg`'s/`json`'s own precedent) —
  none hide a collection-level diff behind a silent catch-all.
- All 6 mutation `diff()` match bodies are hand-written per-variant (field-by-field construction or
  explicit `old != new` comparisons), not derived via clone+apply+between.

## Full gate

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio"
```
**276 passed; 5 failed** (brep×1, mesh×3, model×1 — see above; object/cad/drawing all clean).

```
cargo test -p semio-s-plugin-stdio --lib
```
**1483 passed; 14 failed; 1 ignored** (vs. w1b baseline **1231 passed; 0 failed**). Of the 14
failures: **5 are the real semio bugs above**, the other **9 are foreign** (`csv`/`json` standard
engines — `artifacts::csv::standards::v_rfc4180::*`, `artifacts::json::standards::v_rfc8259::*` —
entirely outside this ticket's brep/mesh/model/object/cad/drawing scope, not attributable to any of
the 6 agents verified here).

```
bun ./📜️script.ts policy
```
**21524 high-priority breaches** (vs. w1b baseline **21513**, +11 net across the whole concurrent
W2 fan-out). Filtered to the 6 subsets under review: **exactly 2 breaches per subset, 12 total**,
and every single one is the same pre-existing pair every report claimed
(`taxonomy/emoji-prefix` on the `📄set-snapshot` dir, `os-state-authority/item-scope-global` on the
composer's `VALIDATOR_ENTRY: OnceLock`) — confirmed by direct inspection of the flagged lines, not
just trusting the reports. **Zero new policy breaches attributable to any of the 6 subsets'
real implementation work.**

Scope check: `git status --porcelain` confirms all 6 subsets touched **zero files outside their own
`✳️<subset>/` directory** — no cross-subset or shared-file writes.

## Overall verdict

**FAIL — not ready to close.** object, cad, and drawing are solid (real diffs, real validators,
real passing tests, correct DIALECT wiring, zero new policy breaches). brep, mesh, and model each
have one real, reproducible, non-foreign bug blocking their own law tests:
- brep: fix the `field_sweep` test fixture (`sweep_b`'s edge `e1.end_vertex` needs to actually
  differ from `sweep_a`'s).
- mesh: port `object`'s `NamedAdded<T>{index,item}` fix (or the equivalent) into mesh's
  `apply_named`/`between_named` so re-added items land at the correct position instead of always
  appending at the end.
- model: fix `SetElement`'s `spatial_id: Option<Option<String>>` `OpText`/`OpBinary` round trip
  (custom double-Option (de)serialization, or restructure the field to avoid nested `Option`).

Additionally: **cad's missing `w2a-cad-report.md` should be written** before this wave is
considered closable — the underlying work is real and passes, but the process gap (no report,
confirmed only by re-deriving from `git status`/direct file reads) should not repeat.
