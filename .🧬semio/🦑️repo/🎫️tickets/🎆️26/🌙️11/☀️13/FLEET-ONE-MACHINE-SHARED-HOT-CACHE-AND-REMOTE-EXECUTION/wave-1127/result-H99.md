# H99 result: portable build scripts, declared host-bound reasons, unit limits

Staged only (full build-system freeze 16:50): nothing landed, nothing built. All proofs ran in scratch overlays (`.../scratchpad/H99/{ov,tree,lim}`, real tree untouched, symlinked siblings, copied edited files). Kit: `T/🧰️scripts/H99/` (`stage/`, `apply-offload.py`, `apply-command-policy.py`, `landing.patch`, `gismap-tiles-carve.patch`, `inventory.py`, `compare.ts`, `sim-worker.ts`, `overlay-file.py`).

## 1. Inventory (what blocks portability)
- Workspace-owned `build.rs`: 7 (the wgpu target and 6 hub component crates). All 7 are `include!("…/🏗️builder/🦀️.rs")` adapters; the old text rule read the include path (and `"/STACK:8388608"`, a linker flag) as `reads-outside-manifest`. Class: unnecessary probe (include is compile-time, linker literal is not a path). Now portable.
- Registry crates (cargo cache, 92 build scripts, class counts by the current text rules, a script with several traits is counted once under its first; Python model `inventory.py` = oracle, TS verdict agrees on all 92):

| class | scripts | examples | decision |
|---|---|---|---|
| hermetic (probe none, no host fact) | 32 | getrandom, naga, rustls | unchanged |
| version probe (`rustc --version`/`-vV`) | 21 | serde, serde_core, quote, zmij, libc, crc32fast, httparse, paste, object | needed, derivable from declared toolchain identity: stand-in rustc |
| compile probe (rustc compiles a probe file; autocfg) | 13 | proc-macro2, anyhow, thiserror, heapless, cap-*, io-lifetimes, rustix | needed, derivable only on an identical toolchain: declared `toolchain-compile-probe` |
| native library / tools | 11 | libsqlite3-sys, blake3, wasmtime*, psm, stacker | needed host fact: host-bound `native-library` |
| git revision | 3 | cranelift-codegen, wasm-bindgen-shared, wasmtime-cache | host-bound `git-revision` |
| bundled assembler | 1 | ring | host-bound `bundled-assembler` |
| network | 1 | codex | host-bound `network` |
| unset-variable reads | 7 | icu_*_data (ICU4X_DATA_DIR), libm (ENSURE_NO_PANIC), ash | derivable: unset = keyed as absent |

Measured effect on the 381 `not-portable` build-script declines in `.cache/b/r/m/u/x/offload.jsonl` (since 11-24; 318 were `spawns-process`): 297 (78 %) portable, 68 (18 %) `toolchain-compile-probe`, 12 host-bound with a declared reason, 4 crates not in this host's cargo cache. Top offenders and fate: serde_core 52 and serde 52, zmij 43, quote 36, libc 30, crc32fast 10, target-lexicon 6 become portable; proc-macro2 36, anyhow 15, thiserror 10 need the toolchain-bound path.

