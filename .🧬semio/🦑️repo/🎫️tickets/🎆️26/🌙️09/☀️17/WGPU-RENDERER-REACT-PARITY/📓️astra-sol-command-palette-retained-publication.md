# Command Palette Retained Publication

## Checkpoint 17 evidence

Checkpoint 17 isolates two independent failures.

Step 44 successfully resolves and activates the exact German Set Theme palette row. At console time `391998`, `activate_search_item` receives query `Thema festlegen`, offers `command.os.os.setThemeId`, and chooses `command-form:os:os.setThemeId`. The resulting WGPU surface is `command.category.appearance`, but it still contains the unexpanded `shell.commands.os.setThemeId` row and lacks the retained form, Execute, and Reset controls. React publishes these exact successor ids in one staged form:

- `command.category.appearance.form`
- `command-os.os.setThemeId-execute`
- `command-os.os.setThemeId-reset`

The cause is retained publication. `sync_dock_tabs` already mounted the Appearance document. The command-form branch changed `expanded_command_id` and revealed that existing document without replacing its owner, so the retained tree still represented the earlier unexpanded state.

Step 45 is separate. The initial `Meta+p` reaches WGPU at `416931`; the accessible input receives every value through `Thema festlegen`; Escape is dispatched at `419434`. The reopen chord at `419792` contains only Meta down/up and no `p`. Retiring the focused projected input leaves DOM focus on `body`; the root-scoped keyboard owner therefore cannot admit the next non-modifier key. The palette query reducer did not erase the stored query and the Set Theme search producer did not return an empty result.

## Neutral contract and production repair

The shared ShellSearch fixture now carries the canonical category surface, expanded key, form, Execute, Reset, and stale unexpanded-row ids. Its schema requires every field. The existing native staged-command law consumes those neutral ids.

The WGPU command-form activation now sets the proposed expansion, transactionally republishes the mounted category through `republish_shell_panel_document`, and only then reveals the category tab. Retirement admission is reserved before successor publication by that existing helper. Refusal restores the exact previous expansion, leaves the exact prior retained owner readable, preserves the palette/query for retry, and reveals no stale successor. A successful retry replaces generation and revision together, contains the form/Execute/Reset controls in that same owner, excludes the stale unexpanded row, and terminally retires the prior lease. No capacity or guest-refresh path changes.

The browser keyboard fixture separately specifies that retirement of the focused `ui.search.input` returns focus to the canvas before the next `Meta+p`. Accessibility mirror publication owns that handoff because it knows both the previously focused retained key and whether the successor projection contains it. It focuses the supplied canvas fallback only when a previously active renderer-owned projection disappears. Existing accessible focus restoration remains unchanged when the exact keyed successor exists, and ordinary external focus is not captured.

The browser boot supplies its real canvas as the mirror fallback. No document-global keyboard listener or compatibility path was added.

## Tests

The new retirement law first failed against production with 36 passing keyboard-scope cases and one failure: the removed focused input left `document.body` active instead of the canvas. After moving the handoff to accessibility mirror publication, the exact focused browser gate passed:

```text
Test Files  1 passed (1)
Tests       37 passed (37)
```

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧪️tests/⌨️browser-keyboard-scope/🟦️.ts' --silent=false --reporter=verbose
```

The accessibility interaction regression also passed:

```text
Test Files  1 passed (1)
Tests       10 passed (10)
```

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx' --silent=false --reporter=verbose
```

All four edited JSON fixture/schema files parse with `jq`; both edited Rust sources parse with Rustfmt under edition 2024; `git diff --check` passes.

No Cargo command was run in this lane. The root-owned native gate must run the exact filter `picking_an_arg_carrying_command_opens_its_form`. That law now includes refused-retirement preservation and the successful exact retry, so one filter covers the owner/refusal/successor contract.

## Native68 fixture correction

Native68 reached the registered law for the first time and failed before activation at `mounted Command category document`. The production lifecycle was coherent: `refresh_ui` publishes every `shell_owned_panel_leaves` entry with `publish_shell_panel_document` and inserts the returned retained lease. The test called only `sync_dock_tabs`, which reconciles the dock roster and paths but deliberately does not publish panel bodies. It therefore asserted a mounted lease that its own setup never created.

The law now mounts the initial category through the actual production publisher, reads the initial header from that returned lease, and inserts that exact owner into `panel_documents` before forcing retirement refusal. The refusal, retry, generation/revision replacement, staged form/Execute/Reset, stale-row absence, and terminal retirement assertions remain intact. The corrected source parses with Rustfmt and passes `git diff --check`; execution remains on the root-owned filter above.

## Shared-index note

The first attempted browser repair placed the handoff in `input-wire` focusout. It did not observe DOM removal reliably and was restored in the working tree to its original behavior. A concurrent staged snapshot still contains that rejected attempt; the final staged boundary must take the current working-tree `input-wire` content, which is byte-identical to `HEAD`. The accepted repair is solely in accessibility mirror publication plus its browser-boot fallback.
