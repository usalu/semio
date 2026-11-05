# Original Native Progress Observation

A testkit-only scoped observer now receives actual NativeDecodeProgress before the existing Job cancellation/deadline predicate runs. The observer returns unit and cannot supply a callback verdict. No control, source identity, clock, deadline, grant, ledger or StepContext field was added or replaced.

The implementation stays in the existing Job context testkit. A thread-local optional boxed observer owns the active callback; a Drop guard restores the previous owner before destroying the displaced observer. Nested scopes and unwind therefore restore the preceding observer. Test configuration allocates and destroys observer boxes outside measured original turns. Invocation uses a RefCell borrow and dynamic call, with no allocation.

Schema/corpus and Ajv/lodash/SQLite oracle were authored first. The proper-source law reproduced the missing hook (1pass1fail5assertions); the implementation now passes2tests8assertions.

Two native integration cases are declared in semio-framework-job as original-native-progress, requiring testkit. The first uses one genuine OriginalJobFixtureCaller, persistent decoder and an admitted cancellation alias. It performs256 actual prepared/committed native steps; the observer cancels that alias at completed256, and the original predicate returns cancellation. The observed callback body is checked for zero heap activity. The second checks nested restoration, an unwound nested scope outside any measured turn, and subsequent actual native checkpoints through the restored outer observer.

The IFC completed256 regression is being ported onto the same hook and genuine caller. This retains the existing accepted binary frontier assertion instead of replacing it with an external post-step cancellation.

Native execution remains held by the coordinated compiler owner while rich receiving APIs and callers are brought into agreement. No native green or browser proof is claimed. Final source-cap measurements will accompany the coordinated coherent marker.
