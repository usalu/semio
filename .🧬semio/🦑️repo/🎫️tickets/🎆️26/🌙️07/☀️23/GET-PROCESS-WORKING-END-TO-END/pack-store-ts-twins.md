# Pack and Store composition TS twins

Result: `bun test` over `🎒️pack/**/🧪️tests` is 115 pass, 1 fail; over `🏪️store/🧩️composition/**/🧪️tests` (excluding the off-limits `🚪️open/🏭️operation/🫴️record/`) is 121 pass, 1 fail (that one failure is the off-limits Record tree). No Rust changed; every failure was TS-twin, schema or fixture drift, no Rust regression found.

## Pack (15 to 1)
- Ajv: draft-07 schemas were compiled with the 2020 Ajv (`json/🧪️tests/🧱️ownership`, `value/🛫️encode/🫳️borrowed/⏳️cursor`); both now use the default draft-07 `ajv`.
- JSON ownership: Cargo dependency set now includes `pack-json-canonical`, `value-native-receiving-original`, `value-ordered`.
- Schema drift: JSON receiving schema gained `ownedHandoffCancelPrefixes`; value receiving schema gained `allocationPolicies` and `documentReceiving.totalAllocationCeiling` (fixtures are truth, Rust reads them).
- Canonical tree: `retainedPolicy.maximumCopyBytes` is 8192 per fixture; owner facade re-exports the carved `semio_framework_pack_json_canonical` crate.
- Source-text laws re-pointed: generic `decode_record_body_exact<`/`admit_document<`, `source.original_source_identity()`, `source.take_authority_into(&mut self.original`.
- Ordered oracle (`value/🗂️ordered` source-contract, imported by the LEB oracle twin): removed the SharedOwnership region whose fixture `👥️shared-owner` no longer exists.
- Remaining, environmental: `json/🧪️tests/🚮️absence` needs `SEMIO_TEST_ARTIFACT_DIR` from the repo harness and, on a loaded host, exceeds its fixed 30 s test timeout (inner isolated `test-reference` run took about 80 s). Not a law drift; run it through `repo`.

## Store composition (46 to 0 outside Record)
- `📨️emission/📦️owned`: close/retire laws moved from `grant: RetainedCloneGrant` to `effect: &mut RetainedAllocationEffect` / `OriginalRetirementPoll`; `admit_original_into`, `advance_returned_snapshot_read`, `M::open_birth_demand(request)`, `return session.close_step(cx)`; preparation close law now slices `close_result_value`.
- `🌳️graph/🤝️group` isolation harness: shims `semio_os_vcs`, `semio_os_store_one_item`, `crate::owned_document_closure` so the production group module compiles standalone. Needs `SEMIO_TICKET_DIR` (harness sets it).
- `🚪️open/📜️history/🏭️factory/🧪️tests/🛬️receiving`: markers follow `cx.retained_grant()`, `parent.admit_loan(&mut *self.source_loan,cx)`, `cx.replacement_is_stale(...)`, `OriginalJobFixtureCaller::admit`; birth fixture snapshot copy is 1048576.
- `🚪️open/🏭️operation/📦️catalog`, `retirement/🌊️hydration-and-source-birth` (12), `📄️catalog-retirement` (7), `📸️snapshot-custody` (10): laws re-pointed to the effect-based owners (`retire_initial_snapshot_owned`, `admit_typed_controlled_retirement`, `box_close_step`, `admit_verified_physical_pack`, `drive_cursor_receipted`, `MemberSnapshotDecoder`), snapshot-custody VCS test switched to draft-07 Ajv, rustfmt path fixed.
- `📬️publication/🤝️group` unit twin passes (only needs `SEMIO_TICKET_DIR`).

## Off-limits, not touched
- `🚪️open/🏭️operation/🫴️record/🧪️tests/🟦️.ts`: "the floor law selects a turn copy strictly between one and two receivers" fails (`turnCopyOverReceiver` ratio is 2.5, expected strictly between 1 and 2).
