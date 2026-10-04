# CAD Semantic Ownership And Direct Consumer Census

## Persisted Authority

The actual Snapshot has nine fields: schema, id, four optional model children, ordered drawings, literal reference groups, and ordered nodes. Children retain local child identity independently from target artifact identity. Each target includes kind, standard, and subset strings; model slots require s.stdio.semio/v1/model and drawing children require s.stdio.semio/v1/drawing.

Six SQL tables contain all eighteen authored fixture rows. Root owns literal schema/id. Model slots carry slot identity and all five child strings; drawings also carry independent ordinals. Reference groups carry literal model-definition keys, including empty, NUL/Unicode, and __proto__ keys with empty groups retained. Each reference carries three strings, origin xyz, optional quaternion xyzw, optional scale, width, two flags, and optional opacity. Ten binary64 fields carry exact integer words and closed finite/infinity/nan class discriminants. Nodes retain ordered id/label/kind strings, including duplicate and empty identities and unknown literal kinds. There is no hidden JSON/Part21/native SQL carrier.

SQL declares 39 reference columns. The authored malformed corpus independently exercises every relationship family, slots, booleans, ordinal uniqueness/gaps, optional quaternion coherence, raw-word/class consistency, and schema pollution. Existing Source independently interprets actual generated files with BunSQLite rather than trusting the same provider's inverse alone.

## Current Concrete Storage Gaps

Native relation indices, group frontiers, ordinal staging and final reference storage use BTreeMap/BTreeSet. Their allocator requests are opaque at this API seam. A count estimate, inline root charge, or tuple-size reservation does not settle their actual node requests. Ordinary derived native BTreeMap construction uses an additional per-entry 128-byte estimate. No current generic canonical admitted literal map was found in the first-party code.

Existing first-party RetainedOrderedMap stores page Vec owners but exposes a separate retained-clone/insertion cursor authority without the current value/DSL ingestion API. Existing graph PropertyBag is a graph domain owner; its implementation demonstrates admitted contiguous members and bounded byte comparison but is not a CAD type to import or wrap. Shared Provider explicitly recommended an actual CAD-owned contiguous reference index.

## Intended Domain Storage

A CAD reference index will directly own Vec<(String,CadReferenceList)> sorted by literal UTF-8 bytes. It is the actual Snapshot field owner, not a mirror lowered after decode. Controlled construction reserves the concrete tuple Vec request using NativeDecodeControl::allocate_vec; each String and reference Vec uses the same cumulative control. Borrowed key comparison advances bounded byte units; insertion/shifts publish bounded row work. There is no root heap allocation to charge unless an actual backing request occurs.

Its semantic API needs len/is_empty, iter/keys/values/values_mut, get/get_mut/contains_key, insert/remove, borrowed indexing, IntoIterator and FromIterator. Ordinary insertion remains available for existing direct synchronous mutation consumers. Controlled insertion refuses if no admitted slot remains and never falls back to uncontrolled growth.

FromValue and DslField ingestion must preserve map wire shape and duplicate-key semantics while constructing this owner directly. ToValue and controlled native encoding traverse its literal entries. Partial typed values retire through the current cold owner. Existing ordinary parser and mutation APIs retain their intended synchronous role; no compatibility wrapper around BTree is introduced.

## Actual Consumers Requiring Pairing

- Snapshot declaration and empty_cad_snapshot initialize final reference authority.
- The forest reference fixture in schema/inferences builds model-definition entries through FromIterator and supplies the actual Snapshot.
- Schema diff stores a sparse map of changed reference lists; its apply implementation inserts each changed domain entry into the actual Snapshot. It need not turn this separate diff authority into a mirrored Snapshot.
- Reference mutation inverses and diffs call get, clone each affected reference list, and emit their sparse diff. Root's shared Mutation inverse Result work owns that protocol boundary separately.
- Editor reference sampling and inspection call get; editor census calls iter/values for semantic budgets.
- Existing Native tests call iter/values_mut/borrowed indexing and compare all literals/raw words.
- SQL projection borrows index entries; SQL reconstruction directly constructs admitted final entries.

Only the real Snapshot constructors and consumers that require the new owner type must change. Other unrelated BTree maps, sparse diff authority and geometry/numeric inverse APIs remain outside this allocation closure.

## Baseline Qualification

The Source baseline has genuine exact-scalar RED and two separately identifiable stale cancellation assertion failures. Root alone will collect Native compiler prerequisites and then actual numerical allocator baseline. No native or Source numerical production repair has been mounted while awaiting that baseline. Read [gate readiness](📓️cad-actual-owner-allocation-gate-readiness.md) for exact executed counts and log.

## Actual Native Hook And Diagnostic API Readback

Current CAD implements the four SQL/native conversion hooks but does not override preflight_sqlite_snapshot_encoding or retire_sqlite_snapshot. The current kernel default preflight refuses UnsupportedOwner; default retirement uses ordinary drop. These facts are part of the staged original I/O baseline, not successful route claims.

The exact first-party positioned error API is TextError::from_value_error(error,TextSpan::at(1,1)); its implementation preserves both kind and message. Compiler prerequisites should call that actual producer instead of inventing a shared DSL helper or converting ValueError to String. Native record ingestion/emission closures require TextError at the physical diagnostic boundary.

A final paid closure needs the actual owner-declared borrowed preflight and typed current cold RetireOwned for completed and partial owners. Cold retirement itself constructs cursor/scaffold allocations; those must be explicitly qualified rather than described as allocation free.

## Current Controlled Derive Boundary

Live restored-lane readback of framework DSL derive lines 800 and 808 shows both __dsl_to_record_controlled and __dsl_from_record_controlled return ValueError. Ordinary __dsl_from_record returns TextError. The actual shared native record producers intentionally accept TextError construction callbacks and return ValueError to the Snapshot trait. The staged [typed prerequisite diff](📓️cad-typed-native-intrinsic-prerequisite-capsules.md) preserves that concrete controlled error through positioned TextError only at those two callbacks; it does not assign ordinary parser diagnostics to the controlled derive or convert the refusal into String. All shared Projection/row/IEEE/schema/native-control producers use direct typed propagation.

## Full Persistent Consumer And Child Retirement Revalidation

CadArtifact in the parent schema owns the same reference BTreeMap as CadSnapshot. Its to_snapshot clones the field, while from_snapshot and set_snapshot move it. Its derived builder's empty_snapshot is another actual constructor. The canonical Vec-backed index must replace both persistent fields and those constructors together; a conversion back to the old document BTreeMap would create an unpaid hidden authority. The root empty_cad_snapshot and forest fixture return/empty constructors are also explicit pairing sites. Current store/♻️retirement/🦀️.rs already implements ArtifactChild<S>:RetireOwned, so child handles reuse this first-party owner rather than a new bridge. The [concrete frontier design](📓️cad-semantic-scalar-and-concrete-frontier-design.md) preserves the exact wire map shape and paired intrinsic value/DSL producers.
