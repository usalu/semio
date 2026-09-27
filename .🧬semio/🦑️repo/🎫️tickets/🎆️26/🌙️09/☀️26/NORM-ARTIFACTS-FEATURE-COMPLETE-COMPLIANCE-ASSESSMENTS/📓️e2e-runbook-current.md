# E2E Runbook — Norm Artifacts (current code, Wave B/C/D)

**Author:** Composer 2.5 · **Date:** 2026-09-26 · **Supersedes:** `📓️audit-e2e-user-path.md` (Wave A — stale)

Wave A claimed flat scalar subjects, English-only results, and read-only JSON inputs. Current code (post–Wave B/C) has hierarchical `Din4108Snapshot` documents, schema-driven localized Inputs editors, bilingual `CheckResult` rows with remedies, and granular mutation verbs. This runbook reflects **current** sources only.

---

## 1. Open one family editor (DIN 4108 template)

| Item | Value | Evidence |
|------|-------|----------|
| **launch.json name** | `🛠️dev📕️norm🧱️din4108⚛️react` | `.vscode/launch.json` L6295–6314 |
| **Command** | `bun nx run workspace:dev -- din4108` | same |
| **Port** | `6091` (`S_OS_PORT`) | same |
| **App id** | `s.norm.din4108@1/*#editor` | `SEMIO_APP` in launch.json; `CONTROLLER_ID` in `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs` L32 |
| **Browser URL** | `http://127.0.0.1:6091/` | `serverReadyAction.pattern` in launch.json; playground boots pinned app via `VITE_SEMIO_APP_ID` (`🧰️framework/…/♻️activation/🌐️serve/🟦️.ts` L71) |
| **Wgpu variant** | `🛠️dev📕️norm🧱️din4108🧊️wgpu🌐️wasm`, port `6191` | `.vscode/launch.json` L6317–6334 |
| **Playground registry** | `variant: "din4108"`, `app: "s.norm.din4108@1/*#editor"`, `ports.react: 6091` | `🧰️framework/…/🎮️playgrounds/🟦️.ts` |

Boot layout: **Edit** mode, row split **Inputs 42%** / **Results 58%** (`🎭️modes/✏️edit/🦀️.rs` L16).

---

## 2. Current mutation verbs — where they live and what they change

All eight editor verbs are declared in `create_din4108_app()` (`✏️editor/🦀️.rs` L98–105, L240–288) and implemented in shared `🖥️app-surface/🦀️.rs`.

### `setField`

- **Args:** `{ path, value }` — camelCase JSON path into the document value tree (`🖥️app-surface/🦀️.rs` L656–663, L1574–1578).
- **UI location:** **Inputs** window (`norm.din4108.play.inputs`) — every scalar field is a bound control (number input, select, bool select) that fires `setField` on change (`render_value_editor`, L1073–1128).
- **Labels (en/de):** field labels from `🏷️field-meta/🦀️.rs` via `chrome(label_en, label_de, locale)`; e.g. root `airtightnessN50` → **Airtightness n₅₀** / **Luftdichtheit n₅₀**.
- **Effect:** one undoable semantic mutation bundle over the targeted path (`commit_value_tree_edit`, L645–653).

### `insertItem`

- **Args:** `{ path, index, value? }` (`L666–673`).
- **UI location:** Inputs tree on **array** nodes — action row **Add item** / **Eintrag hinzufügen** (`render_array_editor`, L1060–1065). Inserts at `index` (default append uses `items.len()`).
- **Effect:** inserts into arrays such as `zones`, `zones[].windows`, `elements`, nested layer lists (field-meta paths in `🏷️field-meta/🦀️.rs`).

### `removeItem`

- **Args:** `{ path, index }` (`L676–683`).
- **UI location:** each array row in Inputs — **Remove** / **Entfernen** under the row group (`L1039–1046`). Path uses list-element selector (`list_element_path`).
- **Effect:** removes one array element.

### `applyRemedy`

- **Args:** `{ checkId, remedyIndex }` (+ optional OneOf via `setField` on remedy options) (`L742–760, L686–740`).
- **UI location (two surfaces):**
  1. **Results** window — expand a **Fail** or **Warning** check; child rows **Apply — Remedy: …** / **Anwenden — Abhilfe: …** (`build_check_tree_item`, L901–944).
  2. **Inspection** panel (`norm.din4108.play.inspection`) — **Apply 1**, **Apply 2**, … on remedy rows (`render_inspection`, L1362–1385).
- **OneOf remedies:** Results tree offers **Choose — {option}** / **Wählen — {option}** rows that call `setField` on `remedy.target.path` (`L903–923`).
- **Effect:** reads the current report, writes `remedy.required` (SI) or selected OneOf string into `remedy.target.path` (`apply_remedy_edit`, L686–740). Uses cached report or runs `F::evaluate(document)` once for lookup (`L759`) but does **not** persist that report to the UI cache.

