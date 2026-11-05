# 🔍️ Auxiliary Publication Diagnostics

The native admission failure remains unproven at a specific write boundary. Earlier profiling finished outer source capture and memo replay but did not reach runtime-entry phases. Auxiliary FD 3 publication is a strong candidate, not an established cause. No admission, capacity, runtime-store or dependency-store setting was bypassed or weakened.

The owned auxiliary publisher previously forwarded the original error without its callback or event origin. It now supplies a closed writer phase as a second refusal callback argument, preserving its raw first primary value and identity, including explicit undefined. Owned command execution wraps only auxiliary transport failures in OwnedAuxiliaryPublicationFailureV1, retaining the exact original cause and a frozen descriptor/phase/code/syscall/operation context. Cancellation, deadlines, progress and output-consumer failure ordering stays unchanged. The diagnostic includes descriptor and writer phase even when native error formatting discards the stack. Native data properties are read without invoking accessors or proxy traps.

Added schema-owned phase and diagnostic context definitions and language-neutral fixtures for missing pipe, error event, premature close, end callback, and synchronous end failure. The focused standalone test uses fake ports, independent Ajv validation, Bun equality assertions, first-failure retention and accessor safety; it needs no manufactured script birth and starts no real child or large stream. Execution is queued through the ordinary shared capacity gate. No new cargo invocation was started. A green pure-port test will establish diagnostics only, not a reproduced or repaired host EPERM.

Verification: the focused pure-port suite passed all six tests with 40 assertions in 214 ms after its ordinary host-capacity wait. Actual transport failure location remains to be established by the next genuine repository invocation.

Verification: the focused pure-port suite passed six tests with 40 assertions in 214 ms after its ordinary host-capacity wait. Actual transport failure location remains to be established by the next genuine repository invocation.

Root execution subsequently reproduced the genuine old restricted-namespace admission failure and the new diagnostic identified descriptor 3, end-callback, code EPERM, syscall write after captured source memo replay and before runtime entry. A fresh invocation under the current full-access environment is owned by the root agent; transport remains unchanged pending that outcome.
