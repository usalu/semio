# Ephemeral Transfer Preparation Ownership

Exact replacements in Wires, Block3D, Note and other ephemeral owners repeat the same preparation, base-read, cancellation and disposal mechanics. These mechanics belong to Store, while the domain supplies its constant-work admission callback, infallible owned mutation-to-state transfer, and exact owned state/mutation retirement factories. The shared helper must not encode, clone, diff, apply or scan arbitrary values.

The infallible transfer follows successful admission. A consuming fallible callback returning only an error String would lose the rejected mutation's retirement authority; fallible or base-dependent computation stays in a custom preparation. Zero item grants cannot call the transfer. A positive item with zero content-copy bytes can move already-owned payload exactly once. Cancellation and unused prepared roots drain through supplied retirement factories with terminal-empty checks.

A neutral four-case fixture was authored before implementation, including zero items, zero bytes, cancellation, and an ordinary transfer. Native tests compare the transferred String with serde_json, assert its allocation address is unchanged, and close 12KiB inputs/results with one-item/seven-byte grants. Native validation is pending.

## Shared Construction Lifecycle

The transfer implementation now uses a Store-owned preparation lifecycle shared with partial-mutation construction. `ArtifactEphemeralPreparationTask` receives immutable base state, retained mutation ownership and a bounded grant. The Store retains input/base/result authority, validates progress receipts, and handles cancellation plus terminal-empty retirement. Domain tasks only construct the next state and close their own scratch buffers. Writer needs this form because changing selection or lint generation preserves a potentially large engagement String; copying the whole base in one callback is inappropriate. Its construction task will copy UTF-8 bytes under the grant and retain partially copied buffers for bounded close.

The direct transfer API remains unchanged. Its three native tests cover the four neutral grant cases, handed-off/aliased roots and large abandoned base roots, and intact oversized-input rejection. Native validation is queued in `ephemeral-transfer-preparation-native-1.log`; no runtime pass is claimed.

## Block3D Consumer

The concrete world-window preview now uses the transfer factory with explicit scalar state/mutation retirement. A new test publishes the existing neutral codec vectors and closes each publication with one-item/one-byte grants. `block3d-window-transient-native` now selects all preview tests and is registered in the root project and both launch registries. Native validation is queued in `block3d-transfer-native-1.log`.

Writer now uses this shared construction lifecycle. Three neutral cases cover selection, lint and input replacement with a 12KiB Unicode base; each also cancels before publication and closes with one-item/one-byte grants. The task copies bytes into a preallocated bounded buffer; it converts ownership to String only after copying the complete immutable source, preserving UTF-8 without another whole-input validation scan. Native validation is queued in `writer-partial-native-1.log`.

## Writer Oracle Result

`abstraction-ownership-validation:writer-window-state-oracle` exited 0. It executed the new three 12KiB UTF-8 vectors through the TypeScript mutation implementation, Ajv schema validation and independent JSON Patch, then the existing two-window ownership/generation vectors. Debug output is in `🗑️generated/writer-partial-oracle-1.log`. This establishes multi-language expected values; native byte-grant construction and cleanup remain queued.

## Independent Lifecycle Audit — 2026-09-09

### Actionable: admission measures logical length while transfer retains allocation capacity

The direct transfer moves the owned mutation into the next root without normalizing its heap allocations. The affected preflight functions count `String::len()` (and Puzzle 2D collection lengths), so an owner with a short logical value and a capacity greater than the 1,048,576-byte Store maximum is admitted and then retained. For example, `String::with_capacity(1_048_577)` followed by `push('x')` passes each single-string preflight below, and the transfer then moves that allocation unchanged.

- Note: `note_composite_window_transient_preflight` adds only `transient.engagement_input.len()` at `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:141-145`.
- Puzzle 3D counts the two popup identifiers and engagement input by length at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:143-151`.
- Puzzle 5D counts the engagement input by length at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:160-164`.
- Puzzle 2D charges logical string lengths and vector lengths, then visits nested values, but never charges `String::capacity()` or any retained vector capacity at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:181-220`.

This is not repaired by retirement: `RetireOwned for String` retires logical bytes, then drops the now-empty vector and its entire allocation (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:76-103`). That final allocator operation is acceptable once a bounded allocation was admitted; it cannot make an unadmitted allocation satisfy the Store bound. Charge retained capacities for every owned `String`/`Vec` (including nested Puzzle 2D values), reject overflow before `begin`, and add short-content/oversized-capacity fixtures for every affected owner. The existing unit fixtures exercise logical payloads and one-byte retirement, but do not construct an oversized-capacity owner; e.g. the Writer fixture builds its 12 KiB value with `repeat` at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/🧩️partial-construction/🔣️.json:2-6`, and the direct-transfer rejection covers a large capacity only for its standalone String factory at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🧪️tests/🔬️unit/🦀️.rs:62-75`.

