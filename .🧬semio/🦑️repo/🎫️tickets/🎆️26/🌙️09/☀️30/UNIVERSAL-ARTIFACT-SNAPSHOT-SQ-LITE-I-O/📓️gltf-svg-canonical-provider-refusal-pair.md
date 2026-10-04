# GLTF and SVG Canonical Provider Refusal Pair

## Actual mounted scope

GLTF's actual Snapshot imports its SQL module and owned Pack module. Its mounted provider implements SQL projection, reconstruction, preflight, native decode and subset validation. Native encode remains the shared default. SVG's actual Snapshot imports its SQL provider and implements projection, reconstruction, preflight and subset validation; native decode and encode remain the shared defaults. No module, provider opt-in, native hook or default implementation was added.

GLTF's adjacent Pack/🛫️encoding file is unmounted. Its typed preparation remains unmounted; actual DslField implementations and native encode availability were not filled in.

## Change

The mounted SQL hooks/preflight return ValueError directly; existing semantic helper errors retain their authored kinds. GLTF native decode keeps controller errors intact, uses intrinsic SemioError and actual protocol PackError enum conversion, and retains positioned DSL error.kind/error.message. Input file refusal is OwnershipLimit; missing logical RecordSpec is UnsupportedOwner. The existing declared owned Pack reconstruction producer now returns ValueError directly from the controlled binding and backing admission, with its collection byte overflow authored as OwnershipLimit.

GLTF subset control/schema/data errors retain ValueError until one IoError::from_value_error boundary. Existing exact table/row/value comparison, reconstruction owners, document retirement, traversal, raw IEEE companions and u64 split-word pairs remain unchanged.

SVG subset control/root/document errors use IoError::from_value_error; literals have explicit InvalidValue, and an unknown named subset has UnsupportedOwner. Existing typed XML SQL/preflight helper calls and Tiny/Basic conformance diagnostics are unchanged. No ordinary SVG parser or writer was edited.

The unmounted GLTF encoding draft now uses ValueError throughout instead of a local OutputError enum and String terminal projection. The unused Field(String) case was removed. All actual literal field construction, owned record guards, BFS extras frontiers, UTF/text/byte admission and callbacks remain unchanged.

## Exact files

- [📸️snapshot/🪶️sqlite/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs)
- [📸️snapshot/📦️pack/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs)
- [📦️pack/🛫️encoding/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📦️pack/🛫️encoding/🦀️.rs)
- [📸️snapshot/🪶️sqlite/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs)

## Verification

All four rustfmt --edition 2021 --emit stdout parser checks exited 0; no in-place formatting. Receipts are 🗑️generated/gltf-svg-typed-parser-0.log through -3.log in this ticket. This does not establish Rust type checking or runtime behavior.

No Cargo command, fixture/schema/test change, Source rerun, or Native run was performed. Root owns subsequent authentic GLTF23/SVG10 Native cohorts and the existing unsupported-default RED behavior. Ordinary PackError::Schema constructors were left unchanged: the actual Store alias is protocol::codec::PackError, which still contains Schema(String). No message classifier or new compatibility conversion was added.
