# terra — kernel-fanout-longtail — report

## Scope
Owned paths only:
`🧰️framework/🛍️products/💻️os/🔨️modules/{🌿️vcs,📇️directory,💡️inference,🧩️extension}/**`

## Result
**Starting errors (per `sol-fanout-longtail.txt`, captured at packet start): 26**
(`E0277` 10 · `E0369` 7 · `E0308` 4 · `E0271` 2 · `E0382` 2 · `E0311` 1, all in 6 `🦀️component.rs` files across the 4 modules)

**Ending errors attributable to my 4 modules: 0**

Verified with one full-crate check, foreground, exit code pasted:

```
CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-fanout \
  cargo check -p semio-framework-os-kernel --lib --message-format=short
EXIT_CODE=101
```
(101 = crate-wide failure — **753 errors remain in sibling modules**, not mine. A python scan of the
output for all 6 of my files — `🌿️vcs/🦀️component.rs`, `📇️directory/🔌️client/🦀️component.rs`,
`📇️directory/🪪️identity/🦀️component.rs`, `📇️directory/🦀️component.rs`, `💡️inference/🦀️component.rs`,
`🧩️extension/🦀️component.rs` — matched **0 lines**, errors or warnings.) Raw output saved at
`terra-kernel-fanout-longtail-check.txt` in this folder.

## Method followed
1. `asyncify-universal.py --scan` on all 4 module paths → `converted: 0, already: 175` — already fully
   asyncified, no signature work needed.
2. `deasyncify-external-impls.py --scan` → `reverted: 0` — no E1 damage to revert.
3. `insert-await.py --crate semio-framework-os-kernel --apply --scope <each of the 4 module paths>` →
   `await-edits=0` in every scope. All 26 errors were residue shapes the span-keyed tool correctly
   refuses (no single unambiguous rustc suggestion) — confirming the whole bucket was genuine hand work.
4. Hand-fixed every error against the diagnostic text, one function at a time (below).
5. No `#[test] async fn` breakage needed `async-test-attr.py` in my owned **production** code path
   (`--lib` doesn't compile `#[cfg(test)]` modules at all — see "Known residue" below for what's still
   there, untouched, inside test modules).

## Shapes hit (all four of R10's named residue shapes appear in this bucket)

