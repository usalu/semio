# Rooted Borrowed Canonical Traversal Design

## Public Safe Contract

Keep typed scalar/indexed support for existing Config visitors. Add an optional borrowed-root method to ArtifactCanonicalJson and require Sync for worker transfer. Public borrowed values contain a scalar, a borrowed typed source, an array iterator, or an object iterator of borrowed key/value pairs. Constructors accept Send iterators with lifetimes tied to the source borrow. Native slice/map iterators advance once per node; no ordinal nth, range search, key rescan, serialization, or collection is permitted.

Flow accepted the proposed exact names: ArtifactCanonicalJsonValue, ArtifactCanonicalJsonArray::new, ArtifactCanonicalJsonObject::new, and canonical_json_borrowed_root. Framework wrapper traversal owns Edit/metadata. App-specific recursive Widget/Dictionary/Tree functions can return values without orphan trait implementations.

## Store-Private Rooted Lifetime Capability

The only lifetime projection is private inside the sealer encoder, tied to its exact immutable boxed edit. Public callers never supply raw pointers, hash state, bytes, or unsafe proof. Before reading retained frames, verify the original root allocation identity. Clear every traversal frame before moving or retiring the edit. Declare the encoder before the edit owner so Rust field-drop order also destroys borrowed iterators before the root during unwinding.

The capability is never serialized. Checkpoint replay reconstructs fresh borrowed traversal from the frozen root and replays actual bytes with the existing authority/prefix checks. Send is allowed only when the typed source is Sync and iterator state is Send.

Cancellation retires one borrowed frame per grant, then domain mutation/post-root owners. Native borrowed iterators do not own root payloads; their bounded setup/next/drop remains an app-reviewed source obligation. A custom iterator that clones/collects or hides an unbounded destructor is not granted boundedness by the type alone.

## Required New Laws

Language-neutral nested maps, empty maps, Unicode/control-escaped keys larger than4096bytes, sorted serde object order, small byte grants, measured key bytes, cross-worker live-owner transfer, serialized checkpoint replay, cancel in every encoding phase, exact frame-before-root drop ordering, root/iterator destructor counts, stale/rebound root rejection, and serde_json/Node crypto parity. Existing seven sealer tests remain regression gates.
