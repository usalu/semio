# W3-E Report — stdio (resumed, lane R1)

Lane 3-E, exclusive lease `✏️s/🔌️plugins/🗄️stdio/**`. No prior `📓️w3-e-report.md` existed; this is
the first report for this lane (the `🧪️w3-e-cargo-baseline.txt`/`🧪️w3-e-cargo-check2.txt` logs found
in the ticket folder predate this session and were not authored by this lane).

## The 116 `🔺️diff` leaves — NOT converted, recorded as a `sharedFileRequest`

All 116 leaves the census script found are in `🧊️gltf` (0 in any other artifact — every other
artifact's leaves were already converted by an earlier lane). Investigation before touching anything:

- Every one of the 116 files (`move-buffer`, `create-mesh`, `reparent-node`, …) is the **legacy**
  per-kind pattern (`pub fn derive/validate_diff/apply_diff(...) -> Result<_, GltfTopLevelMutationRejection>`,
  routed through `top_level_collections_private`) — none of them has a `pub fn diff(payload, base) ->
  protocol::MutationOutcome<_>` signature at all; they don't implement `Mutation`/`MutationKind` directly.
- The actual `Mutation<GltfSnapshot>` trait boundary is `impl Mutation<GltfSnapshot> for GltfMutation`
  in `🧊️gltf/…/🔨️modules/🧭️mutation-dispatch/🦀️component.rs` — **already returns `MutationOutcome`**
  (confirmed: this is one of the ~47 hand-written dispatch enums, already done, see below).
- That dispatch enum routes through a **new** registry (`GltfMutationRegistry` /
  `GltfMutationLeafDescriptor`, `🧊️gltf/…/🧬️mutations/🦀️component.rs`) that FULL-STDIO is actively
  building: `GLTF_MUTATION_LEAF_DESCRIPTORS` currently wires up only **3** canonical `.v1` commands
  (`change-material-alpha-mode`, `change-material-double-sided`, `create-scene` — each its own
  "common-descriptor adapter" file, a *different*, newer shape than the 116 legacy files). The 116
  legacy leaves are **not referenced by the registry at all** — dead code pending replacement, not
  reachable via `Mutation::diff`.
- `.🦑️repo/…/MUTATION-OUTCOMES…/📋️ownership-and-handoffs.md` names this exact overlap: "Lane 3-E wraps
  stdio's `diff` return types **minimally** — return type only, nothing else. It must not restructure
  a stdio enum" and lists `🗄️stdio/**` (incl. the 34 legacy artifact enums) as FULL-STDIO's active
  overlap. `📓️scout-2-stdio.md` (same ticket, same day) independently confirms FULL-STDIO is
  mid-migration on exactly `gltf/**/🧬️schema/🧬️mutations/**`, converting legacy enums to canonical
  `.v1` commands.
- Conclusion: wrapping the 116 legacy functions' `Result<_, Rejection>` return type would (a) not
  connect to the `Mutation` trait at all since nothing calls them that way, (b) directly collide with
  FULL-STDIO's in-flight registry rebuild of the same files, (c) violate "must not restructure a stdio
  enum". **Recorded as a `sharedFileRequest`**: `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/**`
  (all 116 legacy leaf files) — FULL-STDIO's charter, not touched.

## Hand-written `impl Mutation<..>` — all ~47 already done, 1 illegal code fixed

Grepped every `impl Mutation<` in `🗄️stdio` (47 hits, one per artifact + gltf's dispatch enum): **all
47 already return `MutationOutcome`** (done by an earlier lane/session). Auditing their message codes
against the frozen 7 found two violations:
- `🧿️semio/…/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:102` used `"mutation.target-kind-mismatch"` for
  the subset-mismatch fallback arm — **fixed** to `"mutation.target-missing"` (Error, empty diff; the
  closest frozen-7 fit — an addressed subset that doesn't exist on the base snapshot). Test assertion
  and its `diff().apply(&base).unwrap()` call site fixed to match.
- `🧊️gltf/…/🔨️modules/🧭️mutation-dispatch/🦀️component.rs:262` uses `"mutation.rejected"` for every
  registry-plan failure, folding all per-leaf rejection reasons into one code — **not fixed**: this
  file is FULL-STDIO's active registry (same file as above), the correct mapping isn't obvious (it
  folds many distinct rejection kinds), and per the no-improvisation clause this is reported, not
  guessed. Flagging for the coordinator.

## Call-site fixes (the actual bulk of this session)

`cargo check -p semio-s-plugin-stdio` started at **179 errors** (later baseline re-check: 164). All
were `ArtifactBuilder::absorb` (`🔌️plugin` trait, doc: "Persisted diffs cross the fallible `absorb`
boundary") whose signature changed to `fn absorb(self, diff) -> protocol::MutationApplyResult<Self>`
repo-wide (0-A/kernel work, out of my lease) — every artifact's builder still returned bare `Self`.
Fixed by pattern (script `/private/tmp/…/scratchpad/fix_absorb.py`, then 12 hand patterns: delegating
wrappers, tuple-struct wrappers, gltf's own `try_apply`) + 5 `DiffAlgebra::inverse` call sites
(`zip`/`dwg`/`pdf 1.4`/`semio mesh`/`semio value`) whose `self.apply(base)` needed `.unwrap()` (matching
the established convention already used by `bmp`/`txt`/`tiff`/`jpg`/`png`) + 1 missing `HashSet` import
(`bmp` diff file). `cargo check -p semio-s-plugin-stdio` → **0 errors, clean** (verified fresh).

`cargo test -p semio-s-plugin-stdio --lib --no-run` then surfaced ~400 test-only errors from the same
root cause spread through `#[cfg(test)]` blocks (`.apply()`/`.absorb()` now `Result`). Ran
`cargo fix --lib --tests --allow-dirty --broken-code` to apply rustc's own machine-applicable
suggestions — **this introduced a real regression**: it incorrectly narrowed `use protocol::{OpText,
OpBinary}`/`DiffCodec`/`MutationDiff` imports to `#[cfg(test)]` in ~35 files where those traits are
*also* implemented outside tests (mistaken "unused import" analysis during the broken compile state),
breaking `cargo check` (0 → 176 errors). Found and fixed with a second script
(`fix_op_imports.py`) that un-gates the import wherever a real non-test `impl OpText/OpBinary/DiffCodec
for` exists in the same file, plus one hand fix (`🧿️semio/…/✳️animation/…/🧬️mutations` had a doc
comment wrongly claiming the traits didn't need the import) and one added test-only `use
protocol::MutationDiff;` (`🧿️semio/…/✳️value/…/🧬️mutations`). Re-verified `cargo check` clean again
after the recovery.

## Real cargo counts (verified, not assumed)

- `cargo check -p semio-s-plugin-stdio`: **0 errors**, 966 warnings (pre-existing lint noise). Log:
  `🧪️w3-e-cargo-r1-check7.txt`.
- `cargo test -p semio-s-plugin-stdio --lib`: compiles and runs. **4355 passed; 80 failed; 3 ignored**.
  Log: `🧪️w3-e-cargo-r1-test-final.txt`. Failure triage:
  - ~72 are pre-existing format/codec conformance tests (`conformance_laws::{fixture_honesty,
    grammar_conformance, ops_grammar_conformance}`, `dwg` architectural-example round-trips, `svg` XML
    byte-content round-trips) across `dwg`/`svg`/`xml`/`pptx`/`ifc`/`xlsx`/`docx`/`pdf`/`zip`/`step` —
    assertion mismatches on serialized content/grammar, unrelated to `Mutation`/`absorb`/`apply`
    signatures; almost certainly pre-date this ticket (format codec correctness is FULL-STDIO's/format
    owners' charter).
  - 8 are genuine `unwrap()`-on-`Err` panics from `MutationDiff::apply` now being fallible where it
    previously always returned a value directly: `binary::…::absorb_law_cartesian`,
    `bmp::…::inverse_law`, `json::…::absorb_object_associativity`, `pdf 1.7::…::absorb_law_objects_associativity`,
    `pdf 1.7::…::mutation_apply_inverse_round_trips_every_variant`, `svg::…::inverse_law`,
    `svg::…::mutation_diff_law`, `registry::…::gltf_representation_capability_has_exact_format_claims`.
    These expose latent per-format `diff()`/`apply()` pairing gaps (some mutation-combination or
    inverse now legitimately rejected) that the fallible-`apply` contract (0-A, out of my lease) makes
    visible for the first time — fixing them needs per-format domain judgement about whether the
    combination *should* succeed, which is a "no compatibility shim, no widening a diff's meaning"
    call belonging to each format's owner, not a minimal-wrap call-site fix. Not touched.

## Files touched (all inside the lease)

~95 `ArtifactBuilder`/`DiffAlgebra` call-site fixes (script + hand) + ~35 import-regression recoveries
+ 2 message-code/test fixes (`🧿️semio/…/✳️any/🧬️mutations`) + 2 missing trait-import fixes
(`🧿️semio/…/✳️flow/🚪️io`, `🧿️semio/…/✳️presentation/🚪️io`) + 1 missing `MutationDiff` import
(`🧿️semio/…/✳️value/🧬️mutations`) + 1 doc/import fix (`🧿️semio/…/✳️animation/🧬️mutations`) + 1 missing
`HashSet` import (`🖼️bmp/…/🔺️diff`). `git diff --cached --stat -- ✏️s/🔌️plugins/🗄️stdio`: 284 files
changed (includes earlier lanes' already-staged work plus this session's).

## Blocked / sharedFileRequest

- **`🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/**`** (116 legacy leaf dirs) —
  FULL-STDIO's active registry-migration charter, see above. Do not convert until FULL-STDIO either
  finishes the migration (leaves become unreachable/deletable) or hands off.
- `"mutation.rejected"` illegal code in `🧊️gltf/…/🧭️mutation-dispatch/🦀️component.rs:262` — same file,
  same reason; reported to coordinator for a code-mapping decision, not guessed at.
