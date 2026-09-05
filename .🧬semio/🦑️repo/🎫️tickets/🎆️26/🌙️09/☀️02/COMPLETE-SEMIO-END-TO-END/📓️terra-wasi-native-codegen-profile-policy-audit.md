# WASI-Only Codegen Profile Policy Audit

Status: read-only current-source audit, 2026-09-04. No build was started, so this is a policy and wiring recommendation, not timing or runtime proof.

## Verdict

**RED — the current workaround is target-blind.** The nine
`[profile.dev.package.*] codegen-units = 1` sections in root
[`Cargo.toml`](/Users/ueli/Documents/semio/Cargo.toml:254) select the normal
`dev` profile, not `wasm32-wasip2`. They therefore serialize native development
codegen for `semio-s-plugin-stdio` as well as WASI linking. That directly
explains why a native stdio gate can spend 15–20 minutes in LLVM/codegen even
though the reported `rust-lld::wasm::ElemSection::writeBody` crash is specific
to a WASI component link.

The clean correction is a dedicated **`wasm-dev` Cargo profile**, selected only
by component-producing WASI commands. It retains the one-CGU linker mitigation
but restores ordinary, parallel native `dev` compilation. It must be an atomic
profile-and-resolver change; merely deleting the root overrides would restore
native speed but re-open the WASI crash in the descriptor and dev component
paths.

## Current evidence

| Boundary | Current source | Consequence |
| --- | --- | --- |
| Target-blind override | [`Cargo.toml:260-294`](/Users/ueli/Documents/semio/Cargo.toml:260) sets `codegen-units = 1` for `semio-framework-os-kernel`, `semio-framework-math`, stdio, cad, gis, procedural, process, puzzle and sourcing under `profile.dev.package`. | The host/native build of each named package uses one CGU too. There is no target selector in those manifest sections. |
| Native baseline | [`Cargo.toml:250-253`](/Users/ueli/Documents/semio/Cargo.toml:250) leaves `dev` incremental and has no profile-wide CGU choice. | Once the nine overrides are removed, native dev returns to Cargo/rustc's normal parallel codegen policy; release remains intentionally one CGU and is outside this performance packet. |
| Existing durable component profile | [`Cargo.toml:326-337`](/Users/ueli/Documents/semio/Cargo.toml:326) already defines `wasm-release` with one CGU and deterministic ship settings. | Production component identity must continue using this exact profile; do not relax its one-CGU, LTO, strip, or identity limits. |
| OS development component builder | [`framework/os/dev/script.ts:92-100`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%92%BB%EF%B8%8Fdev/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%93%9C%EF%B8%8Fscript.ts:92) defaults `SEMIO_PLUGIN_PROFILE` to `dev`; [`:976-984`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%92%BB%EF%B8%8Fdev/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%93%9C%EF%B8%8Fscript.ts:976) makes the profile part of both cargo invocation and output location. | This is the primary dev-launch owner for the new `wasm-dev` selection. |
| Common descriptor builder | [`plugin/describe/script.ts:52-65`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%93%87%EF%B8%8Fdescribe/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%A6%80%EF%B8%8Frust/%F0%9F%93%9C%EF%B8%8Fscript.ts:52) directly builds `wasm32-wasip2` with default `dev` and assumes a `debug` output. All per-plugin `describe` commands call it. | This is a required migration owner, not an optional convenience path. Leaving it at `dev` makes fresh descriptor generation reintroduce the WASI linker fault. |
| Native OS runtime lookup | Generated `PLUGIN_WASM_PROFILE_DIRS` is emitted at [`plugin/registry/script.ts:984-994`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%93%87%EF%B8%8Fregistry/%F0%9F%93%9C%EF%B8%8Fscript.ts:984) and consumed by real `os run` at [`run/bin.rs:88-105`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8F%83%EF%B8%8Frun/%F0%9F%93%A6%EF%B8%8Fbin.rs:88). | It currently searches `debug`, then `wasm-release`; it would not see fresh `wasm-dev` components. |
| MCP runtime lookup | [`mcp/workspace.rs:82-140`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%89%EF%B8%8Fmcp/%F0%9F%8F%A0%EF%B8%8Fworkspace/%F0%9F%A6%80%EF%B8%8F.rs:82) duplicates the same two-entry fallback independently. | It too would fail to activate a newly produced dev component unless changed in the same packet. This duplication remains an existing ownership defect, but profile alignment cannot wait for its broader extraction. |
| Root preflight and registry verifier | Root [`script.ts:21787-21800`](/Users/ueli/Documents/semio/%F0%9F%93%9C%EF%B8%8Fscript.ts:21787) and registry [`script.ts:1955-1974`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%93%87%EF%B8%8Fregistry/%F0%9F%93%9C%EF%B8%8Fscript.ts:1955) also hard-code `debug`, then `wasm-release`. | The former produces a false missing-artifact report; the latter can compare a profile-selected component hash against a descriptor. |
| Component-only scale fixture | [`framework/os/dev/script.ts:5086-5094`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%92%BB%EF%B8%8Fdev/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%93%9C%EF%B8%8Fscript.ts:5086) and [`fixtures/scale/script.ts:20-23`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%A7%AB%EF%B8%8Ffixtures/%F0%9F%94%8C%EF%B8%8Fscale/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%A6%80%EF%B8%8Frust/%F0%9F%93%9C%EF%B8%8Fscript.ts:20) link a WASI component in default `dev` and read `debug`. | Update the two component-link invocations and their expected path. WASI `cargo check` gates need not choose the one-CGU link profile because they do not execute the linker. |

