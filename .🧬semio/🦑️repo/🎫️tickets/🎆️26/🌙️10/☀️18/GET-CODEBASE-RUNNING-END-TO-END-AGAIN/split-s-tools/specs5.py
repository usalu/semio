"""📐 C6UI chunk tables for the flat test roots: tag -> rows (first item line of the ORIGINAL snapshot as printed by ol.py, folder emoji, kebab, doc) and optional end_line (first item line that stays in the root)."""
FRAG = {}

FRAG["vtu"] = dict(rows=[
    (11, "🧫", "vt-shared-fixtures-and-corpus-laws", "Vt shared fixtures and corpus laws: the size, key, row, history and byte helpers, the shared corpus loaders and the pyte screen corpus law."),
    (194, "⌨", "vt-key-mouse-paste-and-focus-encoding-laws", "Vt key, mouse, paste and focus encoding laws: the key and keypad encoders, the bracketed paste, the focus reports and the mouse encodings with their gating."),
    (395, "📟", "vt-screen-sequence-laws", "Vt screen sequence laws: the child mode tracking, the query answers, the osc handling and the cursor, erase, scroll, sgr, utf8, alternate screen and reset sequences."),
    (606, "📜", "vt-history-reflow-and-selection-text-laws", "Vt history, reflow and selection text laws: the history eviction, the viewport anchoring, the reflow, the selection text and the search."),
    (769, "🖥", "vt-pane-input-search-and-browse-laws", "Vt pane input, search and browse laws: the pane helpers, the key passthrough, the viewport snap, the search, the browse mode, the paste and the focus events."),
    (907, "🖱", "vt-pane-pointer-selection-and-paint-laws", "Vt pane pointer, selection and paint laws: the wheel, the child mouse reports, the selection and clicks, the scrollbar, the theme colours and the painted marks."),
    (1149, "🎲", "vt-robustness-law", "Vt robustness law: arbitrary output, resizes and queries never panic."),
])

FRAG["tuiu"] = dict(rows=[
    (14, "🪟", "tui-layout-and-window-laws", "Tui layout and window laws: the row text helper, the weighted, auto, padded layouts, the window layouts and chrome silhouettes, the window controls and hits, the wizard and the remount."),
    (269, "📊", "tui-table-diff-parser-and-theme-laws", "Tui table, diff, parser and theme laws: the table helpers, header, rows, selection and flex columns, the cell diff, the ansi parser round trips, the text width, the theme and the wasm host smoke."),
    (475, "📐", "tui-geometry-text-cell-and-ansi-laws", "Tui geometry, text, cell and ansi laws: the rect operations, the character cells and truncation, the cell buffer writes, the full redraw diff, the run emission and the parser key and mouse decoding."),
    (728, "📟", "tui-vt-scene-and-layout-laws", "Tui vt, scene and layout laws: the vt screen cursor, wrap, scroll region, sgr, alternate screen and resize, the scene node accessors and removal and the column and window layout operations."),
    (924, "⌨", "tui-widget-key-handling-laws", "Tui widget key handling laws: the list, select, tabs, input, log and table key handling and the widget preferred sizes."),
    (1106, "🎨", "tui-widget-paint-and-window-silhouette-laws", "Tui widget paint and window silhouette laws: the list, select, tabs, log, input, divider, chip, navbar and footer painting, the terminal widget passthrough and the window stack tab silhouettes and hairlines."),
    (1377, "🚦", "tui-chrome-engine-wasm-clipboard-and-pty-laws", "Tui chrome, engine, wasm, clipboard and pty laws: the navbar and footer chrome, the engine focus, dispatch and render laws, the wasm host setup, the osc52 payload and the pty spawn, kill and resize."),
])

FRAG["paintu"] = dict(rows=[
    (8, "🧱", "paint-fixtures-and-loading-border-laws", "Paint fixtures and loading border laws: the action, text, stack and button fixtures, the paint setup and the text, stack, button, section and tree loading and waiting border laws."),
    (284, "🎚", "number-stepper-slider-and-field-paint-laws", "Number stepper, slider and field paint laws: the stepper borders and mixed placeholder, the slider readout, rail and compact edit and the field and tree item description paint."),
    (494, "🔽", "select-popup-drop-and-drag-paint-laws", "Select popup, drop and drag paint laws: the open select popup clamping, scrolling and hit testing, the production sync pass, the drop target flag, the activatable stack and the draggable tree item."),
    (803, "✍", "input-caret-and-focus-ring-paint-laws", "Input caret and focus ring paint laws: the input fixtures, the caret, selection and live edit buffer paint, the retained caret cadence and the accent focus ring on every control."),
    (1011, "🧵", "retained-paint-grant-laws", "Retained paint grant laws: the glyph count helper and the grant-bounded retained text, input, select and tree paint with their cancellation and exact owner witnesses."),
    (1271, "📶", "progress-and-image-paint-laws", "Progress and image paint laws: the progress fraction and track paint and the image source decoding, upload identity, content rect, raster quads and placeholders."),
    (1496, "🦴", "skeleton-celebrate-and-chrome-paint-laws", "Skeleton, celebrate and chrome paint laws: the skeleton predicate and blocks, the celebrate cycle, ring thickness and colour, the section and field chrome bands and the number stepper catalog icons."),
])

