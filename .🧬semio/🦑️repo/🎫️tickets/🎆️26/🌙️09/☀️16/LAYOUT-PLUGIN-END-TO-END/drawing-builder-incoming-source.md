# Retained Incoming Builder Ownership

Status: authored and uncompiled. No Job or payload runtime was launched by this lane.

The canonical Builder owns incoming:Option<JobPayloadPageSource>. The first accepted handoff stores the exact original backing before receipt posting. An underfilled item/depth or cancelled/yielded turn leaves the input unchanged. Incoming can coexist with the existing staged page; further admission remains blocked without effects until that staged owner is committed or closed. No replacement allocation or retained raw-pointer marker exists.

Subsequent incoming progression uses the same actual ledger alias and fixed counter transaction as the canonical Builder, then transfers the existing backing into staged ownership with zero copied bytes and no allocation. An already held reservation can be reused only at its same exact extent and ledger. Incompatible state retains all real owners on refusal. The allocator path refuses to replace an incoming owner.

The pure incoming_backing_identity observer reports actual retained ownership. Retirement quotes prioritize that backing and require its exact release extent before dropping it. Pending counter/alias and staged close remain separately owned. Empty initialization, terminal checks and consuming take_finished guards include incoming. The terminal take method remains Root-authored.

The closed neutral incomingWork fixture and genuine Original two-backing law were authored before these fields and methods. The law checks denied handoff, exact pointer retention, blocked zero-work advance, denied close preserving both pointers, and exact paid release of both sixteen-KiB backings. It is unrun. Root preserves the older four payload-size matrices in the separate finite payload-domain public-port fixture without claiming Host qualification from that fixture.

The Builder/ledger source candidate is stable for Root review and a separately authorized production check. Broader Core compilation remains blocked by worker source/payer adoption and real driver/receiver integration. This is not a whole-Job SourceReady or acceptance claim.

[Working File Packet](drawing-builder-incoming-working-files.json)
