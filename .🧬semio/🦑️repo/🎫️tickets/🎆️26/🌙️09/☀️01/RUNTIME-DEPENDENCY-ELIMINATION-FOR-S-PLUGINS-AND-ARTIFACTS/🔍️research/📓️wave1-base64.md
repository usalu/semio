# Wave 1 — eliminate `base64` from the seven s-plugin production crates

Scope: `process`, `lowpoly`, `cad`, `remodel`, `draw`, `raster`, `space`. Nothing else touched
(no serde/wasm-bindgen/png/image edits).

## 1. API surface found (step 1)

Grepped every `base64::`/`BASE64.` call site across the seven plugins (29 files, ~90 call
sites). All of it was **one** engine/alphabet: `base64::engine::general_purpose::STANDARD`
(RFC 4648 §4 standard alphabet, padded), both `.encode()` and `.decode()`. No `URL_SAFE`, no
`NO_PAD`, no other engine anywhere in the seven plugins. `draw` aliased it as `BASE64` via
`use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};`; the rest used
`base64::engine::general_purpose::STANDARD.encode(...)`/`.decode(...)` directly after
`use base64::Engine;`/`use base64::Engine as _;`.

## 2. Existing framework base64 — found, and it changed the plan

Before writing anything I grepped `🧰️framework` for `base64_encode`/`b64_encode`/existing
codecs, per the ticket's instruction to check `🚪️io/🦀️component.rs` and `🎒️pack/`. That search
turned up a **complete, already-correct, dependency-free RFC 4648 codec already implemented**
at `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` (the `//#region 🔤️Base64` block):
`base64_standard_encode`/`base64_standard_decode`/`Base64Error`, with its own unit tests
(RFC vectors, full-byte round trip, malformed/non-canonical rejection). I did **not** know
this existed when I started and had already hand-written a first draft of the same algorithm
before finding it — I discarded that draft rather than ship a second implementation.

**I moved this implementation, verbatim (byte-for-byte identical algorithm, only wrapped in a
generic `impl AsRef<[u8]>` front door so `&str` call sites keep compiling without adding
`.as_bytes()` everywhere), out of `📡️replication` and into a new crate.**

### Why `🚪️io/🔤️base64` and not leaving it in `📡️replication/⚙️codec`

`semio-framework-replication` (`protocol` crate) is described in its own manifest as "the
replication contract: lane-tagged client/server frames, causal mutation envelopes, the
mutation trait family, conflict vocabulary, the .spr record format" — it exists specifically
for the optimistic-replica/authority wire protocol. Seven unrelated s-plugins (a CAD editor, a
raster/paint tool, a space/workflow shell, ...) have no business depending on that vocabulary
just to base64 a PNG. Making `cad`/`draw`/`raster`/etc. depend on `semio-framework-replication`
directly would have pulled in causal/conflict/mutation code they never use, purely to reach one
codec function two levels down its dependency graph — the opposite of "no legacy/compat,
clean long-term shape."

The ticket's own text named `🚪️io` as the preferred home. `🚪️io` had no Rust package yet
(it's currently `#[path]`-included wholesale into whichever product/plugin crate wants its
dialect/dispatch vocabulary — a 2900-line file with its own `dsl` crate dependency, not
something this slice should turn into a new standalone crate). So, following the ticket's own
fallback instruction ("if `🚪️io` has no rust package yet, create one following [`🔢️hash`'s]
shape"), I created a **new, narrowly-scoped subdirectory and package**:

- `🧰️framework/🔨️modules/🚪️io/🔤️base64/🦀️component.rs` — the codec (moved from replication).
- `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🔣️rfc4648-base64-vectors.json` — new fixture.
- `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}` — the crate,
  `semio-framework-io-base64`, mirroring `🔢️hash`'s package shape exactly (module-root file +
  `📦️packages/🦀️rust` glue, registered in the root workspace `members` list).

This does **not** touch or absorb the existing giant `🚪️io/🦀️component.rs` — that file is
untouched, still `#[path]`-included the same way it always was. Only the new `🔤️base64`
subdirectory is new.

### Moved, not copied — confirmed

- **Deleted** the `//#region 🔤️Base64` ... `//#endregion 🔤️Base64` implementation block from
  `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` and replaced it with:
  ```rust
  pub use semio_framework_io_base64::{base64_standard_decode, base64_standard_encode, Base64Error};
  ```
