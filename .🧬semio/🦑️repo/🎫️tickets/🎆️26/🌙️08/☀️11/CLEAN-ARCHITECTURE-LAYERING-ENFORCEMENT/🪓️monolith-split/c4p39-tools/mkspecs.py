"""📐 C4P39 spec generator: turns the per-file chunk start tables below into `specs/<n>.json` for fragsplit.py.
Each start is an item line (or region line) of the ORIGINAL snapshot; it is backed up over the contiguous doc/attribute/comment lines above it (never over a `//#endregion`) and must then follow a blank line or the header.
A chunk runs to the line before the next start; the root keeps every line before the first start. usage: mkspecs.py"""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.dirname(HERE)
STAGE = "/Users/ueli/.cache/c4p39/stage"
TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests"
FILES = {
    1: "🎨️wgpu-theme-editor-and-accessibility",
    2: "🔗️hub-projection-workspace",
    3: "🔬️wgpu-panel-anchor-model",
    4: "🔬️wgpu-shell-chrome-parity",
    5: "🔬️wgpu-shell-input",
    6: "🧪️wgpu-time-travel",
}
T = {
    1: [
        (14, "🧰️theme-editor-fixtures", "🧰️ Theme editor fixtures: the accessibility and publication fixtures, the nested settings shell, the control-id walk and the retained control state the theme laws read."),
        (105, "🎚️theme-publication-and-sections", "🎚️ Theme publication and sections: retained republish without guest refresh, invalid input, the canonical document, and the colors, spacing, fonts, number, metrics and appearance sections."),
        (333, "🔁️theme-tokens-and-leaves", "🔁️ Theme tokens and leaves: re-tokenised painting, the publication ceiling, the reset predicate, the General and Theme leaves and the rows' authored coordinates."),
        (482, "♿️accessible-names-and-activation", "♿️ Accessible names and activation: chrome control names and shortcuts, pressed buttons on actual hit ids, focusable field labels and the accessibility and pointer activation routes."),
        (695, "🪞️mirror-address-liveness", "🪞️ Mirror address liveness: the settings toggle shell and the accessibility address staying live, stale or delayed across frames."),
        (768, "🧭️settings-strip-and-keybindings", "🧭️ Settings strip and keybindings: the constrained tab strip, mobile panel buttons, inline chords, the flagged keybinding rows and accepted dock names."),
        (1009, "📐️passive-regions-and-geometry", "📐️ Passive regions and geometry: passive chrome regions, compact theme spacing, named metrics and their localized authoring guidance."),
    ],
    2: [
        (5, "🔗️projection-fixtures-and-live-workspace", "🔗️ Projection fixtures and live workspace: the hub projection fixture records, the route overlay and summary laws, the resolution band and the live sign-in to the retained workspace."),
        (215, "🪢️severable-relay-and-ledger", "🪢️ Severable relay and ledger: the loopback relay whose connections can be cut, the collaboration ledger and the native drive and sign-in helpers."),
        (342, "📣️replay-refusals-and-agent-offer", "📣️ Replay refusals and agent offer: the shared replay-refusal vocabulary, the unserved guest refusal, the space artifact creation replay and the agent bridge offer scope."),
        (460, "⏱️frame-pumps-and-edit-ledger", "⏱️ Frame pumps and edit ledger: remote and liveness probes, the frame pump, document opening settle, paired pumping and the applied-edit and membership ledgers."),
        (589, "🤝️two-shell-collaboration", "🤝️ Two-shell collaboration: two live native shells collaborating on one hub document."),
        (746, "🧭️native-guest-journey", "🧭️ Native guest journey: the shared guest journey fixture and the hubless mount, undo, redo, copy and cancellable open laws."),
        (944, "🏗️creation-door-and-cross-shell-meeting", "🏗️ Creation door and cross-shell meeting: the live artifact creation door, the cross-shell file handshake, the window painter and the cross-shell meeting driver."),
        (1159, "🖱️cross-shell-collaboration-and-sign-in-task", "🖱️ Cross-shell collaboration and sign-in task: native and React collaboration, shared cursors, the long guest turn and the cancellable sign-in verb."),
    ],
    3: [
        (8, "🪟️spawned-documents-and-host-panel", "🪟️ Spawned documents and host panel: the fresh shell, spawned document routing, the host panel codec, claim order and stale session identity."),
        (166, "📌️anchor-ids-resize-and-default-dock", "📌️ Anchor ids, resize and default dock: anchor group and id mapping, panel width and resize hits, and the shared default dock fixture."),
        (326, "🧱️anchor-geometry-and-state", "🧱️ Anchor geometry and state: anchor insets, banding and chrome-hosted style, the open-anchor snapshot, census, column projections and path reconciliation."),
        (527, "🗄️dock-override-round-trip", "🗄️ Dock override round trip: dock skeleton application, tab moves, the shell config round trip, persistence idempotence and the theme panel gating."),
        (688, "⚙️settings-keybindings-hug-and-mobile", "⚙️ Settings, keybindings, hug and mobile: the Settings branch and shell-owned leaves, remappable keybindings, content hug and the mobile flattening."),
        (895, "💬️command-form-and-agent-chat", "💬️ Command form and agent chat: the staged command form and the agent chat transcript, cancel and composer laws."),
        (1123, "🌳️tree-units-and-chrome-bands", "🌳️ Tree units and chrome bands: the persisted tree units, the Display branch leaves and the navbar and footer band fixtures."),
    ],
    4: [
        (16, "📚️fixtures-and-parity-builders", "📚️ Fixtures and parity builders: the shared picker, switch, controls, safe-area, boot and icon fixtures and the parity app, mode and window-kind builders."),
        (130, "🎛️example-picker-and-role-groups", "🎛️ Example picker and role groups: the dialect-keyed example picker, boot example resolution, the roles group, role switching and mode stepping."),
        (327, "🛑️chords-controls-and-compute-status", "🛑️ Chords, controls and compute status: shared keybinding rows, the alt axis, the surface overlay controls and the compute status pill and pane."),
        (591, "🗣️chrome-i18n-and-switch-order", "🗣️ Chrome i18n and switch order: bilingual chrome literal laws, chrome key coverage and the create-before-retire session switch."),
        (753, "🪟️window-scope-laws", "🪟️ Window scope laws: the window scope fixture, seeded active windows, instance icon overrides and app-wide chord routing to the owning window."),
        (1014, "🛟️safe-area-and-incoming-layout", "🛟️ Safe area and incoming layout: the chrome panel safe area, the overlay row beside floating panels and incoming layout identity, maximize and pruning."),
    ],
    5: [
        (4, "🏷️scene-labels-dock-tabs-and-select-wheel", "🏷️ Scene labels, dock tabs and select wheel: locale chrome labels, one tab stop per dock stack and the immediate select owning the wheel."),
        (139, "🌍️retained-world-capture", "🌍️ Retained world capture: the topmost retained world owning press and release, capture preservation, rebasing and fixed capture grants."),
        (421, "⌨️retained-pane-focus-and-keyboard", "⌨️ Retained pane focus and keyboard: concrete window pane actions, keyboard focus routing and successors, pane owner activation and the select and typeahead key mapper."),
        (715, "🪟️window-publication-journals", "🪟️ Window publication journals: published tree and canvas documents, scripted window body fixtures, transfer journal refusals and the standalone multi-app variants."),
        (955, "🧭️dock-order-key-events-and-drag", "🧭️ Dock order, key events and drag: the silhouette cap gap, dock window order, key action events, the content focus tracker, dock drags and body presses."),
        (1252, "🖥️display-and-table-transfer", "🖥️ Display and table transfer: published tree handle routing, the display transfer host fixture and laws and table surface transfers through the shell host."),
        (1744, "🎯️canvas-pointer-wheel-and-catalogue-drag", "🎯️ Canvas pointer, wheel and catalogue drag: published canvas capture, the renderer canvas pointer sequence, secondary drag, retargeting, wheel bursts, catalogue drops and double click."),
        (2067, "🔎️quick-search-palette-keyboard", "🔎️ Quick search palette keyboard: the palette chord, overlay fields not counting as typing and escape closing the palette."),
        (2137, "📷️canvas-camera-ownership", "📷️ Canvas camera ownership: closed and reopened windows, sibling components, sequential close, mounted identity, retired deadlines and remount retirement of the pending camera."),
    ],
    6: [
        (23, "🧰️harness-and-recorders", "🧰️ Harness and recorders: the corpus and session builders, verb and refusal names, chords, the session shell, the band painter, the action recorder and the retained body painter."),
        (175, "⏪️band-law-and-chords", "⏪️ Band law and chords: the shared band corpus, the polite announcement, control captions and reserved actions, and the remappable time-travel chords."),
        (375, "📐️band-layout-and-focus", "📐️ Band layout and focus: the band above the footer, the subfooter reserve, control registration, shared transitions, the editor target and keyboard focus."),
        (661, "🪟️reveal-indicator-and-refusals", "🪟️ Reveal, indicator and refusals: History tab reveal, phone panel, reply status, operation publications, localized history refusals and lane refusals."),
        (946, "👥️notices-peer-edits-and-finalize", "👥️ Notices, peer edits and finalize: transient notice projection, the shared peer edit corpus, the roster badge and the keyboard-only finalize prompt."),
        (1157, "🎛️staged-editors", "🎛️ Staged editors: every editor kind in the Actions form and in a chrome dialog, axis splicing, number facets and keyboard and pointer staging."),
        (1460, "📚️history-body", "📚️ History body: the guest's real producer, paged transaction rows, refused edits, list and chip editors, replay progress and the single scroll region."),
        (1819, "📡️reprojection-blur-import-and-context-menu", "📡️ Reprojection, blur, import and context menu: reprojection announcements, host window blur, cancellable import tasks and disabled context menu reasons."),
    ],
}


