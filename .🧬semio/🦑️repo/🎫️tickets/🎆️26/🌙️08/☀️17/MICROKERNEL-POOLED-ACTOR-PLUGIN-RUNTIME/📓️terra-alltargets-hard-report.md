# 📓️ `alltargets-hard` report

Packet: drive `cargo check --all-targets` to 0 on `semio-framework-os-kernel` and `semio-framework`
(the two crates that had just gone `--lib`-green), then `cargo test` both. `CARGO_TARGET_DIR`
throughout: `/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-alltargets-hard`.

## Bottom line

| target | result |
|---|---|
| `semio-framework-os-kernel --lib` | **EXIT 0** — verified fresh at the very end, never left broken |
| `semio-framework-os-kernel --all-targets` | **EXIT 101, 1545 errors** (from a fresh 2746-error baseline — 44% reduction, zero regressions) — **NOT closed**, see "What's left" |
| `semio-framework --lib` | **EXIT 0** — verified fresh at the very end |
| `semio-framework --all-targets` | **EXIT 0** — closed this session (was 30 errors: 26 `#[test] async fn`, 3 `InteractionOutline` missing-`.await`, 1 unrelated) |
| `semio-framework` `cargo test` | **EXIT 0 — 160 passed / 0 failed** (first time this crate's tests have run; no prior baseline existed) |
| `semio-framework-os-kernel` `cargo test` | **still UNRUNNABLE** — `cargo test` compiles the same lib-test target `--all-targets` does, so it inherits the 1545 errors. This was already true before this packet (the prior "779 passed" baseline is stale) and remains true after. |

## What I actually did, in order

### 1. `semio-framework-os-kernel --all-targets`: 2746 → 1899 → 1879 → 1569 → 1545
Standard span-keyed `insert-await.py` / `remove-bad-await.py` rounds (the pre-existing, already-
vetted shared tools — reports `terra-hard-semio-framework-os-kernel-{apply,removebad}-r{1..5}.{txt,json}`),
alternated with hand fixes for the E0728 "residue shape 1" blockers the tool cannot cross (`.await`
inside a sync closure):
- `🗣️dsl/🦀️component.rs` (both `OpCodec::decode` impls, mirrored code): hoisted `let record_offset =
  reader.position().await as u64;` out of the `.map_err(|error| ...)` closure — the R10 §1 remedy,
  verbatim the same shape the prior `alltargets-kernel` packet already fixed at a third call site
  in this same file.
- `🎒️pack/🧪️testkit/🦀️component.rs`: `assert_dsl_pack_bidirectional`'s `parse_dsl`/`encode_pack`/
  `decode_pack` params changed `impl Fn` → `impl AsyncFn` (this codebase already establishes the
  `impl AsyncFn`/`async |x| ...` pattern in `📡️replication/🔢️scalar` and `🧮️math/🎯️sampling`); all
  call sites converted to `async |...|` closures. `decode_closure`'s inner closure (feeding
  `fuzz_truncation`/`fuzz_bit_flips`, which stay sync — shared with many other crates) bridges via
  `crate::os_io::resolve_ready`, the exact sanctioned E5 pattern already used in `📡️spr/🧪️testkit`'s
  `fuzz_truncation_never_panics_history_reader_open`.
- `📡️spr/🧪️testkit/🦀️component.rs`: `assert_policy_matrix`, `assert_chronological_determinism`,
  `assert_conflict_spr_round_trip`, `assert_channel_frame_corpus` — same `Fn`→`AsyncFn` treatment,
  all call sites (both here and in `🏪️store/🦀️component.rs`) converted to `async |...|`. One more
  `resolve_ready` bridge for `format::recover` inside `fuzz_truncation`'s decode closure.
- `🏪️store/🦀️component.rs`: fixed the `assert_conflict_spr_round_trip` `encode`/`decode` closures
  (→ `async move |...|`) and a `decode` closure resolving `LinkState` (→ `async |state| ...`, with
  `.await` added on `resolver.resolve(...)` and `DemoSnapshot::decode_pack(...)`, both genuinely async).
- **`terra-hard-freshdemostore-await-fixer.py`** (saved, R10-compliant): the SAME defect shape the
  prior packet's `terra-store-artifactstorenew-await-fixer.py` fixed for `ArtifactStore::new(`, here
  for `fresh_demo_store()` — 31 `let (mut) X = fresh_demo_store();` bindings missing `.await`, with
  193 stray `X.await.method()` uses downstream (receiver awaited, not the method result). Scoped
  per-binding by brace-depth tracking (never global name-keyed), so a same-named local in a
  different test function is never touched. `found 31 bindings, 193 stray uses; applied 224 edits`.
