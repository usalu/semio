# PDF Producer Closure Memo Boot

The governed PDF producer failed to load the remote generation exchange while its closure memo imports and schema were changing on the shared tree. The current codec reads and writes the checksummed `version`, `sha256`, and `entries` envelope. The schema now requires those three fields and retains its bounded tuple and reference constraints.

The codec obtains its fragment and reference limits directly from the existing generation policy. This removes a dependency on the memo envelope structure at module initialization. The policy and schema limits agree at 32,768 memo entries and 65,536 references per entry. The codec also enforces the overall reference and byte limits and verifies the digest.

A fresh Bun process through `repo tools exec --light bun` imported the full remote generation exchange. Ajv and the memo reader checked five JSON fixtures. The probe verified an independently calculated SHA-256 digest and a successful round trip, and the reader rejected an incompatible envelope, duplicate references, an invalid reference, and a tampered digest. The encoder rejected duplicate fragment names. The governed PDF producer run is tracked separately in this ticket.