The local toolchain corroborates the causal distinction: `rustc -C help` describes
`codegen-units=N` as dividing a crate into units for parallel optimization, and
`cargo help build` documents that a `--target` build uses a distinct target output
directory. Neither gives Cargo manifest profiles a target-triple selector.

## Smallest clean implementation packet

1. In root [`Cargo.toml`](/Users/ueli/Documents/semio/Cargo.toml:250), retain
   `profile.dev` exactly as the native baseline and remove all nine
   `profile.dev.package.*.codegen-units` overrides. Add:

   ```toml
   [profile.wasm-dev]
   inherits = "dev"
   codegen-units = 1
   ```

   This applies one CGU to the full component-link graph chosen by an explicit
   `--profile wasm-dev`, including future large plugin roots, rather than keeping
   a stale manual list of known offenders. Do **not** alter `wasm-release`, its
   build environment, parser limit, function ceiling, receipt hashing, or
   marker-last identity publication.

2. In OS dev [`script.ts:92-100`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%92%BB%EF%B8%8Fdev/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%93%9C%EF%B8%8Fscript.ts:92), make the development default
   `wasm-dev` and the ship default `wasm-release`. Reject any override outside
   exactly those two values; an unbounded `SEMIO_PLUGIN_PROFILE` lets a caller
   reselect `dev` and silently restore the crash. Keep the output calculation
   through existing `cargoProfileDir` ([repo library `:2868-2870`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%9B%91%EF%B8%8Frepo/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%9A%EF%B8%8Flibrary/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%9F%A6%EF%B8%8F.ts:2868)); it already maps custom profiles to their own output folder.

3. Change the shared descriptor helper to build and locate `wasm-dev`, not
   default `debug`. Every plugin-local `describe` target inherits that single
   correction, avoiding a fleet-wide copy edit.

4. Update all three runtime/preflight directory lists atomically to
   `['wasm-dev', 'wasm-release']`:

   - registry generator output and its independent source-side check;
   - root `os run` preflight;
   - MCP workspace resolver and its skipped-test diagnostics.

   Then regenerate the registry-owned Rust artifact file solely via
   `@semio-tech/plugin-registry:generate`; never hand-edit it. The normal OS
   runner, MCP headless runtime, root preflight, and generated constant must
   name the same ordered pair.

5. Route both scale-fixture component links through `--profile wasm-dev` and
   alter only their `wasm-dev` expected output path. Leave their `check-wasm`
   commands as checks.

## Identity guard: do not merge profile fallback with receipt verification