- `📇️directory/🦀️component.rs`: **R9 propagation**. `fold()` carries an explicit, pre-existing E1
  tag (`Iterator::fold`'s `FnMut` signature is fixed outside this repo). Its only helper,
  `upsert_member`, is pure (no I/O, no tokio/fs — verified by reading the body) and was left `async`
  by the blind codemod despite being consumed exclusively by `fold`. Made `upsert_member` sync per
  R9's one-hop-backward rule, tagged, `.await` dropped from its two call sites.
- **`Box::pin` for self/mutual recursion (R10 residue shape 3)**, found via the compiler, not
  guessed: `🗣️dsl/📖️grammar/🦀️component.rs`'s `print_symbol` ↔ `print_alternatives` (mutual) and
  `print_prim`/`print_repeat_arm` (self), plus `🖱️ui/…/🧊️wgpu/🦀️component.rs`'s `stamp_items` (self,
  tree recursion over `UiTreeItemNode`).

### 2. `semio-framework --all-targets`: 30 → 0
- `async-test-attr.py --apply` on `🧰️framework/🔨️modules/🚪️io` and `🕹️interaction` — 26
  `#[test] async fn` sites (illegal Rust; the exact error class W3's rule 1 already documented).
  The dependency `semio-framework-async-macros` was already a regular (non-dev) dependency in
  `🧰️framework/📦️packages/🦀️rust/Cargo.toml`, so no registrar action was needed despite the tool's
  informational "manifests_needing_dev_dependency" note.
- `🕹️interaction/🦀️component.rs`: `def.outline()` missing `.await` (`InteractionDefinition::outline`
  is `async fn`) — 3 E0609 errors, one-line fix.

### 3. A real "encoder writes almost nothing" bug found and fixed at scale — 372 sites, 16 files
While investigating, spotted the exact defect class this ticket has flagged twice before (14-of-17
byte-writer calls; the workflow traversal that never ran): production wire-encoding calls
(`write_bool`/`write_varint_u64`/`write_bytes`/`encode_envelope`) in `📡️spr/🧵️channel/🦀️component.rs`
(162 sites) plus 15 other files (`🗣️dsl/📖️grammar` 99, `🏪️store` 42, `🎒️pack/🔢️value` 31,
`🗣️dsl/🖋️notation` 11, `🖥️platform` 7, `🧊️wgpu` 3, `📡️spr/⌨️cli` 3, `📡️spr/🎮️command` 3, `📇️directory`
family 6, `🚪️io` 2, `💡️inference` 1, `🗣️dsl/🧬️schema` 1) — all `async fn`s called as bare statements,
producing a `warning: unused implementer of \`Future\` that must be used` (not an error — compiles
clean, does nothing). Fixed properly; see the corruption incident below for how this actually went.

## ⚠️ A severe bug in a tool I wrote — full account, because R10/rule-38-style honesty matters more than a clean narrative

My first attempt at fixing the 372 unawaited-Future warnings was a whole-file-byte-offset tool
(`terra-hard-unawaited-future-fixer.py`, still in the ticket folder, **do not reuse**): it read
`cargo check --message-format=json`'s `byte_end` (a UTF-8 **byte** offset into the whole file) and
spliced `.await` into the file at that index treating it as a Python **string** (Unicode codepoint)
index. This codebase's constant emoji region markers and doc-comment emoji mean byte offset and
codepoint offset diverge sharply after any multi-byte character — so the insertion landed at drifted
positions: sometimes inside doc comments (harmless garbage prose, compiles clean), sometimes mid-
identifier (`GKind::Text` → `GKind::T.awaitext`), sometimes inside a string literal (`` `.grammar` ``
→ `` `.gramm.awaitar` ``), sometimes as an orphan token before `{`/`}`/a fresh statement. **369
edits applied blind before I checked `--lib` and found it broken.**

Recovery, in order:
1. Derived the exact mathematical inverse of the (buggy but deterministic) insertion algorithm —
   for `N` unique raw byte-offsets processed descending, the `i`-th one's final position in the
   fully-edited string is `pos_i + 6*(N-1-i)` — and used it to verify-then-strip 278 of the 369
   insertions across 15 of 16 files with **zero ambiguity** (`terra-hard-undo-corrupted-await.py`).
   **This also correctly proved that some of the 369 "corrupted" insertions had coincidentally
   landed in the semantically right place** (`🖥️platform/🦀️component.rs`'s `add_app`/
   `set_active_app_id`/`set_panel_visibility` calls, since little emoji content preceded them in the
   file) — reversing blindly would have silently reintroduced the original no-op bug there. Caught
   by hand-checking the file after the "safe" pass and re-adding those `.await`s by hand.
2. Wrote a byte-safe **per-line** replacement (`terra-hard-safe-await-fixer.py`) using rustc's own
   `line_end`/`column_end`/captured line-`text` from the JSON (never a whole-file offset): for each
   diagnostic, compare the CURRENT line to the diagnostic's own captured snapshot; if identical,
   convert the byte column to a character offset by encoding/decoding just that one line (safe,
   since drift within a single line is far smaller than across a 3000-line file) and insert; if the
   line already diverged (leftover corruption from the first tool), recover the original via
   `difflib` before inserting. **370/370 verified clean**, applied.
3. One file (`🗣️dsl/📖️grammar/🦀️component.rs`) still had 91 leftover corrupted `.await`s that
   belonged to **no** diagnostic's recorded line at all — the original drift was severe enough that
   insertions landed on lines the JSON never named. Found via two more passes: a syntactic sweep
   (`terra-hard-final-corruption-sweep.py`, 61 sites — `.await` glued to a following identifier
   character, or starting a line whose previous line already ended a complete statement) and a
   final purely diagnostic-driven pass (`terra-hard-diagnostic-remove-bad-await.py`, 13 sites) that
   read the COMPILE ERRORS my own corruption had caused and stripped exactly the flagged token.
4. The remaining handful of real (non-corruption) errors this exposed — `E0733` recursion-needs-
   `Box::pin` (4 sites) and the `📇️directory` R9 case — are the genuine defects listed in §1/§2 above.

**Every one of these five recovery tools is saved in the ticket folder per R10's "save the recovery
tool" instruction.** The first, broken tool is also kept — with a large warning docstring — since a
sibling packet might otherwise reach for it.

**Verification after recovery**: `semio-framework --lib`/`--all-targets` both fresh EXIT 0,
`cargo test -p semio-framework` fresh EXIT 0 (160/0), `semio-framework-os-kernel --lib` fresh EXIT 0
— all pasted above from the literal last commands run this session, not reused from mid-session.

## Nothing regressed
- `os-kernel --lib`: EXIT 0 before this packet started (the prior packet's headline win) and EXIT 0
  now, re-verified fresh as the very last action.
- `semio-framework --lib`: EXIT 0 throughout (never broke, even during the corruption incident —
  the corruption's syntax errors were all in doc comments or `#[cfg(test)]` code, invisible to
  `--lib`, which is exactly why `--all-targets` is the gate that caught them and why rule 26 exists).

## What's left (for the next packet — do not re-derive)
- `os-kernel --all-targets`: **1545 errors, still `lib test` only** (both bins and their tests stay
  green). Dominant files by error-line mentions: `🏪️store` (599) › `🗣️dsl/🧬️schema` (279) ›
  `🗣️dsl` root (139) › `📡️spr/🎮️command` (113) › `📡️spr/🧪️testkit` (110) › `📡️spr/💎️materialize` (105)
  › `📡️spr/⌨️cli` (91) › `💡️inference` (91) › `🎒️pack/⌨️cli` (78) › `🌿️vcs` (63) › `🎒️pack/🔢️value` (52)
  › `📡️spr/🧵️channel` (51). `insert-await.py` is at a genuine fixpoint (`no unambiguous .await edits
  left`) — per R10 this residue is HAND work, not tool work. The dominant shape (427 `E0308` +
  hundreds of `impl Future<Output=T>` mismatches across `String`/`i32`/`bool`/`u64`/`PathBuf`/
  `&[String]`/`&ArtifactEnvelope`/`Value`) is the same "unawaited method call on a real value"
  pattern as the `fresh_demo_store`/`ArtifactStore::new` fixes above, but spread across many
  differently-named methods in many files — **no single fixer script covers it**; it needs the same
  kind of per-file investigation this report's §1 did, file by file.
- `os-kernel cargo test`: still fully blocked until the above closes — cannot be run this session.
- Once `os-kernel --all-targets` is green, re-verify `cargo test` against the historical **779
  passed** baseline **by name**, not count (rule 11) — the crate's test surface has almost certainly
  grown since that number was recorded.

## Tools added to the ticket folder
`terra-hard-guard-round.sh` (insert-await→remove-bad-await→known-corruption-revert→`--lib` recheck,
reusable for further os-kernel rounds) · `terra-hard-freshdemostore-await-fixer.py` ·
`terra-hard-unawaited-future-fixer.py` (**broken — byte offset used as char index, do not run
`--apply` again; kept only as a documented cautionary artifact**) · `terra-hard-undo-corrupted-await.py`
(exact mathematical reversal of the above) · `terra-hard-safe-await-fixer.py` (its correct byte-safe
per-line replacement — reusable) · `terra-hard-final-corruption-sweep.py` (syntactic leftover-
corruption sweep) · `terra-hard-diagnostic-remove-bad-await.py` (final diagnostic-driven cleanup).

## Files touched (hand edits, exact list)
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs` ·
`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit/🦀️component.rs` ·
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs` ·
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` ·
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️component.rs` ·
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` ·
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` ·
`🧰️framework/🔨️modules/🖥️platform/🦀️component.rs` ·
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` ·
`🧰️framework/🔨️modules/🕹️interaction/🦀️component.rs` ·
plus every file touched by the safe automated passes (span-driven, within `🔨️modules/**`):
`🎒️pack/🔢️value`, `📡️spr/🎮️command`, `📡️spr/⌨️cli`, `📇️directory/🔌️client`, `📇️directory/🪪️identity`,
`💡️inference`, `🗣️dsl/🖋️notation`, `🗣️dsl/🧬️schema`, plus the `products/os`-scoped and framework-root
`🚪️io/🦀️component.rs` (two distinct files, same module name). No file outside my owned paths
(`🧰️framework/🛍️products/💻️os/🔨️modules/**`, `🧰️framework/🔨️modules/**`) was touched. No
`lease-request` needed this session.
