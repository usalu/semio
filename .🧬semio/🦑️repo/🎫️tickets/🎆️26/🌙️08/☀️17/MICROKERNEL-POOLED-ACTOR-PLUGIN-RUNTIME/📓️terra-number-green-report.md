# 📓️ terra-number-green report — `semio-framework-number` de-asyncify (packet `number-green`)

## Scope

Owned path: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔢️number/**`. Nothing outside it was
touched. `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-number`
for every build below.

## Diagnosis

`🧰️framework/🔨️modules/🔢️number/🦀️component.rs` (4290 lines) is the crate's entire implementation,
included by `📦️packages/🦀️rust/📦️glue.rs` via `#[path]`. Baseline:

```
$ CARGO_TARGET_DIR=.../target-number cargo check -p semio-framework-number --lib --message-format=short
   (602 lines of output)
error: could not compile `semio-framework-number` (lib) due to 620 previous errors
EXIT:101
```

Error-code breakdown of the 620: `E0308` 327, `E0599` 176, `E0277` 29, `E0369` 27, `E0609` 24,
`E0600` 10, `E0605` 4, `E0053` 2, `E0282` 1, plain `error:` 1 — overwhelmingly "expected `T`, found
`impl Future<Output = T>`" / "no method named `X` found for opaque type `impl Future<...>`", the
signature of a blind async-signature codemod whose call sites were never given `.await`.

**R9 verification, done before any edit:**
- `.await` count in the whole file: **0**, before the codemod's damage and after — nothing in this
  crate ever suspends.
- I/O markers (`std::fs`, `tokio`, `reqwest`, `ureq`, `File::`, `TcpStream`, `spawn`, `sleep`,
  `SystemTime`, `async_std`, `smol`) in the whole file: **0** matches for every pattern.
- `async move` / `async {}` blocks: **0** — the only use of the `async` keyword anywhere in the file
  is the 384 `async fn` signatures.
- Pre-existing `// 🚫️async: E<n>` tags: **0** — nothing had already been hand-classified.
- The crate's own doc comment (top of file) confirms the domain: "Arbitrary-precision integers and
  rationals, modular arithmetic, primality/factorization, certified interval arithmetic, the
  abstract-algebra trait hierarchy (`Ring` through `Field`) every exact numeric/symbolic consumer is
  generic over" — pure computation by design, consumed by other framework/plugin crates as a
  library, never doing its own I/O.
- Consumer side: every core type (`Natural`, `Integer`, `Rational`, `ModInt`) has E1 impls
  (`std::fmt::Display`, `std::str::FromStr`, `Ord`, `PartialOrd`, `From<...>`) that call directly into
  the arithmetic helpers, and the `Ring`/`CommutativeRing`/`IntegralDomain`/`GcdDomain`/
  `EuclideanDomain`/`Field` trait hierarchy underlies essentially every function in the file — R9's
  "E1 propagates one hop backwards" reaches the entire call graph here, not a subset of it.

Conclusion: this is the full-file case explicitly anticipated in the packet brief ("almost certainly
pure computation... expect R9 reversions to be the dominant fix"), verified rather than assumed. All
384 `async fn` are R9-eligible; none needed to stay `async` (no E2/E3/E4/E5 exception applies — no
`const fn`, no `extern`/`main`, no fn-pointer-slot values, no executor bridge).

## Fix

Wrote `terra-number-deasync.py` (this folder) — a structural, non-name-keyed, whole-token
`\basync\s+fn\b` → `fn` substitution scoped to this one owned file. Safe because:
- It edits `fn` *declarations*, not call sites — the R10 hazard (std-method-name collisions) applies
  to `.await` insertion at call sites, not to stripping `async` from a signature.
- Verified before running: 384 code-line matches, 0 comment/docstring matches, 0 string-literal
  matches (spot-checked first/last 5 and all trait-declaration matches).
- No `.await` removal needed — there were none to remove.

```
$ python3 terra-number-deasync.py --scan  <file>   → found 384 'async fn' occurrences
$ python3 terra-number-deasync.py --apply <file>   → applied. remaining 'async fn' occurrences: 0
```

`git diff HEAD --stat` on the file: **384 insertions(+), 384 deletions(-)**, line count unchanged
(4290 → 4290) — exactly the 384 `async fn` → `fn` line rewrites, nothing else touched.

## Acceptance

1. **`cargo check -p semio-framework-number --lib`**
   ```
   Checking semio-framework-number v0.1.0 (.../🔢️number/📦️packages/🦀️rust)
   Finished `dev` profile [unoptimized] target(s) in 0.27s
   EXIT:0
   ```
2. **`cargo check -p semio-framework-number --all-targets`**
   ```
   Checking semio-framework-number v0.1.0 (.../🔢️number/📦️packages/🦀️rust)
   Finished `dev` profile [unoptimized] target(s) in 0.50s
   EXIT:0
   ```
   (Includes the 97 `#[test] async fn` in the file, which the same substitution fixed — a bare
   `#[test] async fn` is otherwise a distinct compile error not visible under `--lib` alone; rule 26
   caught it as intended.)
3. **`cargo test -p semio-framework-number`** — named set, not a count:
   ```
   test result: ok. 97 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
   Doc-tests semio_framework_number: 0 passed; 0 failed; 0 ignored
   EXIT:0
   ```
4. **Payoff — `cargo check -p semio-s-plugin-note --lib`**:
   ```
   error: could not compile `semio-framework-3d` (lib) due to 296 previous errors
   EXIT:101
   ```
   No longer aborts on `semio-framework-number` — now aborts on a **different unowned crate**,
   `semio-framework-3d` (`🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️component.rs`), same async-codemod-residue
   shape (`impl Future<Output = mesh::Vec3>` missing `.x`/`.y`/`.z`/`.dot`/`.scale`/`.normalize`/etc.,
   `Result<(), MeshKernelError>` expected found future). Confirmed identical blocker for
   **`cargo check -p semio-s-plugin-stdio --lib`** (same crate, same 296-error signature, `EXIT:101`).
   Lifted to `📌️important.md` as a cross-packet finding — `semio-framework-3d` needs its own packet
   before the fleet-readiness question can be measured. **Not touched here** (outside `🔢️number`'s
   path scope, per rule 3).
5. **Rule-26 baselines, re-verified at end of packet**:
   - `cargo check -p semio-framework-os-kernel --lib` → `EXIT:0` (57 warnings, all
     `async_fn_in_trait`, R7-sanctioned — 1m25s build)
   - `cargo check -p semio-framework --lib` → `EXIT:0` (27 warnings, same class — 1m18s build)
   - Both were reported RED by an earlier same-day cross-packet finding (`terra-actor-green`, live
     peer parse-error edit in `🗣️dsl/🧬️schema/🦀️component.rs`); that edit has since landed or
     self-resolved. Both confirmed GREEN independently, not inherited from the stale note.

## Residue

None inside the owned crate — `--lib`, `--all-targets`, and `cargo test` are all clean with zero
warnings from this file (the only warnings on the os-kernel/framework runs above originate in
`🚪️io`/`📡️spr`/`🌿️vcs`/`🛂️manifest`/`🎠️kernel`, none of them `🔢️number`).

Outstanding for the program, not this packet: `semio-framework-3d` (296 errors) is now the crate
blocking the entire 63-crate fleet's first compile. Named and logged in `📌️important.md`.

## Files touched

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔢️number/🦀️component.rs` — 384 `async fn` → `fn`,
  no other changes.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-number-deasync.py`
  — new repair tool (kept in ticket folder for reuse per R10).
- `.../📌️important.md` — cross-packet finding added (number-green done, os-kernel/framework
  reconfirmed green, `semio-framework-3d` named as the new fleet blocker).
- `.../📓️terra-number-green-report.md` — this file.
