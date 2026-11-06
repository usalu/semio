# FEM Explicit Cold Issuance

Read-only actual API correction: SharedIssuer currently exposes owned(), not typed(). Value shared sealed owner has SealedShared::admit(value,issuer,grant)->RetainedWorkOutcome<(Self,Progress),(ValueError,T)>. Exact birth grant is items1,capacity SealedShared::<String>::birth_bytes(),depth1; this can seal an existing authoring String without a NativeDecodeControl or NativeSnapshotFailure. Existing sealed frontier tests use that exact grant.

Clean cold construction can validate the complete authoring graph first, issue node ID and material ID/name through SharedIssuer::owned(), build public FemNode/FemMaterial fields, and collect into existing PagedList FromIterator. Keep this explicitly cold. Each later error must retain and paid-close already admitted sealed rows; unadmitted value remains in the returned (ValueError,String) pair. Do not silently discard that pair or convert rich interactive failure. Node/material validate() reuse scoped identity/finite property rules; collection-level duplicate/reference checks must also remain, not only row validation.

Current FEM native.rs Result losses: primary selects only primary/quote/allocation cause; NativeReceive advance selects .result.map_err(primary) twice (2D307/341,3D301/335); rx.take and rx.close_step also select .result and lose actual work; generic advance wraps selected validation/take/frontier errors in ValueError. receive_authoring and validate_authoring still Result boundaries; from_reference currently delegates them through a fresh huge local decoder. Root must separate cold direct issuance from rich interactive advance rather than flattening. Existing native close_step already returns RetainedWorkOutcome and must retain its child work.

Exact shared implementation: framework/value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs. Node/material and persistent collection paths are listed in 📥️fem-cold-and-relational-ports.md. No source edits/jobs.

## Correction: persistent list construction

The snapshot fields use `PersistentList`, not `PagedList`; the latter’s `FromIterator` does not apply. Cold construction must fund the persistent builder or explicit leaf and branch admissions, retain partial sealed row/page owners, and close or recover them on every later refusal. The former collection shortcut is invalid. `SealedShared::original_transfer_demand` and `try_unwrap` can recover unique staged handles under an exact grant, but refusal must not abandon the returned owner.
