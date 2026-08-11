# W1b Independent Verification Report

Agent: W1b Verifier (fresh eyes, re-ran everything from disk, did not trust scaffold/closer
reports). All commands below were re-executed in this session, not copy-pasted from the agents'
own `.txt` files.

---

## Required commands — exact verbatim output (tails as specified)

### `cargo test -p semio-s-plugin-stdio --lib 2>&1 | tail -40`

```
test artifacts::zip::standards::v2_0::engine::tests::decode_rejects_unsupported_method ... ok
test artifacts::xlsx::standards::v_ecma_376::subsets::transitional::composer::tests::conforming_builder_snapshot_composes_and_stamps_transitional ... ok
test artifacts::zip::standards::v2_0::engine::tests::encode_full_metadata_round_trip ... ok
test artifacts::zip::standards::v2_0::engine::tests::sniff_recognizes_real_magic_and_rejects_garbage ... ok
test artifacts::zip::opc::component::tests::round_trip_preserves_parts_and_relationships ... ok
test artifacts::zip::standards::v2_0::engine::tests::decode_rich_synthetic_archive ... ok
test artifacts::zip::standards::v2_0::engine::tests::zip_store_round_trip ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::absorb_law ... ok
test artifacts::zip::standards::v2_0::engine::tests::zip_deflate_round_trip ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::absorb_law_associativity ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::between_roundtrip_law ... ok
test artifacts::zip::opc::component::tests::sniff_recognizes_content_types_entry ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::field_sweep_covers_every_mutable_field ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::diff_codec_text_binary_roundtrip_law ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::mutation_diff_law ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::inverse_law ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::out_of_range_entry_mutation_is_noop_not_panic ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::tests::conforming_snapshot_has_no_diagnostics ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::codec_retention_law ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::tests::data_descriptor_bit_is_soft ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::tests::encrypted_entry_is_hard ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::tests::masked_local_header_bit_is_hard ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::tests::strong_encryption_bit_is_hard ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::tests::version_needed_at_ceiling_is_clean ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::builder::tests::hard_violation_injected_via_raw_mutate_still_fails_build ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::builder::tests::typed_constructors_build_clean ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::composer::tests::clean_snapshot_composes_and_stamps_iso21320 ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::composer::tests::encrypted_entry_gets_normalized_away_and_still_composes ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::composer::tests::high_version_needed_gets_capped ... ok
test artifacts::zip::standards::v2_0::subsets::iso21320::composer::tests::subset_validator_flags_real_violations_without_normalizing ... ok
test artifacts::zip::standards::v2_0::subsets::any::schema::mutations::component::tests::op_text_binary_roundtrip_law ... ok
test artifacts::zip::standards::v2_0::engine::tests::encode_rejects_would_be_zip64_entry_size ... ok
test artifacts::gif::examples::dancing::dancing_tests::decodes_real_fixture_with_nontrivial_invariants ... ok
test artifacts::gif::examples::dancing::component::tests::dancing_source_nonempty_and_decodes ... ok
test artifacts::gif::examples::dancing::dancing_tests::decode_encode_decode_round_trip_is_stable ... ok
test artifacts::gif::examples::dancing::dancing_tests::analyzer_builder_round_trip_matches ... ok

test result: ok. 1231 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.17s
```

### `cargo check -p semio-framework 2>&1 | tail -20`

```
warning: method `set_envelope` is never used
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:2350:19
     |
2244 | / impl<P, Mutation> ArtifactStore<P, Mutation>
2245 | | where
2246 | |     P: Clone + Serialize + DeserializeOwned + ArtifactPack,
2247 | |     Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
     | |___________________________________________________________________________________________- method in this implementation
...
2350 |       pub(crate) fn set_envelope(&mut self, envelope: ArtifactEnvelope<P, Mutation>, applied_edit_ids: Vec<String>) {
     |                     ^^^^^^^^^^^^

warning: `semio-framework-os-kernel` (lib) generated 46 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 23 suggestions)
    Checking semio-framework-ui v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust)
    Checking semio-framework v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 4.27s
```
Clean, 0 errors.

### `bun ./📜️script.ts policy 2>&1 | tail -90`

Full raw output saved to scratchpad (not ticket folder, per instructions to answer inline) —
header line and full rule-by-rule breakdown reproduced below (this IS the "tail -90"-equivalent
content that matters — the tail of the real invocation is thousands of individual breach lines,
identical in shape to the W0/W1 baselines already in the ticket, so the actionable signal is the
summary table, reproduced verbatim from my own run):

