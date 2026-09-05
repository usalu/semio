# Materialized Module Hand Repair

Scope: the four still-outstanding developer output trees, with registry producers and actual runtime consumers repaired together. The 57 materialized plugin directories and 26 extension mirrors have now moved through individually written no-overwrite commands to the catalog's exact names. Raw public plugin IDs remain unchanged.

## Handpicked Current Catalog

The schema-first authority is `OS/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`. Its 59 explicit ID/name pairs were individually chosen after reviewing the source catalog and domain owners. Most plugin roots retain their already-handpicked source-owner emoji. Extension outputs share the flat deployment namespace with all plugins, so some need a different, still domain-specific identity.

| Extension Identity | Selected Emoji | Meaning in the Flat Deployment Namespace |
| --- | --- | --- |
| cad-extension-aec-building | 🏢️ | Building model. |
| cad-extension-aec-building-energy | 🔥️ | Building heat/energy. |
| cad-extension-aec-building-structure | 🏟️ | Built structural enclosure. |
| cad-extension-spatial-shape | 🔷️ | Geometric shape. |
| flow-extension-bim | 🏘️ | Building-information model. |
| flow-extension-brep | 🧊️ | Boundary-representation solid. |
| flow-extension-dictionary | 📚️ | Key/value lexicon. |
| flow-extension-draw | 🎨️ | Drawing operators. |
| flow-extension-list | 📃️ | Ordered list. |
| flow-extension-logic | 🔀️ | Logical branching. |
| flow-extension-math | 🧮️ | Numeric operators. |
| flow-extension-primitive | 🔤️ | Primitive typed values. |
| flow-extension-text | 📝️ | Text values. |
| imperative-extension-control | 🎮️ | Execution control. |
| imperative-extension-effect | 📣️ | Observable effects. |
| imperative-extension-logic | ⚖️ | Boolean comparison/decision. |
| imperative-extension-math | ➕️ | Arithmetic evaluation. |
| imperative-extension-text | 🔡️ | String operations. |
| playbook-module-procedural | ⚙️ | Procedural execution. |
| process-extension-concrete | 🏙️ | Concrete construction process. |
| process-extension-metal | 🔩️ | Metal fabrication. |
| process-extension-robotic | 🤖️ | Robotic fabrication. |
| process-extension-wood | 🪓️ | Wood processing. |
| sourcing-module-beams | 🪜️ | Load-carrying linear members. |
| sourcing-module-slabs | 🧇️ | Waffle/slab construction. |
| sourcing-module-windows | 🪟️ | Window products. |

All names are explicit data: no palette, hash, slug heuristic, fallback, or name-generation loop is used. The parser rejects missing/extra fields, undeclared IDs, malformed or generic emoji, stacked/interior emoji, duplicate public IDs, and repeated sibling emoji. Its focused Nx test passed against Ajv and the independent emoji-regex oracle. The existing developer hot-swap tests now assert mapping from exact physical names back to public IDs, including an explicit negative for raw IDs and undeclared decorated folders; 6 focused tests passed.

## Remaining Work and Safety

Directory publication/lookup code now uses the authority. External runtime-installed extensions are not confined to the current plugin catalog: they receive a required authored `directoryName` wire field, independent of their public ID, with strict filesystem admission and actual sibling collision checks. The Rust host's extension store is in-memory, so its wire codec retains this field without inventing filesystem placement.

JCO interface names will be admitted only by finite unions of exact declared parent directory contracts and exact interface basenames, with hostile unknown-parent/custom-name tests. This is not a generated-tree exemption. Binaries and old deployment mirrors remain preserved until their precise current ownership and references are handled.

## Exact Authored Output Changes

For every one of the 56 complete plugin outputs and all 26 extension mirrors, the single authored bridge `semio_s_plugin_<actual component stem>.js` moved to `🌉️bridge.js`. The energy output has no bridge and none was invented. The 26 extension `install.json` files moved to `📥️install.json`; only `directoryName` and the corresponding current `moduleUrl` were updated. Version, label, public ID, package hash, installation timestamp and all other existing metadata remain unchanged. The catalog generator, checked-in generated URL functions, developer materializer, extension-store materializer, and installation-record readers now use the explicit bridge/install names. Kernel's owning agent updated the public-ID-only URL interface separately. No broad replacement or old output cleanup was executed.