def back_up(ls, s):
    k = s
    while k > 1:
        prev = ls[k - 2]
        if prev.strip() and not prev.startswith("//#endregion") and (prev.startswith("///") or prev.startswith("#[") or prev.startswith("//")):
            k -= 1
        else:
            break
    return k


os.makedirs(HERE + "/specs", exist_ok=True)
for n, rows in T.items():
    orig = f"{HERE}/orig/{n}.rs"
    ls = open(orig, encoding="utf-8").read().split("\n")
    if ls[-1] == "":
        ls.pop()
    starts = []
    for item, folder, doc in rows:
        s = back_up(ls, item)
        if s > 1:
            assert not ls[s - 2].strip() or True, (n, s)
        starts.append(s)
    assert starts == sorted(starts) and len(set(starts)) == len(starts), (n, starts)
    chunks = []
    for i, (item, folder, doc) in enumerate(rows):
        a = starts[i]
        b = (starts[i + 1] - 1) if i + 1 < len(rows) else len(ls)
        while not ls[b - 1].strip():
            b -= 1
        chunks.append(dict(name=folder, folder=folder, doc=doc, ranges=[[a, b]]))
        print(n, folder, a, b, b - a + 1, "prev:", repr(ls[a - 2][:50]))
    spec = dict(file=f"{TESTS}/{FILES[n]}/🦀️.rs", original=orig, stage=STAGE, chunks=chunks)
    json.dump(spec, open(f"{HERE}/specs/{n}.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
