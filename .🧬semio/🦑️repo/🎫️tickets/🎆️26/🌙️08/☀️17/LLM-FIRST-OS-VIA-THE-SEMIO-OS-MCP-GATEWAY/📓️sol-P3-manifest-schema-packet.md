# 📓️ sol → terra packet brief — P3-manifest-schema (verbatim)

Saved verbatim per §0 of the packet brief, as received from sol (main chat coordinator) at session start.

---

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P3-manifest-schema**. Model: Sonnet 5. Coordinator ("sol") is the main chat.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`, `…/📓️design-decisions.md` (**D5 and D6 are your charter**), `…/📓️luna-actions-audit.md` (the census of what you are changing), `/Users/ueli/Documents/semio/CLAUDE.md`.
Save this brief verbatim as `…/📓️sol-P3-manifest-schema-packet.md`.

## 1. Context you must respect
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` is the most contended file in the repo. The peer ticket `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`'s packet `A3-kernel-types` **has been accepted** and released it, but other packets there (`E1-describe`) will return to it later, and other tickets are live in neighbouring files. Therefore:
- **Re-read every file from disk immediately before each edit** and re-check `git log --date=iso --oneline -3 -- <path>`.
- **Surgical, region-scoped edits only. Never rewrite a file wholesale.**
- Your new Rust types go in **new `//#region` blocks**; you do not reorganise existing regions.