Before these moves, a read-only SHA-256 snapshot recorded 2,554 payload files totaling 3,016,391,662 bytes, excluding the 26 installation records that need current placement data. After all 83 directory moves and all 82 bridge moves, every payload matched the original size and SHA-256. No WASM, descriptor, compiler declaration, or JavaScript payload bytes changed.

Root hand-authored the 82 exact `interfaces` parent contracts: 56 plugin output parents and 26 extension mirrors. Root's read-only audit found all 82 paths and checked 410 positive/hostile matches against picomatch, with no duplicate JSON keys. The authored emoji output parents themselves are not reserved. Companion and interface filename registration is still underway.

## Verification State

- Existing directory-catalog regression: 11 registry tests passed before adding the authored bridge URL case.
- New authored bridge URL case: red run failed specifically because the old URL function still required a filename; 11 existing tests passed. The first green attempt stopped during a concurrent taxonomy publication window before test execution, so it is not a pass.
- External directory boundary: 5 TypeScript extension-store tests passed at the quick level, including schema/emoji oracles, public-ID separation, actual sibling collision rejection, installation lookup and uninstall.
- Rust extension codec: the new neutral `directoryName` round-trip case first failed on the missing field. After the narrow wire field implementation, all 9 `os_extension::tests::` passed through the existing kernel Nx route. No filesystem admission is claimed for the in-memory Rust store.
- Finite explicit parent-array scope: one focused test passed with 24 assertions across strict parser, Ajv fixture validation, discovery classifier and extracted actual normalizer matcher. Named reuse of exact parent arrays is being added with hostile fixtures; no brace-alternative filename reservation is admitted.

## Completed Exact Compiler Boundaries

The finite-scope feature now accepts either an explicit array of exact parent-contract IDs or one explicit named set, with strict unknown/duplicate/empty/ambiguous-scope rejection. All existing single-parent scopes are unchanged. Literal filename validators also reject brace alternatives. The expanded language-neutral fixture passed one focused Nx test with 60 assertions; the actual normalizer loaded the current taxonomy with zero diagnostics (that empty ignored-output inventory is a loader check, not file coverage).

All 33 interface basename contracts now refer to nine hand-authored finite sets of the 82 exact parent IDs. The actual emitted presence/absence matrix passed one existing repo-library Nx test with 13,330 assertions, checking 2,027 declaration files, direct/indexed resolution, the actual normalizer matcher, picomatch and TypeScript companion resolution. All 246 component companion files (82 JS/declaration/WASM triples) are individually registered; one companion-matrix Nx test passed 3,445 assertions, including 82 WASM headers and TypeScript declaration resolutions. Root authored 192 companion contracts; this lane authored the last 54 for imperative-text, playbook-procedural, process concrete/metal/robotic/wood and sourcing beams/slabs/windows in both physical roots. Source dispositions remain adapter-source/package-glue, with no purity exemption.

The 31 retained old plugin workers moved from `🟨️plugin-worker.js` to `🧵️plugin-worker.js`, preserving their bytes and separating them from the host-shim sibling. The complete 2,554-file / 3,016,391,662-byte snapshot still matched after those moves and all bridge moves, before the later import-specifier edits below.

## Read-Only Build-Preparation Guards

`assertPluginOutputChildren` replaced an unknown-child deletion sweep with exact current-child validation and an error naming the preserved stale path and normal producer route. `assertExtensionOutputsFresh` replaced worker deletion and host-shim overwrites with read-only stale-output checks. The language-neutral `🛡️deployment-preservation.json` checks untouched unknown files, directories, workers and stale shims against independent Node/WebCrypto digests. Three new cases failed on the old behavior; the repaired focused run passed all five cases. No broad materialization or destructive preparation route was executed by this lane.

## Vendor, Shared Worker and Watch Markers

These runtime-private names were handpicked: vendor mirrors use `🪞️vendor`, the provider uses `🤝️bytecode-alliance`, the WASI browser bridge uses `🪟️preview2-shim`, the font bundle uses `🔤️guestslim-typst-fonts.bin`, and the multiplexed worker owner uses `🧵️shard`. Public `/plugin-modules` and `/extensions` route identities are unchanged. All 56 plugin component files have 237 exact vendor import-specifier edits; root owns the 26 mirror files and their 108 corresponding edits, preserving their raw public `../../plugin-modules/` prefix. TypeScript AST span removal and SHA-256 prove every other byte of all 56 plugin files unchanged, including their original final blank line. Root independently proved the same for all 26 mirrors (see `🪞️extension-import-review.md`). The patch tool's automatic single-newline trimming was detected and repaired through exact EOF hunks.

