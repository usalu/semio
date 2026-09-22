# Chat Input, Driver Locale, and Settings Tab Scope

## Source findings

This packet is source review and test design only. It does not claim a native or browser result.

The retained chat composer and the React composer already agree on behavior: both use a long-text input, submit a bare Enter through the same send action, and disable editing while the agent bridge is not open. The retained input lacks the accessible name React supplies. `UiInputNode` carries `placeholder` but no explicit accessibility label, and `PanelProjection` consequently creates its `PanelRecord` with no label. React deliberately uses two different strings: the accessible name is `Message to the agent` / `Nachricht an den Agent`, while the placeholder is `Tell the agent what to do…` / `Sag dem Agent, was zu tun ist…`. The placeholder must not become a generic name fallback.

The driver editor has one confirmed locale gap. React resolves built-in IDs through `settings.driver.default` and `settings.driver.compact`, which are `Default` / `Compact` in English and `Standard` / `Kompakt` in German. WGPU's `driver_rows` embeds the English strings. Custom driver labels are document data and must remain byte-for-byte authored across a locale change.

The reported six-to-three Settings-tab difference has a separate concrete cause. The WGPU tab painter stops at the first chip beyond panel width before drawing or registering every remaining child. The mobile painter and drag-preview walk use the same early break. React retains every declared tab in a horizontally scrollable row and reveals the active tab. WGPU accessibility and activation currently derive solely from visible pointer hits, so removing the break alone would still lose declared tail semantics. The durable boundary is a bounded scroll owner plus a semantic row catalog: all six declared Settings children remain in presented accessibility authority, while pointer hits contain only nonempty clipped rectangles. Accessibility Focus/Activate first reveals a declared tail tab, then publishes the next accepted frame.

## Schema-first packet

### Explicit input name

Add `accessibilityLabel?: Label` to `UiInputNode` in the shared WGPU UI schema. The field is optional because a surface may use a separately labelled field relationship, but the Chat producer must always provide it. `PanelProjection` maps only this field into `PanelRecord.label`; `placeholder` remains presentation help.

The neutral fixture has two locales and two bridge states:

| Locale | Bridge | Accessible name | Placeholder | Enabled | Enter action |
| --- | --- | --- | --- | --- | --- |
| en | open | Message to the agent | Tell the agent what to do… | yes | `sendChatDraft` |
| en | closed | Message to the agent | Tell the agent what to do… | no | none admitted |
| de | open | Nachricht an den Agent | Sag dem Agent, was zu tun ist… | yes | `sendChatDraft` |
| de | closed | Nachricht an den Agent | Sag dem Agent, was zu tun ist… | no | none admitted |

The native law must publish the real Chat document through retained ingress, paint and ACK it, inspect the presented accessibility node, then exercise Enter through the retained router. The mounted React oracle must inspect the textarea by role and accessible name for the same four rows. A schema round-trip law must reject unknown members and preserve an absent label as absent; no placeholder-to-label compatibility path is permitted.

### Driver labels across retained locale refresh

Extend the existing driver-editor fixture with display-label rows rather than changing driver identity data:

| ID | English | German | Ownership |
| --- | --- | --- | --- |
| `default` | Default | Standard | localized built-in |
| `compact` | Compact | Kompakt | localized built-in |
| `custom.focus-flow` | Focus Flow | Focus Flow | authored custom data |

The native law dispatches the real locale mutation, drains the one-owner-at-a-time mounted refresh cursor, paints and ACKs General, then reads the presented Select options. The React law changes `uiI18n` through the same `en` to `de` vector and inspects the mounted chooser. Both laws must prove IDs remain unchanged and the custom label is not translated.

The WGPU resolver should accept `(driver_id, authored_label, locale)` and localize only the two closed built-in IDs. It must not infer built-in status from label text and must not translate arbitrary custom labels.

### Settings child-row overflow

Add a neutral row fixture with the exact ordered IDs `app`, `general`, `theme`, `keybindings`, `default-apps`, `conflicts`, a constrained BottomRight width, and a bounded horizontal scroll delta. Native laws must establish:

1. the declaration and presented accessibility catalogs retain all six IDs in order;
2. initial pointer hits are only the visible clipped prefix;
3. horizontal scroll changes only the row offset and the next accepted frame exposes tail pointer hits;
4. accessibility activation of `default-apps` or `conflicts` reveals the target before dispatch and publishes the correct retained body;
5. locale refresh changes the six labels without resetting the active-tab reveal;
6. drag insertion uses translated/clipped geometry and never drops a declared tail member.

The bounded state key is `(anchor, active_branch_path, row_index)`. The offset is clamped to authored row extent minus viewport extent. Closing or replacing the branch retires its exact offset; ordinary refresh preserves it.

## Exact source seams

