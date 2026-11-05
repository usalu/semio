# Semio SDK Target Review

Reviewed the actual natural_file_sdk Rust target and root Cargo manifest without compilation. All new local devdependency paths resolve to the exact package names, and every requested testkit feature is declared. Mesh and Flow populated demos are public without testkit; the earlier blanket statement that every demo needs testkit was too broad. Base uses the correct editor component export.

NaturalFileCodec uses static string fields format_kind, extension and media_type plus bool binary, matching these assertions. ArtifactEditor is the correct SDK trait. First-party to_json_string takes ToValue and returns String; independent serde_json parsing and equality therefore use the actual public bridge. Every neutral pointer names a real top-level snapshot field. The populated demo inputs include all selected fields; object mesh is Some and base subset wraps populated Flow.

The target invokes all 19 actual editor encode/decode implementations, requires nondefault and schema-correct content, checks exact snapshot and independent JSON roundtrip, deterministic bytes, empty/truncated/corrupted refusal, and 361 cross-subset decoder cells. Each actual editor mounts its own format before the cross-subset table runs. No concrete API/path/feature mismatch was found in this bounded review. Actual Rust compilation and execution remain pending.

Scope: snapshot natural payloads only. Object/Kit/Model child references do not prove recursive archive-member preservation; authored example selection, UI edit actions, browser persistence/reopen and native physical lifecycle require their separate tests.
