# Independent UI Contract Owner Audit

Source inspection only, 2026-10-15. No Cargo, compiler, build/cache/store, lock, GPU or runtime execution was performed. The implementation checkpoint reports BOTH22/7178, expanded strict0, and106 owners/535rows; this audit does not independently reproduce those results. Five generated Value-derive observer refusals and unfinished WGPU completion remain explicit.

## Blocking Findings

1. **P1: Actions cannot resolve its actual Value derive output.** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/📦️packages/🦀️rust/Cargo.toml:22-25` declares Values and Value-derive but omits `semio-framework-value`. Canonical `🎬️action/🦀️.rs:14-16` and `:48-50` derive ToValue/FromValue for ActionId and Trigger with an explicit `::semio_framework_value` path. The real derive implementation `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs:341-343` resolves that path; `:1574`, `:1799`, `:1825`, `:2121` emit trait implementations through it. A transitive dependency through Values does not place this crate in Actions’ extern prelude. Add `semio-framework-value = { workspace = true }` in Actions normal dependencies.

2. **P1: Presentation lacks its direct canonical Value dependency.** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/📦️packages/🦀️rust/Cargo.toml:22-29` omits `semio-framework-value`. The actual production `🖼️presentation/🔖️labels/🦀️.rs:26-38` directly implements `::semio_framework_value::ToValue`/`FromValue` and names DslValue/ValueError/ValueRefusalKind. Numerous actual Presentation records also derive Value traits using the same expansion path. Add `semio-framework-value = { workspace = true }` to normal dependencies.

3. **P1: Components lacks the direct Budget dependency named by its moved invariant body.** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:1040` directly imports `semio_framework_ui_contract_budget::{TREE_WINDOW_FIXED_NODE_HEADROOM,TREE_WINDOW_BODY_NODE_BUDGET}`. `🧩️component/📦️packages/🦀️rust/Cargo.toml:22-28` omits that dependency. Add `semio-framework-ui-contract-budget = { path = "../../../📏️budget/📦️packages/🦀️rust" }` to normal dependencies, or deliberately consume the already canonical reexport instead and refresh the source boundary observation. The direct manifest declaration is the clearest ownership statement.

These are actual unresolved owner API edges, independently visible without native execution. Correct declarations and rerun the registered Source cohort/strict/census before authorizing any native attempt. The previous census describes the declared graph and cannot establish that every body-named external crate was declared.

## Inspected Ownership Boundaries

All ten new owner manifests and retained Document were read. Literal `#[path]` mounts reached from their declared lib/integration entries were checked for physical existence; no missing literal mount was found. This simple inspection is not a cfg-aware closure census and does not replace the canonical262144B/32files/4000-line gate. All eleven manifests retain ordinary metadata, no enlarged owner cap. Typegen integration target is explicit; conformance dependencies are dev-only; Values testkit is declared and requested by actual observer consumers.

Values retains canonical UiValue/UiList/UiMap and private `UiCollectionHandle` (`🌱️values/🦀️.rs:633`) and `UI_VALUE_ARENA` (`:740`). Its generic fixed collection copy/compare/retirement implementations remain beside private storage. Foreign downstream types implement the canonical first-party protocols in their actual type owner; inspected operation roots own operation state rather than foreign inherent implementations. `try_credited_clone` exposes only a real canonical owned value and a nonblocking pending/error outcome. No raw numeric handle constructor or second arena issuer was found in these inspected ports.

The testkit observation takes `&UiValue`, snapshots private generation-bearing handles internally, and exposes only live/retired observations. Its contention fixture holds the real arena. It neither publishes handles nor creates aliases/resources. Inspection found no production observer admission path.

Bindings copy retains exact source/candidate/pending roots in one ManuallyDrop owner; separate source/candidate transfers track returned bits, and closing retires every remaining root. Collection alias copying now delegates to the canonical Values owner and preserves contention as pending. This is source ownership analysis, not proof of runtime custody laws.

Document retains actual records, assembly, residency, patch and accessibility projection; its reexports do not make it an empty facade. Lower owners’ manifests do not depend on Document, except intentional upper Validation and dev-only Conformance consumers. No child-to-aggregate cycle was found among inspected normal manifests. The direct-dependency omissions above mean the public type/caller closure is not yet ready for native admission.

## Required Qualification

The exact92 TypeScript projection declaration bodies and serde behavior remain claimed by the implementation checkpoint; this audit does not claim fresh byte parity or native serde execution. After the three concrete edge corrections, repeat the authorized Source gates and conservative declared closure census, preserve the five generated-observer refusals, then independently perform the managed lock/native preparation. No native/GPU/lock/completion qualification follows from this report.