- `UiInputNode`: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`
- input `PanelRecord`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- Chat producer: the same Shell target's `build_agent_chat_ui`
- React Chat oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx`
- WGPU driver resolver: `driver_rows` in the Shell target
- React driver resolver and translations: `🛠️ShellHelpers/🟦️.tsx` and `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`
- retained locale fixture: `🧑‍🎨engine/🧬️schema/🌐️settings-locale-panel-refresh/🔣️.json` and its fixture/test twins
- driver lifecycle fixture: `🧑‍🎨engine/🧬️schema/🚗️driver-editor/🔣️.json` and its fixture/test twins
- WGPU tab row: `paint_anchor_tab_bar`, `dock_tab_insert_preview`, and `paint_mobile_tab_bar` in the Shell target
- native accepted-frame harness: `🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs`

## Sequencing

1. Register the input-name schema and neutral/mounted laws; capture the missing-name RED; implement the typed field and Chat value.
2. Extend the driver locale fixture and capture the English-on-German WGPU RED; implement the closed built-in resolver.
3. Register the six-tab constrained-row laws; capture the missing-tail RED; implement semantic catalog plus bounded scroll/reveal.
4. Run the focused UI/React Nx gates, the UI engine gate, the renderer native filters through the root-owned lane, then a fresh paired browser Settings/Chat receipt.

## Test-first implementation status — 2026-09-21

The language-neutral Chat fixture and schema now cover the closed `en/de × open/closed` product matrix. The mounted production `AgentChatPanel` law resolves the textarea by its localized accessible name, proves the placeholder is distinct, proves closed bridge disablement, and submits one trimmed bare-Enter turn only for an open bridge. The scoped React target is green: 7/7 tests in `🗑️generated/astra-runtime/chat-driver-locale/react-chat.log`.

The driver fixture now owns three display-label rows: the `default` and `compact` IDs are localized built-ins, while `custom.focus-flow` is authored data. An actual React resolver oracle changes `uiI18n` through English and German and is green with the rest of the driver contract: 4/4 tests in `🗑️generated/astra-runtime/chat-driver-locale/driver-ts.log`. The native law `driver_rows_localize_only_closed_builtins_and_preserve_authored_custom_labels` is registered for the root-owned Cargo lane.

The native Chat law `the_chat_composer_projects_its_explicit_localized_accessible_name_without_using_the_placeholder` projects the real shell-authored Chat body to retained records and requires an explicit label independent of placeholder and bridge state. It is registered fail-first; production still has no typed input label field at this checkpoint.

The corrected locale fixture now says a locale mutation requires guest refresh. The scoped mounted React test is green 2/2 in `🗑️generated/astra-runtime/chat-driver-locale/react-locale.log`. Native law `locale_and_terminology_changes_require_one_full_guest_refresh_and_settle` requires the existing `UiDirtyScope::Full` plus settle lane for both axes and refuses the duplicate shell-only cursor. Production is held pending its actual native RED.

The shared semantic UI additions have an independent strict JSON boundary and generated-TypeScript consumer law at `🧪️tests/🌳️ui-contract-presentation/🟦️.ts`: omitted fields resolve to `standard`/`button`, explicit values remain `compact`/`checkbox`, and unknown or non-closed values are refused. The scoped Bun/Nx receipt is green 2/2 in `🗑️generated/astra-runtime/chat-driver-locale/ui-contract-presentation.log`.

## Native144 RED and production repair — 2026-09-21

Native144 executed the five new laws against production and recorded the intended causes: the retained Chat Input projected no semantic label, German driver rows still showed the English closed builtin label, and locale/terminology mutations did not owe a settled Full guest refresh. The exact root-owned receipt is `🗑️generated/astra-runtime/renderer-native144-compact-ax-locale-red/run.log`.

Production now carries an optional typed `UiInputNode.accessibilityLabel`, projects only that explicit label into `PanelRecord`, and gives the Chat draft locale-owned names independent of its placeholder. The UI wire golden explicitly serializes `accessibilityLabel: "Message"`; every Rust initializer declares the optional field. Driver display resolution localizes only the closed `default` and `compact` IDs, preserving authored custom labels. `setLocale` and `setTerminology` now both sync dock labels, owe `UiDirtyScope::Full`, and owe settle; the obsolete shell-only localized-panel cursor and its duplicate publication lane were removed.

The corrected language-neutral locale fixtures now describe `refreshScope=full` and `settleRequired=true`. A scoped React run recorded both mounted locale laws green (2/2) and the General publication-lane law green. The combined log is `🗑️generated/astra-runtime/chat-driver-locale/locale-final.log`; its sole unrelated failure is the concurrently changed Compact Tree grid-template assertion, whose old expanded fallback no longer matches the new scoped CSS variable.
