# Strengthened Shared Laws and Small Stdio Readiness Audit

## Shared Six-Law Follow-Up

Fresh read-only source review, no Cargo or runtime execution. The original six actual RED laws remain valid repair authority. Current `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🧮️allocation/🦀️.rs` keeps six tests and closes the three identified evidence deficiencies:

- Copy law47 compares every returned projected UTF8 byte and reconstructed blob octet; reconstructed text48 and repeated literal copies50 also compare exact outputs.
- cancellation_diagnostic_bytes24 obtains actual checkpoint error String capacity using a separate canceled control outside observation. Copy cancellation59 requires exactly one interior cancel, full retained payload admission and **exact** requested = payload + separately measured transfer diagnostic + returned native diagnostic capacity. The former broad lower bound is absent.
- IEEE law77 checks the table remains empty after initial cancellation. Law79 cancels inside actual nonempty text and blob copying, requires exactly one interior cancel and typed Canceled, admits complete payload/Cell/value backing, compares exact observed paid backing plus actual two diagnostic capacities, and checks no committed row.

No new concrete defect was established in this narrowed review. These source assertions await Root's current whole38 execution; this audit does not claim GREEN. The independently measured diagnostic allowance is bounded and cannot silently exempt arbitrary extra allocations as the prior lower-bound assertion did.

## Five Small Stdio Native Drafts

Each actual Snapshot already mounts its SQL trait provider and opts into ArtifactPack's SQLite codec. Each provider currently omits decode_sqlite_snapshot_native and encode_sqlite_snapshot_native, so both strict trait defaults remain the active hook authority. None of the five Snapshot roots includes its adjacent 🚦️native module. Draft existence must not be conflated with runtime mount.

### 💾️binary

Draft `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs`: native entry/control lines 8, 10, 21, 24. Actual mounted provider `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`; Snapshot Pack opt-in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` lines 80.

### 📊️csv

Draft `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs`: native entry/control lines 16, 18, 31, 32. Actual mounted provider `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`; Snapshot Pack opt-in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` lines 219.

### 📑️tsv

Draft `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs`: native entry/control lines 9, 11, 20, 21. Actual mounted provider `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`; Snapshot Pack opt-in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` lines 147.

### 🪟️bmp

Draft `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs`: native entry/control lines 27, 35. Actual mounted provider `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`; Snapshot Pack opt-in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` lines 143.

### 🗜️deflate

Draft `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs`: native entry/control lines 26, 28, 44, 46. Actual mounted provider `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`; Snapshot Pack opt-in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` lines 135.

## Concrete Mount Prerequisites

All five drafts create a fresh NativeDecodeControl/NativeEncodeControl with **limits.max_value_bytes**, not the caller's remaining construction allowance. None contains allocation_stage or owned_bytes settlement. Mounting these bodies unchanged would reset owned allowance and omit native ownership from the persistent transfer ledger, including refusal and cancellation. Adapt actual producers through remaining-allocation child stages and settle actual child owned bytes on every result; keep semantic max_value_bytes separate.

All five draft decode/encode entrypoints return String; lower helpers also return String and repeatedly project ValueError::into_message before the actual capability terminal. Typed refusal classification is therefore lost internally. Convert concrete scan/copy/size/physical helpers to ValueError results, with explicit InvalidValue/OwnershipLimit/WorkLimit/AllocationFailed/Canceled choices and no diagnostic message reclassification. Preserve actual helper failure causes until the existing trait terminal String projection.

The upstream current Semio framing APIs remain mixed: wrap_binary_controlled and wrap_text_controlled return Result<_,String> (`🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:142,211`), while unwrap_binary_controlled and split_text_preamble_controlled return SemioResult at166,225. Therefore a local native Result signature change alone cannot guarantee typed preservation through framing; an explicit typed framing authority is required without parsing existing messages or adding a legacy overload.

Deflate's actual first-party DeflateEncodeControl adapter already returns ValueError from admit/checkpoint, but encode immediately projects the physical encoder refusal to String. The bare compress_zlib/decompress_zlib exports also return String; their interfaces are separate from the two strict SQLite native trait hooks and cannot establish hook readiness. CSV's field_value uses try_reserve_exact but maps allocation failure to an untyped String; its source allocator is concrete and should preserve AllocationFailed before any terminal conversion. BMP has its own handwritten numeric/stride/header readers and Binary/Text physical copying; these must preserve typed errors and settle actual allocations too.

No external runtime dependency was introduced or required by these draft bodies. This audit did not compile their syntax/types or assert semantic fidelity, exact carrier schema retention or cancellation cleanup. Full runtime45 baseline belongs to Root; mounting should follow genuine owner assertions rather than treating these draft entrypoints as completed implementations.