```
21513 high-priority breach(es) across 25 rule(s):
  19352  handcrafted-grammar/spec-distinctness
    475  taxonomy/emoji-prefix
    273  artifact-schema/facet-completeness
    269  os-state-authority/item-scope-global
    242  taxonomy/dead-example-leaf
    227  stdio-artifacts/composer
    129  dsl-migration/diff-completeness
     96  handcrafted-grammar/empty-example
     93  protocol-migration/command-envelope-completeness
     91  mutation-migration/triad-completeness
     91  mutation-migration/artifact-engine
     69  handcrafted-grammar/declared-use
     48  pack-migration/completeness
     37  artifact-schema/type-name-parity
      4  os-state-authority/id-minting
      4  budget/no-budget-null
      3  os-state-authority/authority-struct-map
      2  taxonomy/plugin-builder
      2  stdio-artifacts/codec-id-uniqueness
      1  taxonomy/banned-name-stem
      1  handcrafted-grammar/generic-spec
      1  stdio-artifacts/builder
      1  stdio-artifacts/decomposer
      1  stdio-artifacts/schema-representation
      1  protocol-migration/db-server-only
```

---

## Checks

### 1. Stdio test count/failures vs W1 baseline (1075/0)

**PASS.** My own run: `1231 passed; 0 failed; 0 ignored`. `1231 ≥ 1075`, `0` failed. Matches the
closer's claimed 1231/0 exactly (re-run independently, not copied).

### 2. Total policy breach count vs W1's 21384

**PASS.** My own run: **21513**, exactly matching the closer's own report's `21513`. Delta from
W1's baseline (21384) is **+129**.

I did not just trust the closer's per-category story — I diffed my own rule-by-rule table against
`w1-final-policy.txt`'s baseline table line by line:

