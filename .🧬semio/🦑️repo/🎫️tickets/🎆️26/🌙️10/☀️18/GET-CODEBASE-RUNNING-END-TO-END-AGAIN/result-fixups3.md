# result-fixups3

Executor `fixups3`. Input: `audit-verify-1019d.md` (I3, I4, I8, coordinator item 5). No build, no `bun repo`, no cargo, no modifying git command, no sub-agent. Rust edits are NOT compiled (see Verification).

## Changes
1. I3 mesh facade (`L3 = 🧰️framework/🔨️modules/🧊️3d`)
   - `L3/🥽️mesh/🦀️.rs`: removed the three explicit `pub use` lines for `semio_framework_3d_mesh_{journal,surface,quality}` (nine names). Repo-wide grep (tracked and untracked, tickets excluded, wasm-gated code included) found exactly two importers of those names: the `mesh-surface` and `mesh-quality` `[[test]]` targets of `semio-framework-3d-mesh-tests`, which reached them through `semio_framework_3d::mesh::*`.
   - `L3/🥽️mesh/🎨️surface/🧪️tests/🔬️unit/🦀️.rs`: `use semio_framework_3d_mesh_surface::MeshSurfaceJob;`
   - `L3/🥽️mesh/🔎️quality/🧪️tests/🔬️unit/🦀️.rs`: `use semio_framework_3d_mesh_quality::{analyze_polygon_soup, MeshQualityReport};`
   - `L3/🥽️mesh/🧪️tests/📦️targets/📦️packages/🦀️rust/Cargo.toml`: dev-dependencies on `semio-framework-3d-mesh-surface` and `-quality`.
   - `L3/📦️packages/🦀️rust/Cargo.toml`: removed the `-journal`, `-surface`, `-quality` dependencies added only for the facade (the file is now identical to HEAD). No other consumer of the facade names exists; journal's own tests only use `MeshModelingOps`.
   - Not touched: the pre-existing glob `pub use semio_framework_3d_mesh_document::*;` (audit section 3; out of this task).
2. I4 `FileStamp`: no change needed, narrowing is correct. All users of `FileStamp` are in `🛎️services/👀️file-watcher/🦀️.rs` (the struct, `DirectoryProbe`, the probe step, `OwnedFileChangeWatcher` private channel/`previous` types). HEAD made it `pub` (and `cancelled`/`previous` `pub`) only because the external companion crate mounted `🔬️file-watcher-unit` through `testkit::file_watcher`. That mount is gone; the unit test is now `#[cfg(test)] #[path] mod tests;` inside the watcher module, a child module that reaches the private items through `use super::*`. Required visibility = private, which is the current state. Nothing in `🛍️products/💻️os` or elsewhere references `FileStamp`, `.previous` or `.cancelled`. The reactor-turn `#[doc(hidden)]` drops are not part of this task and were not changed.
3. I8 `[DEBUG]`: removed the `console.log("[DEBUG] Actual host refused ...")` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/🌱️birth/🛂️host.ts` (file last modified 12:29, no other agent in the last 30 min). The preceding `expect(message)` assertions stay.
4. Item 4 (`semio-framework-server` test closure): left to tco2. `result-tco2.md` does not exist yet and a `bun repo cargo ... check --tests -p semio-framework-server` run (started ~16:16) is active, so tco2 is on it.
5. Coordinator item 5: re-pinned the two stale owner-projection sources, the way the law documents (current-source approval; the pinned policy and its baseline preimages are untouched, so nothing was weakened).
   - `📚️library/🔣️taxonomy.json` (`semanticOwnedFileProjectionContracts.readme-license-owner-leaves-v1.currentSourceRevisions`):
     - `testing-readme-owner-current`: `🧪️test/README.md` currentPreimage updated to sha256 `7e449d5e5d9e867f2f78cfecbf9d2dc45f80c90622a1fb6e388197d100069ca5`, 15997 bytes; new `approvalSha256`.
     - new `devcontainer-readme-owner-current`: `.devcontainer/README.md`, baseline commit `101a6b4ea83acc82d6fbdc0607e6ae5d876825ae` / blob `bef162d2bd2970294ae0a16469b0250f5f41a030` (the 14260-byte, sha `97d64e93...` version equal to the policy preimage), currentPreimage sha256 `198022b8f5bf77c0ef69f37813e9b74c4dade651ea6b4e4b9d3a1b5029c20759`, 9380 bytes.
   - `📚️library/🖼️assets/🪪️current-source-approval/🔣️.json`: regenerated for the test README.
   - new `📚️library/🖼️assets/🪪️devcontainer-current-source-approval/🔣️.json`: approval document for the devcontainer README (policy identity unchanged, sha `82d188a5...`).
   - `📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts`: the "all unrelated policy owners retain their own strict preimages" test excluded only the first approved source; it now excludes every source in the registry (`Object.values(registry).some(...)`). Required because the registry now has two rows.
   (`📚️library` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.)

## Verification
- `bun build --no-bundle` of `🛂️host.ts`: ok. `bun test ./<🛂️host.ts>` (explicit file): 2 pass, 0 fail.
- `bun test` explicit files (SEMIO_TEST_ARTIFACT_DIR set, sequential):
  - `🧪️tests/🏠️owner-projection-policy/🟦️.ts`: 35 pass, 0 fail (was 34 pass, 1 fail before).
  - `🧪️tests/🔖️readme-current-source-revision/🟦️.ts`: 33 pass, 0 fail (1 fail after adding the second row, fixed by the filter above).
  - `🧪️tests/🟢️readme-current-source-activation/🟦️.ts`: 10 pass; `🧪️tests/🖼️runtime-taxonomy-asset-paths/🟦️.ts`: 1 pass; `🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts`: 2 pass.
  - `🧪️tests/🗺️testing-readme-coordinates/🟦️.ts`: 12 pass, 2 fail (`case-phase-selection` prose and the documentation gate registration vs the Nx route). Both unrelated to approvals (the third failure from docs-build2, the stale approval, is fixed). Left to the docs/testing owners.
- Rust: `bun repo cargo ... check -p semio-framework-3d-mesh-surface/-quality/-tests/semio-framework-3d/os-services --lib --tests`: NOT RUN. A `bun repo dependencies sync --all` (pid 80068, started ~16:17) was running. Static evidence only: grep shows no remaining user of the nine names outside the three owning crates and the two test targets that now import them directly.

## Open items
- Compile the mesh family (`semio-framework-3d`, `-mesh-tests` incl. `mesh-surface`/`mesh-quality` targets, `-mesh-journal`) and `semio-framework-os-services --lib --tests` once the dependency sync is done. The tests crate's `[[test]]` targets for surface/quality are the only place the facade removal can break.
- `.devcontainer/README.md` was modified at 16:11 by another agent; the approval pins sha `198022b8...` (9380 bytes) at 16:25. If it changes again, regenerate the second approval (same two files plus the taxonomy row).
- Pre-existing `[DEBUG]` panic messages (stall diagnostics, not console logs) remain in `L3/🥽️mesh/🛠️modeling/🔁️reconstruction/🧪️tests/🦀️.rs:46` and `L3/🥽️mesh/🧪️tests/🔬️jobs/🦀️.rs:217,254`; out of scope.
- The new untracked files (approval JSON, the three mesh crates' `📦️packages`) must be tracked before ticket close.
