# Serializer Host Producer Audit

The current Draw PNG Serializer leaf still projects the document through SemioDrawingSnapshot. Its source states gradients, blend modes and fill rules are lost. The new owned exportDocument command uses the native document scene pipeline and genuine captured snapshot read; these are separate production paths. No end-user equivalence is established yet.

The inspected framework trait is io::io_mechanism::Serializer<S>. serialize borrows &S and ArchiveChildren and returns a Send future. IoEntry.run is a synchronous fn(&IoPayload)->IoResult<IoPayload>; its typed entry wrapper invokes resolve_ready on serialize. The host currently assumes serialization completes on the first poll. Adding yields to a leaf would violate that bridge, and cancellation of a future holding unretired physical owners would violate ownership.

The separate plugin::ArtifactSerializer trait is also a borrowed async interface; its comments explicitly describe first-poll completion. Neither gives a StepContext, cancellation token, captured store read, close demands or owned publication result.

Completion requires a first-party owned IO producer capability in the actual routing host, with declared work/source/pixel/output ceilings and genuine snapshot custody. Its lifecycle should match existing InteractiveJob: admitted construction, bounded advance, retained original owners during failure/cancellation, close_step with exact demands, then segmented artifact publication. The source should expose the typed Serializer job capability through the existing IO registration rather than a PNG-only host special case. All IO entry constructors and any generated metadata consumers must be updated schema-first in the same integration lane.

The current command route can continue independently and supplies the near-term production end-user PNG export route. The old Serializer route cannot be declared repaired by swapping a synchronous helper or by pumping unlimited grants in an async body. This audit records source findings only; no host behavior was run.
