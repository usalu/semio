# GLTF, LAS and Deflate Controlled API Prerequisites

The earlier independent readback identified retired typed API shapes in these staged owner-local implementations. Root repaired only those API prerequisites; Native feature baselines and subsequent behavior mounts remain separate work.

GLTF controlled field projection and controlled record allocation now use the existing `OutputError::Refusal(ValueError)` conversion, preserving the actual refusal kind.

LAS controlled field failures now use `TextError::from_value_error` at the real positioned boundary. Invalid literal diagnostics explicitly use `InvalidValue`; arithmetic and caller row refusals explicitly use `WorkLimit`, based on the producing predicate.

The Deflate encoder bridge now implements the canonical control trait with `Result<(), ValueError>`. Child control failures retain their categories. File bounds and file-size arithmetic use `OwnershipLimit`; an impossible backwards progress event uses `InvariantViolated`. Only the existing owned String return boundaries project `ValueError::into_message`.

All three files parsed with `rustfmt --edition 2021 --emit stdout`, exit zero. Deflate first exposed an extra closing parenthesis in the narrow patch; Root corrected it and reran parsing successfully. Parsing does not establish typechecking or runtime behavior. The Deflate first-party engine Cargo edge and owner hook remain staged before the authentic owner Native baseline.
