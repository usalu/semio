# Explore: stdio wasm32-wasip2 Link Failure

Read-only investigation, 2026-09-06. Answers the 5 questions from the coordinator prompt.

## TL;DR

This is **not a new bug** — it is a known, currently-OPEN, extensively-documented blocker
already tracked under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/`.
The exact failure mode is **not** an out-of-memory / `--max-memory` linear-memory problem.
`wasm-component-ld` 0.5.25 first runs `wasm-ld` to produce a linked core module, then fails
while decoding that core for componentization because **wasmparser enforces a hard ceiling of
1,000,000 defined functions**, and stdio's linked core exceeds it:

```
functions count exceeds limit of 1000000 (at offset 0xdc7)
```
(quoted verbatim in `📓️sol-stdio-component-link-boundary.md:15`, 2026-09-03)

The coordinator's failing command used `--profile wasm-dev` (unoptimized, `opt-level = 0`).
Per `📓️explore-stdio-blocker.md:33` (2026-08-28/09-04 note) stdio was already known to only be
buildable "at -O optimized profile only (never debug)" — i.e. the `wasm-dev` attempt was
*expected* to fail this way even before today. But — important nuance — later evidence
(2026-09-03) shows even the optimized `wasm-release` profile has **not** been proven to
complete recently either (see §1). The last actually-served, successfully-linked stdio core
(2026-08-18) predates 48 subsequent commits that grew stdio's artifact tree, including two
PDF-standard-subset commits **yesterday** (2026-09-05, see §4) — i.e. stdio has almost
certainly grown past whatever margin it had on Aug 18.

---

## 1. Prior art: has anyone linked stdio/raster for wasm32-wasip2 since 2026-08-18?

**raster**: yes, trivially — raster is small (64K rust source, 1 `.rs` file at the package
root; its `📦️packages/🦀️rust` dir has no dependency on the giant `🗿️artifacts` tree at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`, so it links fine). Served core: `.../plugin-modules/raster/semio_s_plugin_raster_component.core.wasm`, 109,432,792 bytes, mtime 2026-08-17 18:40. No evidence raster itself was ever the source of a link failure.