### Related verbs (not in scope question but needed for E2E)

| Verb | UI | Purpose |
|------|-----|---------|
| `setActiveExample` | **Catalogue** panel → **Demo Session** / **Demo-Sitzung** | Loads bundled example (`📚️examples/🎬️demo-session/🦀️.rs`) |
| `evaluate` | **Actions** pane (top-left) → **Evaluate** / **Auswerten** | Recomputes and caches `CheckReport` for current document revision (`NormEvaluateCommandWork`, `🖥️app-surface/🦀️.rs` L1939–1942) |
| `setSnapshot` | Actions pane (destructive whole-document replace) | Still available; cap now `NORM_RETAINED_RAW_BYTES = 524_288` (L1796), not 8 KiB |

**Report cache rule:** `cached_report_for` keys by `(family, document_revision_hash)` (`⚖️compliance/🦀️.rs` L855–872). After `setField` / `insertItem` / `removeItem` / `applyRemedy`, revision changes → **no cached report** → Results render empty until **Evaluate** is run.

---

## 3. How `evaluate` produces a localized report

### Trigger

- User runs **Evaluate** / **Auswerten** from the window **Actions** pane (`ActionDefinition` in `✏️editor/🦀️.rs` L245–246; shell label **Actions** / **Aktionen**, `🐚️Shell/…/wgpu/🦀️.rs` L29911–29912).
- Retained job `NormEvaluateCommandWork` invalidates cache, runs `Din4108Family::evaluate`, stores report (`🖥️app-surface/🦀️.rs` L1939–1942; evaluate impl `✏️editor/🦀️.rs` L215–217 → `💡️inferences`).

### Windows and verdict copy

| Surface | body_key | Shows |
|---------|----------|-------|
| **Results** / **Ergebnisse** | `norm.din4108.play.results` | Grouped check list (`render_report`, `🖥️app-surface/🦀️.rs` L949–967) |
| **Artifact** / **Dokument** panel | `norm.din4108.play.artifact` | Summary headline via `render_summary` (L1149–1180) |
| **Inspection** / **Inspektion** panel | `norm.din4108.play.inspection` | Full selected-check card (`render_inspection`, L1326–1390) |

**Complies / does not comply (summary):**

- Artifact panel: `{family label} — {total} checks: {verdict}: {fail} failing, …` where `verdict` is **complies** / **erfüllt** or **does not comply** / **nicht erfüllt** (`render_summary`, L1153–1157).
- Results part headers: `{part} — {complies\|does not comply} (✓pass ⚠warn ✗fail · u={worst})` / `{erfüllt\|nicht erfüllt}` (`part_verdict_label`, L870–882).

**How it could comply (remediation):**

- On Fail/Warning checks, expanded Results rows show **Explanation** / **Erläuterung** plus remedy actions (`build_check_tree_item`, L894–944).
- Inspection panel lists each remedy with action text, current → required quantities, and target path (`render_inspection`, L1362–1385).

### Current result-row fields (not Wave A `message: String`)

**`CheckResult`** (`⚖️compliance/🦀️.rs` L233–246):

```text
id, part, clause, subject { entityId, path, label { en, de } },
status, title { en, de }, explanation { en, de },
computed, limit, utilization, annex, remedies[]
```

**`Remedy`** (L193–201): `target`, `current`, `required`, `bound`, `options`, `action { en, de }`, `applicable`.

**Results list row label** (`check_row_label`, `🖥️app-surface/🦀️.rs` L830–848):

```text
{index}. [{Pass|Warning|Fail|Not applicable}] {title} — {subject[@ path]} — {computed} vs {limit} u={utilization} — {clause}
```

(German status: **Bestanden**, **Warnung**, **Nicht bestanden**, **Nicht anwendbar** — `status_label`, L225–230.)

**Inspection card fields** (L1336–1361): Id, Part, Clause, Subject, Subject path, Status, Title, Explanation, Computed, Limit, Utilization, Annex, Remedy rows.

Locale axis: shell `view_state.locale` → `protocol::Locale::En` / `De` for `LocalizedCopy.resolve` (`protocol_locale`, L203–208). Chrome strings use `LocalizedLabel::native(en, de)`.

---

## 4. Concrete browser script (do not execute here)

Use **React** playground unless testing wgpu parity.

### 4.1 Launch

1. VS Code: run **`🛠️dev📕️norm🧱️din4108⚛️react`**.
2. Open **`http://127.0.0.1:6091/`** (no path — app is pinned at boot).

### 4.2 Load example

1. Open bottom-left **Workbench** dock.
2. Select panel **Catalogue** / **Katalog**.
3. Under section **Examples** / **Beispiele**, click **Demo Session** / **Demo-Sitzung** (`setActiveExample` with `exampleId: "demo-session"`).

### 4.3 Baseline evaluate

