# P8o Independent Draw, Flow, And Forms Final Audit

## Verdict

**REJECT.** This is a read-only source/static audit of the current worktree. The repaired store, composition read surface, app job context, and most Draw/Flow/Forms continuation mechanisms contain the intended structural changes. Two remaining release gates are disproved by current source:

1. The public **command** entry point does not apply Draw/Flow/Forms' 8 KiB/16 KiB pre-deserialization envelope. Only the public action entry point does. This violates the requested action/command admission boundary.
2. Two authored Forms public-action assertions are internally impossible: `FormsTryValues::get_json` returns the content identifier, while the tests assert that it returns the raw JSON content. They cannot pass on this source.

No Cargo/build/test command was run, and no source, status, JSON, cache, or Git state was modified by this audit.

## P0 Findings

### P0-1: DFF command dispatch bypasses command-specific predecode limits

`plugin_handle_action` first calls `validate_public_action_envelope`, which identifies Forms/Draw/Flow action IDs and rejects body sizes above Forms 16,384 bytes or Draw/Flow 8,192 bytes before `serde_json::from_str` ([plugin component.rs:16956-16963](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L16956), [plugin component.rs:17000-17004](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L17000)). The selected limits and covered IDs are explicit at [plugin component.rs:16896-16899](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L16896); the exact-max/+1 and malformed/hostile tests cover that helper only ([plugin component.rs:16976-16997](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L16976)).

The distinct public `plugin_handle_command` path invokes only the generic 262,144-byte / 4,096-byte-string validator and then immediately deserializes `ManifestCommandInvocation` ([plugin component.rs:17028-17032](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L17028)). It never calls `validate_public_action_envelope` or an equivalent command-ID classifier. `validate_public_json_envelope`'s generic ceiling is established at [plugin component.rs:16836-16841](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L16836) and enforced at [plugin component.rs:16909-16953](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L16909).

App/mode commands are then converted via `A::command_from_action(command_id, ...)` and dispatched through the same typed command channel ([plugin component.rs:13104-13127](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L13104)). Consequently, an externally supplied command envelope for a DFF command can reach generic serde with a body above the required 8 KiB/16 KiB cap. There is no command-route max/+1/malformed/hostile regression test.

Required repair/gate: add a bounded raw `commandId` classifier before `serde_json::from_str` in `plugin_handle_command`, apply the identical command-specific ceilings, and exercise exact/max+1/malformed/hostile fixtures through both public endpoints.

### P0-2: Forms public-action tests assert content through an ID accessor

`TryValueContent` stores both `id` and bounded `chunks` ([Forms config component.rs:198-202](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L198)). `FormsTryValues::get_json` returns `value.id`, not any reconstructed chunk content ([Forms config component.rs:225-230](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L225)); raw content remains reachable only as `content_chunks` ([Forms config component.rs:251-257](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L251)). IDs use `"try-" + 64 hex digits + "-" + 16 hex digits`, so their byte length is fixed at 85 ([Forms config component.rs:158-169](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L158)).

