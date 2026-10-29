# 1029 Audit Census: source-boundary gate before activation

STATUS: 0 violators / 1638 closure crates measured (134 roots, run 02:44 10/29, rc 0, no source edited). Two policy violations found: raised `rust-source-boundary` limits in two activation-closure crates (section 3).

## 1. Inputs and method

- Roots file: `C/1029-tools/census/roots.txt` (134 package names, one per line).
  - 38 activation names (`C/1029-tools/census/activation-names.txt`): 11 hub plugin components (`semio-hub-<id>-component`), the demonstrator bundle crate `semio-hub-demonstrator`, the puzzle lib `semio-hub-puzzle`, 4 framework wasm component packages (`semio-framework-surface-component`, `-editor-component`, `-os-node-graph-component`, `-os-flow-component`), and the 21 extension `…-component` packages from `C/1029-audit-matrix.md` section 1.3.
  - 96 unique package names from the 99 Cargo manifests under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{🧊️3d,◻️2d,🖐️5d}` (`C/1029-tools/census/puzzle-names.txt`). These are the workspace members that the puzzle `--workspace` wasm checks compile. They include test-only crates, so they are not all activation-closure members.
- Tool: `bun C/1028-tools/gatecensus/census.ts C/1029-tools/census/roots.txt`, run from `/Users/ueli/Documents/semio`. Real gate closure reader and measurer, no compile. Raw output: `C/🗑️generated/1029/census-run1.txt`. Stderr was empty, so no `NOROOT` lines. Runtime about 66 s, foreground.
- Summary and checks: `C/1029-tools/census/summarize.ts`, `top.ts`, `boundary-meta.ts`.
- Limits: the measurer uses `min(262144 B, 32 files, 4000 lines, per-crate rust-source-boundary)`. The per-crate row `lim` field confirms this min behaviour, so a raised per-crate value has no effect on the measured limit.

## 2. Result

- Closure: 1638 crates, 1638 rows. No row was skipped (no crate lacked `src/lib.rs`). Closure reader errors: 0.
- Every activation name and every puzzle root produced a row.
- Violators (bytes, files or active lines above limit): **none**.
- The 1028 gatecensus found 23 violators. All are gone from the closure or split under the cap:
  - `semio-s-artifact-stdio-ifc-4x3`, `-4-model`, `-4`, `-2x3-base-model`, `-2x3-base`: not in closure any more. The `ifc` closure now measures only `semio-s-artifact-stdio-ifc` at 13404 B / 2 files.
  - Energy `mut-*` and `mut-model`, and `native-mutations`: now 66077 B (mut-model) and 199238 B / 27 files (native-mutations), under 32 files.
  - `plugin-decl-views` 229012 B / 26 files; `plugin-job` 255040 B / 14 files; `plugin-tool-run` 222365 B / 17 files; `plugin-window-config` 256549 B / 20 files; `plugin-instance-app` 259481 B / 16 files. All under the cap now.
  - `os-store-engine` 257005 B / 13 files (1028: 254646 B). It grew about 2.4 KB since 1028 and sits at 98%.

## 3. Raised limits (policy violation)

Raising `[package.metadata.semio.rust-source-boundary]` above defaults is forbidden. A grep of the 901 manifests that declare the block found 2 raised crates. Both are in the activation closure:

| crate | declared (B / files / lines) | Cargo.toml | measured (B / files / lines) | effect |
|---|---|---|---|---|
| `semio-framework-os-kernel` | 1048576 / 128 / 12000 | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` | 118543 / 7 / 1460 | none (min with default), reached via `semio-hub-demonstrator` |
| `semio-s-artifact-fem-2d-editor` | 2097152 / 128 / 24000 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | 179020 / 21 / 2699 | none (min with default), reached via `semio-hub-fem` |

Both measure under the default limit, so no refusal today. The raised values are still policy violations and would hide a future overflow if the gate ever honoured them. Action for the owners: restore the defaults or delete the block. This audit made no edit.

Per-crate limits that are stricter than the default are not violations. 11 crates declare them: `semio-framework-io-base64`, `-composition-admission`, `-pack-source`, `-os-store-link-schema`, `-os-store-blob-identity`, `-os-inference` (lines 3000), `-os-identity`, `-plugin-component-receipt`, `-artifact-reference-borrow`, `-replication-crypto`, `-replication-error`. All are under their own stricter limit.

## 4. Watch list (above 95% of a limit)

Not violators, but the next small growth will refuse them.

| crate | bytes | files | lines | worst | crate dir |
|---|---|---|---|---|---|
| `semio-framework-os-vcs` | 261954 / 262144 | 31 / 32 | 3258 / 4000 | 99.9% | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/📦️packages/🦀️rust` |
| `semio-framework-plugin-instance-app` | 259481 / 262144 | 16 / 32 | 3124 / 4000 | 99.0% | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📦️packages/🦀️rust` (dispatch lane is editing this area) |
| `semio-s-artifact-energy-model-snapshot` | 257478 / 262144 | 29 / 32 | 2607 / 4000 | 98.2% | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` |
| `semio-framework-os-store-engine` | 257005 / 262144 | 13 / 32 | 2892 / 4000 | 98.0% | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📦️packages/🦀️rust` |
| `semio-framework-plugin-window-config` | 256549 / 262144 | 20 / 32 | 1928 / 4000 | 97.9% | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📦️packages/🦀️rust` |
| `semio-framework-plugin-job` | 255040 / 262144 | 14 / 32 | 2766 / 4000 | 97.3% | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📦️packages/🦀️rust` |
| `semio-framework-value` | 253949 / 262144 | 30 / 32 | 3367 / 4000 | 96.9% | `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust` |
| `semio-s-artifact-stdio-binary-snapshot` | 79607 / 262144 | 31 / 32 | 956 / 4000 | 96.9% (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` |
| `semio-s-artifact-stdio-xml-snapshot` | 233541 / 262144 | 23 / 32 | 3852 / 4000 | 96.3% (lines) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` |
| `semio-framework-plugin-reactor-turn` | 250572 / 262144 | 21 / 32 | 3114 / 4000 | 95.6% | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust` |

File-count watch: `semio-s-artifact-energy-model-mut-zone` is at 30 / 32.

## 5. Caveats

- The dependency walk is an over-approximation (path deps, default features, optional deps included). It can over-count, not under-count, which is the safe direction for this gate.
- Roots are package-level. The census does not reproduce per-target cfg beyond `wasm32-wasip2`. It measures source only, not compile output.
- The 96 puzzle 2d/3d/5d members include test-only crates, so their rows are not activation-closure members. None of them is a violator, so this changes no result.
- The 1028 unsupported proc-macro origin error (`semio-s-artifact-energy-model-native-derive`) did not recur. Closure reader errors: 0.
- Peers keep editing. This is a 02:44 snapshot. Re-run `census.ts` after any split or limit edit.

## 6. Files

- Roots: `C/1029-tools/census/roots.txt` (plus `activation-names.txt`, `puzzle-names.txt`)
- Census output: `C/🗑️generated/1029/census-run1.txt`
- Raised-limit scan: `C/🗑️generated/1029/boundary-meta-raised.txt` (script `C/1029-tools/census/boundary-meta.ts`, input `C/🗑️generated/1029/cargo-manifests.txt`)
- Summaries: `C/1029-tools/census/summarize.ts`, `C/1029-tools/census/top.ts`