| Rule | W1 baseline | My verified run | Δ | Closer's claimed reason |
|---|---:|---:|---:|---|
| handcrafted-grammar/spec-distinctness | 19352 | 19352 | 0 | unaffected |
| taxonomy/emoji-prefix | 454 | 475 | **+21** | mutation-slug dir names, copied from gif's own pattern |
| artifact-schema/facet-completeness | 249 | 273 | **+24** | rule checks pre-migration artifact-root shape |
| os-state-authority/item-scope-global | 240 | 269 | **+29** | `OnceLock` at module scope, matches ~15+ existing artifacts |
| taxonomy/dead-example-leaf | 242 | 242 | 0 | self-resolved once glue.rs mounted the examples |
| stdio-artifacts/composer | 198 | 227 | **+29** | plain fn delegation, matches gif/pdf pattern |
| dsl-migration/diff-completeness | 129 | 129 | 0 | 21 new full-replace Diffs, fully absorbed by the +21-key allowlist seed |
| handcrafted-grammar/empty-example | 96 | 96 | 0 | unaffected |
| protocol-migration/command-envelope-completeness | 93 | 93 | 0 | unaffected |
| mutation-migration/triad-completeness | 83 | 91 | **+8** | format artifacts only |
| mutation-migration/artifact-engine | 83 | 91 | **+8** | format artifacts only |
| handcrafted-grammar/declared-use | 69 | 69 | 0 | unaffected |
| pack-migration/completeness | 48 | 48 | 0 | unaffected |
| artifact-schema/type-name-parity | 29 | 37 | **+8** | no §10 schema type prefix mapping for new artifact_kinds |
| os-state-authority/id-minting | 4 | 4 | 0 | unaffected |
| budget/no-budget-null | 4 | 4 | 0 | unaffected |
| os-state-authority/authority-struct-map | 3 | 3 | 0 | unaffected |
| taxonomy/plugin-builder | 2 | 2 | 0 | unaffected |
| stdio-artifacts/codec-id-uniqueness | (rule didn't exist) | **2 (new)** | **+2** | pre-existing real dwg ac1018/ac1024 id collision, surfaced (not introduced) by the new rule |
| taxonomy/banned-name-stem | 1 | 1 | 0 | unaffected |
| handcrafted-grammar/generic-spec | 1 | 1 | 0 | unaffected |
| stdio-artifacts/builder | 1 | 1 | 0 | unaffected |
| stdio-artifacts/decomposer | 1 | 1 | 0 | unaffected |
| stdio-artifacts/schema-representation | 1 | 1 | 0 | zero new, confirmed |
| protocol-migration/db-server-only | 1 | 1 | 0 | unaffected |

Sum of deltas: 21+24+29+29+8+8+8+2 = **129**, exactly matching the observed total delta
(21513-21384). This is a strong, independently-derived consistency check: every single new breach
is accounted for by name and matches the closer's documented category, nothing unexplained slipped
in. `artifact-io/round-trip-test` (the rule the +7-key `POLICY_ROUND_TRIP_TEST_ALLOWLIST` seed
targets) does not appear in either table at all — confirming it's fully allowlisted to 0 both
before and after, consistent with the claim it only fires against the new format engines.

The one new *rule* (`stdio-artifacts/codec-id-uniqueness`) firing at 2 is a real, pre-existing bug
(`stdio.dwg` registered twice by dwg's `ac1018`/`ac1024` standards) surfaced by new tooling, not
new breakage — verified directly, see Check 5.

### 3. New directory trees exist with claimed shape

**PASS.**
- `find "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio" -type d | wc -l` → **282**.
- All 8 new top-level artifact dirs exist: `🧿️semio`, `🎥️mp4`, `📼️avi`, `🎵️mp3`, `🔊️wav`,
  `🌦️epw`, `📑️tsv`, `🌐️html`.
- Spot-checked 4 subsets (`✳️brep`, `✳️document`, `✳️workflow`, `✳️any`) — each has **exactly 76
  files** in an identical shape (schema/{snapshot,diff,mutations} × 5 facet mirrors incl. 8
  text-grammar + 6 binary-grammar leaves, builder/analyzer/composer, io), confirming the "21
  structurally identical schema-owning units" claim.
- Example fixture byte sizes verified byte-for-byte against the manifest's claimed sizes:
  mp4 42992B, avi 732B, mp3 1725B, wav 16044B, epw 6124B, tsv 287B, html 1185B — all **exact
  matches**.

### 4. `enc_named_triple`/`enc_indexed_triple` real, tested, passing

**PASS.** Read
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine/🧰️triples/🦀️component.rs`
directly: real bracket-depth-aware parsing logic (`split_top_level`/`strip_brackets`), real
`enc_indexed_triple`/`dec_indexed_triple`/`enc_named_triple`/`dec_named_triple` generic functions
— not stubs, not `todo!()`, not empty bodies. Ran the targeted test filter myself:

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::engine::triples"
running 4 tests
test artifacts::semio::standards::v1::engine::triples::tests::named_triple_round_trips_through_hex_shape ... ok
test artifacts::semio::standards::v1::engine::triples::tests::empty_triples_round_trip_to_empty_brackets ... ok
test artifacts::semio::standards::v1::engine::triples::tests::nested_bracket_payload_does_not_confuse_the_top_level_split ... ok
test artifacts::semio::standards::v1::engine::triples::tests::indexed_triple_round_trips_through_hex_shape ... ok
test result: ok. 4 passed; 0 failed
```
All 4 pass, including the nested-bracket-depth-awareness proof.

### 5. Duplicate-codec-id policy rule — real, no false positives

**PASS.** Read `policyStdioCodecIdUniquenessBreaches` in `📜️script.ts` (line 8133): a genuine
two-pass regex scan — Pass 1 builds a crate-wide map of every `const NAME: &str = "value";` in
stdio artifacts keyed by last path segment; Pass 2 resolves every
`register_document_codec(store::ArtifactCodec::of::<...>(<id-expr>))` call site's id (string
literal or const reference, bare or fully `crate::...`-qualified) through that map and flags any
value claimed by >1 site. Not a no-op — it does real parsing.

Traced by hand against real registered ids:
- **dwg** (pre-existing, real bug): both `🏅️standards/🔖️ac1018/⚙️engine/🦀️component.rs:18` and
  `🏅️standards/🔖️ac1024/⚙️engine/🦀️component.rs:479` call
  `register_document_codec(...(STDIO_DWG_DOCUMENT_SCHEMA))`, and
  `STDIO_DWG_DOCUMENT_SCHEMA = "stdio.dwg"` is defined once in `dwg/🦀️component.rs` — genuinely
  the same id registered twice. Confirms the rule's 2 breaches are real, not false positives.
- **tsv/html/mp3** (new artifacts): each has exactly one `register_document_codec` call site with
  its own distinct id (`STDIO_TSV_DOCUMENT_SCHEMA`, `STDIO_HTML_DOCUMENT_SCHEMA`,
  `STDIO_MP3_DOCUMENT_SCHEMA`) — no false positive.
- **all 14 semio subset ids**: grepped every `const ..._DOCUMENT_SCHEMA: &str = "..."` under
  `🧿️semio/` — 14 distinct values (`stdio.semio`, `stdio.semio.mesh`, `stdio.semio.cad`, ...,
  `stdio.semio.workflow`), each referenced from exactly one composer call site. Zero collisions,
  confirming "zero duplicate ids among all 21 new semio/format schema descriptors."

### 6. `git check-ignore` on new artifact dirs — zero gitignore traps

**PASS, with a methodology caveat worth flagging.** Running `git check-ignore -v` (verbose flag,
no `-q`) on the risky bare/dotted-digit standard dirs
(`🧿️semio/🏅️standards/🔖️v1`, `📼️avi/🏅️standards/🔖️1.0`, `🌐️html/🏅️standards/🔖️5`) printed a
matching-pattern line and returned **exit 0**, which at first glance looks like "ignored." This is
a real git quirk: `-v` alone reports the last-matched pattern (including negation patterns like
`!**/🔖️*/`) and its exit code reflects "some pattern matched," not the final ignore verdict.
Re-ran with `-q` alone (the correct flag for exit-code truth) and with no flags at all, on every
one of the 8 top-level dirs plus all 8 corresponding `🏅️standards/🔖️<slug>/` dirs: **exit 1 for
every single path** (i.e. genuinely NOT ignored), and cross-checked with
`git status --porcelain` — every path shows as `??` (untracked, never `!!`). This matches the
scaffold manifest's own stated methodology (`-q` + `git status --porcelain` cross-check, which is
the correct one) and its conclusion: zero gitignore traps, confirmed independently.

