# 📓️ terra — packet `spine-upstream` report

## Result

3 of 4 owned crates fully green (`--lib` AND `--all-targets`, `deflate` feature included where
relevant). The 4th (`semio-framework-replication`) is **fully green within its owned scope** but
blocked at the crate boundary by two files this packet does not own. See "Lease-request" below —
this is the one thing standing between the SDK and a real number.

| crate | `--lib` | `--all-targets` | notes |
|---|---|---|---|
| `semio-framework-schema-derive` | ✅ exit 0 | ✅ exit 0 | proc-macro crate, whole-crate sync (E3) |
| `semio-framework-os-kernel-dsl-derive` | ✅ exit 0 | ✅ exit 0 | proc-macro crate, whole-crate sync (E3) — was **already green** when I started (fixed by another session between the coordinator's measurement and my read) |
| `semio-framework-mesh-engine` | ✅ exit 0 | ✅ exit 0 | `cargo test`: **20 passed / 0 failed** (verified, not just compiled) |
| `semio-framework-replication` | 🟡 0 in-scope errors, **66 errors in 2 files outside my `path_scope`** | 🟡 0 in-scope errors, same 66 | see lease-request |

**Headline number** (`cargo check -p semio-framework-plugin --lib`, pasted in full at
`terra-spine-plugin-check-final.txt`):

```
error: could not compile `semio-framework-replication` (lib) due to 66 previous errors; 20 warnings emitted
EXIT=101
```

Cargo stops at the first failing dependency (no `--keep-going`), so **`semio-framework-plugin`'s
own code has not been reached yet** — this number says nothing about the SDK itself. I also ran
`--keep-going`: nothing else in the closure fails; `semio-framework-replication` is the only
blocker between `semio-framework-plugin` and a real compile. Per the coordinator's incident
notice, the SDK file (`🔌️plugin/🦀️component.rs`) is currently in its **reverted, pre-async**
state (19 `async fn`) regardless — so even once replication is unblocked, this same command will
report the *old* baseline until `sdk-dedyn` re-runs the (now tag-aware) codemod on that file.

## Lease-request — blocks `semio-framework-replication` going green

`📡️replication`'s crate root mounts two files from **outside my owned scope** via `#[path]`:

```rust
#[path = "../../../⚠️diagnostic/🦀️component.rs"]        mod diagnostic;   // ← not mine
#[path = "../../../⚠️diagnostic/📍️span/🦀️component.rs"] mod span;        // ← not mine (0 errors currently)
#[path = "../../../🌱️value/🦀️component.rs"]              mod value;       // ← not mine
```

(`🌱️value/🦀️component.rs` itself further mounts `🌱️value/🔀️serde/🦀️component.rs`.) These are
the **only** consumers I found repo-wide (`grep -rl` over `🧰️framework` and `✏️s` for each
path string) — neither module has its own `Cargo.toml`/crate, so they are reachable exclusively
through `📡️replication`'s compile. All 66 remaining errors are in these two files:

- `⚠️diagnostic/🦀️component.rs` — **11 errors**. Shape: `Fault::new(origin, code: impl Into<FaultCode>, ...)` called with an unawaited async `FaultCode::new(...)` producing `impl Future<Output=FaultCode>`, which doesn't satisfy `Into<FaultCode>` (E0277/E0308), plus one similar `String`-typed case at line 61/153.
- `🌱️value/🦀️component.rs` — **3 errors**, `🌱️value/🔀️serde/🦀️component.rs` — **52 errors**. Shape: this is a **hand-rolled `serde::Serializer`/`serde::Deserializer` implementation** (`ValueSerializer`/`ValueDeserializer` and friends) — every trait method (`serialize_bool`, `serialize_i8`, …, `deserialize_any`, …) is `async fn`, which is **exactly E1** (`Serializer`/`Deserializer` are externally-declared serde traits; the codemod should never have touched them, matching the pattern `deasyncify-external-impls.py` already handles for `impl X for Y` blocks — I confirmed with `--scan` over my 4 crates that it finds 0 damage there, since this damage lives outside my scope). This looks like a straight run of `deasyncify-external-impls.py --apply` over `🌱️value/**` would clear the bulk of it; the remaining handful are probably the same "closure/`.map()` can't `.await`" shape I fixed 30+ instances of inside `📡️replication` (see method below).

I did **not** touch these two files. Whoever owns `🌱️value`/`⚠️diagnostic` (or the coordinator,
if unowned) should run the same method I used on `📡️replication` (below) — it's the single
remaining step between the SDK and its real error count.

## Method used on `semio-framework-replication` (~530 in-scope errors → 0)

1. `deasyncify-external-impls.py --scan` over the crate: 0 damage (E1 was not the problem here).
2. `insert-await.py --apply --scope '📡️replication'`: 0 unambiguous edits applied. The tool's
   `AWAIT_CODES`-gated walk only fires when a diagnostic carries a structured "consider awaiting"
   child suggestion; most of this crate's breakage was downstream **type-inference cascades**
   (one missing `.await` on a `usize`/`bool`-returning call silently turns a `Vec`'s element type
   into `impl Future<...>` for every subsequent `.push`, corrupting dozens of call sites at once)
   that don't carry that shape of suggestion.
3. A first attempt at a blind "add `.await` after every call to a locally-defined async fn/method"
   sweep script **mis-fired badly** — it doesn't scope-check by type, so it matched `.len()`,
   `.is_empty()`, `.contains()`, `.map()`, `.to_string()`, `Vec::new()`, `HashMap::new()`,
   `OnceLock::new()` etc. against *unrelated* first-party async methods of the same name and
   corrupted `#[error("…")]` (thiserror) attribute macros outright (`.await` landed inside the
   attribute's token stream, e.g. `#[error("bad magic").await]` — a hard parse error). I do not
   recommend reusing that script as-is; flagging it as a near-miss for whoever builds the next
   version. **1173 edits applied, then substantially reverted/repaired by hand.**
4. Recovery, in order: (a) grep+fix the 25 corrupted `#[error(...).await]` attribute sites by
   hand (these were the actual parse-error root cause hiding hundreds of cascade errors behind
   `E0425`/`E0432` "not found" noise); (b) wrote `remove_bad_await.py` (companion to
   `insert-await.py`, saved in the ticket folder) — parses `cargo check --message-format=json`,
   finds every diagnostic child suggestion literally titled "remove the `.await`" with a
   structured `suggested_replacement`, and applies it compiler-verified, to fixpoint. This alone
   removed 150+ of my own bad insertions safely. (c) Hand-fixed the remainder: `.map(closure)`/
   `.ok_or_else(closure)`/`.filter(closure)`/`Option::is_none_or(closure)` sites where the closure
   itself can't `.await` (not a rustc-suggestable shape) — rewrote each as an explicit `for`/
   `match` loop; two `recursion in an async fn requires boxing` sites (`Box::pin(...)`, the
   standard idiom, not the double-future pattern R1 bans since these are plain recursive fns, not
   trait methods); found and fixed **9 real E1 sites** missed by the scanner because they're not
   `impl X for Y` blocks — `#[serde(with = "module")]`, `#[serde(default = "fn_name")]`,
   `#[serde(skip_serializing_if = "Type::method")]` all call the named fn/module by string from
   derive-generated sync code, same contract as E1, tagged accordingly.
5. `#[test] async fn` breakage under `--all-targets` (182+20 sites, `semio-framework-replication`
   + `semio-framework-mesh-engine`): used `<ticket>/async-test-attr.py --apply` scoped to each
   crate per the coordinator's guidance (`semio-framework-async-macros`'s `#[async_test]` is now
   green and sanctioned). **Found and fixed a real pre-existing bug this surfaced**: `⚙️codec/🦀️component.rs`
   and `🚰️source/🦀️component.rs` had lost their `#[cfg(test)] mod tests { ... }` wrapper at some
   earlier point (indented test regions with no enclosing `mod`/`cfg` — braces still balanced, so
   nothing had ever caught it) — their test fns were compiling unconditionally as part of the
   plain `--lib` build. Harmless while they were bare `#[test]`, but it broke `--lib` the moment
   `async-test-attr.py` rewrote them to `#[semio_framework_async_macros::async_test]`, since dev-
   dependencies aren't linked outside the test profile. Restored both wrappers.
6. R7 (`#![allow(async_fn_in_trait)]`, not the compiler's suggested `+ Send` bound): applied to
   `semio-framework-mesh-engine` (`MeshExporter`/`MeshImporter`) and implicitly satisfied in
   `semio-framework-replication` (no first-party public trait with an async method there).
7. Re-ran with `--features deflate` explicitly (rule 22) — caught 2 more false-positive `.await`
   on `Vec::len()`/`Vec::new()` inside `#[cfg(feature = "deflate")]` bodies that the default
   feature set never compiles. Both fixed; verified `--lib`/`--all-targets` × `deflate` all green
   within scope.

## Two proc-macro crates — reasoning to reuse for every `✨️derive`/`✨️macros` crate

A proc-macro's public entry (`#[proc_macro_derive]`/`#[proc_macro]`/`#[proc_macro_attribute]`)
**cannot** be `async fn` — the signature is language-fixed to `fn(TokenStream) -> TokenStream`
and rustc rejects it outright (E3). I went further than tagging only the entry points: **the whole
crate stays sync**, on the reasoning that a proc-macro runs inside rustc at compile time, where
there is no executor to poll anything — threading `block_on` through purely-syntactic helper fns
that never do I/O would be manufacturing an E5 bridge to solve a problem that doesn't exist. Both
`semio-framework-schema-derive` and `semio-framework-os-kernel-dsl-derive` now carry this as a
crate-doc note plus `// 🚫️async: E3 proc-macro entry` on each of the 9 actual entry points (1 + 8).
**Important for whoever reads this next**: the coordinator has patched `asyncify-universal.py` to
skip fns tagged `// 🚫️async: E<n>` and any `#[proc_macro*]` entry — without those tags a later
codemod re-run would silently re-break both crates.

## Files touched (created/updated) — all within owned `path_scope`

- `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🔨️modules/📡️replication/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🚰️source/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/📖️dictionary/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/⚔️conflict/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/📐️format/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🆔️ids/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` (dev-dep on `semio-framework-async-macros`, added by `async-test-attr.py`)
- Not touched: `🧰️framework/🔨️modules/📡️replication/🔐️crypto/🦀️component.rs` (0 damage found)

Ticket-folder scratch (kept, not deleted per rule): `insert-await` report
`terra-spine-replication-await.json`; `remove_bad_await.py`, `await_sweep.py` (the mis-firing
sweep, kept as a documented near-miss) in the scratchpad — referenced above, physically at
`/private/tmp/claude-501/.../scratchpad/`, not the ticket folder itself (scratchpad, not
ticket-folder, since these are throwaway dev tools, not findings); final check transcripts saved
as `terra-spine-plugin-check-final.txt` and `terra-spine-replication-lib-final.txt` in the ticket
folder.

## Acceptance — commands run, output, exit codes

```
$ CARGO_TARGET_DIR=<scratchpad>/target-spine cargo check -p semio-framework-schema-derive --lib
    Finished `dev` profile [unoptimized] target(s) in 0.40s
EXIT=0

$ cargo check -p semio-framework-schema-derive --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.33s
EXIT=0

$ cargo check -p semio-framework-os-kernel-dsl-derive --lib
    Finished `dev` profile [unoptimized] target(s) in 0.39s
EXIT=0

$ cargo check -p semio-framework-os-kernel-dsl-derive --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.39s
EXIT=0

$ cargo check -p semio-framework-mesh-engine --lib
    Finished `dev` profile [unoptimized] target(s) in 0.75s
EXIT=0

$ cargo check -p semio-framework-mesh-engine --all-targets
    Finished `dev` profile [unoptimized] target(s) in 18.84s
EXIT=0

$ cargo test -p semio-framework-mesh-engine
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
EXIT=0

$ cargo check -p semio-framework-replication --lib
error: could not compile `semio-framework-replication` (lib) due to 66 previous errors; 20 warnings emitted
EXIT=101   (all 66 in ⚠️diagnostic/🌱️value, outside my path_scope — see lease-request)

$ cargo check -p semio-framework-replication --all-targets
same 66, all out-of-scope; 0 in-scope
EXIT=101

$ cargo check -p semio-framework-replication --lib --features deflate
same 66, all out-of-scope; 0 in-scope
EXIT=101 (deflate feature confirmed clean in-scope)

$ cargo check -p semio-framework-replication --all-targets --features deflate
same 66, all out-of-scope; 0 in-scope
EXIT=101

$ cargo check -p semio-framework-plugin --lib
error: could not compile `semio-framework-replication` (lib) due to 66 previous errors; 20 warnings emitted
EXIT=101   (full output: terra-spine-plugin-check-final.txt)

$ cargo check -p semio-framework-plugin --lib --keep-going
same single blocker (semio-framework-replication), nothing else in the closure fails
EXIT=101
```

`CARGO_TARGET_DIR` for every command above:
`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-spine`.

## Not done / explicitly out of scope

- `⚠️diagnostic/**`, `🌱️value/**` — outside `path_scope`, see lease-request above.
- Did not touch `✏️s/**`, `🧰️framework/🔨️modules/⏳️async/**`, `🚪️io/**`, `🎒️pack/**`,
  `🔀️dispatch/**`, `🏪️store/**`, guest SDK `🔌️plugin/**`, root `Cargo.toml`/`Cargo.lock` — all
  per the "NOT yours" list.
- Did not run `git`-modifying commands.
