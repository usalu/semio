# 🧾 Wave 2 manifest audit of 📕️norm (read-only)

Scope: every Rust crate under `✏️s/🔌️plugins/📕️norm`. No source was edited. Checker: `🔍️audits/wave2/manifest-check.py` (run with `python3 -I`). Unresolved items are listed with `file:line`.

## 1. Source boundary (`source-boundary-scan.py -v`, limits 262144 bytes / 32 files / 4000 lines)

Scan output: 200 crate rows, 198 `ok`, 2 `OVER`. The 313 package manifests found by the checker are more than the scan's 200 rows. I did not reconcile that gap.

| Status | Crate (manifest) | Bytes | Files | Lines | Finding |
|---|---|---|---|---|---|
| OVER | `semio-s-artifact-norm-vdi3805-model` (`🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml:16`) | 180958 | 14 | 4276 | 276 lines over the 4000-line limit (same `[lib]` as the schema root `../../🦀️.rs`). Largest mounted subtrees: `🧪️reference-fixture` 539 lines, `📚️catalog-model` 357, `🧮️sheet-attributes` 300. Fix: move `🧪️reference-fixture` and `📚️catalog-model` into a sibling crate (new `Cargo.toml` under `📦️packages/🦀️rust`, workspace member, path dependency, `pub use` in the old crate). Splitting files does not help. |
| OVER | `semio-s-artifact-norm-iso16757-model` (`🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml:16`) | 156351 | 12 | 4433 | 433 lines over. Same fix (move a mounted subtree into a sibling crate). I did not rank the mounted files for this crate. |
| near | `semio-s-artifact-norm-en1996-schema` (`🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml:16`) | 209461 | 23 | 3932 | 98% of the line budget. Split off a sibling crate before the next addition. |
| near | `semio-s-artifact-norm-en1999-diff` (`🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml`) | 131016 | 5 | 3903 | 98% of lines. |
| near | `semio-s-plugin-norm-structural-scene` (`🎬️structural-scene/📦️packages/🦀️rust/Cargo.toml:16`) | 151347 | 30 | 3483 | 30 of 32 files. |
| near | `semio-s-artifact-norm-en1996-mut-loads`, `semio-s-artifact-norm-en1991-mut-gravity`, `semio-s-artifact-norm-en1991-mut-snow`, `semio-s-artifact-norm-en1996-mut-walls` | 34-38 KB | 31 each | 714-767 | 31 of 32 files. |
| near | `semio-s-artifact-norm-iso16757-diff` | 133336 | 8 | 3634 | 91% of lines. |

Also at 28 files (watch list): `en1993-mut-fire-fatigue`, `en1993-mut-thin-walled`, `en1993-mut-connections`, `en1995-mut-footbridges`, `din16798-mut-zones`, `en1992-mut-members`, `en1999-mut-members`, `en1999-mut-inputs`, `en1997-mut-foundations`, `din18599-mut-building`, `en1991-mut-general`, `din4108-mut-layers`, `vdi3805-mut-geometry`.

The scan flags only the 2 OVER crates above. Its 200 rows are the scan's own closure view; the excluded `🏭️bridge` binary is not in its scope.

## 2. Manifests and project wiring

Checked: 313 `[package]` manifests under norm, plus all `[workspace]` roots and `[workspace.dependencies]` path entries.

- **Path dependencies:** all resolve to an existing `Cargo.toml`. 0 problems.
- **Workspace membership:** 312 of 313 package manifests are members of their nearest workspace (`✏️s/🔌️plugins/📕️norm/Cargo.toml` lists 15 members, artifact roots list their own). Members listed in each `[workspace]` all exist.
  - Exception (intentional): `✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.toml:5` declares its own `[workspace]`, and the norm root excludes it (`✏️s/🔌️plugins/📕️norm/Cargo.toml:23` `"**/🏭️bridge/**"`). The file's own comment explains why. Its path deps (lines 13-30) all resolve. It has `📜️script.ts` but no `📋️project.json`.
- **`📋️project.json`:** only 17 crates have one. They are the umbrella crates `📦️packages/🦀️rust` of the artifacts, the contract, and mechanics. The other 296 crates have none. `📌️important/build-loop.md:33` says every new Rust crate needs `📋️project.json` and `📜️script.ts` "like its siblings". The siblings do not have them, so the rule and the tree disagree. Decide which one is authoritative.
- **Target wiring:** every `cwd` and `repo ./📜️script.ts <cmd>` in the 17 `📋️project.json` files resolves, except the en1996 case below.
- **`📜️script.ts` package lists:** 17 umbrella scripts. Each artifact umbrella lists its sibling crates in `packages:[…]`. All listed names exist and sit inside their artifact. 40 sibling crates are not listed (full list in section 5).

### Problems