FRAG["engu"] = dict(rows=[
    (21, "🧱", "engine-fixtures-and-scene-lifetime-laws", "Engine fixtures and scene lifetime laws: the stack, button, clock and layout pool fixtures, the scene lifetime, slider, stepper and tooltip documents, the candidate retirement, the window clock, the caret and the pointer capture transfer laws."),
    (506, "💬", "tooltip-and-accessibility-fixtures-laws", "Tooltip and accessibility fixtures laws: the accepted control tooltip dwell and overlay paint, the tooltip text across candidate discard and the retained walk, layout drive and accessibility, blur commit, select and settings document fixtures."),
    (845, "⚙", "settings-close-and-accessibility-commit-laws", "Settings, close and accessibility commit laws: the settings document paint and stepper commit, the accessibility dispatch identity, the surface close, the blur commit and the select listbox projection laws."),
    (1169, "🖼", "facade-frame-storm-and-mounted-layout-laws", "Facade, frame, storm and mounted layout laws: the apply tree and frame laws, the retained paint walk, the dispatch click, the resize and interactive storms, the large layout shaping job and the mounted layout snapshot, theme and revision laws."),
    (1518, "🎭", "scene-host-overlay-body-and-frame-laws", "Scene host, overlay body and frame laws: the recording and backpressure scene hosts, the scene at queries, the open and modal overlay body placement and the atomic frame candidate and scene host routing laws."),
    (1942, "🏆", "golden-widget-conversions-and-paint-goldens", "Golden widget conversions and paint goldens: the widget node conversions, the statistics helpers and the stack, text, button, separator, input, select, toggle, key value, slider, stepper, ring and icon select paint goldens."),
    (2390, "🔬", "golden-tree-gaps-introspection-and-widget-measure-laws", "Golden tree, gaps, introspection and widget measure laws: the tree action rows, the known gap goldens, the widget kit coverage, the private window state and the widget wrap and measure laws."),
    (2804, "🖱", "widget-interaction-and-render-laws", "Widget interaction and render laws: the interaction maps, the widget render registrations for inputs, selects, sliders, steppers, rings, fields, sections and trees, the scroll regions and the draw text channels."),
    (3097, "📐", "surface-content-height-and-worker-close-laws", "Surface content height and worker close laws: the ui surface slot table, the intrinsic surface content height, the discarded candidate, the retained input discard and the original layout admission and worker close."),
])

FRAG["evu"] = dict(rows=[
    (9, "🎯", "event-fixtures-hit-test-capture-and-focus-laws", "Event fixtures, hit test, capture and focus laws: the select, tree, input, stepper and stack fixtures, the hit test pruning and scroll offsets, the select popup commit and wheel, the pointer capture and the focus cycling."),
    (422, "🪟", "overlay-drag-drop-and-scroll-laws", "Overlay, drag-drop and scroll laws: the overlay open, dismiss and placement, the drag promotion and drop acceptance and the scroll routing and thumbs."),
    (619, "✍", "edit-state-and-hover-reveal-laws", "Edit state and hover reveal laws: the text edit state, caret, selection and clipboard routing and the hover reveal."),
    (896, "🖱", "interactivity-and-scene-command-laws", "Interactivity and scene command laws: the press, release, click and scrub interactivity laws and the scene command and hit routing."),
    (1206, "📍", "anchored-placement-and-tooltip-dwell-laws", "Anchored placement and tooltip dwell laws: the anchored overlay placement and the tooltip dwell, reveal and dismissal."),
    (1352, "⌨", "widget-keyboard-laws", "Widget keyboard laws: the keyboard handling of every widget kind."),
    (1718, "🧭", "flow-direction-focus-visible-and-search-moment-laws", "Flow direction, focus visible and search moment laws: the right-to-left flow mirroring, the focus visible rule and the search line moment."),
    (1918, "🪞", "mirrored-number-and-text-keyboard-laws", "Mirrored number and text keyboard laws: the mirrored number law and the text keyboard law."),
])