There is one material follow-up needed before treating the directory migration as
green. Registry `check` chooses the **first extant** artifact from its profile
list and compares its SHA-256 to a descriptor hash
([`registry/script.ts:2032-2041`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%93%87%EF%B8%8Fregistry/%F0%9F%93%9C%EF%B8%8Fscript.ts:2032)). A descriptor emitted from `wasm-dev` and a catalog receipt emitted from
`wasm-release` cannot be assumed byte-identical. The existing `debug` first
fallback has the same ambiguity.

Therefore make the check profile-explicit in this packet: runtime resolution may
choose `wasm-dev` before `wasm-release`, but a hash/receipt assertion must verify
only the profile actually declared by its descriptor/receipt (or, until that
schema field exists, only the canonical `wasm-release` publication artifact).
It must not accept a first-found dev artifact as proof of a release receipt. This
does not weaken the identity gate; it removes an unsound source of false
match/mismatch. The stdio catalog-root path already explicitly selects
`wasm-release` at [`stdio/script.ts:481-482`](/Users/ueli/Documents/semio/%E2%9C%8F%EF%B8%8Fs/%F0%9F%94%8C%EF%B8%8Fplugins/%F0%9F%97%84%EF%B8%8Fstdio/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%A6%80%EF%B8%8Frust/%F0%9F%93%9C%EF%B8%8Fscript.ts:481), and stays untouched.

## Required proof, in dependency order

1. A fast source-only unit under `@semio-tech/framework-os-dev:test` must assert
   the non-ship selected profile is `wasm-dev`, ship selects `wasm-release`,
   arbitrary `SEMIO_PLUGIN_PROFILE` values reject, and `pluginCargoArgs` retains
   the WASI target/stack arguments. The current only tests an explicit
   `wasm-release` args array ([`dev/script.ts:6043-6065`](/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%92%BB%EF%B8%8Fdev/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%9F%A6%EF%B8%8Ftypescript/%F0%9F%93%9C%EF%B8%8Fscript.ts:6043)).
2. Add a language-neutral JSON fixture with `{ mode, override, expectedProfile,
   expectedDirectory }` normal/ship/invalid cases. Have the TypeScript selector
   and a tiny independent parser validate it. This proves profile routing and
   avoids using an ambient process environment as the oracle.
3. Extend registry source tests to require the generated native resolver,
   root preflight and MCP resolver each enumerate exactly
   `wasm-dev`, `wasm-release` in that order; include stale `debug` as a negative
   path. The actual owner targets are
   `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache` followed by
   `bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache`.
4. With an isolated target after active builds yield, run one bounded component
   link such as the scale fixture using `--profile wasm-dev`; assert the output
   is under `wasm32-wasip2/wasm-dev`, and confirm the supported native stdio
   exact gate compiles without a `profile.dev.package.semio-s-plugin-stdio`
   override. Record elapsed compile phases separately; a source/compile result
   is not an identity or runtime claim.
5. Finally run the existing immutable stdio catalog-root/identity operation with
   its existing `wasm-release` command and fresh, dedicated target. It must
   retain all current raw/core/descriptor hash and one-million-function limits.
   A `wasm-dev` artifact must neither satisfy nor mask that result.

The registered entrypoints already exist: OS dev is exposed in
[`.vscode/launch.json:2460-2630`](/Users/ueli/Documents/semio/.vscode/launch.json:2460),
registry generate/check in [`:6079-6160`](/Users/ueli/Documents/semio/.vscode/launch.json:6079),
and strict stdio catalog-root remains
[`@semio-tech/stdio-plugin:catalog-root`](/Users/ueli/Documents/semio/%E2%9C%8F%EF%B8%8Fs/%F0%9F%94%8C%EF%B8%8Fplugins/%F0%9F%97%84%EF%B8%8Fstdio/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%A6%80%EF%B8%8Frust/%F0%9F%93%8B%EF%B8%8Fproject.json:55). No new launch command is needed.

## Nonclaims

This audit did not run Cargo or an Nx build and does not claim the WASI linker
fault is fixed, that native timing has improved, or that stdio component identity
now passes. It recommends a target-separated configuration that preserves the
current WASI safety workaround and all production identity constraints.
