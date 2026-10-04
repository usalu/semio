# Nine Owner SQLite Typed API Prerequisites

Owner SQLite internal chains were ported to the actual shared `ValueError` producer contract on 2026-10-03. This prerequisite change adds no capability hooks, paid SQL copy behavior, PNG strict behavior, schema changes, test changes, fallback, global conversion, compatibility layer, or message classification.

## Boundary Changes

- Binary, CSV, TSV, BMP, PNG, LAS and Deflate to/from SQLite capability methods execute existing logic in typed closures and project `into_message` only at the actual owned `String` capability return.
- GLTF Write/Read, all ten entity modules and encoding preflight remain typed internally. The owned to/from/preflight capability returns project `into_message`.
- Existing shared row/table/float/projection/control refusals preserve their supplied category. Literal/variant defects use `InvalidValue`; row/work/ordinal arithmetic uses `WorkLimit`; byte/storage arithmetic uses `OwnershipLimit`; the checked u32-by-u32 BMP multiplication, which cannot overflow u64, reports `InvariantViolated` if violated.
- LAS and SVG validation explicitly project typed failures at the actual message-only `IoError` return. SVG Tiny/Basic borrowed controlled validators now return `ValueError` internally, with count arithmetic `WorkLimit`.
- Shared XML owner projection/reconstruction/preflight APIs remain actual `String` boundaries; SVG forwards those unchanged. Separate closed logical native helper String cleanup, Native owner behavior mounts, paid SQL copying and PNG strict changes remain staged.

## Verification

Ran `rustfmt --edition 2021 --emit stdout` for all 22 changed Rust files; 22 returned exit 0. Output was discarded, so no generated artifact remains. This is parser validation only. No Cargo, Native Nx, typecheck, feature test, or runtime was run by this agent. Root retains the sole Cargo lane.

## Exact Changed Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 7; terminal `into_message` sites 2.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 0; terminal `into_message` sites 6.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 2.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 2.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 15; terminal `into_message` sites 2.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 13; terminal `into_message` sites 8.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 29; terminal `into_message` sites 7.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🖼️texture/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🧩️extras/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🖌️material/🦀️.rs` — parser exit 0; typed signatures/closures 4; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🎬️animation/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️encoding/🦀️.rs` — parser exit 0; typed signatures/closures 6; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🦴️skin/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/📄️document/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🏔️mesh/🦀️.rs` — parser exit 0; typed signatures/closures 4; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🎥️camera/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/📦️buffer/🦀️.rs` — parser exit 0; typed signatures/closures 5; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🌳️node/🦀️.rs` — parser exit 0; typed signatures/closures 3; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 2.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` — parser exit 0; typed signatures/closures 2; terminal `into_message` sites 2.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🦀️.rs` — parser exit 0; typed signatures/closures 1; terminal `into_message` sites 0.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🧬️schema/🦀️.rs` — parser exit 0; typed signatures/closures 1; terminal `into_message` sites 0.
