# Store Native Allocation Bridge Readiness

2026-10-03. Execution ownership: shared Store snapshot-capability Native record helpers; no TIFF/HTML production changes and no Cargo invocation. Root owns sole Native verification lane. This report supplements the read-only native-allocation-bridge-unconverted-footprint audit.

## Contract and Mounted Test Scope

Semantic `max_value_bytes` counts domain SQLite values. Independent `max_allocation_bytes` counts admitted owned backing retained cumulatively by one `SqliteSnapshotControl`, including successful, refused and canceled Native producer stages. Child `NativeDecodeControl`/`NativeEncodeControl` must receive `allocation_remaining_bytes`, return their actual `owned_bytes` with their result, and settle that amount through the existing core `allocation_stage`. No refunds or renewed per-stage ceiling. Current production helpers are deliberately unchanged before genuine new helper/owner assertions.

New hand-authored neutral JSON fixture/schema and Source/Native facets live under `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🧪️tests/💰️allocation`. Literal `文🌠` repeated20,000 times is140,000 UTF-8 bytes. Contract explicitly separates semantic allowance1, allocation allowance16MiB, tiny allocation1, and success/refusal/cancellation reference ledgers. Ajv validates the closed contract; independent Bun SQLite checks literal text and actual UTF-8 BLOB byte length; fatal TextDecoder verifies the exact valid interior boundary. Reference arithmetic settles each admitted stage regardless outcome.

Four staged Native owning laws are mounted in the actual public `sqlite_snapshot_native_admission` integration binary, selector `sqlite_snapshot_native_allocation_`:

1. Shared encoder success settles backing; exact first-used allocation ceiling succeeds with identical bytes, then the same caller refuses second metadata allocation before literal construction. Native backing remains independent of a semantic allowance1.
2. Shared decoder parser+binding success settles backing; exact first-used allocation ceiling succeeds, then the same caller refuses before a subsequent binder. Semantic allowance1 alone does not constrain Native backing.
3. Both actual input/output and Binary/Text routes cancel inside the literal byte-copy frontier. The admitted canceled backing remains charged. Replaying with that exact allocation ceiling leaves zero capacity, and the same caller refuses another producer without resetting its ledger.
4. Explicit typed InvalidValue refusal after controlled ownership retains admitted bytes and the exact owner-authored error message at the helper's existing String terminal. ValueError becomes positioned TextError via `from_value_error`, rather than an invented global String conversion.

New integration module path is mounted only as tests in `snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs`. Existing portable ownership Source facet imports the new Source laws; no new script or target was created. Existing shared output fixture callbacks were narrowly aligned from generated ValueError to the helper's positioned TextError boundary. Its older `max_value_bytes=1` ownership-negative assertion remains until runtime authorizes semantic/allocation contract correction; the staged bridge will require that old ownership-only assertion to use max_allocation_bytes instead.

## Public JSON and CommonMark Consumers

One new actual declared public I/O law per JSON/CommonMark owner is mounted in its existing snapshot SQLite Native facet, names `sqlite_snapshot_json_public_allocation_budget_is_independent_of_semantic_values` and `sqlite_snapshot_md_public_allocation_budget_is_independent_of_semantic_values`.

Each builds the actual artifact declaration, exports Binary/Text full semantic SQLite files directly from the typed snapshot, reimports the whole owned state while asserting no Native encode/decode phases, and queries the actual file independently with Bun SQLite integrity/FK checks. Independent SQLite row values recompute exact semantic bytes (numeric8, null0, UTF-8 text and intrinsic BLOB width) and agree with the physical reader. That exact semantic ceiling must admit actual erased native output with independent default backing capacity. Allocation1 must refuse the actual public native output. These laws do not establish the JSON independently handwritten input bridge or whole public-transfer cumulative backing.

Current actual I/O `run_snapshot_hop` creates fresh SqliteSnapshotControl instances for separate provider/check/metadata operations. Therefore the helper bridge alone cannot demonstrate a single ledger across physical SQL, projection, reconstruction, metadata and multiple hop phases. This concrete separate-control gap was reported to Root. Source/physical/reconstruction slot allocations remain outside this scoped proof.

## Unmounted Implementation Drafts

Complete literal replacement drafts, with no inclusion/mount, are adjacent:

- `snapshot-capability/🛬️native-decoding/🚦️allocation/🦀️.rs`
- `snapshot-capability/🛫️native-encoding/🚦️allocation/🦀️.rs`

They retain the actual RecordSpecProducer/envelope/parser/binder/emitter pipeline. `allocation_stage` supplies remaining backing; each inner Native operation returns `(result,native.owned_bytes())` on success and all ordinary Result refusal/cancellation paths. Actual ValueError and positioned TextError conversions remain at existing declared String terminals. No ordinary parser/printer fallback, domain schema inference, altered semantic SQL budget or carrier payload authority was added. The production helper files have not been modified.

## Executable Routes and Actual Evidence

Existing owning kernel target physically resolves through `os/📦️packages/🦀️rust/📜️script.ts` SnapshotNativeAdmissionTestScript and selects `semio-framework-os-kernel --test sqlite_snapshot_native_admission`:

- Root Native baseline: `bun nx run @semio-tech/framework-os-kernel:test-snapshot-native-admission --skip-nx-cache --args='quick sqlite_snapshot_native_allocation_'` (four staged laws; unrun).
- Source-only: `bun nx run @semio-tech/framework-os-kernel:test-snapshot-native-admission portable --skip-nx-cache` (the portable branch returns before Cargo).
- JSON owning selected Native: `bun nx run @semio-tech/stdio-json-rs:test --skip-nx-cache --args='quick --lib sqlite_snapshot_json_public_allocation_'` (one staged law; unrun).
- CommonMark owning selected Native: `bun nx run @semio-tech/stdio-md-rs:test --skip-nx-cache --args='quick --lib sqlite_snapshot_md_public_allocation_'` (one staged law; unrun).

