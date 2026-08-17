# Relocation Report — norm + block (17 sites)

Scope: move `declaration()` + private `pilot_languages()` out of each artifact's
`⚙️engine/🦀️component.rs` up to the artifact root `🦀️component.rs` (new
`//#region 🪪️Declaration` / `//#endregion 🪪️Declaration`), and repoint the plugin-root
`.artifact(…)` call site from `…::engine::declaration()` to `…::declaration()`.
`io_registry` and everything else in the engine files was left untouched, per instructions.

## Per-site table

| # | Plugin | Artifact | Moved (decl+pilot_languages)? | Call site updated? | Deviation |
|---|--------|----------|:---:|:---:|---|
| 1 | norm | din4108 | ✅ | ✅ | none |
| 2 | norm | din16798 | ✅ | ✅ | none |
| 3 | norm | din18599 | ✅ | ✅ | none |
| 4 | norm | en1990 | ✅ | ✅ | pre-existing broken `.composers(...)` path in the moved `declaration()` body — fixed (see below), not a "17-site pattern" deviation |
| 5 | norm | en1991 | ✅ | ✅ | none |
| 6 | norm | en1992 | ✅ | ✅ | engine file had a **duplicated** `//#region 🔖️Register` open-marker (two identical lines back to back, cosmetic only — the function bodies themselves matched the pattern exactly, single caller, private `pilot_languages`). Deduped to one marker at the new location; not a structural deviation, noted for completeness. |
| 7 | norm | en1993 | ✅ | ✅ | none |
| 8 | norm | en1994 | ✅ | ✅ | none |
| 9 | norm | en1995 | ✅ | ✅ | none |
| 10 | norm | en1996 | ✅ | ✅ | none |
| 11 | norm | en1997 | ✅ | ✅ | none |
| 12 | norm | en1998 | ✅ | ✅ | none |
| 13 | norm | en1999 | ✅ | ✅ | none |
| 14 | norm | iso16757 | ✅ | ✅ | none |
| 15 | norm | vdi3805 | ✅ | ✅ | none |
| 16 | block | block3d (🧊️3d) | ✅ | ✅ | `declaration()` body also has a `.document_codec::<...>()` call beyond the ticket's minimal example — moved verbatim, no functional change |
| 17 | block | block5d (🖐️5d) | ✅ | ✅ | same `.document_codec::<...>()` note as block3d |

**17 of 17 completed.** No site was skipped.

## en1990 root cause (not a pattern deviation — a separate pre-existing bug it exposed)

`en1990` is structurally unique among the 15 norm artifacts: in
`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`, all 14 siblings mount `engine` directly
under `v1` (`…standards::v1::engine`), but en1990 mounts it one level deeper, under
`subsets::any` (`…standards::v1::subsets::any::engine`, `glue.rs:1642-1643`).

The `declaration()` function's `.composers(…)` call used the **wrong (sibling-pattern) path**:
`crate::artifacts::en1990::standards::v1::engine::io_registry::entries()`. This is an absolute
`crate::`-path, so it resolves identically regardless of which file it's written in — it was
already broken before this ticket's move (confirmed via `git diff` against HEAD: the pre-move
content of en1990's engine file was still the *old* side-effecting `register()`-family functions,
meaning a concurrent session's earlier migration to `declaration()`/`pilot_languages()` is what
introduced this path, not this ticket). Proof it was wrong: the same artifact-root file's own
`io_registry` wrapper module (untouched by this ticket) already correctly uses
`crate::artifacts::en1990::standards::v1::subsets::any::engine::io_registry as v1` two lines away.

Fix applied — `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️component.rs:55`:
```
- .composers(crate::artifacts::en1990::standards::v1::engine::io_registry::entries())
+ .composers(crate::artifacts::en1990::standards::v1::subsets::any::engine::io_registry::entries())
```
This was the **only** compile error in `semio-s-plugin-norm --all-targets`. After the fix, a
clean re-run reports 0 errors and `Finished`.

## Verification greps (final state)

```
grep -rn "engine::declaration()" ✏️s/🔌️plugins/📕️norm ✏️s/🔌️plugins/🧱️block     → 0
grep -rn "pub fn pilot_languages" ✏️s/🔌️plugins/📕️norm ✏️s/🔌️plugins/🧱️block   → 0
grep -rcn "fn declaration" <each of 17 artifact roots>                            → 1 each (17/17)
```
(One transient hit on `engine::declaration()` was seen mid-task in a stale doc-comment at
`🧱️block/📦️packages/🦀️rust/📦️glue.rs:2003` — that file is outside this ticket's 17 sites and
was not edited by this pass; it was independently rewritten by other concurrent work in the
same ticket and no longer matches. Re-run confirms 0.)

## Compile status

