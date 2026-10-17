"""📐 C4PP chunk tables for the flat renderer test modules. Row: (item_line, folder, doc[, depth[, explicit_end]]); item_line is an item or region line of the ORIGINAL snapshot (backed up over the contiguous doc/attribute/comment lines above it); a chunk runs to the line before the next row's start."""
R = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
E = R + "/🧱️elements"

TABLES = {}

TABLES["uicmd"] = dict(
    orig="uicmd.rs",
    file=E + "/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs",
    rows=[
        (4, "🗂️retained-document-queue-and-editor-fixtures", "🗂️ Retained document queue and editor fixtures: the unbounded step grant, the text editor outbox and focus fixtures, and the retained document close queue, page budget and wrong generation laws."),
        (108, "🎬️window-action-drop-clipboard-and-noop-commands", "🎬️ Window action, drop, clipboard and no-op commands: the action fixtures, the clicked window context, the drop committed region, the mocked OS clipboard laws and the explicit no-op commands."),
        (318, "🧪️scene-fixtures-and-focus-rebase-builders", "🧪️ Scene fixtures and focus rebase builders: the component scene leaf, the identity and clipboard input documents, the seeded scene windows, the focus rebase document builders and the ink intent builders."),
        (740, "🎯️canvas-pointer-and-ink-scroll-dispatch", "🎯️ Canvas pointer and ink scroll dispatch: the canvas2d pointer down action, the React shaped pointer payload, the ink canvas scroll action and the ink scene driving helpers."),
        (902, "✒️ink-gestures-and-closing-owners", "✒️ Ink gestures and closing owners: pointer cancel, streamed note gestures, the ink canvas text and table editing lifecycle, stale scene revisions and the closing window and editor owner laws."),
        (1246, "⌨️text-editor-focus-and-clipboard", "⌨️ Text editor focus and clipboard: stale window close against a successor focus, focus transfer on primary presses, the node graph note caret, the space key and the accepted host clipboard, composition and accessibility."),
        (1465, "📋️accepted-table-editable-text-and-stepper", "📋️ Accepted table editable text and stepper: the table scene nodes, enter and escape on editable text cells, the accessibility value echo and the stepper navigation keys."),
        (1682, "🧩️accepted-control-publications", "🧩️ Accepted control publications: the localized buttons and exact actions of the VFS controls, the block list, the event feed, the world3d overlay, the table row button and the graph checkpoint."),
        (2035, "🪄️presented-focus-rebases-and-ink-intent", "🪄️ Presented focus rebases and ink intent: focus rebasing only after the same host pixels are accepted, and the ink intent barrier, its successors, hidden windows, stale revisions and progress owner."),
        (2309, "🚪️window-close-gestures-and-generic-surfaces", "🚪️ Window close gestures and generic surfaces: queued scene and capture owners at close, active canvas gestures, bespoke surface skipping, pointer button codes and generic fallback surfaces."),
        (2513, "📥️retained-input-commit-receipts", "📥️ Retained input commit receipts: the typed input record, the generation bound completion receipt and the shared draft disposition law."),
    ],
)

TABLES["dockunit"] = dict(
    orig="dockunit.rs",
    file=E + "/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs",
    rows=[
        (10, "🧰️dock-fixtures-and-sample-app", "🧰️ Dock fixtures and sample app: the tab and stack builders and the sample app definition with its windows, layouts and panels."),
        (77, "🪟️window-instances-geometry-and-silhouette", "🪟️ Window instances, geometry and silhouette: concrete window instances, stale kind focus, split extents, stack content rects, the icon slot, the silhouette clip, owned chips and the cap depth."),
        (265, "🧲️drops-and-drop-zones", "🧲️ Drops and drop zones: tab and stack drops across corners, stacks and axes, abandoned drags, the drop zone and the split side."),
        (489, "🔁️layout-diffs-and-even-layouts", "🔁️ Layout diffs and even layouts: layout diffs that keep focus and maximization, subtree reuse, even and default layouts, closing and reordering tabs and the split drop preview."),
        (665, "🛠️actions-utilities-and-tab-actions", "🛠️ Actions, utilities and tab actions: the actions and utilities app, the scoping window, the utility option partition, the painted tab controls, the maximize toggle, tab hits and label truncation."),
        (859, "🗜️closing-collapse-and-split-resize", "🗜️ Closing, collapse and split resize: closing the last, active and only tabs, single child hoisting, inert maximization and the split resize totals, deltas and gutter hits."),
        (1007, "🧭️drag-merge-mobile-and-focus", "🧭️ Drag, merge, mobile and focus: drag to split and merge, the corner tab bar widths, the mobile flat stack, window activation, the drag lane, the focus border, the insert index and the themed axis geometry."),
    ],
)

