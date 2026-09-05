# Surface Hand Review

## Scope and exact choices

Reviewed the complete physical `🧰️framework/🔨️modules/🗺️surface` tree, including ignored compiler output. Paint, terrain, node-graph, and tiled-map identities already describe their actual purpose and differ from their siblings. No nested AGENTS files exist here. No Git writes or bulk rename/replacement scripts were used.

| Exact parent | Old name | Handpicked name | Purpose |
| --- | --- | --- | --- |
| `🗺️surface/🗺️tiled-map` | `🧪️oracle` | `🔮️oracle` | Independent Mercator reference contribution; distinguishes sibling `🧪️tests` |
| `🗺️surface/📦️packages/🦀️rust` | `pkg` | `🕸️bindings` | Browser compiler-binding output owner |

The first choice is declared in the exact owner-scoped `testContributionDirectoryOverrides` entry. The already handpicked `🕸️web-mercator-tile-oracle` case was added to `members-of-tests`; its Rust fixture reader and the contribution's prose path now use that exact name. The Gherkin source description now points to the existing `🦀️.rs`, not nonexistent `🦀️component.rs`. No oracle IDs, vectors, behavior, or assertions changed.

The six compiler-output files moved intact. Four mutually linked names (`framework_surface.js`, `framework_surface.d.ts`, `framework_surface_bg.wasm`, and `framework_surface_bg.wasm.d.ts`) remain literal because wasm-bindgen controls their pairing. Four exact path-scoped central contracts and the three necessary source dispositions declare only this producer-owned set; no ignored-directory exemption was added. `.gitignore`, `package.json`, `Cargo.toml`, and Cargo's literal `tests` directory remain governed reserved names.

The existing package script passes `outputDirectory: "🕸️bindings"` to the normal wasm-pack producer. Package exports now name that directory, with no old `pkg` alias. The one incoming Dev Vite alias was corrected with its owner's approval. The generic package resolver was left unchanged because Surface has a real package manifest at its source package root. Existing `TestScript` now runs the new companion test before its unchanged Cargo test route; no separate permanent script or executable command was added.

## Byte preservation

All six moved compiler output/config files have the exact original SHA-256 values recorded before the move:

| File | SHA-256 |
| --- | --- |
| `.gitignore` | `684888c0ebb17f374298b65ee2807526c066094c701bcc7ebbe1c1095f494fc1` |
| `package.json` | `64a0cc537d6d9e03531366de5632ac291d797001131a53672179d75da5a2203d` |
| `framework_surface.js` | `c92c5654803091cc531bac5554e51f6c716ae92b63b8cd908cc2fa5259455d8e` |
| `framework_surface.d.ts` | `cd0a536a6fda697ee8c6f45361540434d4b7cd5cde503cbd064d705dc866cfff` |
| `framework_surface_bg.wasm` | `f2b9021ea704b4b63f5445073bda7fadf3684f99ff2d0c62b03eb4bbd5c6e7af` |
| `framework_surface_bg.wasm.d.ts` | `25cfd3f39a9496533fdacb10e5cf6f8666c925c102d99b8d6875256ef722978b` |

The frozen Mercator fixture remains SHA-256 `85ef2ebc0bc60345cc9d26ad4d580a047e27bdf69273eab8a8c77392400750d4`. The contribution JSON changed only its stale explanatory path after the move; it is not claimed byte-identical.

## Verification

The hand-authored `🧪️tests/📇️bindings.json` and strict sibling `🧬️bindings.schema.json` define the neutral companion identity fixture. The permanent `🟦️.ts` test validates it with independent Ajv, checks all four exact contracts and rejects an unrelated owner's same basenames, checks the actual six-file roster and absence of old `pkg`, verifies JS declaration/Wasm links, resolves the declaration with independent TypeScript, validates the actual Wasm binary, and checks the package export and generator destination. RED was captured before the contracts/move; GREEN passes **1 test, 18 assertions**.

The independent Python mercantile 1.2.1 adapter passes all **47 frozen vectors**. Its Mercator projection/tile results are third-party comparisons; the repository-owned LOD policy values are specification vectors, not an independently standardized LOD algorithm.

Fresh native `cargo test -p semio-framework-surface --test tiled_map_mercator_oracle`, through Nx, passes **7 tests, 0 failed, 0 ignored, 0 filtered**. An earlier attempt was blocked by the concurrently repaired OS mutation-source exports; the successful retry used the corrected API without bypassing imports or weakening assertions. Dependency warnings remain unrelated to these path repairs.

Final physical audit: **36 entries, 25 governed non-reserved entries, zero emoji findings, zero unresolved directory roles**. All ignored compiler files were included. Evidence is under `🗑️generated/metabolism-glb/`: `surface-audit-final.json`, `surface-bindings-before.txt`, `surface-bindings-red.log`, `surface-bindings-green-resolved.log`, `surface-python-oracle.log`, and `surface-native-oracle-final.log`. No full browser rebuild is claimed by this scoped native/companion verification.

## Separate Trace observation

Trace naming had already been reviewed in `⏳️async-job-repair.md`; no Trace files were changed here. A redundant fresh native run completed with **21 passing and 2 failing tests**, unlike the earlier recorded 23-pass run: `watchdog_tail_uses_the_original_guard_for_admission_and_terminal` unwrapped `None` at clock/tail tests line 50, and `cancellation_latency_measures_requested_to_observed` unwrapped `None` at Trace line 945. Both current failures were sent to the parent and left intact. Evidence: `trace-native.log`. This scoped Surface success does not overwrite that separate failure evidence.