Actual uncached Source-only Nx receipt: exit0,10passed0failed,188assertions,674ms Bun /8.4s Nx. Includes the two new independent allocation laws (58.92ms and8.51ms). Full evidence `🗑️generated/native-allocation-bridge-source-portable.log`. No Cargo command executed in this lane. Rustfmt emitted the new Native facet, both unmounted drafts and two owner facets successfully; parser-only evidence is not compilation or runtime proof.

Four focused invocations are authored in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, orders408.793–796. Bun jsonc-parser validated both catalogs and found all four entries. Existing scripts/targets remain the sole executable owners.

## Changed Files

Shared capability new files: `🧪️tests/💰️allocation/{🦀️.rs,🟦️.ts,🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}` and the two adjacent unmounted helper drafts listed above. Existing test mounts: `🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs`, its `🧩️ownership/🟦️.ts`, and `🪶️native-encoding/🧪️tests/🦀️.rs` positioned fixture callbacks. Artifact test files: JSON RFC8259/base and CommonMark/any `🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`. Both launch catalogs and this ticket report. No production consumer mount, runtime acceptance or universal allocation-completeness claim.

## Current Native Baselines and Fixture Prerequisites

Root CommonMark public receipt is a genuine allocation-contract RED: Nextest `cf89b1c9-3b33-4c12-b438-40410e58db11`,25selected/24passed/1failed,47outside,750ms assertions/18.8s Nx. All24 preceding owner laws passed; the new public allocation law alone failed because semantic SQL allowance constrained actual Native backing. Log retained by Root. JSON public law has no corresponding current runtime receipt here.

The first shared four-law baseline did not reach assertions: `root-authentic-shared-store-native-allocation4-authentic-red.log` contains32 fixture compiler diagnostics. Six measured files were narrowly repaired: canonical Value retained/staged DslField test controlled methods now return ValueError; OS schema decoder fixture metadata producers and Store buffer fixture producer return ValueError; actual String-returning retained/expansion encoder fixture methods explicitly map Native ValueError via into_message; allocation/output fixtures use actual TextSpan::at while retaining all coordinates. No production shared bridge changed. All six repaired files passed rustfmt parser-only emission. No Cargo was invoked in this execution lane. Remaining compiler/runtime verification belongs to Root; the four owning allocation laws still have no genuine assertion receipt.

Current physical SQLite authority readback now returns ValueError from checkpoints, row/value checks, allocation_stage and typed getters. The same three capability fixture owners preserve it until their actual String hook terminals; the two unmounted replacement drafts explicitly map only checkpoint/allocation-stage outer errors to their declared helper String terminals. The initial IoResult subset fixture required a further explicit actual IoError terminal conversion, recorded below. All five subsequent parser checks exit0. Core physical whole32 GREEN was independently reported by Root (Nextest06d51e39-5e6a-445f-87f8-387a478da196,173ms/Nx16.5s), and is a separate scope from the still-unexecuted shared helper four laws.

### Subsequent Thirteen Fixture Gates

`root-authentic-shared-store-native-allocation4-current-typed-prerequisites.log` still reached zero assertions and contained thirteen current compiler gates. Twelve were physically retained preflight fixture calls to newly typed NativeEncodingBound::{new,add,repeated,finish}; those declared String hook terminals now explicitly call ValueError::into_message. The thirteenth was the semantic validator fixture's checkpoint: actual IoError has no From<ValueError>, so the fixture now maps at its actual IoError terminal via `IoError::from(error.into_message())`. The earlier suggestion that direct typed propagation compiled here was incorrect and is superseded by this readback. Both repaired files pass parser-only rustfmt emission. All positions, refusal messages, bounds and observations remain unchanged; no shared Store bridge implementation was mounted.

## Authentic Four-Law RED and Authorized Shared Mount

Root's corrected owning baseline actually executed all four allocation laws: Nextest891d7b1c-7e50-4cef-a84e-3e7e3b395349,4selected/0passed/4failed,56outside,287ms assertions/25.9s Nx. `root-authentic-shared-store-native-allocation4-complete-fixture-prerequisites.log` measures real success/repeated-binding/refusal/interior-cancellation ledger failures after the fixture compiler gates were fixed. Root explicitly authorized the shared helper mount on this evidence.

The actual Store `🛬️native-decoding/🦀️.rs` and `🛫️native-encoding/🦀️.rs` now use the existing allocation_stage. Each Native controller receives the caller's remaining independent allocation allowance and returns its actual admitted owned_bytes alongside its operation result, so failed/canceled construction settles its backing just like success. Current IO-authored typed terminal maps, envelope/producer/record guards and physical parser/writer behavior were preserved through narrow edits. Only the two preexisting Native-ownership fixture limits were switched from semantic max_value_bytes to max_allocation_bytes: decoded compressed/aggregate backing and tiny controlled native output. Exact limit values, callbacks, completion counters and assertions remain; semantic SQLite rejection laws remain unchanged. Parser-only checks for both helpers passed, but mounted runtime verification is pending Root's four-law/public/whole retries. No Cargo command was run in this execution lane.

The two adjacent allocation replacement draft files were removed after applying their behavior to the actual owning implementations; they are not alternate mounted consumers or compatibility paths. STEP/IFC Native owner drafts remain separate and unmounted. Earlier report sections describe the preauthorization state and are superseded by this mount receipt.
