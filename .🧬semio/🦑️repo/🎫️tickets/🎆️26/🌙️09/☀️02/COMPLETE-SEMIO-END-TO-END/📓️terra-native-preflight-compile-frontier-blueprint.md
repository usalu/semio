# Native/WGPU Exact Preflight Compile Frontier

Date: 2026-09-04  
Scope: read-only current-tree audit of the native D1 preflight after the Cargo path repair. This is a compile-frontier plan, not a runtime acceptance.

## Verdict

**RED — the registered native D1 gate cannot reach its owned assertions until real installed-plugin source errors are repaired.** The Cargo manifest/path repair is source-confirmed by `cargo metadata --no-deps`; it does not constitute plugin compilation or native document-open execution.

The old `semio-s-plugin-stdio` first error is superseded at source: its Boolean validation helper is now defined. The current material frontier is the installed **norm** and **draw** plugin fan-in. There is no Cargo package named `semio-s-plugin-terminal`; the third package mentioned in the handoff is interpreted as `semio-s-plugin-stdio` and is called that below.

No Cargo check/build/test was started here. The D0 stdio provider target was active, and a compilation run would have contended with it. Evidence is current-source inspection, fast `cargo metadata --no-deps`, and timestamped prior compiler fingerprints only where explicitly labelled historical.

## What the exact gate actually requires

`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2905-2913` runs, in this order:

1. the language-neutral document-open-plan fixture and native credential-source census;
2. `@semio-tech/framework-renderer-wgpu:check-frame-worker`;
3. `@semio-tech/framework-renderer-wgpu:native-build -- --scale`;
4. seven exact-one native `semio-framework-os-kernel` D1 laws;
5. one exact-one MCP transport law and the native socket-actor oracle.

The native laws are selected with a suffix list plus `--list` exact-one preflight at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:2917-2935`. The registered launch entry is `⚖️gate📄️document-open-native🌎️hub` in `.vscode/launch.json:4466-4473`.

Consequently a green frame-worker check alone says nothing about D1 admission. A failed WGPU native build prevents the owned D1 assertions from even being listed. Conversely, a package compile saying only that the fan-in built is not a SocketGrant/document runtime result.

## Classification of the repair

| Surface | Current classification | Evidence | Why it matters to native D1 |
| --- | --- | --- | --- |
| Cargo paths | source-closed, compile-unverified | read-only `cargo metadata --no-deps --format-version 1` resolves `semio-s-plugin-stdio`, `semio-s-plugin-draw`, and `semio-s-plugin-norm` at their double-package roots | This only restores dependency discovery. |
| stdio / “terminal” | source-provisionally closed; needs owner rerun | `✏️s/🔌️plugins/🗄️stdio/…/✳️🔴️brep/…/🔀️boolean/🦀️.rs:500,522-524` now both calls and defines `issues_scoped_to_new_solids` | The previous single `E0425` fingerprint cannot be retained as a current blocker. |
| norm leaf taxonomy | current RED | 265 direct self-import-shaped mutation leaf roots; representative `…/📗️din16798/…/🏷️change-annex/🦀️.rs:12,21` imports then defines `ChangeAnnex`; 21 nested `🦠️mutation/🦀️.rs` files lack `MutationLeaf` metadata; representative `…/📓️iso16757/…/🛋️remove-selection-constraint/🦠️mutation/🦀️.rs:6-24` | `Mutations` aggregation cannot prove the typed semantic dispatch surface. |
| norm static catalogue | current RED | `EditionId::new` is non-const at `…/📔️vdi3805/🦀️.rs:282-285`, yet is called inside `SHEET_ENTRIES` const rows from :361-370 onward | Prevents the crate from type-checking independently of any WGPU code. |
| norm viewer UI | current RED | the SDK requires `UiAssemblyResult<ComponentTree>` at `…/🔌️plugin/🦀️.rs:26329`; `En1994Viewer` returns `UiNode` at `…/📘️🎈️en1994/…/👁️viewer/🦀️.rs:68-72` | A renderer-type mismatch blocks artifact surface registration and native fan-in. |
| draw leaf taxonomy | current RED | 14 nested leaf payload modules lack `dsl::MutationLeaf` and `#[mutation_leaf(contract = ::protocol)]`; representative `…/🌱create-layer/🦠️mutation/🦀️.rs:9-23`, while it declares a real semantic kind at :30-44 | Prevents the aggregate `DrawingMutation` from satisfying its typed protocol contract. |
| draw authoring traits | current RED | `ArtifactViewer` is synchronous at `…/🔌️plugin/🦀️.rs:26305,26312,26329`; `DrawingViewer` declares `async fn initial_snapshot`, `handle`, and `render` at `…/👁️viewer/🦀️.rs:55-74`. `DrawingPlayApp` similarly implements the synchronous `ArtifactEditor` with async members. | This is a real framework/plugin API mismatch, not a Cargo-path miss. |
| draw UI builder scope | current RED | `TreeItemBuilder::try_id` is the `HasBase` extension at `…/🖱️ui/…/builder.rs:527-539,1393-1399`; the layers panel uses it at `…/📌️panels/🗂️layers/🦀️.rs:60-62` without importing `HasBase`, although `semio_framework_plugin` reexports it at `…/🔌️plugin/🦀️.rs:37402` | This is a narrow current Rust trait-in-scope error. |

