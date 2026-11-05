# Live Gumball Source

## Actual Source

P3 `commands/transforms` owns `GumballGestures { open: BTreeMap<String, GumballGesture>, revision: u64 }`. A gesture contains the original machine states, verb, authoring seed, committed revision, transaction, owned typed entries and addressed IDs. `revision()` changes on gesture writes. The existing `provisional(host)` synchronously walks every open gesture, reads its original net leaf, derives splice rows from the actual base and addressed IDs, appends the relative transform leaf, and computes following selections.

The active P3 tick first replays original context provisional values, then calls this live gesture producer on the resulting host. The new retained22/22 mutation replay preserves the second half once actual typed rows exist; it does not produce the original live gesture splice.

## Custody Requirement

The retained job must hold the actual immutable gesture authority or an issuer-backed original projection across turns, authenticated by genuine instance owner and gesture revision. The current public API exposes no such source. A plain borrowed `with_mut`/inspect callback reference cannot be manufactured into a RetainedCloneRef or retained projection. Retaining only a gesture digest or typed relative leaf loses the original addressed IDs and base-relative splice semantics.

A correct defining owner repair can expose a genuine stable source and bounded original gesture/entry/ID visitation. Merely using `values().nth(i)`, `range(...).next()` or cloning the BTreeMap hides traversal/comparison/allocation. Donating the map out of the live instance without preserving its logical view would temporarily erase open gestures and requires a real restore/invalidation protocol.

No such substitution or synchronous fallback was added. The active tick remains unchanged until this source and the bounded splice producer are complete. Parent root has the exact remaining boundary; no lower source edits or native commands were started for this research.

## Original Immutable Gesture Read Mounted

The original GumballGestures owner now uses the existing persistent OrderedMap for its canonical open windows, preserving lexical window ordering and actual revision/mutation behavior. OriginalGumballRead aliases the actual immutable map root under the initiating OriginalSourceBinding and retains actual shared key/gesture owners at each rank. It exposes the original window and addressed-ID slices plus copied motion scalars only after readiness. It scans the first native transform entry, matching the existing cold gesture semantics; it neither calls ctx.provisional nor clones a gesture. Source identity is the actual canonical owner address, root and revision. Writers replace their canonical root while the old read retains its original immutable entries. Close retires every actual alias through original raw work and requires an unconditional terminal Drop.

The instance operation owner now closes this actual gesture root through the original lifecycle grant/retirement context before session cleanup. The same real ScratchCaller helper funds native source laws; actual first-party Trace was added only as the defining transforms package's dev dependency, authorized by Root and confirmed outside the held lower compilation closure by Runtime. Twelve schema-first cases passed the independent Ajv/SQLite original/live-state oracle before production edits. Three native laws are mounted for lexical capture, source/writer substitution, exact original pointer preservation, actual cancellation/denials, and complete physical cleanup under the original one-fuel/one-or-512-item caller with unchanged 200-call bound. Native typing and execution remain NOTRUN.

The remaining P3 gesture stage is bounded splice generation. Every window must derive its splice from the SAME immutable context-overlaid host base; earlier windows' folded mutations must not become later windows' input. The read source above solves live gesture custody but does not yet implement those splices or migrate the active gumball writer. A defining paid scratch ownership transfer and immutable base/working-copy separation is required before active P3 tick composition. Current active Tick branches still contain cold scene/evaluation calls.

## Shared Tick Authority Groundwork

Both actual P2/P3 instance owners now bootstrap the real operator registry and own FlowOperatorCatalogueOwner. Original maintenance builds/publishes the actual catalogue; ordinary command retention must match its registry identity and generation. Closing uses the same actual original lifecycle grant and raw retirement work. The shared app catalogue lifecycle module forwards the actual posted financial progress and complete original failure rather than inferring CPU from success or copied items. No cold catalogue derivation was added to a tick. This new app lifecycle batch is syntax checked, not native typed.
