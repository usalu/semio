# 2026-10-11 stdio-link: `semio-hub-stdio` wasm link exceeds 1 000 000 functions

Executor: stdio-link (Sonnet). Build #6 failure: `@semio-tech/stdio-plugin` `wasm-component-ld … functions count exceeds limit of 1000000 (at offset 0x17f4)`.

## Verdict

Neither candidate (a) nor (c). Candidate (b) in a wider sense: the bounded fleet is intact, but the **`wasm-dev` profile (the profile the play build links, `opt-level = 0`) keeps every trivial generic adapter as its own wasm function**, and the grant-based retained-clone rollout multiplied the generic bodies the closure instantiates (`ToValue`/`FromValue`/`RetireOwned`/`ErasedSnapshotRetirement`/`CanonicalJsonTree` derives, `VcsArtifactApp<EditorApp|ViewerApp<E>>` runtime). The 09-23 baseline (59 728) was measured with `editor-component-check`, which links `wasm-release` (`COMPONENT_PROFILE`), never `wasm-dev`; the only `wasm-dev` baseline on record is the W4 session (2026-09-27): stdio dev component code section 110.5 MB, 124.4 MB stripped. Today the same assembly is 283.9 MB of code / 318.5 MB core (2.6x) and 1 105 567 functions.

The failing command is not "release": the play build runs `cargo rustc -p semio-hub-stdio --profile wasm-dev --lib --crate-type cdylib --target wasm32-wasip2` (`fresh-6.log` line 6423176, `…/wasm32-wasip2/wasm-dev/build/semio-hub-stdio/…`).

## Refuted candidates

- (a) feature drift: `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` features are unchanged since their creation (commit `4e36b2b5012`, 2026-10-01; later commits do not touch them). `default = plugin-root` -> `component-app-assembly` = 7 text/data crates (txt, xml, json, csv, md, tsv, html) + `full-artifact-catalog` (36 artifact crates, codecs only). The linked hub CGU holds exactly **18 apps** (9 editors + 9 viewers: `Txt`, `XmlAny`, `XmlValid`, `JsonAny`, `JsonIJson`, `Csv`, `Md`, `Tsv`, `Html`). No `full-app-catalog` (176 apps). The shipped-fleet cfg split survived the remote fast-forward and the migration.
- (c) all 10 packages in one component: the 119 linked rlibs contain no `semio_hub_stdio_*` family crate; the nine family hubs are `[dev-dependencies]` (law-only).

## Measurement (reproduced the failure bit-exactly: 1 105 567 functions)

Method: private target dir = APFS clone of build #6's `release-lsTV8U/cargo/target` (13 s, zero extra disk), `cargo rustc … --profile wasm-dev` with a logging linker wrapper (`CARGO_TARGET_WASM32_WASIP2_LINKER`) that runs `gcc-ld/wasm-ld` on the exact link line and keeps the core module; `llvm-nm` for symbols; own wasm section parser (`count.py`, `census.py`, `count-component.py`).

Final core module (stripped), by object of origin (functions that survive `--gc-sections`):

| origin | functions |
|---|---|
| hub CGU (`semio_hub_stdio`, 221 MB object; 18 apps x ~15.1 K = 272 K) | 284 405 |
| 36 artifact crates (codec closure; largest: semio 153 936, dwg 62 405, pdf 46 995, gltf 41 486, step 39 087, txt 34 432, xml 18 296 ...) | ~ 700 000 |
| framework (plugin 35 K, os-kernel 29 K, framework 9.5 K, ui-contract 4.7 K, value 3.1 K ...) | ~ 90 000 |

By name: `core::iter` 288 K, `core::option` 183 K, `core::result` 100 K, `Retirement` 128 K, `VcsArtifactApp` 118 K function names. Per app (hub CGU): `EditorApp<E>`/`ViewerApp<E>` ~15 120/15 155 functions, of which 7 784 are `semio_framework_plugin::component::app` items and 3 449 are std adapters instantiated over `VcsArtifactApp`. Per artifact crate even the smallest (deflate, binary, wav) carries 6-8 K functions. There is no single exploding generic: it is the sum of un-inlined adapters in a closure that grew with the derives. A type-erased app runtime (LB Option B) would shrink the 272 K app share only; the 700 K codec share is the derive output of 36 crates.

Code size: 283.9 MB code, 318.5 MB core. The describe pipeline would have refused it even with fewer functions: `FRESH_COMPONENT_MAX_BYTES` = 268 435 456 B (256 MiB).

## Fix (root, no guarantee lowered)