The historical fingerprints are useful only to order work: norm previously stopped after 816 errors and draw after 210. Their old direct-`ArtifactApp` / `DESCRIPTORS` messages must not be carried forward as current findings: the current norm tree contains the 30 `ArtifactEditor`/`ArtifactViewer` implementations and no current direct `ArtifactApp` implementor was found by source census. Recompile after each packet before asserting any residual count.

## Exact current blockers and safe fixes

### P0 — retain the stdio Boolean guard, then prove it

The previous missing-symbol failure is already source-closed. The current helper captures pre-existing solid ids before Boolean processing (`…/🔀️boolean/🦀️.rs:358-361`), runs validation after orphan collection (:493-510), filters only issues attributable to pre-existing entities (:513-524), and deliberately keeps un-attributable issues as errors (:518-520). Do **not** replace it with an empty/stub filter.

Owner work is only a bounded confirmation after the D0 target is free:

- add/retain an exact Rust law with an invalid pre-existing operand plus a valid new Boolean result (must not blame pre-existing topology);
- add a second case with an unattributable/new-result issue (must fail);
- validate the neutral JSON vector with a non-Rust implementation before its exact Rust law.

This packet is independent of norm/draw and should not alter the D1 gate ordering.

### P1 — make every mutation leaf an actual protocol leaf (norm and draw, parallel)

Each semantic payload must derive `dsl::MutationLeaf` and carry `#[mutation_leaf(contract = ::protocol)]` only when its existing `protocol::MutationKind` semantics and `diff`/`inverse` are coherent. A blanket trait implementation or removing the aggregate bound would make the compiler green while discarding the descriptor proof.

Norm example: `RemoveSelectionConstraint` has a semantic descriptor and reducer at `…/🛋️remove-selection-constraint/🦠️mutation/🦀️.rs:12-24`, but no leaf derive at :6-10. Draw example: `CreateLayer` has a `create/layer/create-layer/CreatedLayer` descriptor at `…/🌱create-layer/🦠️mutation/🦀️.rs:30-44` yet uses only `ToValue/FromValue/DslRecord` at :9-13.

Required audit law per leaf family:

- an independent JSON `mutation-leaf-contract-v1` corpus naming leaf `kind`, `verb`, `entity`, `record`, minimally valid payload, bad kind, bad payload, and inverse round trip;
- an AJV/Node reader rejects the hostile vectors without using the Rust derive;
- an exact Rust law reads the actual `MutationLeaf::DESCRIPTOR`, encodes/decodes the real operation, applies `diff`, and checks its inverse against the same corpus.

The owner should update every discovered leaf in one atomic taxonomy packet, not only the representative. The focused runner must list each suffix and require exactly one FQN before `--exact`; the existing package scripts (`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts` and `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts`) currently route generic package-wide Cargo tests and do not give this exact-one proof.

### P2 — repair generated/taxonomy leaf-root imports (norm)

At `…/📗️din16798/…/🏷️change-annex/🦀️.rs:12,21`, the module imports its own type and defines the same type. This is a direct `E0255` cause. The same source census finds 265 leaf-root files shaped like this. The importer is allowed in sibling `🔺️diff` / `↩️inverse` modules, but not in the defining `🦀️.rs`.

Repair the leaf-root generation/template once, then regenerate or hand-apply the uniform taxonomy tree in the same packet. No re-export/alias workaround: it leaves an ambiguous canonical leaf identity. Add a source-census gate that rejects a leaf-root importing a type from its own mutation module, plus a representative semantic corpus from P1.

### P3 — make VDI static catalogue construction const-correct (norm)

`SHEET_ENTRIES` is intentionally static (`…/📔️vdi3805/🦀️.rs:361-363`) but calls an ordinary constructor `EditionId::new` (:282-285). The minimal source correction is a `const fn` constructor if and only if it remains the pure field constructor currently shown. Do not turn the catalogue into lazy mutable state.

Neutral oracle: a compact `vdi3805-sheet-index-v1` JSON fixture for the first, a published, a reserved, and a historical-proposal row; it asserts count, ids, editions, statuses, and lookup order. The Rust exact law must compare the actual `SHEET_ENTRIES` projection to that fixture. This keeps the current 100-row table authoritative but protects against a copy/paste/edition drift.

### P4 — converge plugin surfaces on the present renderer contracts (norm and draw, parallel after P1)

There are two distinct current trait families and they must not be conflated:

- `ArtifactEditor` and `ArtifactViewer` are synchronous and require `UiAssemblyResult<ComponentTree>` at `…/🔌️plugin/🦀️.rs:26140` and :26329.
- direct `ArtifactApp` is an asynchronous older/general runtime trait; it is not the repair target for the current norm/draw authoring implementations.

