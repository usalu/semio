# 📓️ terra — P3-manifest-schema report

**Baseline HEAD:** `1eaf87e6f52017dc2a5a6806fc926762f141d544`
**Packet:** P3-manifest-schema, ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`
**Charter:** `📓️design-decisions.md` D5 + D6, `📋️master.md` §3.1/§3.2

---

## 1. Files owned/edited — SHA-256 before → after

| file | before | after |
|---|---|---|
| `🛂️manifest/🦀️component.rs` | `fd316c31f4a73b39f0db570a1a25e03054f90ff917f2ae7360ac4cfb8e5ea62a` | `66c6ad350967900e45318100c0e017a9e9920b474003bdece502525e340016f1` |
| `🛂️manifest/🟦️component.ts` | `eceb8ac96c9afa170ecbcc3ed770c464c42646b1003127b320e09f1a3076ce65` | `92f644b356abc093280111513a9d27b04e9d385de8d1194b1c2d13b25d1a5923` |
| `🛂️manifest/🤖️generated/🟦️manifest.ts` | `0e45108c611d6d334f8be2b29a44993ce5ae6ba0a3cfa118e9c46c10a7aeda18` | `0e45108c611d6d334f8be2b29a44993ce5ae6ba0a3cfa118e9c46c10a7aeda18` (**unchanged** — see §4 typegen) |
| `🗣️dsl/🧬️schema/🦀️component.rs` | `f6c976471d000bab576fb30e761bac6155add732971433365d0702626a23f050` | `7eccfd7aaf93823cffb1f651f1fdcdb7245aa9d77a404c327768b75d3f2b8164` |
| `🧬️schema/🦀️component.rs` | `bb967f41b1914dcd1db802ca01d98bce249872dfc49bc0ae6ac471354c65a2db` | `bb967f41b1914dcd1db802ca01d98bce249872dfc49bc0ae6ac471354c65a2db` (**untouched** — see §5) |
| `📺️renderer/…/ShellHelpers/🟦️component.tsx` | `9656b420f83bcfa8d2f5e0c6c9018d1807a1c91269936ba7451352a02cfd1c4b` | `8f3ef3df3c36adfeb33ef5d7a821a8a19ecc405f22895a7c85cc42cf2caec553` |
| `✏️s/🔌️plugins/🧩️puzzle/…/🧊️3d/…/✏️editor/🦀️component.rs` | `83eaf3d7723530d18a0961e1394a698141b0c865de24c66f664f5bdd99b9fc0f` | `07c56415d857c69207553f5c1226dd7f5fbe1b5ccb22bbb3d5b53f809e5d8c5e` |
| `🖱️ui/🧱️elements/💬️UIDialog/🧪️story.tsx` | `a11bf46c9eeb0758ad6848290fc5612df4f66f31de0b67746268de7c7a779917` | `1978b9477a2471b7013dc9886b1d6fcd9175da667b5836592a58a525350f675f` |

Full baseline/after hash dumps: `🧪️p3-baseline-hashes.txt`, `🧪️p3-after-hashes.txt` in this ticket folder.

---

## 2. What changed, per file

### `🛂️manifest/🦀️component.rs` (exclusive)
- New region `//#region 🔖️ArgSchema` (108 lines, all new): `ArgFormat` (9 variants — the 9 from `📋️master.md` §3.1 plus `ArtifactKind{roles}`/`SurfaceApp{roles,dialect_arg}`, needed because `ActionArgControl::ArtifactKind`/`SurfaceApp` — pre-existing host-resolved controls — need SOME `ArgSchema` origin now that `control` is derived, not stored; see the region's header comment), `ArgSchema` (7 variants: `String/Number/Boolean/Vec3/Array/Object/Any`), `ArgPresentation` (4 variants).
- New region `//#region 🔖️ActionSemantics` (193 lines, all new): `ResourceSelector`, `CapabilityEffects`, `ApprovalMode`, `CapabilityPolicy` (`scopes: Vec<kernel::CapabilityId>` — the **real** kernel type, not a parallel string vocabulary; see §3 below), `PreviewMode`, `UndoMode`, `IdempotencyMode`, `ExecutionClass`, `CapabilityExecution`, `ActionSemantics` + `ActionSemantics::for_kind(ActionKind)` matching the §3.1 defaults table exactly.
- **In-place edit** (required, not new) inside the pre-existing `🔖️ActionArgs` region: `ActionArgDef` loses `control: ActionArgControl`, gains `schema: ArgSchema` + `presentation: Option<ArgPresentation>`; new methods `control()` (derivation) and `json_schema()`; the six builders (`text/number/slider/toggle/select/vec3`) and the two host-resolved builders (`artifact_kind/surface_app`) keep their exact signatures, now building `ArgSchema` instead of `ActionArgControl`. Two new free fns `apply_arg_format`/`arg_schema_json_schema` support `json_schema()`.
- **In-place edit** inside the pre-existing `🔖️Commands` region: `CommandDefinition` gains the same `semantics: ActionSemantics` field + `.semantics()/.destructive()/.use_when()/.example()` builders as `ActionDefinition` (per §3.1: "`ActionDefinition`/`CommandDefinition` gain …").
- `ActionDefinition` gains `semantics: ActionSemantics` (init'd via `ActionSemantics::for_kind(kind)` in `new()`) + the same 4 builders.
- Fixed 2 in-file `.control` field reads (both test code) → `.control()`.
- Added `.export()` calls for all 13 new typegen types in `exports_typescript_bindings`.
- Added 5 new `#[test]` fns (see §6) plus the mandated six-helper regression test.
- **Bug found and fixed during this packet** (not present in any spec): `ArgFormat::EntityId { kind: String }` collided with the enum's own `#[serde(tag = "kind")]` — serde hard-errors when a tagged enum's struct-variant field is itself named `kind`. Renamed to `entity_kind`.
- **Bug found and fixed during this packet**: `ArgSchema::String.options: Vec<ActionArgOption>` was wrongly annotated `#[cfg_attr(feature="typegen", ts(optional))]` (`ts(optional)` is only valid on `Option<T>`, not `Vec<T>` — caught by `cargo check --features typegen`, not by the ordinary `-p semio-framework` build). Removed.

### `🛂️manifest/🟦️component.ts` (exclusive)
- Added `ArgSchema/ArgFormat/ArgPresentation/ResourceSelector/CapabilityEffects/ApprovalMode/CapabilityPolicy/PreviewMode/UndoMode/IdempotencyMode/ExecutionClass/CapabilityExecution/ActionSemantics` to the generated-mirror import block + re-exported as bare type aliases (same pattern as `ActionArgControl`).
- Added `export function argControl(def: ActionArgDef): ActionArgControl` — the TS twin of Rust `ActionArgDef::control()`, same priority order (non-empty `options` → Select; `slider` presentation or fully-bounded `Number` → Slider; else Text/Number/Toggle/Vec3; `iconId`/`artifactKind`/`surfaceApp` formats → their controls).

### `🛂️manifest/🤖️generated/🟦️manifest.ts`
**Untouched** — see §4.

### `🗣️dsl/🧬️schema/🦀️component.rs` (new region only)
New region `//#region 🔖️JsonSchema` right after `//#endregion 🔖️Shape`: `shape_json_schema(&Shape) -> Value` (exhaustive over all 26 `Shape` variants — the master.md list plus `Bytes64/Block/Statements/Table/Wire/Angle/Coord/Dir/Dim/EmbedFrom`, which the "…" in the packet brief covers), `record_spec_json_schema(&RecordSpec) -> Value`, private `collect_record_spec_properties` (handles `flatten` by splicing, skips positional-only empty-key fields). 8 tests in a `json_schema_tests` submodule, including round trips over two of the file's own existing fixture-spec fns (`camera_spec`, `writer_note_spec`) — the "existing dsl fixtures" the packet asked for; the file's separate `🧪️fixture-sweep` crate (a heavyweight, feature-gated, repo-wide `.semio` asset walker) was judged out of scope — it proves parse/print round-trip laws over real shipped fixtures, not JSON-Schema derivation, and pulling it in would mean depending on 40+ plugin crates as dev-deps of a new schema fn.

### `🧬️schema/🦀️component.rs`
**No change.** `SchemaCatalog::schema(&self, id: &str) -> Option<&Value>` (L66) already does exactly what §3.4 asks for — returns a registered type's stored JSON Schema by id. Confirmed by reading the file; adding a second, differently-named method would be duplicate surface.

### `📺️renderer/…/ShellHelpers/🟦️component.tsx` (reader)
- Import: added `argControl` alongside the existing `type ActionArgControl` import.
- `resolveActionArgDef`: `def.control.kind !== "select"` → `argControl(def).kind !== "select" || def.schema.kind !== "string"`; the options-relabeling branch now reads/writes `def.schema.options` instead of `def.control.options`/`control:`.
- `renderStagedArgControl`: `def.control` → `argControl(def)`; added explicit `case "artifactKind": case "surfaceApp":` (fallback to a plain text input, with a comment explaining they're host-resolved and never actually reach this renderer) so the switch stays exhaustive over the full `ActionArgControl` union once regenerated.
- Everything else that matched `.control`/`ActionArgControl` in this file (`windowEngagementControlToSpec`, `isUiControlNode`, `declarativeUiChildToTreeItems`) is `WindowEngagementControl`/`UiControlNode` — a different `control`, verified and left untouched.

### `✏️s/🔌️plugins/🧩️puzzle/…/🧊️3d/…/✏️editor/🦀️component.rs` (reader)
- One test (`app_definition_labels_resolve_german_reuse_branded_for_aggregator`): `match &arg.control { ... }` → `match arg.control() { ... options.iter().find(...).cloned().expect(...) ... }` — `.cloned()` added because the match scrutinee is now a temporary owned value, not a field reference, so a borrowed `&ActionArgOption` out of it would have dangled past the `let` statement.

### `🖱️ui/🧱️elements/💬️UIDialog/🧪️story.tsx` (reader — found via full grep sweep, not in the packet's starting list, but genuinely reads `.control`)
- `renderStoryField`: computes `const control = argControl(def);` once, then matches on `control.kind` (previously `def.control.kind` three times) and reads `control.min`/`control.max` instead of `def.control.min`/`.max`.

### Verified false leads (real hits, `.control`/`ActionArgControl`-named, but a DIFFERENT `control` — no change made)
| file | what `.control` actually is |
|---|---|
| `🖱️ui/🧱️elements/🪵️Tree/{🧊️component.rs,🟦️component.tsx}` | `TreeItem.control: Option<Box<UiControlNode>>` |
| `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{🦀️paint,🦀️engine,🦀️reconcile}.rs` | same `UiControlNode` field (no `ActionArgControl` hit at all in these 3 files — the packet's own "measured starting points" list was stale here) |
| `📺️renderer/…/Interpreter/🟦️component.tsx` | `UiTreeItemNode.control` rendered via `renderUiControl` — `UiControlNode` |
| `📺️renderer/…/⚛️react/🧪️index.test.ts` | rendered React elements (`TreeDataItem.control`), not `ActionArgControl` |
| `📺️renderer/…/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` | imports `type ActionArgControl` (unchanged type) but never reads `.control` on anything — no edit, **no lease needed** (contradicts the packet's own "registrar-only, lease" flag for this file; verified empirically) |
| `📺️renderer/…/ShellHost/🟦️component.tsx` | zero `.control`/`ActionArgControl` hits at all — no edit, **no lease needed** (also flagged registrar-only by the packet; verified empty) |
| `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️component.rs` | `UiTreeItemNode.control` → `UiControlNode::Input` |
| `✏️s/🔌️plugins/🎬️sequence/…/📌️panels/📄️artifact/🦀️component.rs` | `item.control = Some(UiControlNode::Toggle(...))` |
| `📐️cad` engine renderer (`spec?.control?.kind === "stepper"`) | cad's own unrelated `UiControlSpec` vocabulary (`"stepper"` isn't even an `ActionArgControl` kind) |
| Puzzle 5d/2d, space (3 more `ActionArgDef`-declaring files) | call the six builders only — producers, not readers; signatures unchanged so nothing to touch |

---

## 3. `CapabilityPolicy.scopes` type choice

Used the real `kernel::CapabilityId` (`Vec<kernel::CapabilityId>`), not a parallel `Vec<String>`. Confirmed clean: `🛂️manifest/🦀️component.rs` already mounts `#[path = "../🎠️kernel/🦀️component.rs"] pub mod kernel;` and already has a field of exactly this type (`ExtensionPointDeclaration.capability_allowance: Vec<kernel::CapabilityId>`, landed by the peer ticket's accepted `A3-kernel-types`) — no new dependency, no cycle, and it makes `ActionSemantics` interoperate directly with `kernel::Broker`'s enforcement primitive instead of inventing a second scope vocabulary the gateway would have to translate.

---

## 4. Typegen — blocked by a pre-existing, unrelated gap

`bun ./📜️script.ts generate` (from `🧰️framework/📦️packages/🦀️rust`) runs `cargo test --features typegen exports_typescript_bindings`, which fails on:

```
error[E0277]: the trait bound `CapabilityToken: TS` is not satisfied
   --> 🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:972:16
969 | #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
972 |     pub token: CapabilityToken,
```

`BrokerCapabilityGrant` (kernel component.rs) derives `ts_rs::TS`; its field `CapabilityToken` does not. This is a derive-macro-level bound, unconditional on any `.export()` call — it would fail on ANY `--features typegen` build of this crate, mine or not. `git log -S "struct CapabilityToken"` dates it to `b92a614c` (2026-08-07), ten days before either ticket opened. **Not caused by P3.**

Fixed my own two bugs first (§2, `EntityId` field-name collision + `ts(optional)` misuse) and confirmed `cargo check --features typegen --tests -p semio-framework` gets down to exactly this one pre-existing error and nothing else — my 13 new types compile cleanly under `--features typegen` on their own.

**Action taken:** lease `📓️lease-P3-kernel-typegen-derive.md` (one-line fix) emitted; `🤖️generated/🟦️manifest.ts` left byte-identical to HEAD (not hand-edited, per this packet's explicit instruction). Consequence: `ArgSchema`/`ArgFormat`/`ArgPresentation`/`ActionSemantics`/etc. exist as Rust types and as hand-written `component.ts` re-exports pointing at generated names that **do not yet exist** in the generated file — TS consumers (`ShellHelpers`, the UIDialog story) will not type-check until the lease lands and `generate` is re-run. Stating this plainly rather than papering over it.

---

## 5. Leases emitted (3)

| lease file | target (theirs) | why |
|---|---|---|
| `📓️lease-P3-shell-wgpu-control-derivation.md` | `📺️renderer/…/🧱️elements/Shell/🧊️component.rs` | registrar-only/H1-H3-contested per this ticket's `📌️important.md`; 6 real `.control` reads, exact diffs given |
| `📓️lease-P3-plugin-component-control-derivation.md` | `🔌️plugin/🦀️component.rs` (OS product) | **discrepancy caught mid-packet**: this packet's own §4 reader list named this file as a direct-edit target, but the peer ticket's `A2-abi-sdk` claims it exclusively (`📌️important.md` collision table + `📋️master.md` row 44, "SDK frozen during their W3"). Treated the peer's collision table as authoritative over our own stale audit; leased instead of editing. File was clean (no uncommitted diff) at read-time. **Update while working**: a concurrent session (not us) has since staged 2 of the lease's 4 sites verbatim (`git diff HEAD` on that file now shows exactly the `&arg.control` → `&arg.control()` fixes at the two non-struct-literal sites) — left alone per "never infer a peer session's live state, don't touch it"; the 2 struct-literal `ActionArgDef { control: ... }` sites (L7174, L7504) still need the lease's `schema: ArgSchema::String {...}` patch |
| `📓️lease-P3-kernel-typegen-derive.md` | `🎠️kernel/🦀️component.rs` | pre-existing typegen-blocking gap, §4 above |

---

## 6. Six-helper regression test + new tests — result

```
test manifest::app_label_tests::six_arg_builder_helpers_derive_the_pre_d6_control ... ok
test manifest::app_label_tests::host_resolved_arg_builders_derive_their_pre_d6_controls ... ok
test manifest::app_label_tests::action_semantics_for_kind_matches_the_defaults_table ... ok
test manifest::app_label_tests::action_definition_semantics_default_from_kind_and_builders_compose ... ok
test manifest::app_label_tests::action_arg_def_json_schema_covers_the_core_shapes ... ok
```
All pass — each of the six builders (`text/number/slider/toggle/select/vec3`) derives byte-identical `ActionArgControl` to what it constructed directly before D6; the two unused host-resolved builders (`artifact_kind/surface_app`) too.

`os_dsl::schema::json_schema_tests::*` (8 tests, `-p semio-framework-os-kernel`): all pass.

---

## 7. Acceptance — full output

### `cargo test -p semio-framework`
```
running 153 tests
...
test result: ok. 153 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
   Doc-tests semio_framework
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Exit code 0.

### `cargo test -p semio-framework-os-kernel`
```
running 1011 tests
...
test result: ok. 1011 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.51s
(+ 3 sibling bin crates: pack/semio/spr — 0 tests each, ok)
   Doc-tests semio_framework_os_kernel — 0 passed
```
Exit code 0.

### `cargo build -p semio-framework -p semio-framework-os-kernel 2>&1 | grep -c "^warning"`
`2` — **not 0**, but both matches are the SAME single warning (one detail line + one "generated 1 warning" summary line), and it is **pre-existing, not ours**: `value assigned to \`pos\` is never read` at `📡️spr/📡️wire/🦀️component.rs:448` — a file we never touched (`git status --porcelain` on it is empty; confirmed by re-running the build after a no-op `touch` on our own file to force a rebuild — same single warning, same location). Zero warnings originate from any file in §1.

### `cargo check --workspace --all-targets`
**Fails — entirely in `semio-framework-plugin-host`, 108 errors, zero of them ours.** Full log: `🧪️p3-workspace-check.txt` in this ticket folder. Sample:
```
error[E0433]: cannot find `semio` in `crate`
   --> 🔌️plugin/🖥️host/…/🦀️component.rs:4377  (wasmtime::component::bindgen! world/import mismatch)
error[E0432]: unresolved imports `actor_bindings::semio::framework::jobs`, `...::reactor`
error[E0119]: conflicting implementations of trait `Debug` for `actor_bindings::...::CapabilityToken` (× ~90, one per WIT type)
error: could not compile `semio-framework-plugin-host` (lib) due to 108 previous errors
```
Confirmed **not ours**: zero mentions of `ActionArgControl`/`.control`/`ArgSchema`/`ActionSemantics` anywhere in the 109-line error list; every failing path is under `🔌️plugin/🖥️host/**`, `🔌️plugin/🌐host/**`, `🔌️plugin/⚛️reactor/**`, `🔌️plugin/🦀️component.rs`, `🔌️plugin/🧬️schema/📜️component.wit` — exactly the peer ticket's `A2`/`B1` `path_scope` (WIT ABI, "frozen during their W3"). `git status --porcelain` on that whole file family shows `M ` right now (a concurrent session mid-edit). Because `cargo check --workspace` aborts the moment a hard-dependency crate fails, it never reached far enough to exercise the two leased-file breakages (§5) or my own downstream TS/reader edits — this run gives no signal on those; they remain verified-by-inspection only (§2) until that peer work lands and a rerun is possible.

### Typegen
See §4 — blocked, not run to completion; `git diff --stat -- 🛂️manifest/🤖️generated/` is **empty** (file untouched).

---

## 8. peer-coexistence

Region line counts, `🛂️manifest/🦀️component.rs`, HEAD → current (every pre-existing region matched by name; only regions we were explicitly told to touch changed size):

| region | before | after | note |
|---|---|---|---|
| `🔖️ArgSchema` | — | 108 | **new**, inserted before `🔖️ActionArgs` |
| `🔖️ActionArgs` | 164 | 336 | required `ActionArgDef` field/method rewrite (§2) |
| `🔖️ActionSemantics` | — | 193 | **new**, inserted before `ActionDefinition` |
| `🔖️Commands` | 103 | 134 | required `CommandDefinition` field/builders (§2) |
| `🔖️Clipboard` | 35 | 35 | unchanged |
| `🔖️Interaction` | 99 | 99 | unchanged |
| `🔖️Utilities` | 79 | 79 | unchanged |
| `🔖️Tools` | 57 | 57 | unchanged |
| `🆔️ElementId` | 85 | 85 | unchanged |
| `🔖️Introduction` | 451 | 451 | unchanged |
| `🔖️TutorialEngine` | 243 | 243 | unchanged |
| `🔖️Tutorial` | 764 | 764 | unchanged |
| `🔖️Dialog` | 78 | 78 | unchanged |
| `🔖️Surface` | 54 | 54 | unchanged |
| `🔖️action-args` | 105 | 105 | unchanged |
| `🔖️TopicContribution` | 27 | 27 | unchanged |
| `🔖️PluginDependency` | 178 | 178 | unchanged |
| `🔖️ArtifactContribution` | 64 | 64 | unchanged |
| `🔖️HostResolvedArgs` | 95 | 95 | unchanged |
| `🔖️DependencyGraph` | 133 | 133 | unchanged |
| `🔖️Kernel` | 4 | 4 | unchanged |
| `🔖️PackageDescriptor` | 144 | 144 | unchanged |
| `ArtifactKind` | 45 | 45 | unchanged |
| `MediaType` | 126 | 126 | unchanged |
| `🔖️AppIo` | 104 | 104 | unchanged |
| `🔖️ConfigSpec` | 57 | 57 | unchanged |
| `🔖️CommandGrammar` | 40 | 40 | unchanged |
| `Media` | 56 | 56 | unchanged |
| `🔖️MediaVocabulary` | 440 | 440 | unchanged |

Every region we did not have an explicit mandate to touch is line-count-identical to HEAD (and, since we made no edits inside them, content-identical). The only region reorganization risk — accidentally reordering or merging an existing region — did not happen: all 26 pre-existing regions besides `🔖️ActionArgs`/`🔖️Commands` are untouched; the two new regions are pure insertions.

`🗣️dsl/🧬️schema/🦀️component.rs`: one new region `🔖️JsonSchema` inserted between the pre-existing `🔖️Shape` and `🔖️Value` regions; neither of those (nor any other region in the file) was touched or reordered.

`🧬️schema/🦀️component.rs`: zero bytes changed (§2).

---

## 9. Files touched — final list

Created/written: `📓️sol-P3-manifest-schema-packet.md`, `📓️terra-P3-report.md`, `📓️lease-P3-shell-wgpu-control-derivation.md`, `📓️lease-P3-plugin-component-control-derivation.md`, `📓️lease-P3-kernel-typegen-derive.md`, `🧪️p3-baseline-hashes.txt`, `🧪️p3-after-hashes.txt` (all in this ticket folder).

Edited (application code): `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx`, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`, `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️UIDialog/🧪️story.tsx`.

Not edited (verified, reasons in §2/§5): `🧬️schema/🦀️component.rs`, `🤖️generated/🟦️manifest.ts`, `Shell/🧊️component.rs`, `🔌️plugin/🦀️component.rs` (os product), `🎠️kernel/🦀️component.rs`, `ShellHost/🟦️component.tsx`, `⚛️react/📦️index.tsx` (renderer target), `⚛️react/🧪️index.test.ts`, `🪵️Tree/{🧊️component.rs,🟦️component.tsx}`, wgpu `{paint,engine,reconcile}.rs`, `Interpreter/🟦️component.tsx`, `🪐️space`/`🎬️sequence` panel files, cad renderer.

## 10. Outstanding / not done

- Typegen not completed (§4) — blocked on peer lease `📓️lease-P3-kernel-typegen-derive.md`.
- `🔌️plugin/🦀️component.rs`'s 2 struct-literal sites (§5) not yet patched by anyone as of this report.
- `cargo check --workspace --all-targets` gives no signal beyond `semio-framework-plugin-host` (§7) — cannot confirm the two leased files, or anything downstream of `semio-framework-plugin-host`, actually compiles clean until that peer crate is fixed and a rerun is possible. Recommend sol re-run `cargo check --workspace --all-targets` once G1 lands, per this ticket's own §5 acceptance note.
