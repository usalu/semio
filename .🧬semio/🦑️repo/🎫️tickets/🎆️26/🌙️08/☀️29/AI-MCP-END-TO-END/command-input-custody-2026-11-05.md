# Command Input Custody

Authored schema-first page ownership cases and a native physical-pointer law before the borrowed admission implementation. A pending or rejected page must remain at its actual caller input; accepted admission must preserve the original 4096-byte backing rather than cloning it. SQLite provides an independent decision oracle.

A separate critical gap remains: a decoded page cannot outlive its OriginalActorInputReceiving decoder ledger merely by a Rust move. The final command path needs a checked same-Source decoder ownership handoff, or a retained stable input owner whose actual page remains borrowed through command execution. Current raw page-set readers also contain unqualified drop/release paths. Borrowed admission alone does not qualify that transfer.

Applied no-clone borrowed page-set admission: validation happens before the caller slot is taken, and success reuses the exact original backing. Shared existing by-value admission uses the same checked invariant. Portable contract oracle passed 1 test / 11 assertions. Native physical-pointer law remains uncompiled, and Source decoder-to-command transfer is still a required separate proof.

CommandPageSet, PagedCommand and PagedCommandReader now expose controlled owned retirement through the existing first-party derive. A new native allocator law measures real page backing (4096 bytes per page), actual deque capacity and every retirement scaffold against paid progress. The old ordinary page reader still has unfunded release methods and is not wired into a native Source owner; this new owned-retirement path is a required foundation, not proof of the final command path. Native laws remain uncompiled.
