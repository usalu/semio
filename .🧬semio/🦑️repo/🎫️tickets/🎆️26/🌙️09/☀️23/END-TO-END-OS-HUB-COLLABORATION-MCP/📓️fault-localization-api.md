# Fault Localization — Frozen API, Family Split, Declaration Checklist (S20, 2026-09-29)

Owner of the framework half (F1) and the codemod (F2): S20. F3 (handcrafted en/de declarations) is split across helper
agents per family below. Everything is prepared on ONE overlay and lands in ONE train with the strict global law green.
Coordinator decisions 08:0x–08:5x: typed record on the wire, apps declare every refusal code with en/de text, hosts and
agents render by code, framework-owned codes declared in the framework catalog, strict law, no ledger.

## 1. The frozen API (F1)

**Raising a refusal (app code, every non-test source):**

```rust
use semio_framework_plugin::app_fault; // re-export of semio_framework::app_fault
return Err(app_fault("remodeling.qc-report.missing"));
return Err(app_fault("process3d.media.export-format").with_parameter("format", format_kind));
```

- `app_fault(code: &'static str) -> Fault` — origin `App`, the code, NO free-text message. The code is always a string
  literal (the census reads it); computed codes (`FaultCode::new(expr)`, `format!` codes) are forbidden in app crates.
- `.with_parameter(name: &'static str, value: impl Into<String>) -> Fault` — one runtime value the declared text shows
  (`{name}` placeholder). At most 8 parameters; name `[a-z][A-Za-z0-9]*` ≤ 32 bytes; value ≤ 64 UTF-8 bytes (clipped on a
  character boundary; control characters become spaces).
- `Fault::from(…)` (answers the untyped `app.message`) and `Fault::new(FaultOrigin::App, …, "sentence")` are gone from app
  crates. A refusal is a code + parameters; the person reads the DECLARED text.
- Types (compile-enforced on the overlay): `FaultCode::new(code: &'static str)`; `Fault::new(origin, code: FaultCode,
  message)`; NO `From<&str>/From<String>` into `Fault` or `FaultCode` (so `?`/`.into()` on text no longer compiles);
  `FaultCode::received(String)` only for a decoded record's `code`; `Fault.parameters: FaultParameters` (boxed, one word;
  `.as_slice()`); `DiagnosticCode` removed.
- Framework crates raise `Fault::new(FaultOrigin::<layer>, FaultCode::new("<literal or const>"), "<developer detail>")`
  (+ `.with_parameter`), catalogued in the framework catalog; never `app_fault`, never `.fault(`.

