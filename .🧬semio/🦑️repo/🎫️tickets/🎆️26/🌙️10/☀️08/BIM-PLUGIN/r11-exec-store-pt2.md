# r11-store-pt2 execution report (plugin composition tests)

Gate label `r11-store-pt2`, logs `🗑️generated/r11-store-pt2/` (`c*.txt`, `r*.txt` human/short checks, `retry-check` loop status in `retry-status.txt`).
Crate path `P = 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## Files and what changed

### `P/🧪️tests/🧩️composition/🦀️.rs` (main composition fixture + laws)
- `ComposedParentSnapshot`, `RecursiveBranchSnapshot`, `RecursiveFixtureMutation` derive `semio_framework_value::RetireOwned`.
- Hand-written `ComposedParentOwnedRetirement(+Factory)` deleted; the fixture uses `semio_framework_value::retirement::OwnedValueRetirementFactory<T>` (grant-admitted `retire_owned`).
- `ComposedParentSnapshotDecodeAuthority`: four `next_close_*_demand` quotes from one `close_demands`, grant `close_step` (decode cursor cancel = `copied_items:1` + the hex page `released_bytes`; value admitted via `store::artifact_retirement_admit_owned`; box closed via `artifact_retirement_box_close_step`; `Complete` only with the terminal witness).
- `ComposedParentRejectedFieldAuthority` (mutation + SPR conflict): four zero quotes + grant `close_step`.
- `ComposedParentEnvelopeOwnedFieldCatalog` derives `FactoryPayloadRetirement` (catalog is a `FactoryRetirement`).
- `RecursiveBranchSnapshotOpen`: `ErasedSnapshotRetirement` re-expressed on the grant currency (same machine as `store::PackMemberSnapshotOpen`: box -> owned admit -> request -> input backing release).
- Authoring needs an installed original operation wire source (677): `ArtifactCanonicalJson for RecursiveFixtureMutation` (`{"value":n}`), `recursive_fixture_wire_source/_schema`, `recursive_fixture_owners_birth_demand`, `recursive_fixture_owners`, `funded_recursive_fixture_owners`; both `ComposedParentApp::build_document_store_owners` and the member `MemberStoreOwner` impl use them. `build_{document,config,draft}_store_owners` return `Option<Result<..>>`.
- `ReadyComposedParentInitialization`: `retirement_demands(body)` + grant `close_step` (candidate -> boxed retirement -> runtime -> envelope hand-off), `step()` pumps via `retirement_self_grant` + `artifact_retirement_box_close_step` / `settle_current_retirement_step(grant)`; candidate catalogs installed with `store::install_unscheduled_catalog`.
- Store authoring calls go through `crate::with_authoring_identity!` (`dispatch`, `apply_one`).
- Helpers `composed_maintenance_turn` / `composed_close_turn` (self-fund the quoted demand); member/store close helpers use `retirement_self_grant`.
- Laws re-expressed: `maintenance_answers_idle_only_when_every_stage_is_idle` (idle = empty receipt on a quoted grant), `retained_composed_replacement_close_query_forwards_whole_member_page_extent` (release-axis quote equals the 4098-byte page slot; undergrant `plugin_page_grant(4096)` yields with 0 released; funded ceiling drains), `retained_composed_replacement_terminal_store_box_requires_whole_physical_grant` (grant `extent-1` yields with 0/0 heap, grant `extent` releases exactly `extent`).
- `pump_worker_job_retirements(8, runtime_lifecycle_grant())`.

### `P/🧪️tests/🧩️composition/📨️emission/🦀️.rs`
- Helpers `child_page_grant`, `child_release_grant`, `child_zero_grant`.
- `MemberOpenRequest` laws on `close_step(grant)` / `next_release_byte_demand` (one-below page backing yields with an empty receipt; exact grant releases `backing`).
- `Emit::{prepare_child_preview_one, prepare_child_one, close_child_one, return_child_one}`, `ChildEmitPreparation::{step, close_step, retirement_demands}`, `ChildEmit::close_one`, `OwnedChildEmit::retirement_demands` migrated to grants/receipts; `PluginCloseStep` replaced by `PluginLifecycleStep`.
- `TrackedChildRetirement` (+ two factories): grant `close_step`, four demand quotes, `retirement_birth_bytes` + `retire_owned(.., grant)` through `frame::admit_retirement_frame`.

### `P/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🧪️tests/🦀️.rs`
- History pages law: quote `command_log.retirement_demands().release_bytes` == app quote; zero-item and release-one-below grants yield an empty receipt with 0/0 heap; the exact `plugin_demand_grant` releases exactly the backing (`Progress{copied_items:1, released_bytes}`).
- Replacement law: pre-decision turns on the quoted grant (`Progress{copied_items:1}`, 0/0 heap), pruned-owner release turns exact/undergrant, closing cancel turn.
- `🟦️.ts` (same dir): language-agnostic source-anchor law now reads the close ladder module (`self.command_log.retirement_demands()`); **bun: 2 pass / 0 fail**.

### `P/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🧪️tests/🦀️.rs`
- `MemberReceiptPublication` implements `retirement_demands` and the `RetainedCloneStep` `close_step`. `🟦️.ts` unchanged: **bun 2 pass / 0 fail**.

### Owner `P/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/`
- Included driver `P/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs`: terminal retirement loop self-funds `mounted.retirement_demands`.
- `🧪️tests/🟦️.ts`: source anchor follows the lib (`grant.maximum_capacity_bytes<bytes`); **bun 1 pass / 0 fail**.

## Compile status
See the end of this file (filled once the gate reaches the plugin crate).