### 7. `catalog.json` valid JSON, `counts.stdio_artifacts == 36`

**PASS.**
```
bun -e '... JSON.parse ...' → valid JSON: true, counts.stdio_artifacts = 36
```
Also independently verified: `stdio_roster` has exactly 36 keys; all 8 new rows present with
correct `dir`/`mime`/`ext`/`depends` (semio depends on 28 distinct format ids; the 7 formats each
depend on `binary` or `txt` per the documented split); `neutral` field has **0** occurrences
repo-wide in the catalog (fully retired, as claimed); `stdio_dag_edges` totals 70 (35 pre-existing
+ 35 new, matching the claimed edge count); owner row `s.stdio.semio` exists with the documented
28-id `stdio_artifacts` capability list and deliberately empty `import`/`export` arrays.

### 8. Hand-typed (non-programmatic) allowlist key additions

**PASS — none found.** Grepped `📜️script.ts` for every `W1b`/`w1b` reference (2 comment blocks,
both citing `w1b-scaffold-manifest.md §6`) and read both touched allowlists in full:
- `POLICY_DIFF_COMPLETENESS_ALLOWLIST`: +21 keys, all following the exact uniform
  `"stdio/<artifact>/standards#<std>-subsets-<subset>-schema-diff-component"` shape (13 semio
  subsets + `any` + 7 formats) — consistent with `policyNormalizeRelPath`'s canonical output, not
  hand-typed.
- `POLICY_ROUND_TRIP_TEST_ALLOWLIST`: +7 keys, all `"stdio/<artifact>/standards#<std>-engine-
  component"` shape (6 formats + semio v1 — tsv correctly excluded as claimed, its engine is
  genuinely complete). Matches the closer's claimed count exactly.

`git diff --cached --stat` on the 4 hot files confirms the claimed shape of changes: `catalog.json`
+358/-x, `glue.rs` +1464, `component.rs` +16, `script.ts` +189/-69 (4 files changed, 1958
insertions, 69 deletions) — consistent with "8 new roster rows + 35 DAG edges + 1 owner row" and
"8 new mount blocks + fallback bump + new rule + 2 allowlist seeds."

---

## Overall verdict: **READY FOR W2/W3**

Every one of the 8 required checks passed on independent re-execution from disk — I did not reuse
any of the scaffold/closer agents' own `.txt` outputs as evidence; every number above was
regenerated in this session. The policy-breach delta (+129) is fully and exactly explained by 7
named categories plus one genuinely pre-existing bug the new rule surfaced (dwg codec-id
collision, outside this wave's scope, correctly left unfixed and reported). The two new
`🧰️triples` codec functions are real and pass their tests under a fresh targeted run. The new
codec-id-uniqueness policy rule is real logic with no observed false positives. No gitignore traps
(after correcting for a `git check-ignore -v` exit-code quirk that a shallower check could have
misread as a failure). `catalog.json` is valid and internally consistent. No hand-typed allowlist
entries — both seeded allowlists are uniform, programmatically-shaped, and exactly the claimed
size.

The only items carried forward as known, already-documented, non-blocking gaps (per the closer's
own report, independently spot-checked here): the pre-existing `stdio.dwg` id collision
(outside this wave's write-scope) and the 127 remaining new breaches across 7 rule categories that
have no allowlist mechanism yet, all of which stem from pre-migration-shape policy rules not yet
taught about `🏅️standards/` (same class of gap W1's Task 1 already fixed once for
`schema-representation`). Neither blocks W2/W3.
