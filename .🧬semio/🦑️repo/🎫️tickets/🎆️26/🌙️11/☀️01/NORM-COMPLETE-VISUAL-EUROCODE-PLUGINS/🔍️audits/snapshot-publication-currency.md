# Snapshot Publication Currency

Current production publish_output receives a retained original Option<O> from frame.output and executes '*output = frame.output.take_original().flatten()'. The value is moved from retained frame custody into the caller's previously empty output slot during publication; it was not constructed in that caller slot.

Therefore this is an original Option<O> header transfer. Its existing neutral header-copy contract remains applicable: quote and post sizeof(Option<O>) copied bytes plus one publication item, admit before taking the original, and preserve the one original work action. Keep the original object intact under insufficient copy credit. The terminal Box release-only contract is separate and does not justify zero publication copy bytes.

Descriptor owns the scoped production correction and the actual source-custody/denied-publication law. No publication native pass is asserted here. The decision follows the actual current move and ownership shape, not only the old expected fixture.