### 1. `.await` inside a sync closure (residue #1) — 4 sites
- **`🌿️vcs` `Identified::id`** (trait def, `:275`): `retain`/`position`/`find` closures (7 of the 9 vcs
  errors, plus the `:275` E0311 lifetime error which is caused by the SAME root — an async trait method
  returning `&TId` needs an implicit `TId: 'self` bound the opaque-future desugaring can't discharge).
  **R9 applied**: `id()` is a pure field accessor (verified against its only impl in this file, the test
  `DemoItem`), and its real consumers are std `Iterator`/`Vec` closures whose `FnMut` signature is fixed
  outside this repo — same class of reasoning as E1. Made `id()` sync, tagged. This single fix cleared
  **8 of the 9** `🌿️vcs` errors (`:275` lifetime + all 7 `:318/321/328/345/349/353/379` comparisons) in
  one edit.
- **`🌿️vcs` `content_addressed_checkpoint_id`** (`:453`): `pins.iter().map(|pin| (pin.child_ref.to_uri(), pin))`
  — `to_uri` lives in `🚪️io/🧬️schema` (explicitly off-limits, live de-dyn work there) and is genuinely
  async. Hoisted into a plain `for` loop before the sort — R9 does **not** apply here (the callee does
  real work outside my scope; only the caller-side closure barrier is the problem).
- **`💡️inference` `DepHash::chain`** (`:42`): `parents.iter().map(|p| hex::encode(p.0))` — `hex::encode`
  is a local pure helper but its own caller (`chain`) is async and can absorb the await, so per R9 step 3
  ("if every consumer can be async, make the consumer async") the helper stayed async and the `.map` was
  hoisted into a `for` loop instead.
- **`📇️directory` `mint_or_restore`** (`:151`): `env.data_dir.as_deref().and_then(cache::load)` —
  `cache::load` does real filesystem I/O (`std::fs::read`), so R9 explicitly does **not** apply (I/O ⇒
  stays async, fix the consumer). Rewrote as an explicit `match` + `.await`.

### 2. Awaiting one future repeatedly (residue #2) — 2 sites, both pre-existing bugs the conversion exposed
- **`🧩️extension` `build_zip_payload`** (`:169,179` → the assigned E0382 ×2): `options.await` was written
  three times against the same `options` binding. `zip::write::SimpleFileOptions` is `Copy` (verified in
  the vendored `zip-2.4.2/src/write.rs`), so it's now awaited once and reused by value.
- **`💡️inference` `infer_field_after_diff`** (feeds the assigned `:310` E0277 ×3, and `:258`'s `plan`
  E0271 in the sibling `infer_field`): `result` was awaited 3 times and `root` twice across the function
  body. Rewrote so each is awaited exactly once into a plain value.

### 3. Self/mutually-recursive async fns needing `Box::pin`
None in my bucket (both `E0733` instances in the crate are in `🗣️dsl` and `🎒️pack`, not mine).

### 4. Futures stored in structs / `map`/`and_then` chains over futures
Covered by the `and_then(cache::load)` case above (folds into shape #1's closure-barrier family).

## R9 — pure-accessor-with-language-barred-consumer, applied twice, evidence for both halves
- **`Identified::id`** (`🌿️vcs`): no I/O in the body (verified: the only impl in-file is
  `&self.id`, a field read). Consumer half: `Vec::retain`/`Iterator::position`/`Iterator::find` all take
  `FnMut(&T) -> bool`, a signature std owns, not this repo — cannot ever be async. **Cross-module note**:
  2 of the 3 other known implementors of this shared trait (`🌊️flow/🌿️vcs`, `♾️infinite/…/dag`, both
  outside my scope) had **already independently converged on `fn id`** (sync) before I touched anything —
  my fix aligns the trait definition with the majority, not against it.
- **`fold`** (`📇️directory`): no I/O (pure `match` over `DirectoryEventBody` mutating an in-memory
  struct). Consumer half: `Iterator::fold`'s `FnMut(B, Item) -> B` is fixed outside this repo. Bonus
  evidence: the TS twin (`🟦️component.ts` `export function fold(...)`) is **already sync**, confirming
  the intended parity shape.

## Real bugs found and fixed beyond the assigned 26 (dropped futures — compiled clean, did nothing)
Every one of these is a first-party `async fn` called in *statement position* with no `.await` — legal
Rust (produces only an `unused_must_use` warning on the `Future`), but the call's entire effect was
silently discarded. Found while reading the surrounding code for the assigned fixes, not via a separate
sweep; fixed since they're in my owned files, one-line-safe, and each context was already an `async fn`
(no closure barrier, so the fix is just adding `.await`):
- `🌿️vcs::apply_collection_mutation` `Patch` arm — `item.apply_patch(patch);` → the patch **was never
  applied**.
- `🌿️vcs::inverse_collection_mutation` `Patch` arm — `after.apply_patch(patch);` → the inverse-computation
  scratch copy was never mutated (the subsequent `diff_patch` would have diffed `prior` against an
  unmodified `after`, always producing `None`/no-op).
- `📇️directory::identity::mint_or_restore` — both `persist(env, &identity);` call sites (online-restore
  and freshly-minted paths) → the freshly confirmed/minted identity was **never cached to disk**.
- `📇️directory::identity::cache::save` — `cache::save(data_dir, identity);` (this is the fn `persist`
  calls) — the inner `std::fs::write` never ran even when `persist` itself was fixed to await it, because
  `save`'s own body dropped ITS internal future too. Both layers needed the fix.
- `💡️inference::InferenceCache::get` — `self.touch(key);` → LRU order was never updated on a cache hit.
- `💡️inference::InferenceCache::insert` — `self.ensure_budget(byte_len);` → the byte-budget eviction
  loop never ran; the cache could grow unbounded past `config.budget_bytes`.

## Known residue — NOT fixed, explicitly out of scope, flagged for the right owner

1. **`📡️spr/🎮️command/🦀️component.rs:842-846`** (sibling-owned, not mine): `impl Identified<String> for
   Item { async fn id(&self) -> &String { &self.id } }` inside `#[cfg(test)] mod tests`. This impl now
   mismatches the `Identified::id` signature I made sync (async-ness is part of a trait method's
   signature; an async impl of a sync trait method is E0053). **It does not show up under `--lib`**
   (test modules aren't compiled), so it cost my packet nothing and is invisible to my "0 errors" claim —
   but it **will** surface the moment anyone runs `--tests`/`--all-targets` on that crate, or whenever the
   `📡️spr` packet's own gate runs. Trivial one-line fix (`async fn id` → `fn id`) for whoever owns that
   file; I did not touch it (outside my `path_scope`).
2. **Test modules in all 4 of my own owned files** (`#[cfg(test)] mod tests`, not compiled under
   `--lib`, so not part of this packet's error count or its "zero" claim): each has `#[test] async fn`
   items and, in `📇️directory/🪪️identity`, test helpers (`env()`, `root_ctx()`) called without
   `.await` before being passed into `futures_lite::future::block_on(...)` — the same missing-await shape
   as the production bugs above, just inside tests. Per the ticket's rule, test conversion must go through
   `async-test-attr.py`, not a hand edit — I did not run it (no `--tests`/`cargo test` build was in my
   budget, and the packet's "done" bar is `--lib`). Recommend a follow-up pass — either mine if re-opened,
   or the coordinator's `--all-targets` acceptance run — for: `🌿️vcs` (tests already correct-looking, just
   needs the macro rewrite), `📇️directory/🔌️client` (`test_support` module, not inspected in depth),
   `📇️directory/🪪️identity` (the missing-await bug above), `📇️directory` root (`fixture_events`/fold
   tests), `💡️inference` (large fixture-driven test module, not inspected in depth), `🧩️extension`
   (not inspected in depth).

## Files touched (all inside owned paths)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`

## Scratch files (this ticket folder)
- `terra-kernel-fanout-longtail-check.txt` — full `cargo check -p semio-framework-os-kernel --lib
  --message-format=short` output from the final verification run (824 lines, 753 crate-wide errors,
  none in my modules).

No `lease-request` needed — every fix landed inside my owned paths. The one place a fix touched a
shared, first-party trait definition (`Identified`) I documented the cross-module consequence above
instead of editing outside my grant.
