# Terra Next-Wave Scout — 2026-08-27

Read-only audit. No production source, manifest, lockfile, ticket metadata, Cargo, Nx, or formatting command was changed/run.

## Fresh dependency boundary

The plan's commands were run:

```sh
bun ./📜️script.ts verify dependencies list rust --raw
bun ./📜️script.ts verify dependencies list js --raw
bun ./📜️script.ts verify dependencies summary --format json
```

The collector explicitly removes both `compose/` and `temp/compose/` (`dependencyIsCompositionManifest` in `📜️script.ts`), so the snapshot is within the governing boundary. It reports 77 Rust raw rows, 71 JS raw rows, and 160 literal external identities; 107 are production-reachable.

### Recommended file-disjoint zero-dependency leaves

| Priority | Exact owner and surface | Reachability | Replacement / deletion packet | Why safe to parallelize |
| --- | --- | --- | --- | --- |
| 1 | `rust:byteorder@1.5.0`; `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`; test oracle in `🧵️retained-command/🦀️component.rs:657-746` | `test-runner`, `productionReachable=false`; one checkpoint encoding oracle | Keep the language-neutral JSON fixture; make the test oracle write the five little-endian `u64`s with owned fixed-width byte helpers, then delete the manifest/lock row. The existing encode/decode assertion is the exact replacement surface. | Only this Cargo manifest and its retained-command test module; no Store/publication, Puzzle/CAD, or app cohort path. |
| 2 | `js:fast-glob@3.3.3`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`; imports only in `🧪️index.test.ts` and `🧪️tests/🧪️transaction-v2/🟦️.test.ts` | `repository-tooling`, `productionReachable=false`; six discovery-parity tests plus the transaction-v2 test | Replace only the oracle-side glob walk with an owned `node:fs` recursive enumerator (lexical byte sort, files only); preserve the existing schema/mapping assertions and delete the dependency row. | Dedicated repo-library test area; disjoint from OS-plugin Cargo and all plugin app sources. |
| 3 | `js:@types/semver@^7.7.1`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/package.json` | `repository-tooling`, `productionReachable=false`; no non-manifest source import was found. The only `semver` hits here are import-pattern fixtures in its own test index. | First prove TypeScript resolution with the package row absent; if resolution is clean, delete this declaration and its lock entry. No code replacement should be introduced. If the compiler proves it is still used indirectly, stop this leaf rather than adding ambient compatibility types. | One test-platform package manifest/lock edge, wholly disjoint from the two packets above. |

`byteorder` and `fast-glob` are real current rows, not stale report artifacts. Do not bundle their package-manifest edits with each other; their source/test owners are independent. The third leaf is intentionally an evidence-first metadata deletion, not a guessed type shim.

## Next app cohorts after Puzzle/CAD and Norm

Latest stable official report: `📊️coordinator-official-tool-jobs-publication-lanes-r3-2026-08-27.json`. It has 774 command rows, 64 bounded rows, zero accepted rows, 833 remaining findings, 53 scan-then-monolith rows, and 28 global-payload stores. The selected three maximize all three requested problem classes without entering current Puzzle/CAD or completed Norm territory.

| Priority | Cohort and exclusive source files | Official counts | Live blocker / packet boundary |
| --- | --- | --- | --- |
| 1 | Process 3D — `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` (2,637 lines) | 33 remaining command rows; **26/53** scan-then-monolith rows; 0 global stores | All 26 flagged commands call `process3d_retained_reduce` (defined line 409, wrapped as one-step completion line 677 and `BoundedArtifactCommandWork` line 922). Split import/export and model mutation preparation, owner-local bounded reducer, publish, and commit into route-qualified resumable jobs. This is the largest monolith-removal packet. |
| 2 | Procedural 2D + 3D — only the two editor roots: `…/🌀️procedural2d/…/✏️editor/🦀️component.rs` (1,545 lines) and `…/🧊️procedural3d/…/✏️editor/🦀️component.rs` (1,709 lines) | 49 remaining command rows; **18/53** scan-then-monolith rows (all 2D); 0 global stores | `procedural2d_retained_reduce` is defined line 205 and converted wholesale to `Complete` line 325. Route-level preparation/proofs are missing; 3D also needs its raw-census/proof reconciliation. Treat 2D and 3D as one cohort only if one owner holds both files; otherwise split them into file-disjoint child packets. |
| 3 | Forms — editor root plus config and two command leaves: `…/📋️forms/…/✏️editor/🦀️component.rs`; `🎚️config/🦀️component.rs`; `🎮️commands/🧪️set-try-value/🦀️component.rs`; `🎮️commands/🧪️set-try-values/🦀️component.rs`; plugin `🗿️artifacts/📋️forms/🦀️component.rs` | 29 remaining command rows; 0 scan-then-monolith rows; **8/28** global-payload stores (the largest cohort) | Replace process-lifetime state with app/operation-owned state: config has `TRY_VALUE_BLOBS` and `TRY_VALUES_BATCHES`; set-try-value has `TRY_VALUE_SESSIONS`, `ACTIVE_TRY_VALUE_GENERATIONS`, `FORMS_INPUT_REGISTRY`; set-try-values has `ACTIVE_BULK_GENERATIONS`, `BULK_SESSIONS`; plugin root has `FORMS_SCRATCH`. Preserve the existing 64-item caps as explicit per-owner admissions and finish the 29 route contracts. |

These three packets are mutually file-disjoint. They also avoid coordinator-owned framework registration, Store/plugin central publication, `📜️script.ts`, Puzzle/CAD, and Norm. Do not schedule Remodel concurrently with Forms' global-state packet: it has 41 remaining rows, three global stores, and production `std::thread::spawn` sites, making it the next candidate after this wave rather than a cleaner third slot.

## Recommendation

Dispatch three source owners now: **Process 3D monolith split**, **Procedural 2D/3D route split**, and **Forms owner-local state removal**. In parallel, dispatch the three dependency leaves only after assigning each manifest owner separately. All six packets must retain the plan's schema-first, differential-oracle, deletion, and exact production-reachability gates; this scout establishes priority and isolation, not acceptance.