While this lane was finishing import edits, another active task's existing `DEMONSTRATOR-END-TO-END-ALL-APPS/🔨️build-components.sh` producer created the canonical vendor and shard destinations. The existing old trees and newly created trees matched fully; neither was overwritten or deleted. After root coordination, exact no-overwrite moves preserved the old copies:

| Old Path Relative to Developer Root | Current Location / Action |
| --- | --- |
| `🔌️plugin-modules/_vendor` | Moved intact to ticket `📦️vendor-recovery`; the already-created canonical `🔌️plugin-modules/🪞️vendor` has the same 11 file payloads. |
| `🔌️plugin-modules/_shard` | Moved intact to ticket `🧵️shard-recovery`; the already-created canonical `🔌️plugin-modules/🧵️shard` has the same worker bytes. |
| `🔌️plugin-modules/.hot-swap` | `🔌️plugin-modules/♻️hot-swap.json`, no-overwrite move. |
| `🧩️extension-modules/.extension-watch` | `🧩️extension-modules/👀️extension-watch.json`, no-overwrite move. |
| Future `.size-report.json` output | Writer now explicitly names `📊️size-report.json`; no existing file was removed. |
| Future `.engine-size-report.json` output | Writer now explicitly names `📈️engine-size-report.json`; no existing file was removed. |

The 14-file snapshot across vendor, shard and both markers totals 8,819,607 bytes; every current canonical payload matched its original full size and SHA-256. The font is 8,757,072 bytes, SHA-256 `05c4bbb7d07a3ee0c77274d546a3a4c5942366ccbc82eedad18b4885aa21fc5a`. The shard worker is 13,578 bytes, SHA-256 `0a7b304e6b787056a1fdbdf55398be3b2a6d9a622cee93d49cfd091a13b980c5`. All ten vendor JS modules match the installed `@bytecodealliance/preview2-shim/dist/browser` bytes; their exact literal basenames, and only those ten paths, have fixed filename contracts and package-glue source dispositions. Four scoped semantic directory memberships resolve correctly under the actual ancestor chain; full taxonomy validation returns no problems.

The vendor URL fixture first failed on the missing current staged-prefix rebase, and the registry fixture first failed on the missing explicit static-directory mapping. The repaired focused URL test and all 12 registry tests passed. One earlier compound filter was rejected by Nx's shell handling of a pipe, and one earlier registry attempt exceeded its 30-second budget; neither is a pass. The actual static-serving regression passed after isolating the three actual production static-serving functions from unrelated esbuild module initialization (jsdom's Uint8Array realm breaks that unrelated import). It validates all 345 plugin/mirror vendor imports against their resolved public URLs, then invokes the actual serving middleware on the 12 vendor/font/worker payloads and checks every returned byte and Node/WebCrypto hash. The output log records the runtime check. This does not claim full browser app activation.

Both current deployed roots now have zero path-emoji findings across 2,763 entries, of which 398 are governed and the remainder have exact tool contracts. This audit includes authored marker names and retained workers, not a generated-subtree exclusion. The full developer suite exceeded its 30-second quick budget before completion; the long-level retry passed all 81 tests. Kernel's owning lane reported successful canonical/direct worker regeneration and freshness checks after the vendor physical all-clear. The two old Vite distribution mirrors remain outstanding and are not claimed compliant; their evidence is in `📤️distribution-repair.md`.

A final preparation audit found the remaining direct deletion of `dev/public/plugin-modules`. The new `assertNoStalePublicPluginOutputs` refuses that stale path without changing it. Its neutral public-tree case failed first on the absent guard; the expanded preservation suite then passed all six tests, including exact Node/WebCrypto content preservation and absent-directory acceptance. The three plugin-scanning/rewrite loops now gate on the explicit catalog mapping instead of an obsolete underscore convention, so vendor and shard owners are never treated as plugin members. No production build-preparation function was executed to make these assertions pass.