For norm, convert every editor/viewer render body from `UiNode` to fallible assembly and an explicit component-tree conversion. Preserve the known-body result and make the unknown-body branch bounded/fallible instead of silently rendering an unbounded string. `En1994Viewer` is the minimal first compiler witness (`…/👁️viewer/🦀️.rs:43-73`). Its current return type is demonstrably wrong.

For draw, remove `async` from every `ArtifactEditor`/`ArtifactViewer` member whose current authoring trait is synchronous; retain the actual synchronous return value and component conversion already present in the viewer. `DrawingViewer` is the minimal witness at `…/👁️viewer/🦀️.rs:41-74`. Do not alter the framework trait to match one plugin; that would expand the active contract migration and rebreak all existing synchronous implementors.

Neutral `surface-render-contract-v1` corpus:

- `{role, dialect, documentSchema, bodyKey, expectedRootKind}` for one editor and one viewer per plugin;
- an unknown body key that must return a bounded diagnostic component, not panic or activate an action;
- a malformed/oversized label vector that must return `PluginAssemblyError` and no partial tree;
- a non-mutating viewer command vector that proves no artifact/draft mutation.

Run it independently in Node against the documented JSON schema; exact Rust laws must render actual app instances and compare the component-tree projection. This is renderer contract proof, not native GPU presentation proof.

### P5 — fix the draw tree-builder extension import and re-evaluate import drift

`try_id` is not an inherent `TreeItemBuilder` method. Import the existing `HasBase` re-export alongside the panel's current imports, preserve `try_id(row_id)` and its bounded assembly error. Removing `try_id` is unsafe: the framework documents it as the stable reconciliation key for reorderable siblings at `…/builder.rs:522-539`.

The prior draw diagnostics also mentioned moved stdio SVG/drawing paths. The current draw crate itself retains compatibility shims at `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/🦀️.rs:500-535`; therefore each alleged unresolved import must be recompiled and reclassified after P1/P4/P5 rather than patched from a stale diagnostic. A genuine moved external import should be updated to the canonical taxonomy path and covered by a decode/encode corpus, never restored as a legacy alias.

## Dependency order and ownership split

| Order | Independent implementation lane | Blocking output | Acceptance boundary |
| --- | --- | --- | --- |
| 0 | stdio Boolean source confirmation | `semio-s-plugin-stdio` clean exact Boolean laws | Only clears the former first error; not native D1. |
| 1a | norm leaf descriptors + leaf-root imports | norm mutation aggregate compiles and exact corpus laws pass | Does not claim catalogue/UI/native runtime. |
| 1b | draw leaf descriptors | draw mutation aggregate compiles and exact corpus laws pass | Independent of norm. |
| 2 | norm VDI const catalogue | actual static table compiles and fixture equality passes | No UI/native claim. |
| 3a | norm synchronous component-tree surfaces | all norm `ArtifactEditor`/`ArtifactViewer` impls conform | No GPU/native claim. |
| 3b | draw synchronous component-tree surfaces + `HasBase` scope | both draw authoring surfaces conform | No GPU/native claim. |
| 4 | recompile installed plugin fan-in in an isolated target | WGPU `native-build --scale` reaches completion | Compile boundary only. |
| 5 | existing native D1 registered gate | native exact-one laws and MCP actor oracle execute after stage 4 | First possible D1 assertion result. |

Do not combine stages 1–3 with D1 issuer/SocketGrant changes. Those are already behind the WGPU preflight and cannot be exercised until the plugin fan-in becomes compilable.

## Required registered gates

After an owner has repaired a packet and the D0 target is clear:

1. Add a package-local `📜️script.ts` subcommand for each corpus that runs its Node/AJV oracle, lists every intended Rust law, requires exactly one FQN per suffix, and invokes those FQNs with `--exact`. Wire its `project.json` target only to `bun ./📜️script.ts <command>`.
2. Run the package-local target uncached for the changed plugin only; it is the owner-level semantic proof.
3. Run the registered final gate, not an ad-hoc Cargo substitute:

```text
bun ./📜️script.ts nx run os-hub:native-document-open-check --skip-nx-cache
```

It must print the native D1 final message only after all earlier frame-worker, WGPU native build, exact kernel law, MCP law, and actor-oracle stages complete. The launch entry cited above is the normal developer command.

## Explicit nonclaims

- No runtime WGPU launch, document SocketGrant exchange, WebSocket connection, or D1 assertion was run by this audit.
- `cargo metadata` validates workspace resolution only; it does not validate the plugin fan-in.
- The stale stdio missing-helper fingerprint is not a current RED because current source defines the helper; it needs a fresh owner run before it can be marked green.
- Historical fingerprint counts are triage aids, not a current compiler terminal. Re-run after each packet and update the live count from that terminal.
- The native D1 law remains **blocked**, rather than failed or accepted, until the registered gate reaches its own exact assertions.