Writer is not part of this finding: it checks both mutation capacity and the preserved base capacity before it creates its task at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs:28-43`. Block3D's transferred preview contains only fixed-width scalar state at `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs:85-113`.

### Actionable: a construction-task error leaves a live publication without Store fault cleanup

Both transient and presence publication advancement propagate `owner.advance(..)` with `?`, before setting the publication fault or beginning close (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4571-4584` and `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4931-4944`). A task error therefore returns to the caller while the publication still retains its preparation, base read, mutation, and task. Dropping it then violates the terminal-empty assertion at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3901-3904`, unless every caller knows to initiate close manually.

The Writer task has a real fallible advance path: its fixed-capacity scratch allocation uses `try_reserve_exact(..)?` at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs:58-61`. Store already handles a stale-generation failure by recording the fault and calling `begin_close` at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4922-4926`; it needs the same owner-retaining transition for a domain task error. Add a Store unit task that deliberately returns `Err` after retaining scratch authority, then prove that fault close drains it with one-item/tiny-byte grants. No current test exercises this path; the Writer law only calls successful `advance_publish_one(..).unwrap()` at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧪️tests/🧩️partial-construction/🦀️.rs:23-48`.

### Open unknown: duplicated bounded preflight work for Puzzle 2D

`TransientStore::begin_publish_one_leased` runs `factory.preflight` before it takes the base read (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4867-4880`), and `ArtifactEphemeralTaskPreparationFactory::begin` runs it again (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:41-47`). Puzzle 2D's preflight is a bounded complete traversal with a temporary pointer queue, not a scalar measurement (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:181-214`). It stops when the fixed Store byte ceiling is exceeded, so this is bounded rather than an unbounded lifecycle defect. The unresolved contract question is whether preflight may perform this bounded linear work twice. If it may not, pass the Store-approved footprint into `begin` (or make the checked admission witness non-forgeable) so the second call cannot repeat traversal or allocation.

## Audit Follow-Up

The Store preparation-error path now records the fault and starts retained close for both presence and transient publications. This also covers a task returning an invalid Prepared receipt: the shared owner keeps that result for exact retirement. A new four-case native regression (two fault types × two lanes) requires unchanged neutral state, an automatically Closing publication, and bounded terminal cleanup without a caller repair step. The existing transfer-native filter includes it. Native validation is still queued.

The Note/Puzzle execution agent is correcting heap-capacity admission and adding oversized-capacity/short-payload rejection tests. The audit distinguishes fixed allocator operations from traversing or copying payload; Writer's one bounded uninitialized allocation and final release of an empty allocation are permitted metadata operations. Its UTF-8 copying and partial buffer retirement respect each byte grant.

## First Native Attempt

`ephemeral-transfer-preparation-native-1.log` exited 1 while compiling the kernel test target. It reports 335 cascading errors, beginning with MutationLeaf source-authority failures: SPR testkit Rust mutation leaves have their descriptor/schema files left in fixture folders. This is the same source/metadata ownership pattern already corrected for the SDK testkit. No transfer native test ran. The source-authority metadata locations will be corrected before the next native attempt.

## Consumer Capacity Admission Repair

The actionable capacity finding is repaired in the four assigned consumers without changing the shared transfer factory.

- Note charges the retained engagement-input String capacity.
- Puzzle 3D charges engagement input, suggestion-menu window id, and suggestion-menu vortex id capacities.
- Puzzle 5D charges engagement-input capacity for both exact Board and World owners through their shared transient preflight.
- Puzzle 2D charges both top-level Strings, top-level candidate Vec element capacity, nested String capacity, nested array/object Vec element capacity, and object-key String capacity. Checked arithmetic rejects cumulative overflow or any value beyond the Store maximum before the bounded traversal queue grows.

The new native unit cases use short Strings or reserved empty Vecs with capacity greater than the Store maximum. Rejected requests must return the same pointer, content, and capacity. Each exact returned mutation is then closed through its RetireOwned cursor with one-item/one-byte grants until a terminal-empty witness is present. Accepted multi-kilobyte logical payload and tiny-grant retirement cases remain for Note and Puzzle 2D/3D/5D.

Exact touched production/test pairs:

- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}

No Cargo command was started for this repair because the parent owns the shared native queue. The prior Note check session is orphaned and has no terminal result; it is not counted as validation.

## Third Native Attempt: Exact Factory Test Setup

`🗑️generated/ephemeral-transfer-preparation-native-3.log` reached four selected native tests after 61m52s of compilation. Three transfer tests passed. The new fault-close regression failed before publication because its presence setup installed one `Arc<DemoSnapshotRetirementFactory>` and passed a separately allocated `Arc` to `begin_publish_one`. Store correctly rejected this as a different retirement authority with `presence publication requires its exact installed local-root retirement factory`.

The regression now creates one trait-object `Arc<dyn SnapshotRetirementFactory<DemoSnapshot>>`, installs a clone, and moves the same exact factory into publication admission. The product exact-factory check is unchanged. Root owns the requeue; no duplicate Cargo run was started.