**Declaring it (the app's definition builder — the same chain that carries `.action_describe`):**

```rust
.fault("remodeling.qc-report.missing", LocalizedLabel::native(
    "Run the quality check before exporting its report.",
    "Führen Sie die Qualitätsprüfung aus, bevor Sie ihren Bericht exportieren."))
.fault("process3d.media.export-format", LocalizedLabel::native(
    "The format {format} cannot be exported.",
    "Das Format {format} kann nicht exportiert werden."))
```

- Manifest: `AppDefinition.faults: Vec<FaultDefinition { code, text: LocalizedLabel, parameters: Vec<String> }>`
  (`parameters` is derived from the text's `{name}` placeholders; wire `{"code","text","parameters"}` camelCase).
- Declare on an app definition of the plugin that raises the code (every app that can raise it is best; the census law
  is per PLUGIN: raised in the plugin ⇔ declared in the plugin, one plugin declares one code one way). Hosts look the
  text up by (plugin, code). `AppBuilder::try_build_definition` refuses a malformed declaration (code grammar, duplicate,
  empty/equal en-de cells, placeholders ≠ `parameters`) — `app-definition.invalid`.

**Framework-owned codes** (`interactive-job.*`, `file-import.*`, `artifact-store.*`, `plugin.*`, … raised by framework
crates) are declared once in the framework catalog `🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json` (schema
`🗂️catalog/🧬️schema/🔣️.json`, `semio.fault-catalog.v1`):

```json
{ "schema": "semio.fault-catalog.v1", "faults": [
  { "code": "interactive-job.cancelled", "en": "The operation was cancelled.", "de": "Der Vorgang wurde abgebrochen." },
  { "code": "file-import.gap", "en": "Part of the file arrived out of order; pick the file again.", "de": "Ein Teil der Datei kam in falscher Reihenfolge an; wählen Sie die Datei erneut aus." }
] }
```

**On the wire:** `semio.typed-operation-fault.v1` = `{schema, code, origin, message, parameters: [{name, value}]}`
(set `s20-patch-fault-code.py`, v3 — `parameters` required, at most 8; on the overlay). Hosts render `text` of the declared code in the person's locale and
terminology with the parameters filled; MCP agents receive `{code, parameters, texts: {en, de}}`.

**Hosts (F1, on overlay):** the shell renders a fault BY ITS CODE — `faultTextV1` (`🏛️ShellHost/🩺️fault/🟦️.ts`): an
`app` fault reads its plugin's declared text (`declaredFaultsV1(programs, pluginId)`), any other origin the catalog;
parameters filled; the error banner shows it (fallback: the code). Rust twin `semio_framework::fault_text` +
`framework_fault_catalog()`; both read `⚠️diagnostic/🧫️fixtures/🧯️fault/🔣️text.json`. **MCP:** every gateway error that
answers a fault carries `details.fault = {code, parameters, texts: {en, de} | null}` (guest faults decoded with the
channel descriptor's declarations). **Landing note:** every committed plugin descriptor must be regenerated (`describe`)
after F3 so `AppDefinition.faults` reaches hosts and the gateway.

**The strict law (global):** `bun ./📜️script.ts verify faults` (permanent census verb) + the Rust app law — every code any
app/framework crate can raise is a literal, and is declared (app definition or framework catalog) with non-empty en and
de text whose placeholder sets are equal and equal to the parameters raised; no `Fault::from`/`FaultCode::new(expr)` in
app crates; no declared code that nothing raises. No exceptions, no ledger.

## 2. Family split for F3 (sizes = census sites: typed codes · `Fault::from` code-like / sentence / dynamic · computed)

| Family | Plugins | Sites | Notes |
|---|---|---|---|
| A | framework (SDK + framework modules) — the catalog | 515 (438 typed codes) | internal invariant codes too |
| B | 🗄️stdio | 480 (117 · 158/10/120 · 75) | 75 computed + 120 dynamic: format codecs build codes → literal per case |
| C | 🧩️puzzle (2d/3d/5d) | 340 (0 · 281/41/18) | 3 editors share the retained module |
| D | 🏗️fem, 🖍️draw | 310 | draw: 87 sentences |
| E | 🌊️flow, 🌀️procedural, 🎬️sequence | 332 | |
| F | 🖨️raster, 🀄️wfc, ➗️mathematical, 📋️forms, 🔱️trinity | 337 | |
| G | 🪐️space, 🏛️architect, 💡️reasoning, 🌿️vcs, 💠️lowpoly, 🌍️gis, 📐️cad, 🗒️note, 🏭️process, 📕️norm, 📏️layout | 317 | |
| H | ✒️writer, 📸️remodel, 🔋️energy, 🎞️animate, 🪵️sourcing, 🧱️block, 🕸️dag, 🎥️shooting, 🎪️demonstrator, 📜️imperative, 📖️playbook | 118 | |

Census data: `.🧬semio/🌐hub/s14-s20-sets/fault-census-2.json` (tool `wp-s20/s20-fault-census.py`). F2's codemod
pre-converts every `Fault::from("code-like")` to `app_fault("code-like")` and pre-fills a declaration stub with the old
English sentence where one existed; F3 finishes the rest by hand.

## 3. Declaration checklist (every helper, every code)

1. Every refusal in the family's non-test sources is `app_fault("…")` (+ `.with_parameter`), never `Fault::from`,
   `Fault::new(FaultOrigin::App, …, text)`, `FaultCode::new(expr)` or a `format!` code.
2. Code grammar `[A-Za-z0-9._/-]`, ≤ 64 bytes, prefixed by the artifact/plugin (`<artifact>.<area>.<reason>`, kebab
   reason). Existing well-formed codes are frozen contract — keep them. New codes for former sentences/format! sites.
3. Every runtime value a former message interpolated becomes a named parameter; the text shows it as `{name}`.
4. `.fault(code, LocalizedLabel::native(en, de))` on every app definition of the plugin that can raise it; no duplicate
   code per app; no declared code nothing raises.
5. en: sentence case, ends with a period, says what happened and what the person can do, no internal jargon (no Rust
   types, no "retained", "reducer", "wire", "owner" …) — internal invariant failures say what broke in user terms
   ("The document could not be updated; reload it.").
6. de: proper German, formal "Sie" (repo convention), same placeholders, same meaning — not a word-by-word copy.
7. No free-text refusal left; no English in a `de` cell; placeholders identical across both cells.
8. `bun ./📜️script.ts verify faults` green for the family's plugins; the family's crates compile (`cargo check --lib
   --tests -p <crates>` on the overlay) and their existing laws that assert refusal codes/messages are updated to the
   new codes (tests assert CODES, never message text).

## 4. F1 on overlay — helper brief (2026-09-29, S20)

**Overlay** `.🧬semio/🌐hub/s14-s20-overlay-faults/` (APFS clone; baseline `…overlay-faults.baseline.json`; diff tool
`.tmp-ticket/wp-s20/s20-overlay-diff.py diff`). Edit ONLY overlay files of your family; never the live tree, never git.

**Census (the law) on the overlay** — per family, fast feedback:
`cd .🧬semio/🌐hub/s14-s20-overlay-faults && GIT_DIR=/Users/ueli/Documents/semio/.git GIT_WORK_TREE="$PWD" NX_DAEMON=false bun ./📜️script.ts verify faults <plugin-dir…|framework>`
(prints the family's violations; full JSON at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤖️generated/🎯️acceptance/🧯️fault-census.json`
inside the overlay). Rules: `fault-from`, `app-raise-form` (FaultCode::new/Fault::new in an app crate), `computed-code`,
`computed-parameter`, `received-code`, `code-tuple`, `declaration-form`, `framework-declaration`, `framework-app-fault`,
`undeclared`, `unraised-declaration`, `declaration-conflict`, `uncatalogued`, `unraised-catalog-entry`,
`catalog-duplicate`, `text-empty`, `text-untranslated`, `placeholder-mismatch`, `parameter-mismatch`, `code-grammar`,
`parameter-grammar`, `unresolved-const`. Self-test: `bun test 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧪️tests/🧮️source-census/🟦️.ts`.

**Compile on the overlay** (ONE private build-dir shared by all helpers, overlay lane only, foreground):
`zsh .tmp-ticket/wp-s20/s20-f1/overlay-cargo.sh <tag> check --offline --lib --tests -p <your crates>` (log path printed;
logs under `.🧬semio/🌐hub/s14-s20-overlay-build/logs/`). The framework crates must be green first (S20 reports it).

**Per family** (census 2026-09-29 10:4x before F3; per-file rule counts + raised codes in
`.🧬semio/🌐hub/s14-s20-sets/census/family-<X>.json`):

| Family | Owners | Files | Distinct raised codes | Open rules |
|---|---|---|---|---|
| A | framework (`🧰️framework`, `✏️s/🔨️modules`, `🌎️hub`) — fills the catalog | 29 | 432 | uncatalogued 653, fault-from 50 (S20 converted), computed-code 18, framework-declaration 41 (`store` schema-json `.fault(` helper) |
| B | 🗄️stdio | 160 | 38 | app-raise-form 363, fault-from 130, undeclared 158 |
| C | 🧩️puzzle | 15 | 237 | fault-from 59, undeclared 281 |
| D | 🏗️fem, 🖍️draw | 63 | 103 | fault-from 146, app-raise-form 54, undeclared 141 |
| E | 🌊️flow, 🌀️procedural, 🎬️sequence | 56 | 189 | app-raise-form 106, fault-from 42, undeclared 260 |
| F | 🖨️raster, 🀄️wfc, ➗️mathematical, 📋️forms, 🔱️trinity | 75 | 159 | app-raise-form 126, fault-from 60, undeclared 228 |
| G | 🪐️space, 🏛️architect, 💡️reasoning, 🌿️vcs, 💠️lowpoly, 🌍️gis, 📐️cad, 🗒️note, 🏭️process, 📕️norm, 📏️layout | 99 | 130 | app-raise-form 322, fault-from 48, undeclared 160 |
| H | ✒️writer, 📸️remodel, 🔋️energy, 🎞️animate, 🪵️sourcing, 🧱️block, 🕸️dag, 🎥️shooting, 🎪️demonstrator, 📜️imperative, 📖️playbook | 56 | 35 | app-raise-form 144, fault-from 37, undeclared 38 |

The 1 287 former `Fault::from("code-like")` sites are already `app_fault("…")` (F2a); their old English sentence is gone
(the literal WAS the code) — write the text from what the code site checks. `Fault::new(FaultOrigin::App, FaultCode::new("x"),
"sentence")` → `app_fault("x")` with the sentence as the en stub. `Fault::from(format!(…))` → a new code +
`.with_parameter` per interpolated value. Also fix every `?`/`.into()` that relied on `From<String> for Fault` (the
compiler lists them).

**Done =** `verify faults <your owners>` prints 0 violations, your crates `check --lib --tests` green on the overlay,
existing laws assert CODES (never message text). Do not close tickets, do not touch `🗑️generated/`, do not delete files
of other families, no git.

## 5. Landing (T6 row 12, LAST T6 step) and pass 2

- Pass 1 (refusals) lands once every family is at 0 `verify faults` violations and the family crates check green on the
  overlay. Rebase onto the post-T6 tree: `python3 .tmp-ticket/wp-s20/s20-overlay-land.py plan` (3-way per file:
  base = the baseline version from history, ours = overlay, theirs = live; report + staged files under
  `.🧬semio/🌐hub/s14-s20-landing/`), resolve conflicts on the overlay, re-plan to 0 conflicts, `apply` inside the train,
  then `verify faults` + the family checks on the live tree. Helpers keep edits anchored (no reformatting, no moves).
- Post-T6 additions to catalogue at the rebase: P9 fail-closed `app.command.no-effect` (params `{action}`, `{notice}`,
  raised by the SDK preview). Every committed plugin descriptor is regenerated (`describe`) after the landing.
- Pass 2 (coordinator decision 11:3x): mutation reports (`MutationOutcome::error/fatal`, `MutationMessage::*`, builder
  `.warn/.info`, ~3.7k sites) localized by code — constructors take `FaultCode` only (the `MutationMessageCode` literal
  arm removed), codemod, family declarations, host renders reports by code, law scope extended. Its own set, same helpers.

## 6. Pass 2 — mutation reports by code (frozen 2026-09-29 20:4x, S20; coordinator decision 20:0x)

**Scope.** Every mutation report (`MutationOutcome::{error,fatal}`, `MutationMessage::{info,warn,error,fatal}`, the outcome
builders `.info/.warn`, the framework's apply rejection) — 3 780 sites incl. tests; 3 584 already use one of the 7 FROZEN
codes of contract C2 (`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md`: generic,
gate-enforced, no per-plugin codes), 196 use drift codes (`mutation.missing`, `mutation.duplicate`, `mutation.missing-target`,
`mutation.child-identity`, `mutation.content-gap`, `wfc3d.slot.missing`, forwarded `error.code`, …).

**API (framework, `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`, re-exported wherever `MutationOutcome` is):**

```rust
/// The frozen report codes (contract C2) — catalogued once in the framework fault catalog (en/de).
pub enum MutationCode { TargetMissing, NoOp, Partial, Clamped, DuplicateId, Invariant, Cascade }
impl MutationCode { pub fn code(self) -> FaultCode /* literal FaultCode::new("mutation.target-missing") … per arm */ }
pub struct MutationMessage { pub level: Severity, pub code: FaultCode, pub target: Vec<String>, pub op_index: Option<u32> }
MutationMessage::{info, warn, error, fatal}(code: MutationCode) -> Self;  .at(target)  .at_op(i)
MutationOutcome::fatal(code: MutationCode, target) / ::error(code: MutationCode, target);  .info(code)  .warn(code)
pub struct MutationApplyError { pub code: MutationCode, pub target: Vec<String> }   // apply_to → Fatal with error.code
```

- Every public constructor takes `MutationCode` (apps can only report the frozen set); `code: FaultCode` on the record so
  framework-internal reports (document-link terminal status, conflict grading) carry their CATALOGUED framework codes.
- NO `message` anywhere (a report is a code + its target; the person reads the catalog text in their locale; the target
  names the element). NO parameters (the 7 texts are generic). NO per-plugin report codes; a new variant needs a contract
  amendment by the coordinator.
- Removed: `MutationMessageCode` (+ its `&'static str` arm) and the census exclusion `FAULT_PASS2_BRIDGE` — in the SAME
  landing. Levels stay explicit at every site (merge-policy semantics unchanged).
- Wire (`ToValue`/`FromValue` + serde twin + schema-first JSON Schema of the outcome record + TS twin):
  `{level, code: "mutation.<frozen>", target?, opIndex?}` — `message` removed; decoding refuses a code outside the set.
- Hosts render a report BY CODE from the framework catalog (React `faultTextV1` path for a non-app origin, wgpu shell,
  MCP outcome messages `{level, code, target, texts: {en, de}}`).
- Law: the 7 `MutationCode::code()` arms are framework raises (catalogued); new census rule `app-mutation-code`: an app crate
  never raises a `mutation.*` code (the namespace is the framework's).

**Work split.**
- P2-F1 (S20): framework API + catalog texts + wire/schema/TS + hosts + census rule; P2-F2 (S20): codemod
  `wp-s20/s20-p2/p2-codemod.py` rewrites every frozen-code site (drops the message, maps the literal to the variant, adds
  the `MutationCode` import next to the `MutationOutcome` one) and writes the drift-site ledger
  `.🧬semio/🌐hub/s14-s20-sets/p2/drift-sites.json` (path, line, code, level, message).
- P2-F3 (helpers, source-only, static): every drift site → the frozen code that MEANS the same (target gone → TargetMissing;
  would break a rule / capacity / malformed payload → Invariant; nothing to do → NoOp; value adjusted → Clamped; only part
  applied → Partial; id taken → DuplicateId; dependents changed → Cascade) — never a new code; the target must name the
  element; forwarded codes (`error.code`, `issue.code`) become the variant the producer maps to; tests assert `code`/`target`,
  never message text. Overlay `.🧬semio/🌐hub/s14-s20-overlay-faults-p2/` (clone of the rebased pass-1 overlay, baseline
  `…-p2.baseline.json`, tools `S20_SET=p2 python3 wp-s20/s20-overlay-land.py …`). Done = no drift site left in the family
  (ledger re-run), `verify faults` 0 on the p2 overlay.
- Landing: after row 12 lands, `S20_SET=p2 … rebase` onto live, compile proof in the overlay lane, then its own T7 row.

## 7. Fault classes (row 12, frozen 2026-09-29 21:0x, S20; coordinator decision 20:5x)

Every declared code carries ONE class (schema-first enum `FaultClass`, kebab on the wire). Hosts and os-mcp answer a fault
BY ITS CLASS — no per-code map, no fallback (an undeclared code at runtime is itself a defect: `internal`).

| class | meaning (who fixes what) | os-mcp `GatewayErrorCode` |
|---|---|---|
| `input-invalid` | the request itself is wrong (argument, value, file content, unknown action/kind); correcting the input fixes it | `InputInvalid` |
| `precondition-failed` | the input is fine but the current state does not allow it (nothing selected, element gone, window not open, locked, already done) | `PreconditionFailed` |
| `conflict` | someone else changed it concurrently (stale revision/generation); reload and redo | `RevisionConflict` |
| `permission-denied` | the person or agent lacks the right (read-only, revoked) | `PermissionDenied` |
| `unavailable` | transient: busy, not ready, offline, budget/timeout; retrying unchanged later can succeed | `PluginUnavailable` (retryable) |
| `cancelled` | the person or the system cancelled the operation | `Cancelled` |
| `internal` | a defect: broken invariant, internal routing/ownership/registry, lost page — the person can only reload/report | `Internal` |

- Declaration: `.fault(code, FaultClass::InputInvalid, LocalizedLabel::native(en, de))`; framework catalog entry
  `{"code", "class", "en", "de"}` (schema `semio.fault-catalog.v1` gains the required `class` enum).
- `FaultDefinition { code, class, text, parameters }` (+ owned TS projection); record v3 `semio.typed-operation-fault.v1` gains
  the required `class`, stamped by the producer from the app's declarations (app origin) or the catalog (any other origin).
- Law: `verify faults` rules `class-missing` (a declaration or catalog entry without a class) and `class-unknown`.
- REVIEW (helpers, per family, work lists `.🧬semio/🌐hub/s14-s20-sets/class/family-{A,BD,CEFG,H}.json`, pre-classified by
  `wp-s20/s20-class/class-heuristic.py`; `rule` shows why, `default` = no cue): correct ONLY the `class` field of each entry
  (one entry per line; keep code/en/owner untouched), judging by what the raise sites check (read them: `verify faults`
  census JSON lists every raise site) and by the table above. The same code raised for different reasons is reported in the
  helper's report (never split codes now). S20's codemod `wp-s20/s20-class/class-apply.py` writes the reviewed classes into
  every `.fault(` declaration and the catalog.
