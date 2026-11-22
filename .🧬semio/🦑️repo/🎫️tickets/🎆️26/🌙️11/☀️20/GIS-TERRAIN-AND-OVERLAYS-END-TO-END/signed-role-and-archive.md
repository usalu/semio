# Signed Role Switching and Archive Export

The signed Terrain role switch captures the exact published node, role and document owner. The session gate drains actor turns and seals new Editor work. The worker confirms the captured client has a live, acknowledged Hub frontier and an empty mutation backlog. The successor reopens the same document with its new surface through the ordinary verified Hub checkpoint and Commands tail. Its local shell instance is not the document authority. A failed successor opening attempts to restore the predecessor role when the original owner remains current.

The shell's Export Document action reads the existing schema-owned archive from the signed actor. The worker sends the standard Read Document Archive app command and checks that the same artifact and base revision own the reply before and after capture. The closure contains the parent Pack and SPR plus the owned member packs defined by the existing archive codec. Cancellable, bounded worker pages carry the bytes to the shell, with progress and exact request, client and Space/document matching.

Archive Import continues to open a separate local document, as declared by that shell action. The signed Check In request still uses the acknowledged Hub frontier with the same extract used before a role switch. A reload before changes reach that frontier remains subject to the transport's existing outbox behavior.

## Verification

- The archive and role transport fixture passed four cases. Its independent Buffer oracle matched the reassembled bytes, and it exercised an out-of-sequence page, rejection of unrelated client and Space responses, and exact cancellation.
- The ownership fixture passed all 12 cases, including the prior role after a same-node replacement.
- Strict TypeScript traversal of the actual worker entry and shell archive entry reported seven existing Diagram, NodeGraph and Interpreter errors, with no diagnostic in the edited paths.
- The touched source passed Git's whitespace check. The complete signed journey and large native archive emission remain unverified in the browser.
