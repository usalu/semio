# Gzip Worker Proof Contract Independent Review

Recorded 2026-10-21T19:43:52.068Z. Supports actual95408 repair without repeating the role inventory. Bounded read-only producer/consumer analysis; no Source execution, CLI/Native run, cache read/mutation or compatibility fallback. Finite9input pins are in `🗑️generated/2026-10-21-gzip-worker-proof-independent-pins.json`.

## Required Authentication and Membership

Actual encodeGraphRecordsV1 hashes compressed gzip bytes for each record filename. Manifest digest instead hashes its exact UTF8 bytes. There is no independently authenticated inflated-record SHA field. Therefore worker row.hash must remain SHA256(packedBytes), matching filename and existing parent expectedHash/retirement member joins. Inflated JSON is authenticated transitively by gzip decoding of those same pinned packed bytes; comparing its SHA to filename would reject valid producer output.

Producer v2 manifest has graph remainder object, projects[name,compressedDigest] rows, dependency compressedDigest array and optional externalNodes compressedDigest array. A manifest must emit every digest as a required sibling snapshots/digest.gz link. Existing parent reads all classified rows, requires linked paths present with missingforbidden and retains physical row/hash fences; a new DTO is unnecessary for this membership proof if every linked gzip row authenticates its own packed filename. Missing/partial/nonhex/corrupt manifests/records continue refusing. Do not soften unknown path classification. Duplicate project names may conservatively refuse as not producer-emittable.

Graphrest and record references require complete bounded traversal of JSON keys and values: project fingerprints may occur inside project metadata/tasks, dependencies inside arrays, external fingerprints in external-node objects. Rehydrating the full graph is unnecessary for reachability. Every embedded SEMIO_SOURCE_CLOSURE/SEMIO_CARGO_LOCK marker must retain strict64hex+termination validation and candidate inventory join. Manifest project/dependency/external digest fields identify packed records, not source/lock inventory fingerprints and not inflated hashes. Actual BuildGraphV1 makes namedInputs and externalNodes optional; the manifest must not inherit obsolete whole-v1graph required fields.

## Decode and Budget

Worker bytesAt already charges packed physical read bytes against the original shared proof budget and checks bounded nofollow descriptor/stamps. Before gunzip, authenticate packed digest; maximumOutputLength must be bounded by actual producer records.maximumRecordBytes and remaining proof credit. Charge inflated bytes exactly once, and report expandedBytes=packed+inflated so parent accepted/deferred proof work agrees with actual read/decode. Existing original control/deadline/cancellation, child limits and session terminal/debit responsibilities remain at their owners; no funding reset or renewed credit is justified.

Current graph record limit is268435456bytes and current ReadResult row expandedBytesmaximum is536870912. Gzip overhead can make packed+inflated slightly exceed twice the raw maximum for incompressible records. The repaired closed row bound must intentionally cover the actual packed+inflated envelope, or refuse a lower producer-owned limit explicitly; do not silently undercount inflation to satisfy the old wire schema.

## First Failure and Test Requirements

Preserve physical/read/gzip/UTF8/JSON/schema/reference primary failure through cleanup and worker failure packet. Add the new finite kind to bounded PayloadRefusal diagnostic eligibility, so decompression/authentication failures retain original contextual facts. A refusal must not become empty success, an optional missing link, a broad unknown-retention exception or a fresh proof budget.

The meaningful actual-producer regression must include a v2 manifest plus project/dependency/external gzip records with source/lock fingerprints at all three locations, mandatory missing-record refusal, compressed-byte tamper/name mismatch, malformed gzip, expansion bound, malformed JSON/root and missing inventory references. Python gzip/hashlib independently checks the actual producer compression/hash/decode relationship; SQLite independently checks retained reference reachability. This analysis is not an executed regression or final candidate clearance. High owns schema/corpus/implementation and Root owns final selection/dispatch.