**stdio**: **no.** Every attempt since has failed at the exact same wasmparser ceiling. Chronology found by grepping ticket notes under `🌙️08` and `🌙️09` for `wasm-component-ld`:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️explore-stdio-blocker.md:33` (dated 2026-09-04 per its own header, filed under the 08/28 ticket) — acceptance-matrix row: *"Registry — Monolithic stdio component exceeds function limit — **BLOCKED** — stdio has 36 artifact families; `wasm-component-ld`/wasmparser rejects the monolithic component because its function count exceeds one million. Clean build at -O optimized profile only (never debug)."*
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/✅️acceptance-matrix.md:27` — same BLOCKED row, with an added measurement: *"the 176-variant/77-method `StdioApps` closure expands linearly to 1,702,133 bytes, 254 function tokens and 13,728 match arms rather than pathologically emitting one million functions, while the total crate expansion legitimately exceeds 128 MiB. Actual N=0/2/8 linked-core growth remains unmeasured."*
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-stdio-catalog-root-completion.md:7,31,43` (2026-09-03) — three separate isolated/clean-root attempts, all ending: *"`wasm-component-ld` failed to encode the component: wasmparser reported `functions count exceeds limit of 1000000 (at offset 0xdc7)`. The failure preceded core extraction and publication."* One run (`:43`) used a script-created dedicated `CARGO_TARGET_DIR`, compiled the full dependency chain + stdio source, and still hit the same ceiling. Cleanup deleted the target/staging state; **no stdio core, descriptor, registry row, or completion marker exists**.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-stdio-component-link-boundary.md` (2026-09-03, full text read) — the most detailed packet:
  - `:15` gives the exact error text quoted above.
  - `:20` — *"A fresh debug-profile link using `wasm-component-ld --skip-wit-component` did not produce its linked core within the enforced 20-minute aggregate diagnostic bound. It was cancelled."* — i.e. even isolating the linker step (skipping component-wrapping) from a debug build didn't finish in 20 min.
  - Measurements section: multiple streamed-expansion probes (67 MB / 21s, 134 MB / 60s) prove **huge proc-macro expansion** (>128 MiB total crate expansion) but could not by themselves explain a literal million functions; an exact-delimiter probe isolated just the `StdioApps` 176-variant/77-method dispatch closure and found it "linear and compact" (1.7 MB expanded, 13,728 match arms) — i.e. **the dispatch macro itself is not the pathological function-count source**; the true source is unresolved.
  - Repair/verification section: *"A fresh `wasm-release` core-only build with sccache disabled reached the single stdio `rustc` optimized codegen/LTO/link process but did not emit a core before the 20-minute aggregate wall. It was cancelled... There is therefore no admissible optimized count/size yet, and merely changing the catalog profile is not claimed sufficient."* and later: *"The optimized `wasm-release` profile is unproven."*
  - Blocked by an unrelated concurrent regression: 3× `E0599` in `semio-framework-os-kernel` (`DirectoryClient::set_token`/`mint_session` absent at `📇️directory/🪪️identity/🦀️.rs:176,189,192`) prevented even getting a fresh N=0/2/8 representative measurement — "belongs to the concurrent S3 owner and was not patched here."
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-registry-monolithic-wasm-frontier-refresh.md:9,33` — *"The monolithic `stdio` catalog remains **BLOCKED**. The last admissible full-root result is still the documented `wasm-component-ld`/wasmparser rejection before raw-component/core extraction... The component limit remains correctly fail-closed at one million defined functions."* Explicitly warns fixture/unit tests of the parser limit "must not be relabelled as a real growth curve."
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📋️master-plan.md:427` — same BLOCKED summary in the master plan's own acceptance rollup.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️fable-explore-vcs-provider-frontier.md:223,304` — notes VCS's own cdylib depends on stdio as a *library* (not stdio's `PluginApp`/`StdioApps` surface) and speculates VCS's own component build "is plausible... unaffected" by this ceiling, but was never built/confirmed.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/GIS-MAP-END-TO-END/📓️status.md:243` — independent confirmation from a different app-ticket: *"`semio-s-plugin-stdio` separately fails at `linking with wasm-component-ld failed`."*

Other, unrelated `wasm-component-ld` failures found in the same grep (different root causes, listed for completeness, none are the memory/function-count issue):
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/DEMONSTRATOR-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🧪️verification-log.txt:25` and `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION/scratch-force-build4.txt:16131-16133` — a *duplicate symbol* problem (`semio_plugin_install_bundle`/`semio_plugin_bundle_installer_link_shim` exported multiple times when one component bundles several plugin crates), documented and fixed separately per `📋️master.md:64` in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE/` (gate the two `#[no_mangle]` fns behind a `semio-plugin-embedded` feature). Not stdio-specific and not the function-count ceiling.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/S-END-TO-END/📓️lld-elemsection-crash.md` — the LLVM-22 `ElemSection::writeBody` crash that motivated `codegen-units = 1` on `[profile.wasm-dev]`/`[profile.wasm-release]` (see root `Cargo.toml:257-258,304`). Already worked around.

### `target-*/wasm32-wasip2/wasm-dev/*.wasm` currently on disk

```
target-gen3d/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm      64,078,390 bytes   2026-09-06 19:29
target-lowpoly-boot/wasm32-wasip2/wasm-dev/semio_s_plugin_lowpoly.wasm 28,488,010 bytes   2026-09-06 20:30
```
(`find /Users/ueli/Documents/semio -maxdepth 4 -path '*wasm32-wasip2/wasm-dev/*.wasm'`, run 2026-09-06.) No `target-stdio*` directory exists at all — every prior stdio diagnostic explicitly deleted its isolated `CARGO_TARGET_DIR` after concluding (per `sol-stdio-component-link-boundary.md`'s final bullet: *"All ticket-local expansion logs, temporary representative sources, and their isolated Cargo targets were deleted after these conclusions were recorded."*). Existing target dirs at root: `target-p2d-wasm`, `target-energy-e2e`, `target-p3d-desc`, `target-gen3d`, `target-lowpoly-boot`, `target-raster`, `target-s-e2e`, `target-p2d-e2e`, `target-s-e2e-wasm`, `target-s-e2e-wgpu` — none named for stdio.

---

## 2. Dev-script post-processing / other paths to `core.wasm`

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`:

- `PLUGIN_WASM_STACK_BYTES = 8 * 1024 * 1024` (line 104).
- `pluginCargoArgs(packageName, profile)` (line 125-127) builds exactly:
  `["rustc","-p",packageName,"--target","wasm32-wasip2","--profile",profile,"--","-C","link-arg=-zstack-size=8388608"]` — this is exactly the command the coordinator ran (confirmed by the parameterized test at line 6683: `pluginCargoArgs("semio-s-plugin-vcs", profile)` → `["rustc","-p","semio-s-plugin-vcs","--target","wasm32-wasip2","--profile",profile,"--","-C","link-arg=-zstack-size=8388608"]`).
- `describeBuiltPlugin` (line 354) is the only caller of `runCmdStatus("cargo", pluginCargoArgs(...), ...)` (line 372) that actually invokes `cargo rustc`. There is **no other Rust build path**.
- `materializePlugin` (line 380-402, doc comment: *"One target's MATERIALIZE stage: jco transpile, `wasm-opt`, bridge/host-shim file"*) calls `describeBuiltPlugin` (line 401), which is where the `.core.wasm` hash is taken (line 359: `pluginFileDigest(componentModule.replace(/\.js$/, ".core.wasm"))`) — i.e. **jco's own transpile step extracts `<component>.core.wasm` from the already-linked wasm component that `cargo rustc`/`wasm-component-ld` produced**. If `wasm-component-ld` fails, `cargo rustc` exits non-zero, `describeBuiltPlugin`/`materializePlugin` never reach the jco step, and **no `core.wasm` is emitted or updated** — the served one stays whatever was last committed to the plugin-modules cache (2026-08-18 for stdio).
- `stagePluginDescriptor` (line 226) only copies/validates already-built artifacts (`.js`, `.d.ts`, `.core.wasm`, JSON, `.descriptor.semio`, see the `files` set at line 214); it does not build anything.
- Searched `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` for `core.wasm`/`transpile`/`jco`/`componentize`: only taxonomy/discovery/test-fixture files reference these terms generically (registry metadata, test fixtures for nested-cargo-package detection) — no alternate build pipeline exists there either.

## 3. Toolchain

- `rustc --version` → `rustc 1.99.0-nightly (c4af71034 2026-07-06)`.
- `rustup toolchain list` → active toolchain is `nightly-2026-07-07-aarch64-apple-darwin` (pinned by `/Users/ueli/Documents/semio/rust-toolchain.toml`: `channel = "nightly-2026-07-07"`, `targets = ["wasm32-unknown-unknown", "wasm32-wasip2"]`).
- `rustup target list --installed` → includes `wasm32-wasip2` (and `wasm32-wasip1`, `wasm32-unknown-unknown`, `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, `aarch64-apple-darwin`).
- `wasm-component-ld` **is present**: `~/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/wasm-component-ld`, 4,975,168 bytes (sic — `ls -la` above prints 4577280 for `wasm2es6js`, `wasm-component-ld` itself is 4,975,168... actually recorded size below), dated 2026-07-18. So this is **not** a missing-toolchain-component problem.
- `~/.cargo/bin` has `cargo-component`, `wasm-bindgen`, `wasm-bindgen-test-runner`, `wasm-pack`, `wasm2es6js`, `trunk`, `cargo-nextest`, `cargo-llvm-cov`, plus the usual `rustup`-shimmed `cargo`/`rustc`/etc. Nothing unusual.

### `.cargo/config.toml` (root)

At the time of writing, `git diff .cargo/config.toml` is **empty** — the working tree matches `HEAD`. The initial `git status` snapshot the coordinator saw (`M  .cargo/config.toml`) is now stale: an unrelated commit landed mid-session (`0fab4f0c30`, 2026-09-06 20:48:37 +0200, "🐙️ueli...🚩️594 — Rename reasoning plugin outputs...") that happened to touch `.cargo/config.toml` too, but a diff of *that* commit shows it only added the `[target.wasm32-unknown-unknown]` 16 MiB stack-size flag for the wgpu renderer's shadow stack (unrelated to stdio/wasip2). Current live content of `.cargo/config.toml`:

```toml
[target.wasm32-wasip2]
rustflags = ["-Z", "threads=8", "-C", "link-arg=--max-memory=536870912"]
```

Git history (`git log --date=iso -p -- .cargo/config.toml`, since Aug):
- **2026-08-03 16:28:20 +0200, commit `605c40a0`** ("🐙️...🚩️415", ticket `REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT") — this is the commit that *introduced* `[target.wasm32-wasip2]` with `--max-memory=536870912` (512 MiB). Root-Cargo.toml-adjacent comment explains: *"Plugin components declare no memory `maximum`, so V8 reserves the full 4GiB guard region per module per worker (~20 plugins/workers at boot). 512MiB is uniform across all plugins on purpose..."*
- **2026-09-06 20:48:37 +0200, commit `0fab4f0c30`** — unrelated addition of the `wasm32-unknown-unknown` stack-size flag (renderer shadow-stack fix), did not touch the `wasm32-wasip2` block.

This 512 MiB `--max-memory` cap has been in place, unchanged, since Aug 3 — i.e. it was **already active** when stdio's served core successfully linked on 2026-08-18, so it is not a *new* condition explaining today's failure. It constrains the wasm module's growable linear-memory ceiling (relevant to runtime OOM-guard sizing), not the linker's function-count ceiling that the ticket notes above identify as the actual failure. I found **no** ticket note anywhere reporting a `wasm-ld`/`initial memory too large`/`memory is too large` style error for stdio or any plugin — the narrower greps for those exact phrases across `🌙️08` and `🌙️09` tickets returned zero stdio-relevant hits (the only "linking with ... failed" hits unrelated to wasip2 were native `cc`/exit-69 native-toolchain failures, a different subsystem entirely).

## 4. Wasm size / profile settings

Root `Cargo.toml`:
- `[profile.wasm-dev]` (line 258-260): `inherits = "dev"`, `codegen-units = 1`. **No `opt-level` override** — stdio therefore builds at `opt-level = 0` (dev default) under `--profile wasm-dev`. Comment at line 262-266 explicitly explains only `semio-s-plugin-lowpoly` gets `opt-level = 2` (line 267-268) for its 8 ms interactive-step budget, and says this is *"[s]coped to this package rather than raised on the whole profile because the io layer (`semio-s-plugin-stdio`) is not on the render path and is far more expensive to compile."* — i.e. stdio is deliberately left unoptimized in `wasm-dev`, which is exactly what maximizes monomorphized-function bloat and is why `wasm-dev` is the profile flagged "never debug" in `explore-stdio-blocker.md:33`.
- `[profile.wasm-release]` (line 299-306): `inherits = "release"`, `opt-level = "s"`, `lto = "thin"`, `codegen-units = 1`, `strip = "symbols"`, `incremental = false`, `trim-paths = "object"`. Extensive comment block (lines 307-323) documents these choices as deliberate size/speed tradeoffs (opt-level "s" not "z"; thin not fat LTO; codegen-units=1 also works around the LLVM-22 `ElemSection` crash). This is the profile the ticket notes say is required — and, per §1, still **unproven** to complete within the 20-minute build budget as of 2026-09-03.

Sizes actually on disk (`find ... -iname "*stdio*component.core.wasm"`):
```
storybook-static/plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm                                  376,957,060 bytes  2026-08-18 11:14
.🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/🔌️plugin-modules/🗄️stdio/...core.wasm                                 376,957,060 bytes  2026-08-18 11:14
.🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/.stage/🔌️plugin-modules/🗄️stdio/...core.wasm                          376,957,060 bytes  2026-08-18 11:14
♻️mit-bestand/🧺️demonstrator/dist/plugin-modules/stdio/...core.wasm                                              376,957,060 bytes  2026-08-27 08:04
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🗄️stdio/...core.wasm                             376,957,060 bytes  2026-08-18 11:14
```
All five copies are byte-identical and all but one date to 2026-08-18 11:14 — confirming there has been exactly **one** successful stdio link (Aug 18), fanned out to cache/stage/served copies, and it has never been refreshed since (the 2026-08-27 demonstrator `dist/` copy is a stale re-fan-out of the same bytes, not a rebuild — same size).

**Why the served wasm might no longer be linkable at all** (root-cause candidate the ticket notes have not yet nailed down, but which this exploration can newly corroborate): stdio's `🗿️artifacts` source tree (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`, reached from the crate via `#[path]` attributes — 4,688 of them per `sol-stdio-component-link-boundary.md`) is **8.2 GB and 4,104 `.rs` files** covering 36 artifact-format families (pdf, tiff, gltf, docx, xlsx, step, ifc, dwg, dxf, gis formats, etc). Per-artifact-family scoping means every one of ~36 formats × multiple standard versions × multiple subsets independently declares its own `schema/mutations` derive-macro-generated code (`dsl::Mutations`, `value_derive::ToValue`/`FromValue`) — this is the actual likely source of the "legitimately exceeds 128 MiB crate expansion" noted in the ticket, not any single dispatch macro. **48 commits touched this directory since 2026-08-18** (the date of the last successful link), including, most notably:
- `2026-09-05 22:02:04 +0200`, commit `3a6a9d6bfc` ("🚩️591") and `2026-09-05 19:04:38 +0200`, commit `b0dfa0f09b` ("🚩️590") — both touch `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf` (1.1 GB, 482 `.rs` files) **yesterday**, i.e. the two commits immediately preceding the current `HEAD`-adjacent history. The compiler log for the in-progress verbose rerun (see below) shows PDF 1.4/1.7 standard subsets `base`/`a`/`e`/`h`/`ua`/`vt`/`x` each independently compiling full `viewer`/`mutations` modules — consistent with continued, very recent subset-splitting growth (matches the still-open `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` ticket referenced in `explore-stdio-blocker.md` §1, whose explicit goal is to give every subset its own fully-separate implementation — directly multiplying per-subset generated code).
- `tiff` and `gltf` artifact subdirectories were last touched 2026-08-12 and 2026-08-11 respectively (both before the Aug 18 successful link, so not new growth).

No ticket note mentions a 4 GB wasm32 address-space limit or a `wasm-ld: error:` text; the size angle that *is* documented is purely the function-count ceiling, not a byte-size ceiling.

## 5. Conclusion

**Most likely cause(s), in order of evidence strength:**

1. **(Primary, well-documented)** stdio's linked core module exceeds wasmparser's hard 1,000,000-function ceiling enforced by `wasm-component-ld` 0.5.25 during component-wrapping (`sol-stdio-component-link-boundary.md:15`). This is a long-standing, still-open, project-tracked blocker (`✅️acceptance-matrix.md:27`, `📋️master-plan.md:427`, `📓️terra-registry-monolithic-wasm-frontier-refresh.md:9`) — the coordinator's failure is a reproduction of an already-known issue, not a new regression to root-cause from scratch.
2. **(Aggravating, newly corroborated here)** stdio has grown since the one and only successful link (2026-08-18): 48 commits touched its 8.2 GB / 4,104-file `🗿️artifacts` tree, including PDF-standard-subset work as recent as **yesterday** (2026-09-05, commits `b0dfa0f09b`/`3a6a9d6bfc`). Whatever margin existed on Aug 18 has likely shrunk further, which is consistent with — though not proof of — why even the `wasm-release` (optimized) profile has not been observed to complete since (per `sol-stdio-component-link-boundary.md`'s "the optimized `wasm-release` profile is unproven" and its own cancelled 20-minute attempt).
3. **(Ruled out)** `--max-memory=536870912` in `.cargo/config.toml`'s `[target.wasm32-wasip2]` (added 2026-08-03, unchanged since) is *not* the cause: it predates the last successful link and no ticket note reports a `wasm-ld` memory-limit error for stdio — only the wasmparser function-count error, which is a completely different check inside `wasm-component-ld`'s own component-encoding step, downstream of `wasm-ld`.
4. **(Ruled out)** Toolchain/tool absence: `wasm-component-ld` is present and correctly versioned; `rustc`/`rustup` targets are all installed; this is not an environment setup problem.

**Recommended command for a coordinator to actually get a linked stdio (and, separately, raster) wasm**, per the project's own already-recorded guidance:

```
cargo rustc -p semio-s-plugin-stdio --target wasm32-wasip2 --profile wasm-release -- -C link-arg=-zstack-size=8388608
```

i.e. swap `--profile wasm-dev` → `--profile wasm-release` (matches `pluginCargoArgs`'s own `profile` parameter — `dev`'s `🧑‍💻dev` script already supports passing `wasm-release`; nothing needs to be hand-edited). **Caveat, stated plainly because the ticket notes are explicit about it and I did not run this myself (read-only agent, host has two concurrent builds already):** this is *not yet proven* to succeed on the current source tree. The most recent attempt at this exact profile (`sol-stdio-component-link-boundary.md`, 2026-09-03) did not finish within a 20-minute build-budget wall and was cancelled before reaching a link result, and that was *before* the 2026-09-05 PDF-subset commits added more code. A coordinator should expect to need either (a) a much longer/foreground budget for a first `wasm-release` attempt (this profile is `opt-level=s`+`lto=thin`+`codegen-units=1`, i.e. slow to compile for an 8.2 GB source tree) with `RUSTC_WRAPPER=""` / sccache disabled (sccache serializes concurrent builds — see project memory `sccache-serializes-concurrent-builds`), or (b) to pick up wherever the still-open `COMPLETE-SEMIO-END-TO-END` / `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS...` tickets left off rather than re-diagnosing from scratch — the N=0/2/8 representative-growth-curve measurement they were attempting (to actually pin down which artifact family/subset pushes the function count over the ceiling) was itself blocked by an unrelated concurrent regression (`DirectoryClient::set_token`/`mint_session` missing in `semio-framework-os-kernel`) and was never completed.

**Verbose rerun status** at the time of writing (`/private/tmp/claude-501/-Users-ueli-Documents-semio/feae8329-6c36-42e3-88bd-6e53d7e1043a/scratchpad/stdio-wasm-prewarm-2.txt`, 1.3 MB, last grown at 2026-09-06 22:11): still emitting ordinary `rustc` warnings (unused imports, unreachable code, slow const-eval) from deep inside the PDF/TIFF/GLTF artifact subset compiles — it had **not yet reached the link step** as of this check, so no fresh linker `note:`/error text was available to quote verbatim. Re-check its tail once it either errors or completes; if it reaches `wasm-component-ld`, the exact function/offset numbers it reports should be compared against the `1000000`/`0xdc7` figures recorded in `sol-stdio-component-link-boundary.md:15` to see whether the ceiling has moved (it's checking `--profile wasm-dev`, so per point 1 above it is expected to fail regardless of the function-count question, once/if it reaches the link step at all within its time budget).

## Files referenced (for follow-up)

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-stdio-component-link-boundary.md` (full text, most detailed)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-stdio-catalog-root-completion.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-registry-monolithic-wasm-frontier-refresh.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/✅️acceptance-matrix.md:27`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📋️master-plan.md:427`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️fable-explore-vcs-provider-frontier.md:223,304`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️explore-stdio-blocker.md:33`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/GIS-MAP-END-TO-END/📓️status.md:243`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/` (open, related growth driver)
- `Cargo.toml:257-268,299-323` (profile definitions)
- `.cargo/config.toml:16-21` (wasm32-wasip2 `--max-memory` rustflag)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:104,125-127,354,372,380-402`
- `rust-toolchain.toml`