The scalar public-action test feeds 8,192 bytes of content ([set-try-value component.rs:1702-1717](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L1702)) but asserts `get_json(...).map(str::len) == Some(8192)` ([set-try-value component.rs:1760-1763](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L1760)). This assertion is statically inconsistent with its accessor and cannot succeed. The bulk public-action test has the same mismatch, asserting `get_json("b") == Some("1")` ([set-try-values component.rs:486-490](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-values/🦀️component.rs#L486)), although a committed value writes the supplied `content_id` as `id` ([Forms config component.rs:132-143](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L132)) and `get_json` exposes that ID.

This prevents acceptance of the claimed Forms public ActionBus/reopen evidence. The durable representation itself is promising: serialization writes chunk arrays and deserialization rebuilds the content IDs from those chunks ([Forms config component.rs:345-364](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L345)). But the published tests must assert chunks/materialized content through a correctly named API, or the public accessor must be changed consistently. Then run them.

## Confirmed Source-Level Coverage

- **Canonical revision/non-aliasing:** the accumulator hashes complete serialized edit records into domain-separated applied/redo prefixes ([store component.rs:4384-4441](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L4384)), `bump` refreshes it for every mutation/cursor movement ([store component.rs:6452-6464](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L6452)), and the interior load/reset ABA test changes a middle edit while preserving the final snapshot ([store component.rs:9598-9615](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L9598)). Runtime proof remains unrun.
- **Opaque shared reads:** the public snapshot capability keeps its `Arc` field private ([store component.rs:38-80](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L38)), `ArtifactStore` exposes `snapshot_read` rather than its crate-private owner ([store component.rs:4841-4847](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L4841)), and `ChildContentView::typed_read` returns the repo-owned capability ([plugin component.rs:8580-8613](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L8580)). In the audited snapshot/content APIs, no public `Arc` return was found; this is a static observation, not an external-consumer compilation proof.
- **Actual operation propagation and one dispatch path:** `AppCommandJobFactory::create_job` installs the factory-assigned operation ([plugin component.rs:11260-11285](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L11260)); its worker turn creates `AppOperationContext` using actual app/document/operation/generation/canonical revision ([plugin component.rs:11231-11255](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L11231)). The typed dispatch creates and sends that job through the registered action bus ([plugin component.rs:13169-13226](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L13169)); registration is single-factory `register_once` ([plugin component.rs:11497-11502](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L11497)).
- **Draw:** its revision now derives from the canonical dispatch context ([Draw editor component.rs:137-139](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs#L137)); gesture identity includes app/document/operation/generation/base revision and config checkpoints authenticate those fields ([Draw editor component.rs:109-142](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs#L109), [Draw editor component.rs:225-277](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs#L225)). The source includes registry-clear, stale, cancellation, capacity, and depth tests ([canvas-pointer-down component.rs:1291-1419](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L1291)). They were not run.
- **Flow:** the continuation binds parent revision, child revision, app, document, operation, and child; it requires equality with persisted config before it advances ([duplicate-widget component.rs:196-230](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L196)). Search is bounded to 64 rows ([duplicate-widget component.rs:101-161](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L101)); fresh-app replay and shared-child/two-parent source tests are present ([duplicate-widget component.rs:346-385](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L346), [duplicate-widget component.rs:417-458](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L417)). They were not run.
- **Forms bounded staging and typed outcomes:** chunks are bounded to 4 KiB during command deserialization ([set-try-value component.rs:62-91](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L62)), staging keys include app/document/operation/generation/base revision and return explicit faults for invalid/order/full inputs ([set-try-value component.rs:318-432](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L318)), while config mutation failures carry typed error messages rather than ignored booleans ([Forms config component.rs:55-92](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L55), [Forms config component.rs:573-625](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L573)). The broken assertions above mean this lane cannot yet be accepted.

## Unrun Runtime Gates

The following remain explicitly unverified because this audit did not run Cargo, builds, or tests:

- compilation of the audited Rust worktree and all source-authored tests;
- public action and command endpoint behavior after raw JSON decode, including allocation behavior;
- native/release/Wasm per-turn timing, watchdog enforcement, and cancellation responsiveness;
- full operation encode/decode/diff/apply/reopen behavior for Draw, Flow, and Forms;
- same-document/two-app and shared-child/two-parent behavior at runtime;
- canonical revision behavior through actual undo/redo/load/reset, not only the source fixture.

## Exit Gate

Do not mark the Draw/Flow/Forms Phase 8 lane source-acceptable until the public command route receives the same pre-serde DFF classifier/limits and its own hostile-boundary tests, and the Forms public-action tests use an API whose result matches their asserted raw content. Then run the focused public action/command and runtime lanes under native and Wasm.

## Repair Disposition — 2026-08-22

The two P0 source findings above are repaired in the current tree. The original rejection remains the historical audit result; this disposition supersedes its two source blockers only and is not a runtime-pass verdict.

### P0-1 Closed at Source/Static Level

- Public command dispatch now invokes the command-specific raw-envelope validator before generic manifest-command deserialization. Action and command admission share an allocation-free exact-address classifier, reading only the addressed action or command identifier, plus one DFF command-name limit table.
- Forms commands are bounded to 16,384 raw bytes and Draw/Flow commands to 8,192 raw bytes before generic deserialization. Escaped address/identifier keys and escaped identifier values are rejected instead of bypassing classification; an arguments-level command-identifier decoy does not misclassify the envelope.
- Source-authored fixtures cover every named Forms/Draw/Flow command on action and command routes at exact maximum and maximum-plus-one. Additional fixtures cover malformed command structure, hostile 4,097-byte strings, escaped-key/value bypass attempts, and public endpoint routing where exact maximum reaches post-decode ownership validation but maximum-plus-one stops at the DFF bound.

### P0-2 Closed at Source/Static Level

- The Forms accessor is now explicitly documented and tested as the bounded content-ID accessor. The scalar and bulk public-action fixtures no longer compare it with raw JSON.
- Both fixtures assert the 85-byte content ID, resolve raw content from the durably owned chunk leaves, serialize the completed Forms config, clear scalar/bulk staging and continuation registries, cold-reopen, and reassert the same content ID and raw content.
- The obsolete no-op completed-content replay helper was removed. No process-global completed-payload registry was added; serialized owned chunk leaves remain authoritative across reopen.

### Executed Source/Static Gates

- Focused Rust formatting check: exit 0 for the framework plugin component, Forms config, scalar set-try-value, and bulk set-try-values files.
- Static interactivity verifier: exit 0; 775 command rows, all 775 bounded; zero batch-only, forbidden, deleted, or failed rows; one production factory, registration, and dispatch.
- Focused text scans: command admission precedes command serde; all six DFF command IDs share the classifier; stale Forms raw-value assertions and the obsolete completed-content replay helper are absent; no debug output was added.

### Runtime Gates Still Unrun

No Cargo command, compilation, Rust test, native/release/Wasm execution, clippy run, allocation profile, timing benchmark, or console-log runtime verification was performed in this repair lane. The source-authored endpoint and cold-reopen fixtures are therefore unrun coverage, not reported passes. These runtime gates remain separate and outstanding.
