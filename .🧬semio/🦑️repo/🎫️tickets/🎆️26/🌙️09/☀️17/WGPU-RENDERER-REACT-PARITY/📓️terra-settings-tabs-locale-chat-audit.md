# Settings Tabs, German Copy, and Chat Composer Audit

## Scope and evidence

This is a read-only source audit of the current React and WGPU producers. I did not build, run native tests, or drive a browser. The reported React-six/WGPU-three observation therefore remains a runtime symptom requiring a fresh accepted-frame receipt; the conclusions below distinguish it from source-proven product gaps.

## Settings: current sources already describe the six-leaf product

The full React host always supplies both optional Settings providers to `createFrameworkSettingsPanelTab`: Default Apps and Conflicts are passed at [ShellHost:9212-9216](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:9212). The React descriptor therefore contains five framework leaves whenever the theme is unlocked: General, Theme, Keybindings, Default Apps, and Conflicts. Its generic factory only omits the latter two when an embedding caller does not provide those callbacks, as shown at [ChromePanels:1080-1160](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx:1080).

React then prepends any app-declared Settings leaf into that branch at [ShellHelpers:1526-1534](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1526). Thus a six-leaf observed React branch means one app Settings leaf plus the five unlocked framework leaves.

WGPU has the equivalent structure in current source:

- [Shell WGPU:8735-8779](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8735) routes the active app's `panel_tabs`, takes the app Settings children, and prepends them to the framework branch.
- [Shell WGPU:8800-8813](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8800) declares all five framework leaves, omitting only Theme under the same theme-lock condition.
- [Shell WGPU:20337-20358](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20337) has builders for all five framework IDs.
- A persisted dock skeleton cannot simply remove undeclared Settings children: its branch merge appends every default child absent from the skeleton at [Shell WGPU:4094-4110](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4094).

Consequently, with the same active app descriptor and an unlocked theme, static WGPU source predicts the same six leaves. A WGPU count of three is not explained by a missing Default Apps or Conflicts implementation. It points to a post-descriptor boundary: active-session/dock derivation, accepted chrome publication, clipping/layout, or an earlier presented frame. The older checkpoint-18 record already demonstrated that retained Settings geometry can advance while an older World-only packet remains presented ([checkpoint report](./📓️astra-sol-checkpoint18-settings-presentation.md)); that is historical evidence, not a claim about the current run.

### Required fail-first product law

Use the real active app descriptor and two provider capability states, then compare React's rendered tab IDs and WGPU's **presented** tab IDs only after successful input/chrome ACK:

| State | Expected ordered branch |
| --- | --- |
| Theme unlocked, app Settings leaf, both providers wired | app leaf, General, Theme, Keybindings, Default Apps, Conflicts |
| Theme locked, app Settings leaf, both providers wired | app leaf, General, Keybindings, Default Apps, Conflicts |

The native route must obtain IDs from the presented retained/chrome catalog, not the candidate dock. For each ID it must activate the actual tab and require the matching panel document to become visible after the next accepted frame. The React oracle should inspect the same host state. This makes a clipped/obsolete three-row visual result fail without weakening descriptor or publication authority.

There is one structural policy mismatch worth resolving as part of this law: React's reusable factory expresses Default Apps and Conflicts as supplied capabilities, while WGPU unconditionally installs those leaves. The product host currently supplies both callbacks, so it does not explain the observed count. A shared shell-capabilities descriptor should nevertheless be the source of those two presence decisions in both implementations; that prevents embedded hosts from diverging later.

## German copy: the general refresh path is present; two concrete labels are not localized

Locale mutation updates the active locale, rebuilds dock labels, and schedules mounted shell leaves for bounded refresh at [Shell WGPU:10107-10120](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10107). The current `shell_chrome_string` table includes German translations for the Settings categories, Default Apps, Conflicts, and most General/Driver controls. A blanket diagnosis that all German Settings labels are static English is not supported.

Two target-specific gaps are directly source-proven:

1. WGPU's built-in driver chooser always emits literal `"Default"` and `"Compact"` at [Shell WGPU:7975-7984](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7975). React instead resolves those IDs through `shellLabel("settings.driver.default")` and `shellLabel("settings.driver.compact")`, while preserving a custom driver's user-authored label, at [ShellHelpers:4590-4596](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:4590). WGPU should use the same split: localized labels for its two built-ins and stored data only for custom drivers.

2. The WGPU Keybindings body derives each action name by English-oriented `humanize_control_id` at [Shell WGPU:7995-8005](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7995). Its conflict/capture/reset suffixes are localized, but the control-name component is not. This is a source-level i18n debt. It is not yet proven WGPU-only because React's control-label resolver must be checked with the same installed German locale before a parity-specific implementation is selected.

The smallest durable repair for the confirmed driver gap is a shared built-in-driver label resolver keyed by driver ID and locale, with custom labels left as document data. Do not translate custom labels or replace them with IDs.

The acceptance law should dispatch real `setLocale("de")`, wait for the bounded retained refresh and presented ACK, then require German Settings tab labels plus localized built-in driver options. It should run the same UI state through React. It must separately prove that a custom driver label remains exactly authored.

## Chat composer: disabled when disconnected is correct; missing accessible name is not

Both implementations use bridge openness as the editing authority:

- React derives `connected` from `status === "open"`, gives the textarea `aria-label={draftLabel}`, and disables it while disconnected at [AgentChatPanel:196-246](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx:196).
- WGPU derives the same state from `AgentBridgeStatus::Open`, creates a `longText` input with the same Enter-submit action, and disables it when closed at [Shell WGPU:8452-8484](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8452).

The disabled composer is therefore not a parity defect when the bridge is closed. An open bridge with a disabled composer would be a separate runtime fault, and needs its bridge-status receipt.

The accessible-name gap is direct and current. `UiInputNode` has a placeholder but no explicit label field at [component schema:2025-2072](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2025). Shell's input projection carries that placeholder into `InputProps` but produces a `PanelRecord` without `label` at [Shell WGPU:5007-5039](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5007). The shared accessibility projection only uses the explicit record label (apart from the already-supported TreeItem fallback) at [accessibility contract:229-244](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:229), and the browser mirror emits `aria-label` only when that projected label exists at [accessibility mirror:79-84](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:79).

Add an explicit optional accessibility label to the input schema, map it to `PanelRecord.label`, and supply the localized WGPU twin of React's `os.agent.chat.draftLabel` in `build_agent_chat_ui`. The current WGPU catalog has `chat.placeholder` and `chat.send` but no distinct draft-label entry at [Shell WGPU:27773-27778](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:27773). A placeholder must not become the generic accessible-name fallback: React deliberately supplies separate values, and many inputs use placeholders as examples rather than names.

### Chat acceptance law

Publish the real Chat panel through the normal retained ingress and inspect the presented accessibility tree:

- with an open bridge, `framework.chat.draft` is an enabled editable textbox with the localized draft label and long-text/Enter-submit semantics;
- with a closed bridge, the same textbox retains that accessible name but is disabled and no send action is admitted;
- React's mounted textarea must expose the same accessible name and enabled state for both bridge statuses.

## Priority

1. Add the explicit Chat input accessibility label and its two-state retained/React oracle. This is a direct accessibility failure with a narrow typed repair.
2. Add the Settings full-branch presented-ID law before changing Settings feature code. Current source has all five framework leaves and builders; the reported three is a presentation/activation issue until an accepted-frame receipt identifies the missing boundary.
3. Localize WGPU built-in driver IDs through the same shared resolver React uses, then capture the keybinding-row label behavior in the German paired oracle before changing its shared label authority.