1. Open window **Actions** / **Aktionen** (top-left engagement chip on either dock window).
2. Click **Evaluate** / **Auswerten**.
3. Confirm **Results** / **Ergebnisse** window section **Checks** / **Nachweise** lists checks (not **No checks computed.** / **Keine Nachweise berechnet.**).

### 4.4 Change one subject field (`setField`)

1. In **Inputs** / **Eingaben**, expand **Document** / **Dokument**.
2. Find **Airtightness n₅₀** / **Luftdichtheit n₅₀** (path `airtightnessN50`).
3. Change value from **1.5** to **4.5** (known failing value — `🧪️tests/⚖️compliance/🦀️.rs` L419–421).
4. Tab out or confirm so `setField` commits.
5. Run **Evaluate** again (required — cache miss on new revision).
6. In Results, locate check id **`din4108-7.n50`**: row status **[Fail]** / **[Nicht bestanden]**, title **Airtightness n50** / **Luftdichtheit n50**.

### 4.5 Apply remedy on failing check

1. Expand the failing **`din4108-7.n50`** row in Results.
2. Click **Apply — Remedy: …** / **Anwenden — Abhilfe: …** (shows current → required quantities).
3. Run **Evaluate** again.
4. Same check should show **[Pass]** / **[Bestanden]** (`remedy_law_airtightness_n50_fixes_n50`, compliance tests L332–341).

Alternative: **Inspection** / **Inspektion** panel → select check → **Apply 1** / **Anwenden 1** on remedy row → **Evaluate**.

### 4.6 View English and German

1. Open bottom-right **Settings** dock → tab **Language** / **Sprache**.
2. Select **English** (`os.setLocale`, value `en`).
3. Verify Results labels: **Checks**, **Pass**, **complies**, **Explanation**, **Apply**.
4. Switch to **Deutsch** (`de`).
5. Verify: **Nachweise**, **Bestanden**, **erfüllt**, **Erläuterung**, **Anwenden**, panel **Dokument**, **Eingaben**, **Ergebnisse**.

Settings labels: `ui.settings.language.en` → **English**, `ui.settings.language.de` → **Deutsch** (`🛠️ShellHelpers/🟦️.tsx` L4949–4951; wgpu chrome L29775–29778).

### 4.7 Optional: `insertItem` / `removeItem`

1. In Inputs, expand **Thermal zones** / **Thermische Zonen** (`zones`).
2. **Add item** / **Eintrag hinzufügen** → `insertItem` on `zones`.
3. On a zone row, **Remove** / **Entfernen** → `removeItem`.
4. **Evaluate** after each change to refresh Results.

---

## 5. Gaps — what the current UI cannot do (or requires manual steps)

| Gap | Impact on script above | Evidence |
|-----|------------------------|----------|
| **No auto re-evaluate after edits** | Verifier must click **Evaluate** after every `setField`, `insertItem`, `removeItem`, or `applyRemedy` | Report cache keyed by document revision; miss → empty Results (`cached_report_for` + `from_artifact`, `⚖️compliance/🦀️.rs` L869–914) |
| **Only one Catalogue example in UI** | Cannot load `🎬️failing-thin-insulation` from Catalogue; must mutate `airtightnessN50` manually | `ArtifactEditor::examples()` returns only `demo-session` (`✏️editor/🦀️.rs` L62–64) |
| **No committed norm Playwright / Storybook E2E** | Script is manual; no CI browser gate for this path | Wave A audit §5; ticket coordination L39 |
| **Compliance gate not closed** | All 15 families verified in code tests, but ticket goal still open pending gate + E2E sign-off | `📓️coordination.md` L43–44 |
| **Evaluate is explicit** | Opening editor does not populate Results; first **Evaluate** is mandatory | `Evaluate` handler emits zero mutations; report comes from evaluate job cache (`🎮️commands/🧮️evaluate/🦀️.rs` L24–25) |
| **Deep links / routes** | No URL route to a specific check or field; single origin `/` only | Playground session `hostMode: false`, boot app id only (`🎮️playground-session/🟦️.ts` L19–24) |

None of the above block a careful manual verifier; they block ** unattended** one-click E2E until Evaluate is wired post-mutation or a Playwright spec is added.

---

## File index (quick)

| Topic | Path |
|-------|------|
| Launch | `.vscode/launch.json` L6295–6334 |
| App + verbs | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs` |
| Inputs editor | `…/🪟️windows/📥️inputs/🦀️.rs` |
| Results | `…/🪟️windows/📊️results/🦀️.rs` |
| Field labels | `…/🏷️field-meta/🦀️.rs` |
| Shared UI + verbs | `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` |
| CheckResult model | `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs` |
| n50 fail/remedy test | `…/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` L332–341, L419–421 |
| Locale settings | `🧰️framework/…/🛠️ShellHelpers/🟦️.tsx` L4949–4951 |