## 2. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs          (new regions + the ActionArgDef change)
🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts          (TS twin: argControl())
🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts   (regenerate via typegen, do not hand-edit)
🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs   (new region only: RecordSpec → JSON Schema)
🧰️framework/🔨️modules/🧬️schema/🦀️component.rs             (new region only: JSON Schema export helper)
+ every file that reads `ActionArgDef.control` / `ActionArgControl` and must be updated (see §4)
.🧬semio/…/📓️sol-P3-manifest-schema-packet.md, 📓️terra-P3-report.md, 📓️lease-P3-*.md, *.txt
```

## 3. Required result

### 3.1 `ArgSchema` becomes the stored truth; `control` becomes derived (D6)
New region `//#region 🔖️ArgSchema` in `🛂️manifest/🦀️component.rs`:
```rust
pub enum ArgFormat { ArtifactRef, WindowId, EntityId { kind: String }, IconId, Color, Uri, Json, Locale, Terminology }
pub enum ArgSchema {
    String { options: Vec<ActionArgOption>, min_len: Option<u32>, max_len: Option<u32>, pattern: Option<String>, format: Option<ArgFormat> },
    Number { min: Option<f64>, max: Option<f64>, step: Option<f64>, integer: bool, unit: Option<String> },
    Boolean,
    Vec3 { unit: Option<String> },
    Array { items: Box<ArgSchema>, min_items: Option<u32>, max_items: Option<u32> },
    Object { fields: Vec<ActionArgDef> },
    Any,
}
pub enum ArgPresentation { Slider, IconSelect { classifier_kind: String }, Multiline, Hidden }
```
`ActionArgDef` becomes `{ id, label, schema: ArgSchema, presentation: Option<ArgPresentation>, required, default, description }` — the `control: ActionArgControl` **field is removed** and replaced by a method `pub fn control(&self) -> ActionArgControl` deriving it: String+options→Select; Number with `presentation: Slider` or with both min&max→Slider; Number→Number; Boolean→Toggle; Vec3→Vec3; String+`format: IconId`→IconSelect; otherwise Text. `ActionArgControl` itself stays exactly as it is (it is the renderer's vocabulary).
**The six builder helpers (`text`, `number`, `slider`, `toggle`, `select`, `vec3`) keep their exact signatures** so the ~236 declaration sites do not change. Add `pub fn json_schema(&self) -> serde_json::Value` producing JSON Schema 2020-12 for one arg.
Write a test asserting each of the six helpers still derives the control it produced before this change (that is the regression proof for the whole refactor).

### 3.2 `ActionSemantics`
New region `//#region 🔖️ActionSemantics` with `CapabilityEffects`, `CapabilityPolicy`, `CapabilityExecution`, `ApprovalMode`, `PreviewMode`, `UndoMode`, `IdempotencyMode`, `ExecutionClass`, `ResourceSelector`, `ActionSemantics` exactly as specified in `📋️master.md` §3.1, plus `ActionSemantics::for_kind(ActionKind) -> Self` with the defaults table given there. `ActionDefinition` gains `#[serde(default)] pub semantics: ActionSemantics` (defaulting from its `kind`) and builders `.semantics(..)`, `.destructive()`, `.use_when([..])`, `.example(..)`. `CapabilityPolicy.scopes` is `Vec<String>` **unless** depending on `kernel::CapabilityId` is clean here — check; prefer the real type if it does not create a cycle, and say which you chose and why.
**Do NOT define `CapabilityDefinition` here** — per D5 it lives in the gateway crate.

### 3.3 `RecordSpec` → JSON Schema
New region in `🗣️dsl/🧬️schema/🦀️component.rs`: `pub fn record_spec_json_schema(spec: &RecordSpec) -> serde_json::Value` mapping every `Shape` variant to JSON Schema (Bool/Int/UInt/Float/Text/Enum/Tuple/List/Record/Map/Value/Range/Count/Expr/Embed…; carry unit as `x-semio-unit` and `Shape::Ref(kind)` as `x-semio-ref`). Tests over the existing dsl fixtures.

### 3.4 Schema export helper
New region in `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`: a way to get a registered type's JSON Schema out of `SchemaCatalog` (e.g. `pub fn json_schema(&self, id: &str) -> Option<&Value>`) — check whether `schema()` already does this and, if so, say so in your report and add nothing.

## 4. Updating the readers (the risky half — do this deliberately)
`grep -rn --include='*.rs' --include='*.ts' --include='*.tsx' "ActionArgControl\|\.control" 🧰️framework ✏️s | grep -v node_modules` and work the list. Measured starting points (some are `UiControlNode`/keybinding "control" and are NOT yours — verify each hit before touching it):
- Rust: `🖱️ui/🧱️elements/🪵️Tree/🧊️component.rs`, `🖱️ui/…/🎯️targets/🧊️wgpu/{🦀️paint,🦀️engine,🦀️reconcile}.rs`, `🔌️plugin/🦀️component.rs`, `📺️renderer/…/🧱️elements/Shell/🧊️component.rs`, and 6 plugin editor/panel files under `✏️s/🔌️plugins/{🎬️sequence,🪐️space,🧩️puzzle}`.
- TS: `🛂️manifest/🟦️component.ts` (+ generated), `🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx`, `📺️renderer/…/🧱️elements/{Interpreter,ShellHelpers}/🟦️component.tsx`, `⚛️react/📦️index.tsx`, `⚛️react/🧪️index.test.ts`.
**`Shell/🧊️component.rs`, `ShellHost/🟦️component.tsx` and `⚛️react/📦️index.tsx` are registrar-only** (and contested by the peer ticket's H1/H3): for those, emit a `lease-request` with the exact old→new text instead of editing, and continue.
In TS, add `export function argControl(def: ActionArgDef): ActionArgControl` to `🛂️manifest/🟦️component.ts` mirroring the Rust derivation, and point the readers at it.

## 5. Acceptance (FOREGROUND, paste every command's output + exit code)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-kernel
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework -p semio-framework-os-kernel 2>&1 | grep -c "^warning"   # → 0
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check --workspace --all-targets      # the real proof: nothing in the tree broke
```
plus the repo's typegen command for `🛂️manifest` (copy it from the module's `📜️script.ts`/`📋️project.json`) and `git diff --stat -- 🧰️framework/🔨️modules/🛂️manifest/🤖️generated/`.
`cargo check --workspace` is expected to be slow (many minutes) and may surface **pre-existing** breakage from the peer ticket's in-flight A2/B1 work — if it fails, determine whether the failure is in a file you touched or in theirs (`git log --date=iso --oneline -3 -- <failing file>`), and report that distinction precisely. Do not "fix" their files.

## 6. Hard rules
All of `📌️important.md`. In particular: **no background builds**; surgical edits only; re-read before each edit; nothing outside §2 (lease instead); no `.log`; no unpasted claims; never edit `AGENTS.md`; no compat shims — this is a straight replacement of a field by a derivation, with no transition period and no duplicate state.

## 7. Report
`…/📓️terra-P3-report.md`: baseline HEAD + per-file SHA-256 before/after; the complete list of reader files with what changed in each; the leases emitted; the six-helper regression test result; full acceptance output; and an explicit `## peer-coexistence` section showing the manifest regions you added are additive and that no pre-existing region was reorganised (line counts before/after per region).
