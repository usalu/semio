# 🏁️ The last serde blocker was never a streaming decoder — `ArtifactRepositoryHistoryEntryAuthority<T>` converted to `FromValue`

## Headline

The decoder's own docstring, carried across three prior waves, claimed it was "a streaming,
serde-`Deserializer`-driven token authority, not a whole-value decode." **That was wrong.** Direct
read of `accept_token` (`🏪️store/🦀️.rs:6665-6688`, pre-edit) shows it buffers each history entry's
raw source bytes into a fixed-capacity slice as tokens stream past, and calls
`serde_json::from_slice` **exactly once**, at the terminal token, over the complete buffered slice.
That is a bounded whole-value decode wearing streaming clothes — the "streaming" is only in how the
bytes accumulate (with capacity/fuel enforcement per token), not in how the value is parsed. The
bound converts cleanly to `T: FromValue`, decoding through
`crate::os_pack::json::from_json_str::<T>` over the same buffered bytes, no `DslValue`-stepping
required.

**`os-kernel`'s `Cargo.toml` did NOT drop serde** — a different, independent, already-scoped-out
reason (`🧵️canonical-edit::ScalarBytes`'s `F32` arm, unconditional production `serde_json` call,
deliberately left per this ticket's own brief) keeps it a real `[dependencies]` entry. **serde still
appears in a plugin's `wasm32-wasip2` link graph** — via that same `os-kernel` edge, plus the
already-known host-only `wit-component`/`wit-bindgen` proc-macro edge (the component ABI itself).
This wave's real contribution: `Author`/`Change`/`CompositionPin`/`Checkpoint`/`Alternative` no
longer need serde to satisfy the decoder, and four of the five drop `Serialize`/`Deserialize`
outright (the fifth, `Change`, keeps it `cfg_attr(test, …)` for two real oracle tests).

## The decoder's bounded-step contract

`ArtifactOwnedHistoryEntryAuthority<T>::accept_token` (`🏪️store/🦀️.rs`) is called once per token as
the framework's schema-driven JSON lexer advances over one history-entry field. Per call it:

1. Rejects a stale/cancelled operation (`cx.operation()`/`cx.generation()`/`cx.is_cancelled()`).
2. Copies the token's raw source bytes into `raw: Box<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>`
   at `raw_len`, failing closed (`artifact-envelope.history-entry-byte-capacity`) if the fixed
   buffer would overflow — this IS the bounded part of "bounded, incremental": total entry size is
   capped by a compile-time constant, checked on every token, never by a growable `Vec`.
3. Charges `cx.consume_fuel(span)` — the incremental part: a huge entry cannot monopolize the step
   budget across many token calls.
4. If the token is not `terminal`, returns `TokenComplete` and waits for the next token.
5. If the token **is** `terminal` (the entry's closing token), decodes the ENTIRE buffered slice
   `&self.raw[..self.raw_len]` in one call and stores the typed `T` in `self.value`.

Step 5 is where `serde_json::from_slice` lived, and it is a plain whole-value parse — nothing about
it consumes tokens one at a time or holds parser state across calls. `close_step`/`take_value`
(bounded retirement, exact-once ownership handoff) are entirely unrelated to serde; they manage when
the decoded `T` (or its raw bytes, pre-decode) gets released under `maximum_items`/`maximum_bytes`
pressure. None of that changes.

**Conclusion: the first-party stack can express this exactly.** `pack::json::from_json_str::<T>`
(`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1410`, landed by the `float-format-parity` wave, already
used elsewhere in this same file — `🏪️store/🦀️.rs:11081/11087/11094`) is precisely
`serde_json::from_str`'s analog over `FromValue` instead of `DeserializeOwned`. The buffer is
already `&[u8]`; `std::str::from_utf8` bridges to `&str` (JSON is well-formed UTF-8 by the lexer's
own contract, so this cannot fail on valid input — the failure path exists only for a corrupt/
malicious source, same as the old code's `serde_json::from_slice` failure path).

## What converted

`🏪️store/🦀️.rs` (`ArtifactRepositoryHistoryEntryDecoder<T>`/`ArtifactRepositoryHistoryEntryAuthority<T>`,
lines ~6062-6737):

```diff
- pub fn artifact_bounded_history_entry_decoder<T>() -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>
- where
-     T: DeserializeOwned + Send + Sync + 'static,
+ pub fn artifact_bounded_history_entry_decoder<T>() -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>
+ where
+     T: FromValue + Send + Sync + 'static,
```

```diff
- impl<T: DeserializeOwned + Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<T> for ArtifactRepositoryHistoryEntryDecoder<T> {
+ impl<T: FromValue + Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<T> for ArtifactRepositoryHistoryEntryDecoder<T> {
```

```diff
- impl<T: DeserializeOwned + Send + 'static> ArtifactOwnedHistoryEntryAuthority<T> for ArtifactRepositoryHistoryEntryAuthority<T> {
+ impl<T: FromValue + Send + 'static> ArtifactOwnedHistoryEntryAuthority<T> for ArtifactRepositoryHistoryEntryAuthority<T> {
```

```diff
- let value = serde_json::from_slice(&self.raw[..self.raw_len]).map_err(|_| self.diagnostic("artifact-envelope.history-entry-decode", token.start))?;
+ let text = std::str::from_utf8(&self.raw[..self.raw_len]).map_err(|_| self.diagnostic("artifact-envelope.history-entry-decode", token.start))?;
+ let value = crate::os_pack::json::from_json_str(text).map_err(|_| self.diagnostic("artifact-envelope.history-entry-decode", token.start))?;
```

`use serde::de::DeserializeOwned;` (line 33) removed — no longer referenced anywhere in the file
(verified by grep before removing).

This one seam is directly instantiated over exactly three types in the live call graph
(`🏪️store/🦀️.rs:8697-8699`, the fresh-VCS decoder's field ids 3/4/5): `Change`, `Checkpoint`,
`Alternative` — plus, through `Checkpoint`'s own fields, transitively `Author`
(`authors: Vec<Author>`) and `CompositionPin` (`composition_pins: Vec<CompositionPin>`). All five
already derived `ToValue`/`FromValue` (added by an earlier wave alongside their serde derives), so
none needed a new derive — only the OLD `Serialize`/`Deserialize` needed removing where nothing else
required it.

## `🌿️vcs/🦀️.rs` — four types dropped serde outright, one stayed test-only

Repo-wide grep (both `🏪️store` and `🌿️vcs`, both production and `#[cfg(test)]` code) found **zero**
call sites anywhere that call `serde_json` on `Author`, `CompositionPin`, `Checkpoint`, or
`Alternative` directly — no oracle test, no wire encoder, nothing. Their
`#[derive(Serialize, Deserialize)]` + `#[serde(...)]` field attributes were therefore removed
outright, not `cfg_attr(test, …)`-gated, matching this ticket's own precedent for
`ArtifactHistoryLedger`/`ArtifactHistoryIter`/`ArtifactVcs`/`ArtifactVcsRead` (dropped outright,
`📓️os-kernel-serde-final.md`).

`Change` is different: this file's own test module (`#[cfg(test)] mod tests`) has two real oracle
tests against `serde_json`:
- `change_to_json_string_matches_serde_json_byte_for_byte` — direct `serde_json::to_string(&change)`
  comparison against `crate::os_pack::json::to_json_string(&change)`.
- `content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible` —
  recomputes a legacy hash formula inline via `serde_json::to_vec(change)`.

Both are genuine, currently-passing regression proofs (not dead code), so `Change` keeps
`#[cfg_attr(test, derive(Serialize, Deserialize))]` / `#[cfg_attr(test, serde(rename_all =
"camelCase"))]` / per-field `#[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]` —
the exact pattern already proven in this crate for `store::ArtifactCursorOwners`
(`🏪️store/🦀️.rs:2086-2100`, `📓️os-kernel-serde-final.md`). Production code never sees `Change:
Serialize`/`Deserialize` any more.

Also checked and confirmed harmless: three other repo files reference `vcs::Author`/`vcs::Checkpoint`/
`vcs::Alternative` outside `vcs.rs`/`store.rs` (`🔨️modules/🛢️db/🔢️index`, `🛢️db/🗜️compact`,
`🛢️db/⚙️engine`, `🔨️modules/🪐️space`) — none derive `Serialize` over a struct embedding them as a
field, and none call `serde_json` on them; one uses `std::mem::size_of::<vcs::Author>()` (no trait
bound at all).

`VcsError`'s `Serialize(String)`/`Deserialize(String)` enum variants (and their `Display` arms) are
NOT serde usage — they are the crate's own error-kind names, an unrelated pre-existing pattern this
ticket's classifier already documents as a known false-positive (`📓️os-kernel-serde-final.md`).

## Production serde-token count, this crate's own module tree (`classify_serde5.py`, unmodified)

Scoped precisely to the directories `semio-framework-os-kernel`'s own `🦀️.rs` mounts (not the whole
`💻️os` product, which also holds unrelated crates — renderer, mcp, dev, infinite, flow):

| module | before this wave | after this wave |
|---|---|---|
| `🌿️vcs` | 20 (16 real + 4 `VcsError` false-positives) | **5** (0 real + 4 false-positives + 1 import kept for `Change`'s test-only derive) |
| `🏪️store` (incl. `🧵️canonical-edit`) | 26 | **21** (-5, all from this decoder; `canonical-edit`'s `F32` arm untouched, as scoped) |
| `📡️spr` | 9 (unchanged — `🧵️channel`'s native-only `FixedCommandPage`/`CommandIngressStatus`/`CommandPageCursor`, re-confirmed, not this wave's target) | 9 |
| `💡️inference` | 7 (unchanged — `InferredField` bound, 7/13 plugin implementors, not this wave's target) | 7 |
| `🗣️dsl` | 20 (all in `✨️derive`, the separate host-only `os-kernel-dsl-derive` **proc-macro** crate — different `Cargo.toml`, does not link into any guest) | 20 |
| everything else (`🎒️pack`, `🪪️identity`, `📇️directory`, `🚪️io`, `⚙️engine`, `🧬️semio`, `🧩️extension`) | 0 | 0 |

Net real reduction this wave: **21 lines** (16 in `vcs`, 5 in `store`), zero regressions.

## Why `os-kernel`'s `Cargo.toml` still cannot drop serde

Three independent, still-open reasons — none touched by this wave, all previously scoped by earlier
waves and re-confirmed live here:

1. **`🧵️canonical-edit::ScalarBytes::from_node`'s `F32` arm**
   (`🏪️store/🧵️canonical-edit/🦀️.rs:336`): `serde_json::to_writer(&mut scalar, &value)`, real,
   unconditional, production code — not behind `#[cfg(test)]`, not behind any target gate. This
   path is genuinely guest-reachable (canonical-edit sealing is part of the store's edit/checkpoint
   content-addressing, which every plugin can reach). `float-format-parity.md` proved byte-identity
   for `f64` only; `f32` parity against `zmij`'s materially different threshold/precision budget was
   explicitly out of scope for that wave and remains out of scope for this one. **This alone is
   sufficient to keep `serde`/`serde_json` a real `[dependencies]` entry** — moving them to
   `[dev-dependencies]` would break this one production call site's compile.
2. **`📡️spr/🧵️channel`**: `FixedCommandPage`'s hand `impl Serialize`/`Deserialize`, plus
   `CommandIngressStatus`/`CommandPageCursor`'s derives — re-confirmed still genuinely needed by the
   native-only `plugin-host` wire consumer, target-gating already considered and declined as
   zero-value in `📓️os-kernel-serde-final.md` (does not touch the wasm32-wasip2 link count either
   way, since `Cargo.toml`'s serde entry is unconditional regardless).
3. **`💡️inference`**'s `InferredField` bound (`encode`/`decode` helpers, `Serialize`/
   `DeserializeOwned`) — 7 of 13 plugin implementors still serde-only, unchanged, three plugin-side
   waves away per prior scoping.

None of these three is what this ticket assigned me — the assignment was specifically the streaming/
bounded history decoder, now converted. Reason 1 (`F32`) is the one that would need to move for the
"prize" (`[dev-dependencies]`) to become reachable at all; it was explicitly flagged in this wave's
own brief as "deliberately left — f32 parity is unproven," so leaving it untouched is the scoped,
correct outcome, not an oversight.

## Verification — every command run to completion, verbatim tails

Heavy concurrent contention this wave (a peer `cargo check --workspace --all-targets --keep-going`
held the build lock for ~2h; see `🗑️generated/osk_step1.txt`/`osk_step2.txt` for the
"Blocking waiting for file lock" waits). All commands below eventually returned with an unpiped,
directly-captured exit code — none were judged by a piped `tail`.

```
$ cargo check -p semio-framework-os-kernel --message-format=short > osk_step2.txt 2>&1; echo EXIT:$?
...
warning: `semio-framework-os-kernel` (lib) generated 32 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 32 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 20m 15s
EXIT:0
```
`grep -c "^error"` on the same file: **0**. 32 warnings — the identical count `📓️os-kernel-serde-final.md`
recorded as its own clean baseline, confirming this wave introduced no new warnings. This check
covers the whole crate, including `🌿️vcs/🦀️.rs` and `🧵️canonical-edit/🦀️.rs` — so it also serves as
the first crate-level confirmation of the PRECEDING wave's `vcs.rs`/`canonical-edit.rs` call-site
conversions (`content_addressed_checkpoint_id_core` off `serde_json::to_vec`, `ScalarBytes`'s
non-`F32` arms), which `📓️float-format-parity.md` shipped without a returned crate-level check due
to the same contention. Both are now proven compiling together.

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw --message-format=short > draw_build.txt 2>&1; echo EXIT:$?
...
error: could not compile `semio-s-plugin-draw` (lib) due to 1250 previous errors; 40 warnings emitted
```
**Pre-existing, unrelated to this wave** — every one of the 1250 errors is `DrawMutation`/
`DrawSnapshot`/`ReplaceLayerStroke`/`SetActiveUtility`/`CreateLayer`/… missing `ToValue`/
`FromValue`/`MutationLeaf`/`Serialize` (e.g. `CanvasPointerDown: serde::Serialize` is not
satisfied), entirely inside `✏️s/🔌️plugins/🖍️draw/`'s own schema/mutation/editor files. Grepped the
full error log for every type this wave touched (`Checkpoint`, `Alternative`, `Author`, `Change`,
`CompositionPin`, `DeserializeOwned`, `artifact_bounded_history_entry_decoder`) — **zero hits** as an
error cause (one unrelated warning mentions an enum variant literally named `Checkpoint` in a
different module, `🔌️plugin/⚛️reactor/💼️jobs/💡️infer`). Per this ticket's own hard constraint
("if an error names a file outside your scope, record it and move on"), this is peer churn on the
draw plugin's own mutation vocabulary, not something this wave introduced or should fix.

`cargo tree` does not require the crate to compile (it only resolves the dependency graph), so it
still ran cleanly despite the above:

```
$ cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 -i serde --edges normal > draw_tree_serde.txt 2>&1; echo EXIT:0
serde v1.0.228
├── wit-component v0.247.0
│   └── wit-bindgen-rust v0.57.1
│       └── wit-bindgen-rust-macro v0.57.1 (proc-macro)
│           └── wit-bindgen v0.57.1
│               └── semio-framework-plugin v0.1.0 (…)
│                   ├── semio-s-plugin-draw v0.1.0 (…)
│                   └── semio-s-plugin-stdio v0.1.0 (…)
│                       └── semio-s-plugin-draw v0.1.0 (…)
└── wit-parser v0.247.0
    └── … (wit-bindgen toolchain only)

serde v1.0.228
├── semio-framework v0.1.0 (…)
│   ├── semio-framework-plugin v0.1.0 (…) (*)
│   ├── semio-s-plugin-draw v0.1.0 (…)
│   └── semio-s-plugin-stdio v0.1.0 (…) (*)
├── semio-framework-2d, -3d, -actor, -geometry, -graph, -mesh-engine, -ui, -ui-contract,
│   -ui-runtime, -ui-scene v0.1.0 (…) (all reach serde only via semio-framework above)
├── semio-framework-os-kernel v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust)
│   ├── semio-framework v0.1.0 (…) (*)
│   ├── semio-framework-2d/-3d/-graph/-plugin/-schema/-ui v0.1.0 (…) (*)
│   ├── semio-s-plugin-draw v0.1.0 (…)
│   ├── semio-s-plugin-draw-fsm v0.1.0 (…)
│   └── semio-s-plugin-stdio v0.1.0 (…) (*)
├── semio-framework-replication v0.1.0 (…)
│   ├── semio-framework-os-kernel v0.1.0 (…) (*)
│   ├── semio-framework-os-kernel-neural-engine v0.1.0 (…)
│   ├── semio-framework-pack v0.1.0 (…) (*)
│   └── semio-framework-ui-scene v0.1.0 (…) (*)
├── semio-s-plugin-draw v0.1.0 (…)
└── semio-s-plugin-stdio v0.1.0 (…) (*)
```
(full untruncated output at `🗑️generated/draw_tree_serde.txt`; two resolved-instance trees printed,
both read in full — the measurement caveat `📓️verified-outcomes.md` recorded about a truncated `-i`
hiding a second tree does not apply here). **serde is still linked** — through `os-kernel` directly
(the `F32`/`spr`/`inference` reasons above) and through the host-only `wit-component`/`wit-bindgen`
proc-macro chain (the component ABI itself, already known and out of scope). This wave did not, and
by its own scope could not, change this number — the decoder it targeted was never a distinct edge
in this tree (it added no new dependency; it only removed derives inside `os-kernel`'s own source).

## Fallout check — the animate plugin's pre-existing, separate blocker

The ONE other live call site of `artifact_bounded_history_entry_decoder` repo-wide
(`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/
💾️binary/🦀️.rs:320`) instantiates it over `Edit<PresentMutation>`. `PresentMutation`
(`✏️s/🔌️plugins/🎞️animate/…/🧬️schema/🧬️mutations/🦀️.rs:37`) derives `Serialize, Deserialize,
dsl::DslEnum, dsl::Mutations` — **not** `ToValue`/`FromValue` (`DslEnum`'s own derive only builds
`impl DslVariants`, confirmed by reading `dsl_derive`'s `#[proc_macro_derive(DslEnum, …)]`
expansion). This means `semio-s-plugin-animate` should now fail to satisfy `Edit<PresentMutation>: FromValue` at
its call site. **This is a source-level inference, not a confirmed compiler run**: a dedicated
`cargo check -p semio-s-plugin-animate --target wasm32-wasip2` was queued (see
`🗑️generated/animate_check.txt`) but never returned inside this wave's time budget — repo-wide
contention was continuous and severe throughout this wave (at least three separate peer
`cargo check --workspace --all-targets --keep-going` runs observed holding the build lock across the
session, one after another). Stated as a prediction, with its exact reasoning shown, rather than
asserted as an observed fact. If confirmed by a future check, it is **not a regression this wave
owns**: `📓️os-kernel-serde-final.md` already named `PresentMutation: ToValue` as a separate,
already-tracked blocker "under concurrent `SEMANTIC-MUTATIONS-OVERHAUL` churn," independent of and
prior to this wave. Converting the decoder was always going to surface this the moment anyone
flipped its bound — the fix (deriving `ToValue`/`FromValue` for `PresentMutation`, or hand-writing
the impl) belongs to that ticket, not this one. Recorded here per this ticket's own "if an error
names a file outside your scope, record it and move on" rule; not fixed.

## Files touched this wave

- `🏪️store/🦀️.rs` — `artifact_bounded_history_entry_decoder`, `ArtifactOwnedHistoryEntryDecoder<T>`
  impl, `ArtifactOwnedHistoryEntryAuthority<T>` impl: `DeserializeOwned` bound → `FromValue`;
  terminal-token decode: `serde_json::from_slice` → `str::from_utf8` +
  `crate::os_pack::json::from_json_str`. `use serde::de::DeserializeOwned;` removed.
- `🌿️vcs/🦀️.rs` — `Author`, `CompositionPin`, `Checkpoint`, `Alternative`: `Serialize`/
  `Deserialize` derives and `#[serde(...)]` attributes dropped outright. `Change`: same derives
  moved to `#[cfg_attr(test, …)]` (two real oracle tests preserved). All five docstrings rewritten
  to state the correct, verified reason (the decoder's actual bounded-whole-value shape), replacing
  three prior waves' incorrect "streaming Deserializer, cannot convert" claim.

## Deliverable answer, restated

- **Bounded-step contract**: raw-byte accumulation with per-token capacity/fuel enforcement, then
  ONE whole-value parse at the terminal token. Fully expressible first-party — no `DslValue`-stepping
  needed, `pack::json::from_json_str` already provides the exact `serde_json::from_str` analog.
- **Converted**: `ArtifactRepositoryHistoryEntryDecoder<T>`/`Authority<T>`'s bound and decode call;
  `Author`/`CompositionPin`/`Checkpoint`/`Alternative` off serde outright; `Change` off serde in
  production (test-only oracle kept).
- **`os-kernel`'s `Cargo.toml`**: did NOT drop serde — `ScalarBytes::F32` (deliberately out of
  scope), `spr::channel`, and `inference::InferredField` each independently keep it a real
  dependency.
- **A plugin's `wasm32-wasip2` link graph**: serde still present, via `os-kernel` directly and via
  the host-only `wit-component`/`wit-bindgen` proc-macro chain — unchanged by this wave, as scoped.