## 2. What the stage does (schema-first; texts en + de)
- `🧭️probe/🧬️schema/🔣️.json` (+ `🟦️.ts`, `🗝️key`): closed policy of what a script may ask of the host. Version probes are answered by a stand-in `rustc` (shell, from the run key's own `-vV` text: `--version`/`-V` first line, `-vV`/`--version --verbose` all, anything else exit 97 + marker, run withdrawn and done here as `toolchain-probe:unanswered-arguments`). Set DEP_*/CARGO_MANIFEST_LINKS travel with their keyed value (path-like value refused `host-value`); unset names are keyed as absent; other set names stay `host-environment`. `include!` files are inlined for analysis. Target-guarded spawns (`freebsd-version` only under freebsd, `emcc` under emscripten) do not count on other targets. Every remaining decline names a declared reason (`host-bound:<id>`, bilingual text in the policy): git-revision, bundled-assembler, native-library, network, host-path, all-environment, spawns-program, host-environment, host-value.
- Keys (H55/H81 style): for portable and toolchain-bound scripts the run key drops `RUSTC_WRAPPER`/`RUSTC_WORKSPACE_WRAPPER` and gains the carried values, the absent names and `semio-toolchain=sha256(-vV)` (`buildRunKeyInputV1` via `buildScriptProbeKeyV1`); host-bound scripts keep their key.
- Offload: `landing.patch` (6 files + command policy) removes `buildScriptSourceRefusalV1`, `sourceMarkers`, `allEnvironment`, `environmentRead`, `environment.refused` (superseded, no legacy), moves `buildScriptSourcePathV1` into the probe key module, binary marker `RUSTC` becomes `compiler-probe` (skipped only when the stand-in answers), the tool exports `RUSTC` to the stand-in itself (no worker policy change for RUSTC), `toolchain` passed from the wrapper. Worker command policy gains `DEP_` prefix and `CARGO_MANIFEST_LINKS` (`apply-command-policy.py`).
- Unit limits (`stage/capacity/🧮️units/🧱️limits`): declared defaults 20 min / 1536 MB, warn at 80 %, carving debt rows with measured, budget (x1.10) and action; `planUnitLimitsV1(units)` returns en/de warnings (`near`/`carving`/`over`) for the preflight; first row = Gismap tiles (6 000 000 ms, 2200 MB, budget 6 600 000 ms / 2420 MB).

## 3. Gismap tiles carve (crate `semio_s_artifact_gis_gismap_tile_coordinates`)
Source is 31 KB. The only mounted tests are the 3 in `🎨️vector-style/🧪️tests/🔬️unit` and they name only `crate::` and `semio_framework_surface`; the 37-test `gis-policy/🧪️tests/🔬️unit` file is not mounted in this crate. Yet the test unit builds all 11 dev-dependencies: OS kernel with `protocol-laws`+`mutation-testing`, plugin with `artifact-app-testing`, both job-worker testkits, original-snapshot-owners, geo, geojson, tokio, serde, serde_json, async-macros. Carve: delete the `[dev-dependencies]` table (`gismap-tiles-carve.patch`, 11 lines). Not measured (freeze): the 1 h 40 / 2.2 GB is the brief's number; check after landing with `repo cargo check -p semio-s-artifact-gis-gismap-tile-coordinates --tests` and refresh `Cargo.lock` of the gismap workspace; if the unit stays above the default, the row stays as declared debt, else delete the row.

## 4. Proof
- `🧭️probe/🧪️tests/🟦️.ts` 5 pass / 0 fail (77 expects): policy vs schema (Ajv); 22 language-agnostic script fixtures; stand-in model vs fixture answers; **shell stand-in byte-identical to the real `rustc`** for `--version`, `-V`, `-vV`, `--version --verbose`, refusal exit 97 + marker; **two simulated hosts** (other home, lane, rustc path, wrapper set vs unset, checkout dir) compute one `portableKeyV1` through the real patched `buildRunKeyInputV1`; other toolchain, a set DEP_ value, or a host-bound script splits/keeps keys as declared. Before the patch the same two hosts differed (key env carried the wrapper).
- `compare.ts`: TS verdict equals the Python inventory on 92/92 registry scripts; 7/7 workspace scripts portable.
- `limits` 4 pass (policy vs schema, fixture states, en/de plan, Python oracle).
- Offload suite on the overlay (before the key/toolchain wiring and the final stand-in route): 10 pass (all pre-existing cluster tests incl. a real offloaded run and path-leak withdrawal) after updating the superseded rows; `sim-worker.ts` runs the patched tool text like the worker: stdout and OUT_DIR identical to a local run with the real compiler.

## Not verified
- Final offload cluster suite incl. my two new tests (stand-in end to end, unanswered-arguments): blocked. `…/🏗️build/📥️inputs/🔏️revision/🟨️.mjs` has nlink 2 (another hard link in `scratchpad/H94/tree2/…/🟨️.mjs`), so the revision gate refuses ("Bounded physical Source revision required") for every test importing it. A hard-linked scratch tree breaks the real file for the whole fleet: remove `H94/tree2` (not mine, not touched).
- Compile probes stay on their host: the worker must advertise its toolchain digest (`toolchainDigestV1`) and the command route must require it; not built. Until then reason `toolchain-compile-probe`.
- `autocfg`/`version_check`/`rustc_version` users keep working only through that split (autocfg = compile, others = version); not run end to end.
- Landing causes one miss wave for run keys of version-probing scripts (key changes once), then identical on all machines. Landing needs the post-seal slot via `repo land` (areas: cache unit-cache, offload command policy, capacity); `repo typecheck` and the full offload + run-key + unit-cache tests after landing; H90 must call `planUnitLimitsV1` (not wired, no timing source read).
- Gismap numbers and the carve effect (section 3).
