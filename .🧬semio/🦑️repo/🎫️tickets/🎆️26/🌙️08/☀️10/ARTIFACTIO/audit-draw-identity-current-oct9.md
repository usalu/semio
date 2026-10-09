# Draw Identity Independent Source Audit — 9 October 2026

Read-only production audit against `draw-identity-current.md` and `draw-identity-owner-manifest-current.md`. No production edits, Git mutations, source copies or native reruns occurred. Repo MCP is unavailable; this audit neither simulates ticket lifecycle nor changes goals. The listed edit manifest has 79 existing inputs but no hashes; it is an owner/path census, not a hash-only frozen receipt. Source findings below do not establish runtime results. Native receiving remains unproven behind the shared Kernel floor; the supplied 29/493 source, 4/4 isolated native and 605 whole-TypeScript receipts are historical receipts, not runs performed by this audit.

## Concrete Findings

### P1 — Creation/paste receivers fabricate unconditional control authority

All paths below are relative to `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

- `✏️editor/🎮️commands/➕️add-layer/🦀️.rs:40–42`: `prepare_identity` manufactures a 1MiB `NativeEncodeControl` with `|_|true`. `build_layer:31–34` invokes it; command `handle:47` only passes optional operation facts. Real cancellation/progress authority does not enter this boundary.
- `✏️editor/🎮️commands/🎛️edit-selection/🦀️.rs:48–49,61–66`: duplicate and group plans independently manufacture always-true controls. Group also allocates `parts.collect::<Vec<_>>()` outside the admitted control at line 65.
- `✏️editor/📋️clipboard/🦀️.rs:184–186`: synchronous paste helper fabricates an always-true 76-byte control. Both layer and asset loops call it at 139/152.
- `✏️editor/📋️clipboard/🧵️job/🦀️.rs:62–64`: retained `fresh_identity` fabricates an always-true zero-owned-byte control, despite actual job calls having `StepContext` available at 191/211. No caller cancellation reaches hashing/publication. Borrowed buffer zero owned allocation is legitimate; unconditional authority is the gap.
- `✏️editor/🎮️commands/📥️import-image/🦀️.rs:13–18`: image publication fabricates always-true control and then independently calls `prepare_identity`, resetting authority/budget.
- `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs:18–25`: deserializer has no supplied control; fabricates always-true control and drains the SVG job in a tight loop. Isolated identity-leaf cancellation tests cannot prove these original receiving paths.

### P1 — Missing creation operation facts silently become synthetic zero/empty facts

`🚪️io/📝️text/🪪️identity/➕️creation/🦀️.rs:4–13` accepts `Option<AppOperationContext>` and fills absent app/operation/generation with zero; revision and shape document with empty bytes. `✏️editor/🎮️commands/🔀️combine-boolean/🦀️.rs:34` deliberately calls `prepare_identity(...,None)`, while `handle:42–44` has the operation-bearing ArtifactView. Boolean identity therefore excludes available operation facts. This is distinct from an explicitly authored semantic initial-layer ID: a stable authored key needs no external operation context; a physical creation operation must not invent missing authority facts.

### P1 — DrawingIdentity is an open fact, not a sealed admitted constructor input

`🧬️schema/🪪️identity/🦀️.rs:6–11` exposes `pub key` and infallible conversions from string/paged text. Both `DrawingIdentity { key: empty }` and `DrawingIdentity::from("")` bypass `FromValue` nonempty checks at 29–30. `🧬️schema/🦀️.rs:13–16,229–230,235–239` consumes the supplied key directly without validation. Therefore an empty direct constructor identity can become a path/base/group ID even though wire admission rejects it. TypeScript identity is likewise a structural `{key:string}` interface at `🧬️schema/🪪️identity/🟦️.ts:2`. This is a source-level counterexample; no receiving native runtime was run.

### P2 — Editor duplicate bypasses the original DuplicateLayer receiving contract

`✏️editor/🎮️commands/📋️duplicate-layer/🦀️.rs:17–21` calls edit-selection plan. Its duplicate branch (`🎛️edit-selection/🦀️.rs:42–57`) prepares assignment facts, clones, and emits `create_layer` at 55 rather than `duplicate_layer`. It checks only candidate root collision at 51. A subtree descendant target collision reaches CreateLayer receiving instead of the newly audited DuplicateLayer per-assignment checks. The strengthened original DuplicateLayer is not the actual editor duplicate receiver. Whether CreateLayer safely rejects every descendant collision requires receiving runtime and must not be inferred from the strengthened duplicate tests.

## Correct Source Boundaries And Limits

Binary `🚪️io/💾️binary/🪪️identity/🦀️.rs:8–14` owns DRAWID01 framing and SHA-256. Text `🚪️io/📝️text/🪪️identity/📤️publication/🦀️.rs:4–11` owns prefix/lowercase hexadecimal publication and admits the key. Pure constructor/clone code does not calculate hashes or commitment spelling in the inspected regions.

The original structure DuplicateLayer mutation requires an explicit complete assignment set (`🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🦠️mutation/🦀️.rs:14–21`, relative to subsets). Its diff checks all target collisions at `🔺️diff/🦀️.rs:12–16`; inverse consumes the same set at `↩️inverse/🦀️.rs:12–14`. The inverse docstring at 1–3 still incorrectly says it recomputes a content-addressed hash.

Pure clone `🧬️schema/🦀️.rs:659–673` validates missing/repeated sources, empty/repeated/self/source-overlapping targets, exact census and Boolean remapping. Retained host `🔨️modules/🏠️host/🧰️owned/🦀️.rs` (relative to drawing root) scans assignments per turn at 3161–3163, checks every target with document locator at 3165–3169, checks repeated targets at 3173–3176, consumes supplied targets at 3184–3198, validates census at 3264 and clears new authority state at 3265/3282/3301. Mutation digest source/target binding begins at 2891–2892. These source links support separation, not retained runtime completion.

No compatibility wrapper was needed to explain the observed issues. The remaining high-priority work is caller-owned control propagation, mandatory supplied operation facts at external creation boundaries, sealed checked identity admission for direct callers, and actual original receiver integration tests after a changed shared foundation.