- **Deleted** the three now-redundant `base64_standard_*` tests from that same file's
  `#[cfg(test)] mod tests` (they're superseded by the new crate's own, larger test module).
- **Added** `semio-framework-io-base64 = { path = "../../../🚪️io/🔤️base64/📦️packages/🦀️rust" }`
  to `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml`.
- **Call sites re-pointed**: none needed editing. `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
  (lines ~1011, ~1023) calls `crate::base64_standard_encode`/`crate::base64_standard_decode` —
  those names still resolve, now through the `pub use` re-export chain
  (`codec::component::{re-export}` → `pub use crate::codec::*;` at the crate root in
  `📦️glue.rs`), so `wire.rs` is byte-identical to before. I verified this by reading the file;
  I did not need to change it.

Net result: **exactly one** base64 implementation exists in the repo now (`🚪️io/🔤️base64`),
`📡️replication` is a re-exporting consumer of it like everyone else, and both `📡️replication`
and the seven plugins ultimately depend on it via a normal `path` dependency — no shim, no
duplicate algorithm.

## 3. Tests written before the swap (step 4)

Both live in `🧰️framework/🔨️modules/🚪️io/🔤️base64/🦀️component.rs`:

- `matches_rfc4648_vectors` — reads
  `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🔣️rfc4648-base64-vectors.json` (a
  language-agnostic JSON fixture, the RFC 4648 §10 table: `""`, `"f"`→`"Zg=="`,
  `"fo"`→`"Zm8="`, `"foo"`→`"Zm9v"`, `"foob"`→`"Zm9vYg=="`, `"fooba"`→`"Zm9vYmE="`,
  `"foobar"`→`"Zm9vYmFy"`) via `include_str!`, and asserts both encode and decode against it.
- `round_trips_every_byte_and_chunk_remainder` and `rejects_malformed_and_noncanonical_inputs`
  — carried over unchanged from the moved replication tests (full 0..=255 byte round trip;
  seven malformed/non-canonical-padding rejection cases).
- `matches_third_party_base64_oracle` — the differential oracle test. `base64 = "0.22.1"` is a
  **`[dev-dependencies]`-only** entry in `semio-framework-io-base64`'s manifest (confirmed:
  `[dependencies]` in that Cargo.toml is empty, `base64` only appears under
  `[dev-dependencies]` alongside `serde_json`). A tiny in-file LCG (`0x9E3779B97F4A7C15` seed,
  splitmix-style constants, no `rand` crate) generates deterministic byte strings for every
  length 0..=128 and round-trips each one through both codecs both ways, asserting byte
  equality throughout.

I additionally cross-checked the exact algorithm (transliterated line-for-line) in a throwaway
Python script against Python's stdlib `base64` module — RFC vectors, 300 random lengths for
decode, 200 for encode, and all seven malformed-input cases — before ever compiling, since
compute was contended. That was a pre-check only, not a substitute for the Rust run below.

## 4. Swap into the seven plugins (step 5)

For each of the seven `Cargo.toml`s, deleted the `base64 = "0.22*"` line and added, in the same
style as the neighbouring framework deps in that file (short local key, `package =` remap):

```toml
base64_codec = { path = "../../../../../🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust", package = "semio-framework-io-base64" }
```

Call-site rewrite (mechanical, scripted, applied to all 29 files): removed the
`use base64::Engine...;` lines, and replaced the encode/decode receiver expression only —
`base64::engine::general_purpose::STANDARD.encode(` → `base64_codec::base64_standard_encode(`,
same for `.decode(`, and `draw`'s `BASE64.encode(`/`BASE64.decode(` the same way. Arguments were
never touched, since `base64_standard_encode`/`base64_standard_decode` are generic over
`impl AsRef<[u8]>` — the exact same bound the original `Engine::encode`/`::decode` methods had
— so every existing argument expression (bare `&str` like `obj_text`, `&Vec<u8>`, array
literals, `&[u8]`) still typechecks unchanged.

Confirmed post-edit: `grep -rn '^base64 = ' ✏️s --include=Cargo.toml` → no output, repo-wide.
`grep -rn "base64::\|BASE64\." <each plugin dir>` → no output (the one remaining `base64:` hit
in `raster` is an unrelated struct field name, `ImageView { base64: ... }`).

## 5. Verification — PROVEN vs UNVERIFIED, precisely

**`cargo test -p semio-framework-io-base64` — PROVEN BY A PASSING RUN.** I ran this myself and
watched it complete (this was before the coordinator's direction to stop using isolated
`CARGO_TARGET_DIR`s; noted as a mistake below, but the run itself is real and its output is
verbatim, not fabricated). Verbatim tail:

```
   Compiling zmij v1.0.21
   Compiling serde_core v1.0.228
   Compiling serde_json v1.0.149
   Compiling itoa v1.0.18
   Compiling memchr v2.8.0
   Compiling base64 v0.22.1
   Compiling semio-framework-io-base64 v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust)
    Finished `test` profile [unoptimized] target(s) in 56.92s
     Running unittests 🦀️.rs (.../semio_framework_io_base64-809033a8b18e02ee)

running 4 tests
test component::tests::matches_rfc4648_vectors ... ok
test component::tests::round_trips_every_byte_and_chunk_remainder ... ok
test component::tests::rejects_malformed_and_noncanonical_inputs ... ok
test component::tests::matches_third_party_base64_oracle ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

Doc-tests semio_framework_io_base64
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

So: **yes**, the RFC 4648 §10 fixture test and the third-party-`base64`-oracle differential
test both exist as files and have **actually executed and passed**, not merely been written.

**The seven `cargo build --lib --target wasm32-wasip2 -p <plugin>` builds — WRITTEN BUT
UNVERIFIED, all seven.** I confirmed zero of the seven complete. What happened: the shared
`target/debug/.cargo-lock` was held for ~60 minutes by another agent's stuck/near-0%-CPU
`cargo test -p semio-s-plugin-sourcing`, so every plain `cargo` invocation against the shared
target dir queued indefinitely. I mistakenly worked around that by setting an isolated
`CARGO_TARGET_DIR` for a `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-process`
attempt — the coordinator's message explains this was itself a problem (it forces a from-scratch
rebuild of the entire wasm dependency tree, including wasmtime/cranelift/wasmparser, adding to
fleet-wide saturation). That build was still mid-compile (had reached
`semio-framework-os-kernel`, not yet touched `semio-s-plugin-process` itself, and had emitted no
errors up to that point) when I killed it myself on the coordinator's instruction to stop. I did
not see it, or any of the other six, reach `Finished` or fail. **No plugin's wasm compile is
confirmed by me.** The coordinator stated they are running verification centrally now
(repo-wide third-party count 119 → 74, base64's seven confirmed gone from the manifests) — that
is the coordinator's own check, not one I witnessed run to completion for the wasm targets.

## 6. Files touched

- New: `🧰️framework/🔨️modules/🚪️io/🔤️base64/🦀️component.rs`,
  `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🔣️rfc4648-base64-vectors.json`,
  `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/Cargo.toml`,
  `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/🦀️.rs`.
- Edited: root `Cargo.toml` (new workspace member),
  `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` (implementation removed, re-export
  added, redundant tests removed),
  `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` (new path dep).
- Edited (Cargo.toml, `base64` line → `base64_codec` path dep):
  `✏️s/🔌️plugins/{🏭️process,💠️lowpoly,📐️cad,📸️remodel,🖍️draw,🖨️raster,🪐️space}/📦️packages/🦀️rust/Cargo.toml`.
- Edited (29 `.rs` files, call-site rewrite only) across those same seven plugins — full list
  available via `grep -rl 'base64_codec::' ✏️s --include='*.rs'`.

## 7. Mistakes to flag for whoever re-verifies

I ran `cargo test`/`cargo build` with an isolated `CARGO_TARGET_DIR` several times, including
once for a wasm target, before the coordinator told me this forces full rebuilds of heavy
shared deps and worsens fleet-wide contention. I also let several `cargo test -p
semio-framework-io-base64` foreground calls exceed the tool's 600s cap and auto-background
rather than reusing/checking a single attempt, producing duplicate queued processes. The
coordinator killed these. Whoever re-runs the seven wasm builds should use the shared
`target/` dir, one plugin at a time, and should not need an isolated target dir for this slice.
