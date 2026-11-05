# Result ACK and Store Receipt Review

This is a read-only source audit, not native proof.

FullOp acknowledge_result_page first checks the complete token and AwaitingAck stage. For the seven Store lanes it forwards acknowledgement to the exact retained PendingArtifactStorePublication before advancing result_sequence and the mounted stage. The wrapper delegates to the matching durable, ephemeral or erased-window owner.

Each lower Store acknowledge accepts only AwaitingAck and changes phase to Closing before returning. accepted_receipt is explicitly phase-gated, so it returns None immediately after that transition even though the scalar receipt remains retained for physical closure. A duplicate ACK fails the mounted stage check and cannot trigger another lower acknowledgement or increment sequence.

The acknowledged producer page remains owned. The publication unit and pump service its paid cleanup first. granted_retirement_ready additionally requires result_page absent and the entire MountedPagePublication terminal, so the lower Store source cannot retire while framing/page custody remains. Once producer cleanup completes, the maintenance selector next_granted_mounted_retirement admits a closing Store owner even when the mounted stage is Publishing. The main publication unit waits while pending.is_closing, and the getter cannot re-expose its already acknowledged receipt. No recurring capture or replay of the same accepted receipt was found.

Outbound durable publication adds its existing flush guard: lower acknowledge refuses while outbound remains pending; a refused ACK preserves the mounted AwaitingAck page and exact token. This review did not modify ACK behavior.

The new result-page first-party package now also has canonical Nx metadata: project name @semio-tech/plugin-result-page-rs with scope:framework and language:rust tags, beside its Cargo manifest. This follows the existing retained-command owner template and changes no Rust closure or runtime dependency.
