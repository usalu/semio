# Plugin shared birth callers

The declaration context handle consumes the source and installs a sealed shared value in one return-by-value operation; the gesture issuer also calls SealedShared::prepare_birth. Neither currently exposes a retained original allocation marker through the complete caller operation. Both require genuine staged owner and marker custody when the shared birth contract is ready.

Plugin testing/caller contains the non-test OriginalCallerSourceParent::admit invocation. Declaration job birth, declaration media publication/producer, cancellation issuer, and media turn tests also construct that parent. Their signatures and original caller receipts need migration together; a transient marker that vanishes across a callback unwind would not satisfy the new contract. The shared API is in flight, and no compilation is claimed.
