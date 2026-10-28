# IFC Native Pack Boundary Plan

Continue after the native domain reaudit fixes. This is scoped to the IFC graph/codec owner and its IFC4x3 native route; TS IO modules and native editor/schema hooks are untouched.

The shared native Pack grammar is the existing literal flattened Frame in IFC4 model `io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`. It is a generic graph carrier, not the IFC4 strict STEP codec. ADD2 must continue through that literal graph carrier without FILE_SCHEMA rewriting or IFC4 STEP admission.

Planned owner files are new leaves next to that Frame: original Pack receiving, incremental flat-frame-to-graph reconstruction, and original final Pack writing. The parent Frame/Entity/Complex/Node declarations gain only source-owned receiving derives and module mounts. Actual public APIs live on the shared IfcSnapshot/its explicitly reexported receiving cursor. New receiving implements the actual OriginalArtifactPackReceiving and OriginalArtifactSnapshotReceiving interfaces. Every parser, owned binding, flattened source, graph value/row, pending output and final wire must retain its original recipient, ledger identity, source bytes and independently supplied grant until funded take/closure.

The incremental reconstruction preserves existing wire scalar domains, full header roots, ordered simple/complex instances, aggregate and typed values. It enforces forward-only flat children, exactly one ownership use, bounded depth/work and canonical binary values without whole-input scans. No default-control or guessed-grant constructor is introduced.

Normal IFC4x3 route entries will use these owner APIs for original source-pack receiving and final IFC Pack publication, while their graph projection continues to use the shared actual IfcSnapshot. Prefer a small normal declaration-entry constructor that receives/retires typed source and children over custom hidden fallback dispatch. Generic ArchiveChildren/diagnostic custody must be resolved or explicitly refused where the necessary owner interface is unavailable. Narrow SQLite controls remain narrow.

Tests precede each binding: neutral Pack ownership/depth/binary/complex cases; canonical reference Pack equivalence; actual OriginalArtifactPackReceiving prefix/denied/cancel/close tests; registered IFC4x3 to/from FEM host route roundtrips; native output IfcOpenShell validation only when actual Rust execution produces the bytes. Current compiler ingress remains blocked before rustc -vV; source/model probes must remain labelled unexecuted and never substitute for native evidence.
