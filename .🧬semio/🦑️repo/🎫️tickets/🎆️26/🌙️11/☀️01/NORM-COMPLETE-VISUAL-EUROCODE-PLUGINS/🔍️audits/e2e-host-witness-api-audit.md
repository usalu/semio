# EN1990 Browser Witness Extension Map

Read-only source review, 2026-11-04. No browser, server or test launched.

## Scenario Contract

The exact public scenario type is @semio-tech/framework-time-travel-ts/testing/scenario, authored at framework/modules/time-travel/testing/scenario/TypeScript. TimeTravelScenarioV1 requires variant, defaultServes {react,wgpu}, bootWindowPrefix, editorNumbers, paletteQuery {en,de}, stepNumbers, order, steps, context(), prepare(context), beforeStep(step,context,blocked), afterSteps(context,blocked), diagnostics(), exploreInventory(). Each step is async and receives the scenario context. prepare returns null or {name,prerequisite,exempt}. The harness owns browsers, locale iteration, reports and universal history journey.

Existing plugin exemplar: ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts exports timeTravelScenarioV1. Appropriate norm taxonomy: norm/🧪️tests/⏪️time-travel/🟦️.ts plus a sibling language-neutral fixture describing example/path/check/before/after/undo/redo/reload expectations. This source review does not create either.

Reusable public imports include testing/state (page, renderer, currentLocale, context), testing/page (evalSafe, waitUntil, sleep), testing/report (verdict,note), testing/history (documentEntries,allHistoryRows,openHistory,newestEntrySeq,pressRowAction), testing/history-session, testing/wgpu-mirror (mirror,mirrorLocator,mirrorActivate,mirrorFind), testing/chrome and testing/band. The universal journey discovers DOM control roles including spinbutton/input[type=number]/data-stepper-input; use actual UI input actions rather than executing mutations through internal hooks.

## Deterministic Engineering Witness

Example ID road-bridge-compliant contains RailBridge. bridgeSls[id=sls-D1].deckAcceleration = 0.2, deckAccelerationLimit = 0.5; the requested edit is 0.4. Check en1990.a2.acceleration.deck-D1.sls-D1 therefore reports utilization 0.4 -> 0.8, then undo 0.4, redo 0.8 and reload 0.8. This deliberately remains below the limit so check status is not the primary oracle.

The source a2_subject(sls_id,member,field) sets subject.entity_id to sls_id and path bridgeSls[id=sls_id].field; member_label is only label context. Thus the check ID contains deck-D1 but the selection owner is sls-D1. Picking member deck-D1 can filter away the target check and open the wrong inspector entity. Pick the Bridge SLS outliner row sls-D1, or target the exact field row independently of member focus. No owner mismatch exists when the actual subject identity is used.

## Source IR Addressing

Properties constructs row ID norm-inputs.<field.path>, so the target row is norm-inputs.bridgeSls[id=sls-D1].deckAcceleration. Numeric controls use child IDs .stepper or .input depending the actual FieldControl. field-meta declares label Deck acceleration / Deckbeschleunigung and MetrePerSecondSquared; account for unit conversion via the UI display factor, although this acceleration field has SI scale. Prefer locating the exact row by its ID suffix, then its accessible numeric control; shell node IDs may carry body prefixes, so do not assume raw document.getElementById equals the local key.

Outliner entity selection IDs are scoped entity addresses when interaction domain is configured; fallback keys are norm-outliner.<row.path>. Selecting by exact row ID/path or accessible label must identify sls-D1, not just the member label shared with other nodes.

Checks selection-only rows use check_address(check.id), an encoded scoped check identity; older command-style rows use norm-checks-check-<index>. Never pin an array index as the engineering check identity. Row accessible/visible label includes localized status, utilization percentage and title. The 40 % -> 80 % value is rendered by utilisation_percent_text. Detail child IDs <row>.computed/.limit/.utilization exist only for Fail/Warning expanded rows; a passing witness must read the main row percentage or actual accessible utilization surface rather than assuming .utilization exists. Check accessible descriptions include subject label, clause and computed/limit quantities.

Exact source owners: shared app-surface visual-windows/properties (row IDs), properties/controls (numeric child IDs), checks (row addressing and detail gates), checks/utilisation-bar (percentage text), outliner (entity addresses), and EN1990 field-meta plus inferences/working-life-and-bridge-annexes.

## Harness Invocation

Use registered framework-os-dev:time-travel with --scenario repository-relative-norm-module --serve the quoted6094 URL including plugin=en1990,role=editor,example=road-bridge-compliant, --renderer react --locales en,de --chords en,de --out ticketgenerated/host-e2e. The scenario defaultServes may also contain full route URLs. Supply explicit serve to avoid accidental variant/default example drift. Keep locale unlocked on the server when testing both navigator languages.

A true persistence witness must attach to a fresh test folder through the existing sync-card/history helpers before editing and reopen that same backing; a page reload over an unpersisted example alone is insufficient. Reuse puzzle prepare/boot and history helper patterns, but do not assume puzzle-specific board data attributes exist in norm. Assert before/after engineering quantities and document field together so a stale report or failed edit cannot masquerade as passing history behavior.

## Exact Row and Folder APIs

Native row_address is authored at app-surface/entity-editing/interaction-topology/rows: it takes collection from row.path before '[' and encodes [collection,row.id]. For bridgeSls/sls-D1 the canonical address is scope:1:9:bridgeSls6:sls-D1. The TypeScript encoder exists privately at replication/wire/scoped-identity, but the inspected public replication package has no encoder export. A committed expected canonical address can drive the fixture. Scope React locator to outliner then use an attribute suffix selector, avoiding CSS colon escaping and duplicate inspector/check rows.

Public testing/history-session exports attachFolder(path:string,afterPress?:()=>Promise<unknown>). Returns card:boolean, typed:string|null, attachButton:boolean and optional after {cardText,chip,alerts}. afterPress executes immediately after Attach and skips settling/Escape; default React path waits 1.5 seconds, captures state/screenshot and presses Escape. No readFolder helper is defined/exported in the shared time-travel tree. The puzzle reload witness uses filesystem readdirSync to await archive files after an outbound edit.

Reload proof pattern in puzzle/🧪️tests/⏪️time-travel/🔂️reload-check/🟦️.ts: register expected reload, page.reload, waitForBoot, inspect offered reconnect band, click #s-folder-reconnect, await disappearance and reprojectionSettled; if absent use attachFolder(samePath). Then compare actual document values and history edit IDs. A bare playground reload reruns the example and does not establish persistence.
