# Typed Read Lease Ownership

The actual Store read registry currently owns opaque Arc roots, independently allocated return flags, and a boxed fixed slot table. The new neutral Value primitive will own typed roots and inline generation-stamped return flags, admit its Arc and fixed backing before construction, and keep each original root until full granted physical closure. Store remains the event publication authority and mounts this primitive.

The closed schema retains every original sparse/wrap/starvation case verbatim and adds work pages 1, 3, and 64 with empty, Unicode, and long payloads. Constructor refusal and every capacity/release one-short refusal must leave ownership and allocator counters unchanged. Every admitted turn must forward logical copying, new capacity, physical releases, and depth separately. Original roots plus constructor/scaffold births must equal observed frees. Third-party Serde and JSON Patch independently produce the same visitation and serialized payload results.

No live native handle was present at source publication. General35 and Scene6 remain their original accepted cuts; this is a new test-first cut, not current runtime acceptance.
