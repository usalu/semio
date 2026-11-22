# Bounded Reopen of a Large Document Archive

The staged guest ingress reads the existing version-1 memberless archive one ordered window at a time. It verifies the committed extent, minimal field lengths, the final zero-member byte, and SHA-256 before releasing the parent Pack and SPR. Each parent grows in the existing paid paged byte owner. A denied turn preserves the frontier. A cancelled or refused turn leaves its partial pages available for controlled retirement.

The limits for encoded archive bytes, Pack bytes, and SPR bytes are supplied independently. The proof uses an 8,388,608-byte encoded authority and independent limits of 7,340,032 Pack bytes and 65,536 SPR bytes. Those values are test policy. Existing compact command assembly and Store history parsing have their own authorities.

The disk fixture is **5,311,480 bytes** with a 5,311,293-byte canonical Pack and a 179-byte canonical SPR. Its SHA-256 is `cabebea367d5c63654afe2fe35097882c5bf00c983bf8af44906fd9108567c0b`. The Pack embeds a real PDF with a deterministic 1,536 × 1,152 high-entropy RGB image inside a generic test value. MuPDF reopened the PDF and returned the exact image samples. Python independently framed and hashed the archive and SQLite returned identical persisted bytes. The Rust process read through a 4 KiB stack window, retained exact field bytes, and requested at most **4,096 bytes** in one heap allocation. All 13 cases passed, including exact limits, denial, early EOF, incorrect seals and offsets, malformed framing, and cancellation followed by paid retirement.

The typed snapshot receiver stage accepts `RetainedWireByteSource`, including both fields' existing paged owners. It compiled against the linked production Pack, value and native receiving types. The Store hydration stage lets an already decoded snapshot donate a paged `ArtifactGenesisPack` at genesis. The linked law ran that exact genesis admission with the original page pointer, preserved it through an unfunded admission and a paid transfer, and kept the largest heap request at 4,096 bytes. The complete hydration overlay passed the live-preimage patch check. The Store hydration stage still needs a full Store build and a new mounted reopen exchange to drive the existing typed snapshot receiver and history decoder. The two existing app command archive decoders emit contiguous fields, and the mounted maintenance ladder still enters the flat Pack decode. The tested fixture establishes paged custody and byte-for-byte readback at the ingress boundary. Mounted PDF editor edit/save/close/reopen beyond 4 MiB remains unverified.

- [Ingress](large-reopen-ingress.rs)
- [Patch generator for the channel, typed receiver, and Store hydration](large-reopen-stage.py)
- [Native proof runner](large-reopen-check.py) and [test driver](large-reopen-test.rs)
- [Language-neutral fixture](large-reopen-law.json) and [schema](large-reopen-law.schema.json)
- [JSON Schema and MuPDF oracle](large-reopen-oracle.py)
