# Replication Lower Cut Independent Review

Read-only review, 2026-10-15. Read `replication-error-owner-current.md` first. No gates, Originals, compilers, lock updates or production edits. Supplied neutral BOTH 2/38 and strict0 results are not independently rerun; Native/lock remain pending.

Paths below are beneath `🧰️framework/🔨️modules/📡️replication/` unless stated otherwise.

## Source ownership

The single nominal `ProtocolError` now lives in `⚠️error/🦀️.rs` with its Error source, PackError/PackRefusal conversions, Diagnostic fault conversion and adjacent `♻️retirement` owner. Its retirement still refuses genuine transport provider causes instead of inventing disposal authority (`♻️retirement/🦀️.rs` transport branch). The original four retirement test bodies remain adjacent, including actual provider pointer identity, all-axis refusal/conservation and borrowed-versus-owned closure cases.

The lower crypto package imports only that nominal lower error and contains the existing RecordHasher/Signer/SignatureVerifier ports. Its production dependency is first-party error only; error production dependencies are first-party Pack error, Diagnostic and Value. Serde, Trace and async test macros are dev dependencies. No added external runtime dependency was found.

Parent package manifests declare both lower owners directly. Parent lib glue explicitly names `semio_framework_replication_crypto as crypto` and explicitly reexports the error plus four retirement functions. The wire facade still composes crypto ports at lines 80–83; this is coherent named lower composition with no wire-owned error type. Root wildcard `pub use crate::wire::*` therefore does not recreate the removed nominal error. Targeted searches found no remaining old wire ProtocolError import in reviewed Replication/OS SPR paths; OS SPR imports/reexports the explicit parent ProtocolError. Existing wildcard wire consumers still receive crypto ports via that deliberate facade. No broad downstream compilation proof follows from that textual search.

## Package and physical Source paths

Both new Cargo manifests have explicit lib roots `../../🦀️.rs`, finite unchanged caps (error 65536/12/1000; crypto 16384/4/250), and first-party relative dependency paths consistent with the domain nesting. Error's allocator path `../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs` resolves from the actual error domain, not the Cargo package directory. Native fixture include paths remain in their actual adjacent test directories. Permanent owner scripts route real package-native tests and the portable contract rather than source-mounting another package. Framework Cargo members include both new paths at 80–81; taxonomy registry entries and native project descriptors are present. These are Source/path checks, not Cargo resolution or cap-runtime qualification.

## Two test-claim limits

1. `⚠️error/🧪️tests/🦀️.rs` test `pack_conversion_keeps_the_exact_original_provider_cause` constructs only scalar PackRefusal variants and compares displayed source text. Its name/debug wording suggests provider identity, but that test alone proves nominal conversion/source exposure, not the identity of an owned provider object. The separate original retirement test `protocol_transport_cause_refuses_without_dropping_original_provider` does test the actual transport source pointer, so coverage is present; rename the scalar conversion law or keep reports precise.

2. The portable error fixture includes a `crypto` scheme/key/message/signature corpus, but `🔐️crypto/🧪️tests/🦀️.rs` currently hardcodes corresponding values instead of consuming that corpus. Thus the crypto Native law demonstrates borrowed-address and canonical error-type seams for its hardcoded probe; it does not establish that the authored portable crypto corpus drives Native execution. No cryptographic algorithm correctness is claimed by identity-probe traits. Future corpus edits could diverge unnoticed until the Native law actually reads the same data or independently asserts exact equivalence.

## Result

No blocking nominal ownership/facade/path defect found in the reviewed Source cut. Preserve pending Native and lock status. Tighten the two test claims above; neither supplied neutral/strict success nor this read-only review qualifies native allocator/retirement execution after the package move.
