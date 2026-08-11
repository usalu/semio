# W1 (Mechanisms) Independent Verification Report

Verifier: W1 verify agent (fresh eyes, re-ran everything from disk — implementer's report not trusted
without independent reproduction). All commands below were re-run by me, from the current working
tree, not copy-pasted from the implementer's `.txt` files.

**Important caveat discovered during verification**: this repo has live concurrent sessions. Partway
through my run, an unrelated concurrent ticket (`26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`,
visible in `git status` at session start) landed a `Media`/`MediaPayload`/`MediaFingerprint`/
`MediaError`/`MediaClass`/`MediaType`/… relocation from `mesh` into `manifest` while I was verifying,
which transiently broke `semio-framework-plugin` (a downstream crate outside W1's touched files) for
about 2 minutes (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:389` still did
`pub use semio_framework::mesh::{Media, …}`), which in turn transiently broke `cargo test -p
semio-s-plugin-stdio --lib` and `cargo check -p semio-framework-os-run`. Confirmed via file mtimes
(`mesh/component.rs` 12:59:23, `manifest/component.rs` 13:00:06, framework `📦️glue.rs` 13:00:32, vs.
`plugin/component.rs`'s own fix-up landing at 13:03:39 — all timestamps after my first failed run and
before my re-run). I re-ran all 4 gate commands a second time after the concurrent session's own
fix-up landed and they now reproduce cleanly and consistently. This was NOT caused by the W1
implementer (confirmed via `git diff HEAD` — `plugin/component.rs`'s line 389 was untouched by W1,
last touched by W1 at 12:41:59 for Task 2, long before the churn). Per CLAUDE.md/memory
"Concurrent Cargo Workspace Churn" guidance: classified as foreign, not chased, re-polled instead.
All verdicts below are from the **stable, reproducible** re-run.

---

## Required exit-checklist commands (independently re-run, verbatim results)

### 1. `cargo test -p semio-s-plugin-stdio --lib`
```
test result: ok. 1075 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.70s
```
**PASS.** Exactly matches W0 baseline (1075/0) and the implementer's claim. No regression, no growth
(none was in scope this wave).

### 2. `bun ./📜️script.ts policy`
```
21384 high-priority breach(es) across 24 rule(s):
...
      1  stdio-artifacts/schema-representation
```
Full rule breakdown (verified top-of-output):
```
  19352  handcrafted-grammar/spec-distinctness
    454  taxonomy/emoji-prefix
    249  artifact-schema/facet-completeness
    242  taxonomy/dead-example-leaf
    240  os-state-authority/item-scope-global
    198  stdio-artifacts/composer
    129  dsl-migration/diff-completeness
     96  handcrafted-grammar/empty-example
     93  protocol-migration/command-envelope-completeness
     83  mutation-migration/triad-completeness
     83  mutation-migration/artifact-engine
     69  handcrafted-grammar/declared-use
     48  pack-migration/completeness
     29  artifact-schema/type-name-parity
      4  os-state-authority/id-minting
      4  budget/no-budget-null
      3  os-state-authority/authority-struct-map
      2  taxonomy/plugin-builder
      1  taxonomy/banned-name-stem
      1  handcrafted-grammar/generic-spec
      1  stdio-artifacts/builder
      1  stdio-artifacts/decomposer
      1  stdio-artifacts/schema-representation
      1  protocol-migration/db-server-only
```
**PASS.** Total 21384 vs W0 baseline 21564 — **decreased** (well within "did not increase" gate).
`schema-representation` dropped **181 → 1**, matching the implementer's claim exactly. Independently
confirmed the one remaining breach is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧬️schema` — a real,
pre-existing, unrelated unmigrated artifact missing its schema facet entirely, not a
delegating-subset false positive.

### 3. `cargo check -p semio-framework`
```
Finished `dev` profile [unoptimized] target(s) in 0.39s
```
Exit 0, 0 errors. **PASS.** Includes the `🔁️workflow` mount.

### 4. `cargo check -p semio-framework-os-run`
```
error[E0277]: the trait bound `RunArtifact: ArtifactPack` is not satisfied  (×2)
error[E0599]: the method `dispatch` exists ... but its trait bounds were not satisfied
error[E0599]: the method `snapshot_pack` exists ... but its trait bounds were not satisfied
error: could not compile `semio-framework-os-run` (bin "semio-framework-os-run") due to 4 previous errors
```
Exit 101 — **exactly 4 errors**, all four independently confirmed rooted in the single cause
`RunArtifact: ArtifactPack` not implemented (verified by reading each error's root-cause note).
Down from W0's 13. Isolated confirmation: `cargo check -p semio-framework-os-run --lib` → exit 0, 0
errors (lib target 100% clean); `--bins` alone reproduces the same 4 errors. **Matches the
implementer's claim exactly** ("13 → 9 → 0 (lib) / 4 (bin), all RunArtifact-rooted").

---

## Task-by-task verification

### Task 1 — script.ts schema-owning/delegating generalization
Read `git diff HEAD -- 📜️script.ts` directly (68 insertions / 8 deletions). Confirmed real, not
hand-waved:
- `policySchemaIsDelegatingPair` — structural check exactly as described (2 files, rs
  `pub use …::any::schema::*;`, ts literal re-export or meta-stamp-with-no-own-type).
- `policySchemaRootIsOwning` — unmigrated-always-owning + migrated-needs-📸️snapshot, exactly as
  described.
- `policyStdioSubsetKey` widening the field-sweep scope from per-standard to per-subset, exactly as
  described.
- Vocabulary reason-string widening (industry conformance profile/class/view → "...or semantic
  type", with `✳️brep`/`✳️mesh` examples) present.
- **No allowlist entries were added** — confirmed by grepping the diff for `ALLOWLIST`: the only
  hits are two reworded message strings (`standardRel`→`entry.subsetRel`), zero new `.add(...)` /
  `new Set([...])` entries. Consistent with the implementer's claim that none were needed. Check 8
  (programmatic allowlist keys) is vacuously satisfied — nothing to verify.

**PASS.**

### Task 2 — SDK `deserializer_entry_of`/`serializer_entry_of`
Read `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:490-575` directly. Both
functions are real (not stubbed): single-source guard with a proper error on 0/2+ sources, decode via
`<D::From as store::ArtifactPack>::decode_pack`, real `D::deserialize`/`S::serialize` call, re-pack
via `store::ArtifactPack::encode_pack` — same round-trip shape `composer_entry_of` already uses.
`ArtifactSerializer`/`ArtifactDeserializer` traits (lines 420-436) are real, not placeholders. Both
functions are in the export barrel (`pub use app::{...}` at line 9458).

`cargo check -p semio-framework-plugin --lib` → exit 0, 0 errors (fresh re-run). **PASS.**

Test `serializer_entry_of_and_deserializer_entry_of_erase_correctly` exists (line 7276) and reads
correctly. `cargo test -p semio-framework-plugin --lib serializer_entry_of_and_deserializer_entry_of_erase_correctly`
→ **still cannot compile** (exit 101), confirmed the exact 2 errors: `error[E0560]: struct
TutorialBase has no field named document_dsl` and `error[E0609]: no field document_json on type
semio_framework::ExampleDefinition` — both at `🔌️plugin/🦀️component.rs:2969` and `:3256`
respectively, both **confirmed pre-existing/foreign** (field renames `document_dsl`→`artifact_dsl`,
`document_json`→`artifact_json` from other concurrent work, nowhere near this wave's diff hunks).
Matches the implementer's disclosed claim exactly, same field names. **The new test's own code is
correct; the crate's test binary is blocked by disclosed, confirmed-foreign breakage — not a W1
defect.**

### Task 3 — `io_compose_via`
Read `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:333-337` directly — exactly the 4-line
`io_dispatch`-reusing implementation described, doc comment states the max-2-hops invariant.
`cargo check -p semio-framework` passes (see check 3 above), so this compiles clean.

`cargo test -p semio-framework --lib io_compose_via` → **cannot currently compile** (exit 101, 99
errors). Root-caused every error location: 100% confined to `dsl/component.rs`, `mesh/component.rs`,
and `workflow/component.rs` (0 errors in `io/component.rs` itself). This is the **disclosed** Task 5
side-effect ("mounting workflow broke `cargo test -p semio-framework --lib` crate-wide because
workflow's own `#[cfg(test)]` region references nonexistent `store::test_support::assert_op_line_
round_trip`/`assert_operation_round_trip` helpers") — count grew from the implementer's reported 39 to
99 because of the transient foreign `Media` relocation described above stacking more errors onto the
same already-broken test binary at the moment of my run; both are already disclosed as open items in
the implementer's report (item 2 for the store::test_support gap; not the transient Media issue, but
that one self-resolved and isn't W1's responsibility either way).

**Consequence: the implementer's claimed `test result: ok. 2 passed; 0 failed` for
`io_compose_via_chains_two_registered_hops`/`io_compose_via_surfaces_hub_resolve_failure` is NOT
independently reproducible right now** — it was almost certainly true when captured (their Task 3
verification ran at ~12:44-12:45, BEFORE their own Task 5 workflow-mount at ~12:46-12:56 broke the
crate's test compilation), but the current repo state (their own later change) cannot re-run it. This
is disclosed transparently in their own report under "Open item for W7/orchestrator" and Blocking
item 2 — not hidden — but it means **the io_compose_via test is currently unverifiable, not verified**.
Static reading confirms the mechanism/logic is correct; the runtime proof is currently blocked.

### Task 4 — `register_document_codec` finding
Read `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:629-632` directly:
```rust
pub fn register_document_codec(codec: ArtifactCodec) {
    let mut registry = document_codec_registry().write()...;
    registry.insert(codec.schema.clone(), codec);
}
```
Confirmed: plain `HashMap::insert`, silently overwrites on duplicate id, never panics. The
implementer's finding that the master plan's "duplicate-id panic keeps collisions loud" ground truth
is **factually wrong** is independently confirmed by direct code reading, not hand-waved. **PASS** —
this is a real, correctly-diagnosed, well-reasoned blocking finding for the orchestrator.

Test `register_document_codec_same_id_twice_overwrites_silently_not_panics` exists (line 4716) but
— same as Task 3 — currently cannot compile/run due to the same disclosed crate-wide `workflow`
test-region breakage (99 errors, same root cause, 0 errors in `store/component.rs` itself).

### Task 5 — os-run fix
Read `git diff HEAD` on all 4 claimed files directly (glue.rs ×3, run/component.rs, run/bin.rs) —
matches the report's description line-for-line:
- `run/component.rs`: `AppFrame::Emit{in_reply_to,..}`/`Draft{in_reply_to,..}` arms added exactly as
  claimed; `os_dsl::`→`dsl::` swap confirmed at both sites; the two dead self-recursive
  `artifact_pack_path`/`artifact_spr_path` duplicate definitions confirmed removed (was literally
  `fn artifact_pack_path(&self, r: &str) { self.artifact_pack_path(r) }` — infinite recursion, dead
  code); `self.operations.push`→`self.mutations.push` confirmed.
- `🔁️workflow/🦀️component.rs`: confirmed `RunArtifact` (line 1878) has **no** `impl
  store::ArtifactPack for RunArtifact` anywhere in the file, unlike `WorkflowSnapshot` (line 1020)
  which does (line 1077) — exactly the asymmetry claimed.
- Re-ran `cargo check -p semio-framework-os-run` fresh (see check 4 above): **exactly 4 errors**, all
  root-caused to `RunArtifact: ArtifactPack`, matching the report to the error.

**PASS** — Task 5's diff and its "genuinely out of scope, needs `impl ArtifactPack for RunArtifact`
in workflow's own file" conclusion are both independently confirmed correct.

---

## Summary of check verdicts

| # | Check | Verdict |
|---|---|---|
| 1 | stdio crate test ≥1075/0 | **PASS** (1075/0 exact) |
| 2 | Policy total not increased; schema-representation dropped | **PASS** (21564→21384; 181→1) |
| 3 | `cargo check -p semio-framework` clean | **PASS** (exit 0) |
| 4 | `deserializer_entry_of`/`serializer_entry_of` real, compiles | **PASS** (code real; crate compiles; test blocked by disclosed foreign breakage) |
| 5 | `io_compose_via` exists, test passes | **PARTIAL** — exists, correct, framework crate compiles; its OWN test is currently unverifiable (crate-wide test-compile break, disclosed, self-inflicted by Task 5's workflow mount, not hidden) |
| 6 | `register_document_codec` finding not hand-waved | **PASS** (independently confirmed by direct code read) |
| 7 | os-run exact error count/list | **PASS** (4 errors, all RunArtifact-rooted, independently reproduced) |
| 8 | Allowlist keys programmatically computed | **PASS (vacuous)** — no allowlist entries were added |

---

## Overall wave verdict: **READY FOR W1b, WITH ONE CARRIED-FORWARD CAVEAT**

All four required gate commands are green/as-claimed on a stable re-run. Every file-level claim in
the implementer's report was independently verified against the actual diffs and actual compiler
output, not taken on their word. The one genuine, self-inflicted regression from this wave — Task 5's
`🔁️workflow` mount makes `cargo test -p semio-framework --lib` (and therefore Task 3's and Task 4's
own new tests) uncompilable crate-wide, because `workflow`'s own `#[cfg(test)]` region depends on
`store::test_support` helpers that don't exist — was **already disclosed** by the implementer as an
open item, not hidden, and does not block W1b's scaffold work (`cargo check` — not `cargo test` — is
W1b's compile gate). It DOES mean:

- `io_compose_via`'s 2 new tests and `register_document_codec`'s new regression test are logically
  sound (read and confirmed correct) but **not currently runnable** — someone (W1b or a later closer)
  needs to either add the missing `store::test_support::assert_op_line_round_trip`/
  `assert_operation_round_trip`/`assert_dsl_pack_equivalence` helpers or gate workflow's own test
  region behind a feature flag until those exist, before these tests can produce a green
  `test result: ok` line.
- Recommend the orchestrator carry this forward explicitly to W1b/W7 as an addition to the
  implementer's own Blocking Item 3 (which only mentioned the plugin crate's foreign
  `TutorialBase`/`ExampleDefinition` block — the `semio-framework` crate's own test suite is now
  *also* blocked, for a related-but-distinct reason, since Task 5's mount).

No evidence of fabricated numbers, stubbed implementations, or hand-waved findings anywhere in the
report. Recommend proceeding to W1b.