TABLES["paint2d"] = dict(
    orig="paint2d.rs",
    file=E + "/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs",
    rows=[
        (20, "🧰️scene-fixtures-and-drains", "🧰️ Scene fixtures and drains: the empty, raster, paint2d and text editor scenes, the layer hit point and the action drains."),
        (141, "🎨️paint2d-attach-and-selection-publication", "🎨️ Paint2d attach and selection publication: the raster host attach and paint, the layers domain selection interaction, hover, wheel camera and the navigator view mode."),
        (257, "⌨️text-editor-attach-draft-and-key-laws", "⌨️ Text editor attach, draft and key laws: the editor host attach, the explicit draft, large draft paging, the keys the shell owns and the renderer keys against the React fixture."),
        (448, "📤️text-editor-outbox-receipts-and-delivery", "📤️ Text editor outbox, receipts and delivery: credit pressure, typing run signals, ordered pages, the neutral receipt laws, scene echoes, the local echo ledger and the production key route."),
        (735, "🧲️marquee-and-navigator-laws", "🧲️ Marquee and navigator laws: the marquee drag, the point pick, the you are here viewport rectangle, middle drag panning and the navigator wheel."),
        (895, "🪣️host-retirement-fill-and-stroke-laws", "🪣️ Host retirement, fill and stroke laws: phase four binding retirement, the bucket fill region click and the long and mask stroke publications."),
        (1000, "📜️editor-scene-document-laws", "📜️ Editor scene document laws: the canonical scene corpus, the contract scene conversion and the typed scene the editor host takes."),
    ],
)

TABLES["engsurf"] = dict(
    orig="engsurf.rs",
    file=E + "/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs",
    rows=[
        (29, "🗃️fixtures-and-retained-map-records", "🗃️ Fixtures and retained map records: the engine surface fixture constants, the scene builders and the retained map, separator and row records with their painting and publishing helpers."),
        (216, "🚪️engine-close-and-world3d-fixtures", "🚪️ Engine close and world3d fixtures: the exact staged host close, the painted frame, the scene painter and the world3d ladder, attach and intent helpers."),
        (367, "🌐️world3d-window-pointer-and-orbit-laws", "🌐️ World3d window, pointer and orbit laws: the preview window attach, the graph domain selection dispatches, the orbit and wheel camera payload and the bounded thread stack."),
        (533, "🗺️tiled-map-and-board-attach-laws", "🗺️ Tiled map and board attach laws: the production seam attach, the board trace lane, the heap first slot tables, the visible tile reservation and the cancel separation."),
        (737, "🤏️two-touch-gestures-and-hover", "🤏️ Two touch gestures and hover: the shared camera math with distinct transfer rules and the one empty owner transition on hover leave."),
        (852, "♻️retained-map-gesture-lifecycle-and-bridge", "♻️ Retained map gesture lifecycle and bridge: same identity refresh, key replacement and removal, sibling rebase, the tile URL placeholders and the bridge lease revision bump."),
    ],
)

TABLES["kernelrt"] = dict(
    orig="kernelrt.rs",
    file=R + "/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs",
    depth=1,
    rows=[
        (4, "🧰️retained-document-fixtures", "🧰️ Retained document fixtures: the replay recovery job, the retained document and the retained patch builders.", 1),
        (55, "🔢️sequence-and-registry-max-plus-one", "🔢️ Sequence and registry max plus one: the renderer sequence exhaustion with its transactional rollback and the exact max plus one surface and patch owners of the fixed registries.", 1),
        (125, "📄️command-batch-and-document-pages", "📄️ Command batch and document pages: the retained batch document, page saturation, stale cancel and fault builds and the first render opportunities.", 1),
        (234, "📡️presentation-bridge-and-replay-recovery", "📡️ Presentation bridge and replay recovery: the fixed FIFO job progress bridge, the cancelled head, the replay recovery capacity and the rejected production submit retry.", 1),
        (341, "🪪️product-authority-fixtures-and-identity", "🪪️ Product authority fixtures and identity: the admitted request, permit, recovery drain and authority fixtures and the complete identity and field qualification laws.", 1),
        (505, "🧱️product-claim-and-raw-backing", "🧱️ Product claim and raw backing: the claim fault checkpoint, the effect moved into the fixed authority and the raw allocation identity through refusal, claim, drop and backing retirement.", 1),
        (638, "📥️product-ingress-and-refusal-slots", "📥️ Product ingress and refusal slots: kind and input max plus one, incremental recovery of the remainder and the refusal slot overflow.", 1),
        (810, "♻️mounted-replay-drop-and-panic-recovery", "♻️ Mounted replay drop and panic recovery: the process replay sizes, the sub eight millisecond opportunities, the populated drop and the panic after capture.", 1),
        (912, "⏱️turn-fairness-and-abandoned-requests", "⏱️ Turn fairness and abandoned requests: the destroy and close fixtures, the turn slice and gap contracts and the abandonment of undelivered requests.", 1),
        (1002, "📨️fixed-request-queue-laws", "📨️ Fixed request queue laws: capacity plus one ownership, FIFO order, the aggregate page credit, contention and the admission that never parks without a wake.", 1),
        (1149, "🔒️close-registry-and-queue-shutdown", "🔒️ Close registry and queue shutdown: the modulo collision and contention of the fixed close registry and the shutdown release of handles, pages, create fields, surfaces and events.", 1, 1245),
        (1257, "🧵️slot-tables-pool-future-and-delivery-wake", "🧵️ Slot tables, pool future and delivery wake: the heap first slot tables on a bounded stack, the pool future's owner lock and the response delivery wake.", 0),
    ],
)