`🌎️hub/Cargo.toml`: `[profile.wasm-dev.package.<name>] opt-level = 1` for `semio-hub-stdio`, the nine `semio-hub-stdio-*` family hubs and the 37 `semio-s-artifact-stdio-*` crates (Cargo has no name globs). Only the `wasm-dev` profile of the stdio closure is touched; native, `wasm-release`, the shipped fleet (18 apps), the ceilings and all laws are unchanged. `opt-level = 1` costs no compile time (hub unit 5m59s at O1 vs ~5 min at O0) and the dev render-turn contract only benefits.

| experiment (same assembly, same tree) | functions | core bytes |
|---|---|---|
| e0 baseline, `wasm-dev` O0 | 1 105 567 | 318 531 526 |
| e1 hub O1 only | 857 059 | 280 799 305 (still > 268 435 456) |
| e2 hub + 37 artifact crates O1 (shipped config) | **299 058** | **185 428 414** |

## Real link proof (no wrapper, no overrides: the build's own command through `wasm-component-ld`)

`cargo rustc --manifest-path 🌎️hub/…/🗄️stdio/📦️packages/🦀️rust/Cargo.toml -p semio-hub-stdio --profile wasm-dev --lib --crate-type cdylib --target wasm32-wasip2` in the private target: `Finished wasm-dev in 16m 44s`, component `semio_hub_stdio.wasm` **185 427 274 B**, main core module **299 058 functions** (`count-component.py`; limit 1 000 000, size bound 268 435 456). Log: `.🧬semio/🦑️repo/⚡️cache/play-fleet/stdio-link/real-stdio.log`.

The largest family hub got the same real link: `semio-hub-stdio-semio` `Finished wasm-dev in 23m 23s`, component 166 536 005 B, main core module 238 235 functions (`real-semio.log`).

## Family hubs (core module via the wasm-ld wrapper, shipped O1 config, `wasm-dev`)

| hub | apps (playground rows x2) | functions | core bytes |
|---|---|---|---|
| stdio-semio (largest, 19 subsets) | 38 | 238 235 | 166 537 145 |
| stdio-cad | 20 | 197 619 | 177 011 869 |
| stdio-image | 22 | 188 866 | 170 466 599 |
| stdio-pdf | 20 | 164 583 | 143 793 730 |
| stdio-mesh | 10 | 156 503 | 124 256 271 |
| stdio-office | 18 | 151 454 | 131 023 522 |
| stdio-media | 8 | 142 173 | 108 986 808 |
| stdio-binary | 10 | 142 073 | 110 565 405 |
| stdio-bim | 12 | 141 249 | 111 744 239 |

All nine are far below the ceiling and the size bound. Unfixed O0 numbers of the families were not measured (the base hub alone already exceeded the ceiling at O0; semio at 38 apps would be worse).

## Other breakage found and handled

1. The nine family hub roots `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/*/🦀️.rs` still had `plugin_exports!(plugin, XApps)` at 09:35 (the coordinator's earlier fix was not in the tree, all nine files carried an 09:24:56 mtime). Rewrote all nine to the 3-argument form with the same `MountedOwnerPolicyV1` literal as the 43 other hubs (idempotent regex; if the coordinator re-applies, nothing changes).
2. **Open, not fixed (out of scope, needs regeneration):** `shipped_fleet` does not compile natively. `cargo test -p semio-hub-stdio --test shipped_fleet` also builds `[[bin]] semio-hub-stdio-mutation-bridge`, whose generated `🌎️hub/🧩️compositions/🗄️stdio/🤖️generated/🧩️mutations/🦀️.rs:36` still lists `JpgBaselineMutation` (`…jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations`), a type that no longer exists after the jpg migration (E0425). The generator is that same bin (`🏭️bridge/📜️script.ts` runs it), so the owner of stdio-jpg must repair the baseline subset mutations (or regenerate once the bin builds). Until then `shipped_fleet` and `editor_catalog` (same package) cannot run. Log: `native-shipped-fleet.log` in the cache folder.
3. wasm-release was not linked (the build does not use it for this component, my change does not touch it, and a thin-LTO stdio release rustc reaches ~23 GB). The 09-27 release number (59 728 functions) is unverified on today's tree.

## Files touched

- `🌎️hub/Cargo.toml` (47 profile tables, 2 extended and 45 new, + comment block, marker `🧾️ Stdio Dev Optimisation`)
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/{🎵️media,🏠️bim,💼️office,📘️pdf,🔢️binary,🔺️mesh,🖼️image,🛠️cad,🧿️semio}/🦀️.rs` (`plugin_exports!` policy argument)
- this report, `📓️2026-10-10-migration-corrections.md` (entry 56)

Raw data and tools: `.🧬semio/🦑️repo/⚡️cache/play-fleet/stdio-link/` (`core-link.sh`, `run-cargo.sh`, `real-link.sh`, `families.sh`, `count.py`, `count-component.py`, `census.py`, `e*.log`, `real-*.log`, `census-rlib.tsv`, `origin.tsv`).
