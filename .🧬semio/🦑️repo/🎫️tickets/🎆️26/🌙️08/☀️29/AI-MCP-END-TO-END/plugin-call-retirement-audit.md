# Plugin call retained retirement audit

The native one-cell inference ran for about 308 seconds, returned `INTERNAL`, and verified that the 306-byte pack and 204-byte SPR remained exact. Its worker panicked at the `GuestRelayOperation` Drop invariant. That run did not emit the detailed relay refusal trace, so it establishes a nonterminal operation at Drop and leaves its first failure phase open.

`call_plugin` attempts close, reads and retires the close refusal, tries close again, and previously discarded the second status. `GuestRelayOperation::close` stores any close error in the cleanup-failure slot. Its terminal condition requires that slot, the operation result, handle, any next hop, and the Source itself to be retired. As a result, a second unsuccessful close can mask both diagnostics with the worker panic.

`close_operation` settles against the same Source. An outstanding receipt at the original deadline or an unchanged no-progress cleanup turn is an owned refusal. A blind repeated close has no new authority to settle either. In particular, the root close step returns pending while guest escrow remains, and a paid refusal reader needs a live, ready root. A trace of `close.contents`, `close.source`, or `retire.cleanup` should establish which owner actually stalled before changing a close transition.

Keep the Drop invariant. The appropriate fix is to repair the witnessed ownership transition or give the still-owned terminal state back through an existing funded custody path. An ignored result, forgotten owner, replacement Source, or unbounded retry would hide the problem or abandon part of the original operation.

A later full-length replay used the same pinned one-cell pair and enabled relay diagnostics. It reported a single semantic deadline refusal with its original cleanup window retained. It returned without a close error or a Drop panic and verified the exact pair again. The earlier panic remains timing-dependent. No close-state change was made on that evidence.
