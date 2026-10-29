# IDML Boundary Independent Assist

Read-only review of the execution lead's source-written package split. No production source edits, compiler run, source cap changes or test result claims.

The existing descriptor is naturally independent: its sole runtime dependency is the first-party schema registry. Its twenty include_str facet inputs are unique and all resolve at their original paths. These are text facets, not a Rust dependency on the IDML owner; moving the descriptor into its own crate introduces no admission-to-model cycle or duplicate IdmlSnapshot/IdmlPartPatch identity. The canonical schema module reexports the descriptor package explicitly; the snapshot, diff and mutation defining modules remain in the original IDML owner with their orphan-sensitive implementations.

The extracted test root mounts all four original public admission/version/change/part law modules at their original paths, preserving their fixture include paths. Read-only tracked diff shows public import corrections and the XmlSpan qualification correction, with the retained admission algorithms and law bodies otherwise unchanged. The original package intrinsic/native law root is retained in the new test package. The inner cfg(test) marks the purpose-specific test crate, while the ordinary IDML format dependency remains unconditional.

A concrete manifest defect was sent to the execution lead: moved native.rs directly uses semio-framework-pack::snapshot and semio-s-artifact-stdio-contract, but the initially written test package lacked both direct dev dependencies. The lead owns the correction. All five initially declared path dependencies resolve to existing directories. Workspace member publication and canonical source admission remain coordinated by root/Rust; this review does not substitute for their receipt or native execution.

No independent model/fixture was added because this assist changes no implementation behavior and deliberately avoids duplicating the already authored schema/test boundary packet.

The execution lead confirmed both missing direct dev dependencies were added. A direct external crate-name scan across all six moved Rust sources found no further undeclared first-party crate imports. The companion contains thirteen actual test cases, including the retained six native.rs cases. Native execution remains pending.
