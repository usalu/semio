# Layout Companion API Review

2026-10-28. Static review of editor-tests, engine scene/flow test companions, retained diff-plan, SQLite member laws and borrowed mutation decisions. No compiler, Cargo graph or test was started. The Rust owner retains the single queued native lane; combined test coverage/source-budget audit belongs to the independent reviewer.

## Concrete Repairs

1. The editor's public testkit mounts context.rs, which imports plugin artifact_app_laws, OriginalFixtureScope and history_edit_acceptance_law. Those SDK APIs are feature-owned. Its empty testkit feature relied on the editor-tests companion's separate plugin dev-dependency enabling artifact-app-testing, masking a missing edge when the editor is consumed with testkit alone. The editor testkit now explicitly forwards semio-framework-plugin/artifact-app-testing. Production default features remain unchanged; no new dependency was added.

2. The scene law companion is explicitly native: its imports include TextFontFiles, its laws call install_native_text_fonts and match native-only TextFontsInstallError variants, and its font fixtures use the host filesystem. A cfg(test)-only root allowed unresolved native imports when a wasm test build selected it. Its owning root now uses cfg(all(test,not(target_arch="wasm32"))). Native laws, bodies and fixtures remain reachable and unchanged; the production engine's genuine wasm byte/package font doors remain unchanged. Flow laws do not need this host-only restriction.

## Other Boundaries Inspected

The editor companion uses public registered-app context support and public command/panel APIs. The context maintains one OriginalFixtureScope across typed dispatch, settle, render and controlled close; the split does not construct replacement contexts. Public context support is separate from the larger executable cases. SDK testing dependencies now have an owned feature edge instead of relying on companion feature union.

Scene and flow tests mount the existing laws and consume public domain modules. Engine testkit observations expose only repository-owned receipt/state observations (paragraph source count, composition copy position/buffer/receipt and engine text-system/font handles); no owner clone or duplicate machine was introduced. Native font loading uses platform-neutral Path/std filesystem APIs and keeps native methods behind the existing production non-wasm cfg.

Diff-plan's five planning modules are public independent domain modules, with their own existing tests. The former diff owner reexports them rather than mounting duplicate implementation source. No cross-owner pub(crate) or pub(super) call was found in this plan surface.

The SQLite member companion invokes public ArtifactSqliteSnapshot on LayoutSnapshot. It does not call the former private IO codec mount; the model owns the actual implementation. Shared neutral relational fixtures remain at their existing paths.

The mutation decision library depends on content/frames/model/diff, never on the root mutation owner. All58 view cases borrow original payload references; the root remains the sole owned mutation enum. Generic decision projections preserve the incoming RetainedCloneRef lifetime, binding and grant/accounting operations. Parent creation obtains its mutable slot from the original prepared mutation via parent_creation_mut; it does not clone or replace payload ownership. Seven root-dependent unit-law files remain mounted by the root mutation owner, while the independent debug formatter law belongs to decisions.

## Exact Two-file Source Packet

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🎬️scene/🦀️.rs

No workspace membership, router, build-owner, global quota, law body or neutral fixture changed in this packet.

## Bounded Static Validation

Retained companion-review-inventory.ts directly uses the canonical readRustSourceClosureV1 on the two affected owners only. Actual repo-managed source-only run:
repo --service=off tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/companion-review-inventory.ts

- Editor native testkit:17inputs,210538bytes,2655active lines.
- Editor native test+testkit:22inputs,255749bytes,3145active lines.
- Scene native test:23inputs,218391bytes,858active lines.
- Scene wasm test:1root input,2423bytes,0active lines; native law owner excluded.

All native affected closures fit unchanged32input/262144byte/4000line limits. Feature forwarding changes dependency activation rather than editor source ownership; aggregate feature-union/source admission remains independently audited. Static inspection does not substitute for the queued native compiler result.
