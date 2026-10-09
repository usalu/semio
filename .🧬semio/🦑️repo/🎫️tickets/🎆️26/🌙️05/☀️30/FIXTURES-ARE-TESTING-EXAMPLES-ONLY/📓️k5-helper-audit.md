# K5 helper visibility audit

Actual retained k5 diagnostics contain46 located compiler errors, zero native assertions. Eight helper references used crate::os_store::presence_test_retirement; defining cfg(test) module resides inside os_store::component at Store5796. Fresh original Presence/read Sources already use crate::os_store::component::presence_test_retirement. Helpers expose pub(crate) items; original nested test descendants can reach the defining module without exporting a test helper as runtime package API. This fixes an import path, not native proof.

Read native Ordering unresolved at12 and SNAPSHOT_READ_LEASE_CAPACITY at14 belong defining component context. Use explicit std::sync::atomic::Ordering and component’s private constant from its test descendant; do not make public runtime API solely for tests or hardcode1024. Fresh current read Source uses funded helper closure and actual System receipt equality, with capacity/depth demands compared to independent fixed policy.

Remaining k5 errors include21 SnapshotRetirementStep uses, twelve encode/decode progress/signature epoch references, and one RetireOwned generic bound at retained-clone test561. These are outside helper import mapping and cannot be called fixed by path correction. No new compiler run or post-first46 type success is inferred. Later compile/runtime may reveal additional errors; System receipt equality is an actual pending oracle, not a source-shape success.

Preserve final-registry physical receipt regression, same original pointer/backing and supplied grant/refusal axes. Three fresh observer hashes are in 📥️k5-helper-audit.json; concurrent Root changes not adopted.