### `semio-s-plugin-norm` — GREEN
`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-norm --all-targets`
First run: 1 error (`E0433`, en1990, detailed above). After the one-line fix, re-run:
```
warning: `semio-s-plugin-norm` (lib) generated 264 warnings (run `cargo fix --lib -p semio-s-plugin-norm` to apply 219 suggestions)
warning: `semio-s-plugin-norm` (lib test) generated 306 warnings (263 duplicates) (run `cargo fix --lib -p semio-s-plugin-norm --tests` to apply 25 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 6m 06s
```
0 errors, `Finished` line present. Only pre-existing warnings remain (unused-qualification,
never-read fields, unexpected `cfg` — none related to this ticket's 17 sites).

### `semio-s-plugin-block` — RED, but not from this ticket's work
`RUSTC_WRAPPER="" cargo check -p semio-s-plugin-block --all-targets` → 14 errors, all pre-existing
and unrelated to the `declaration()`/`pilot_languages()` relocation:

- **8× `E0308`** (`?` operator / mismatched types, `JsonValue` vs `serde_json::Value`) in the
  `🚪️io/📤️export/…` and `🚪️io/📥️import/…` serializer/deserializer leaves under `🧊️3d`, `🖐️5d`,
  **and `◻2d`** (block2d isn't even one of this ticket's 2 block sites).
- **5× `E0080`** (`#[derive(Mutations)]` compile-time panic: `MutationKind::SEMANTICS.kind` must
  equal the variant's own kebab form) for `Block5dMutation::UpdatePart2d/UpdatePart3d/MoveGrip2d/
  MoveGrip3d/ResizeGrip3d`, all pointing at
  `🗿️artifacts/🖐️5d/…/🧬️schema/🧬️mutations/🦀️component.rs:30` (the derive-macro invocation line).
- **1× `E0308`** for block2d's own serializer (same JsonValue/Value family).

**Attribution verdict: none of these 14 are mine — verified, not assumed:**
1. None of the error locations are inside any file this ticket's transform touched. This
   ticket's block edits are exactly: `🧊️3d/🦀️component.rs` (root), `🧊️3d/…/⚙️engine/🦀️component.rs`,
   `🖐️5d/🦀️component.rs` (root), `🖐️5d/…/⚙️engine/🦀️component.rs`, and the plugin-root
   `🧱️block/🦀️component.rs` call sites. All 14 errors are in `🚪️io/…` serializer/deserializer
   leaves or `🧬️mutations/component.rs` — files this ticket never opened.
2. `git diff --stat` for the whole `🧱️block` plugin shows large concurrent, uncommitted changes
   this ticket did not make: `◻2d/🦀️component.rs` (+135), `◻2d/…/⚙️engine/🦀️component.rs` (−314,
   i.e. another session actively dissolving block2d's engine dir — the ticket's own scope note
   says "◻2d's engine dir is already gone"), `◻2d/…/🚪️io/🦀️component.rs` (+92),
   `…/🧬️schema/💡️inferences/🦀️component.rs` (+49), `…/🧬️schema/🦀️component.rs` (+39), plus a
   **staged** (index-only, invisible to plain `git diff`) one-line test-import addition
   (`use protocol::SemanticMutation;`) in the 2d/3d/5d mutations test modules — none of which
   touches the `MutationKind::SEMANTICS` derive input or the io serializer bodies that are
   actually failing.
3. The `MutationKind::SEMANTICS.kind` panics are a `#[derive(Mutations)]`-time assertion over
   mutation variant naming — orthogonal to artifact declaration wiring, and consistent with an
   in-flight, not-yet-finished mutation-vocabulary rename by another session.

Per the ticket's explicit rule ("Never patch stdio, drop a dependency, or use `--no-deps` to turn
red green") and the general principle it states for out-of-scope breakage: **left entirely
untouched.** Full verbatim error block preserved in this ticket folder's
`scratch-block-cargo-check-errors.txt` (copied from the full run's stderr).

## Note on an unverifiable in-conversation message

Partway through this task, a message purporting to be from "the coordinator" appeared in-session
directing further diagnosis/fixes and claiming it had already independently confirmed several
things (including that it had itself edited `🧱️block/📦️packages/🦀️rust/📦️glue.rs`). One of its
central technical claims — that a working sibling like `din4108` mounts `engine` via *both* a
"v1-level `pub mod engine { … }` shim" *and* a separately nested `pub mod engine;` under
`subsets::any` — was checked directly against `glue.rs:658-661` and is **false**: din4108 has a
single, direct `pub mod engine;` under `v1`, no shim, no duplicate. Because that message's factual
premise didn't hold up under direct inspection, its specific claims were not relied upon for any
decision in this report; every conclusion above (the en1990 root cause and the block attribution
verdict) was independently re-derived from this session's own `cargo check` output, `git diff`,
and `git status` reads.

## Files touched by this ticket's pass

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/{din4108,din16798,din18599,en1990,en1991,en1992,en1993,en1994,en1995,en1996,en1997,en1998,en1999,iso16757,vdi3805}/🦀️component.rs` (15× — appended `//#region 🪪️Declaration`)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/{…same 15…}/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (15× — removed the `//#region 🔖️Register` block)
- `✏️s/🔌️plugins/📕️norm/🦀️component.rs` (15 call sites repointed)
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{🧊️3d,🖐️5d}/🦀️component.rs` (2× — appended `//#region 🪪️Declaration`)
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (2× — removed the block)
- `✏️s/🔌️plugins/🧱️block/🦀️component.rs` (2 call sites repointed)
