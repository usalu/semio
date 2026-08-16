# Coordinator repairs — cross-language parity tests that were never actually running

Found while clearing the "4 pre-existing environmental failures" the W2 barrier kept excusing. None of these were this ticket's code; all were silently-dead parity checks that a directory restructure had disconnected, and each one hid a real defect behind a path error.

## 1. The wire-fixture test pointed at a directory that no longer exists

`💻️os/🟦️backbone-worker.ts` loaded its Rust-generated binary fixtures from
`../../🔨️modules/🏪️store/🔄️sync/📦️packages/🦀️rust/🧫️fixtures/📡️wire` — a pre-restructure path. The fixtures are
actually written by `wire_fixtures_stay_byte_identical_across_rust_and_ts` (`🏪️store/🔄️sync/🦀️component.rs:2357`)
to `CARGO_MANIFEST_DIR/../fixtures/wire`, i.e. `💻️os/📦️packages/fixtures/wire/`, where all 18 `.bin` files sit.

The test had been failing with `ENOENT` — so a cross-language **byte-identity** guard was asserting nothing at all.

## 2. Fixing the path exposed a genuine codec gap

With real bytes flowing, the test failed on `JSON.parse` of the presence blob. The TS assertion still treated the
peer payload as "arbitrary opaque test content" — but the Rust fixture had since been upgraded to a real
`encode_presence_peer` blob, and its own doc comment says why:

> A `PresencePeer` whose `interaction` carries THREE domains … `📦️client-presence.bin`/`📦️server-presence.bin`
> regenerate off this **so the TS vitest twin exercises bit 7** with a realistic multi-domain payload.

The TS twin could not exercise bit 7: `encodePresencePeer`/`decodePresencePeer` in `💻️os/🟦️component.ts` handled
presence bits 0-6 and stopped at `dragGhostJson`. Rust sets `presence |= 1 << 7` and writes an interaction section
(app id, varint domain count, then per domain: domain, granularity, selected[], hovered[]).

**Fixed** by implementing the missing half of the codec: `ArtifactPresenceInteraction`/`ArtifactPresenceDomain`
types, the bit-7 branch in both the encoder and the decoder, and a `readPresenceInteraction` twin of Rust's
`decode_presence_interaction`. The test now asserts the scalar fields, all three interaction domains, and a
**byte-for-byte re-encode** of the real Rust blob — the assertion the fixture was built for.

## 3. The wasm workflow-parity test pointed at two more dead paths

`💻️os/🟦️component.ts`'s `matches the Rust plan_workflow across shared fixtures decoded via wasm` resolved
`fixturesDir` to `🧰️framework/🧫️fixtures` and `rsPkgDir` to `🧰️framework/🛍️products/🦀️rust/pkg` — neither exists.
Corrected to `💻️os/🧫️fixtures` and `💻️os/📦️packages/🦀️rust/pkg`.

The module name `semio_framework_os.js` looked stale too, but was not: it names the crate
`semio-framework-os` (the `🖥️host` package), NOT the neighbouring `semio-framework-os-kernel`. Only the
directory was wrong. `wasm_exports` lives in `🖥️host/🦀️component.rs` behind that crate's `os-host-full`
feature, and `🖥️host/📦️packages/🦀️rust` already declares a proper `wasm` target (`wasmBaseName:
"semio_framework_os"`) — so the fix is `rsPkgDir → 💻️os/🖥️host/📦️packages/🦀️rust/pkg`, built with the repo's
own `bun ./📜️script.ts wasm`, not an ad-hoc `wasm-pack` invocation against the wrong crate.

Running that target (`bun ./📜️script.ts wasm` in `💻️os/🖥️host/📦️packages/🦀️rust`) then fails to compile
`getrandom` for `wasm32-unknown-unknown` — a pre-existing dependency/build-configuration problem in the repo's
own wasm target, untouched by and unrelated to this ticket. So the workflow-parity test now resolves to the
correct module and will run as soon as that target builds; it is NOT claimed as passing here.

**Correction worth recording**: my first attempt built the wrong crate (`semio-framework-os-kernel`) with an
ad-hoc `wasm-pack` call and renamed the test's imports to match its output. That was wrong on both counts — the
original names were correct for `semio-framework-os` — and it is reverted; the stray `pkg/` it produced under the
kernel crate was deleted.

## Result

`bunx vitest run` in `💻️os/📦️packages/🟦️typescript`: **4 failed → 2 failed**, and the two remaining are one test
blocked on a broken repo-wide wasm build target rather than on a path bug. The presence codec is now at parity in
both directions, and the wire byte-identity guard actually guards something for the first time since the
restructure.
