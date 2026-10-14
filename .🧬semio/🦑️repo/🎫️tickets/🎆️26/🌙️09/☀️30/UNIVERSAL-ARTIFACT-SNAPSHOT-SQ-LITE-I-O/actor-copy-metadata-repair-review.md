# Actor Copy Metadata Repair Review

Read-only source inspection, 2026-10-13. Parent reports prior Kernel2458 stopped before Kernel tests on six Actor E0277 errors; no runtime claim.

`🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs:37–40`: existing OriginalJobCallFailure now derives Clone and Copy alongside Debug/serde. Its only payload is the first-party Copy ValueRefusalKind; Semantic has no payload. Existing artifact_retire_leaf implementation remains the one implementation found. These derives correctly establish that the marker owns no dynamic cause/message backing; they do not change OriginalTurnFailure or source retirement accounting.

`🎟️retained-turn/🦀️.rs:4–8`: RetainedTurnInput was already Clone/Copy, with three u64 identities and Copy RetainedCloneGrant. New artifact_retire_leaf is the sole retirement implementation found; no derive RetireOwned competes with it. The existing macro at Value retirement91 reports inline copy width and delegates to existing leaf cursor/quoted cursor birth; it does not claim a heap-backed input field exists. Controlled retirement machinery can still allocate its priced cursor when used; Copy metadata itself is not an allocation.

No duplicate implementation or non-Copy owned field found in either repair. Metadata remains subject to original identity/epoch/grant validation and cannot itself issue a new caller turn.

Initial fresh Kernel43120 log `🗑️generated/registered-full-snapshot-file-native-actor-headers.log` contains only original Nx structural admission/graph preparation through elapsed40007ms. It contains no emitted new compiler frontier or Kernel law result at inspection. The unchanged first concrete result must be read from that original handle/log; no duplicate invocation made.

Subsequent read of that original log shows terminal Cargo compilation failure on three Job library errors, zero Kernel laws: E0053 root3604 TortureJob::step returns ValueError against canonical OriginalTurnFailure; E0308 worker/admission/preparation14 returns Option<&ValueError> from now-canonical step_fault; E0308 preparation15 assigns ValueError to that canonical field. This frontier precedes confirmation of the repaired Actor compilation. High owns these ongoing Job spans; no edits made here.