1. **`🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/📜️script.ts:14`**: the router registers only `regenerate-example-assets`. It has no `runArtifactRustPackageMain` call and no `packages:[…]` list. But `📋️project.json` calls `repo ./📜️script.ts build` (line 47), `check` (line 55), `test` (line 64), `test-snapshot-sqlite` (lines 21, 27, 33) and `regenerate-example-assets` (line 73). Only the last one is registered, so the other commands fail with unknown-command. The 22 sibling crates of en1996 are not listed anywhere. Fix: restore the router the way `🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/📜️script.ts` does (`runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-norm-en1996", {packages:[…22 names…], snapshotSqliteTests:[…]})`) and register `test-snapshot-sqlite`.
2. **Contract umbrella** `📇️registry/🧬️contract/📦️packages/🦀️rust/📜️script.ts:9`: no `packages:` key. It builds the shared contract crates, so this is intentional. The 8 crates under the contract folder are not listed. Informational only.
3. **Mechanics** `🧮️mechanics/📦️packages/🦀️rust/📜️script.ts:4`: `{}` options, single crate. Fine.

## 3. `#[path]` attributes

- 3807 raw `#[path = …]` occurrences in norm `.rs` files. Three are in `//` comments, so they are not mounts.
- Inline modules (`#[path = "."] pub mod x { … }`) are valid Rust. They set a base directory for their children. The checker tracks that base and checks inline directories with `isdir`, file mounts with `isfile`.
- **Result: 0 unresolved.** The earlier flat check reported 198 misses, all of them the inline `"."` form. The module-aware check resolves all of them.
- Counting caveat: 13 mounts sit on the same line as `mod` (for example `#[path="📏️cells/🦀️.rs"]mod semantic_cells;`) or sit where the simple parser skipped them. I resolved those by hand: all exist.

## 4. Repo-root references

- `📋️build-owners.json` (repo root): 290 owner entries under norm. All `root` and `manifest` paths exist. For `project.json` owners the `name` matches the file. 256 owners use `Cargo.toml` manifests, which is valid.
  - Crates with no owner entry, matched by dir or manifest: 42. They are:
    - Compliance and scene: `semio-s-plugin-norm-ndp`, `semio-s-plugin-norm-structural-scene`, `…-structural-scene-foundation`.
    - App-surface sub-crates: `…-app-surface-live-editor`, `…-visual-chrome`, `…-foundation`, `…-live-core`, `…-panels`, `…-viewport3d`.
    - 10 `…-scene` crates (one per artifact en1990 to en1999, under `🎬️scene`); `…-checks-core`, `…-checks-special`, `…-checks-clauses`, `…-checks-parts` (en1993); `…-checks-core`, `…-checks-specials`, `…-checks-resistances` (en1992); `…-checks-catalogue` (en1997); `…-kernel` (en1995); 13 `en1998-mut-*` crates (foundations, buildings, silos-tanks, bridges-towers, members). Total 42 with the items above.
    - `semio-s-plugin-norm-mechanics`: its `📋️project.json` declares `@semio-s/plugin-norm-mechanics`, but that name is not in `📋️build-owners.json`.
  - Fix: regenerate `📋️build-owners.json` with the repo tooling (not by hand). The `repo` MCP server was disconnected during this audit, so I could not run that tooling.
- `🔒️dependencies.json` (repo root): 379 entries. Norm-related entries: 3 (serde, serde_json and typescript). Their `users` paths all exist. I did not find stale norm crate names in it. 0 problems.
- Not verified: the `packages` arrays inside `📋️build-owners.json` owner entries.

## 5. Files

- Checker: `✏️…/🔍️audits/wave2/manifest-check.py` (kept as an input script; it writes a JSON report only when given an output path).
- Full list of 40 crates that are not in their umbrella's `packages:` list:

```
  semio-s-artifact-norm-vdi3805-examples-failing	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/❌️failing/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-vdi3805-examples-failing-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪜️failing-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-vdi3805-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1991-mut-silos-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️silos-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1991-mut-traffic-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️traffic-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-din18599-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1993-parts	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📏️parts/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1993-checks-special	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🔗️special/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1993-checks-core	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧬️core/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1993-clauses	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1993-mut-general-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️general-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1997-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1997-checks-catalogue	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧬️catalogue-core/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-iso16757-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-iso16757-mut-products-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️products-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1992-checks-specials	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🌉️specials/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1992-checks-resistances	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🏗️resistances/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1992-checks-core	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧬️core/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-din16798-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1990-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-din4108-mut-elements-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️elements-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1995-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1995-kernel	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️kernel/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1995-mut-members-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️members-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1999-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-io-text	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-foundations-foundation	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️foundations-foundation/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-foundations-retaining-wall	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️foundations-retaining-wall/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-buildings-annex-building	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-annex-building/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-buildings-site	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-site/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-buildings-storey-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-storey-2/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-buildings-storey	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-storey/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-buildings-system	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-system/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-silos-tanks-silo-tank	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️silos-tanks-silo-tank/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-silos-tanks-tank	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️silos-tanks-tank/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-bridges-towers-bridge-tower	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️bridges-towers-bridge-tower/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-bridges-towers-tower	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️bridges-towers-tower/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-members-assessment	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️members-assessment/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-members-masonry-member	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️members-masonry-member/📦️packages/🦀️rust/Cargo.toml
  semio-s-artifact-norm-en1998-mut-buildings-2	✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️buildings-2/📦️packages/🦀️rust/Cargo.toml
```
- The 42 crates with no owner entry are listed in section 2 (`manifest-check.py` reproduces both lists from the repo state).
