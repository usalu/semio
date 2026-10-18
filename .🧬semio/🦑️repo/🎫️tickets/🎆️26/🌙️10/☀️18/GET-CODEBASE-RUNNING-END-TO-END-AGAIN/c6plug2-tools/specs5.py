"""📐 C6DB1 chunk tables for the flat test roots: tag -> rows (first item line of the ORIGINAL snapshot as printed by ol.py, folder emoji, kebab, doc)."""
FRAG = {}

FRAG["etests"] = dict(rows=[
    (7, "🧸", "fixtures-batch-and-concurrent-ensure", "Fixtures, batch and concurrent ensure: the controlled mount emit and fanout probe fixtures, the declared maximal batch, the create-catalog publication order and the concurrent ensure of one mounted actor."),
    (231, "🏊", "worker-pool-use-and-mount-owner-laws", "Worker pool use and mount owner laws: the worker pool use during shutdown and before the first probe, the published opening join, the cancelled ensure waiter and the mount owner that emits before ready."),
    (464, "📣", "mount-fanout-shutdown-and-mount-failure-laws", "Mount fanout, shutdown and mount failure laws: the fanout wake after the registry unlock, the shutdown interrupt, the catalog open and create laws and the mount failure terminalization."),
    (776, "🔌", "mount-cleanup-fault-round-trip-and-deferred-seams", "Mount cleanup fault, round trip and deferred seams: the cleanup fault that consumes a racing resume, the unknown document, the submit and query round trip, the history replay, the reopen, the exact consistency and the deferred seams."),
    (978, "✍", "writer-residency-compaction-and-hello", "Writer residency, compaction and hello: the join helpers, the writer slot rounds, the filesystem documents beyond the slots, the compaction passes, the shutdown retry owners and the hello sessions."),
    (1332, "📚", "history-wake-and-terminal-close-laws", "History wake and terminal close laws: the history admission retirement, the public wake and the history completion and terminal close laws."),
    (1535, "🧭", "long-module-and-source-laws", "Long module and source laws: the long module and the retained sync hello and retry owner source laws."),
])

FRAG["fix"] = dict(rows=[
    (7, "🧪", "law-executors-and-fault-fixtures", "Law executors and fault fixtures: the law executors, the page lifecycle fixture, the fault category fixtures and oracle and the async executor impls."),
    (285, "🚧", "blocking-executors-and-fixture-helpers", "Blocking executors and fixture helpers: the blocking and deferred executor impls and the ledger, page and job helpers."),
    (475, "📏", "fixed-page-output-and-lease-laws", "Fixed page, output and lease laws: the page maximum, the artifact rejection, the async native fault, the output task, the result owners, the process page preflight and the platform ring."),
    (704, "🪤", "lost-owner-ring-close-interruption-and-queued-callback", "Lost owner ring, close interruption and queued callback: the lost owner ring, the interrupted storage close, the one-owner-per-grant close and the queued callback slot reuse."),
    (945, "🔁", "retry-driver-and-backend-controls", "Retry driver and backend controls: the saturated retry wake, the retry generation maximum, the mock drivers, the shared worker driver, the backend controls and the lost result lease."),
    (1175, "🧯", "pressure-slots-and-lost-backend-laws", "Pressure slots and lost backend laws: the rejected backend and lost owner pressure slots, the lost backend and page handle owners and the typed lane lease."),
    (1417, "🚨", "registration-cancellation-drop-and-cleanup-laws", "Registration, cancellation, drop and cleanup laws: the cancellation owners, the panic and shutdown close, the registered backend pool, the registration saturation, the forged backend kind and the cleanup fault waiters."),
])

FRAG["pbu"] = dict(inner=True, inner_header=2, rows=[
    (11, "🧪", "fill-fixtures-and-mounted-session-helpers", "Fill fixtures and mounted session helpers: the fill test constants and clock, the placement witness, the capture, session, checkpoint and job helpers and the frontier host."),
    (378, "🖌", "brush-slot-laws", "Brush slot laws: the brush slot preview, place on leave, fastened handle refusal, suggestion commit and cancel and the pointer move out of the slot."),
    (655, "🧱", "fill-frontier-checkpoint-and-job-laws", "Fill frontier, checkpoint and job laws: the deterministic frontier, the checkpoint restore, the cancel, supersession, deadline, refusal and saturation closes, the worker count identity and the capture maximum."),
    (1033, "🔎", "candidate-acceptance-preview-and-proximity", "Candidate acceptance, preview and proximity: the retained field stages of an accepted candidate, the drop preview json, the session mirror and the proximity sorted candidates."),
    (1270, "🚪", "fill-base-core-tambour-and-pointer-slot-laws", "Fill base core, tambour and pointer slot laws: the rectangular base core, the door tambour exclusion and the pointer slot acceptance on the node body and the indirect ring anchor."),
    (1501, "📚", "kind-catalog-and-free-handle-laws", "Kind catalog and free handle laws: the document kind catalogs translated into brush candidates, the manifest only documents and the fill without kind catalogs."),
])
FRAG["put"] = dict(rows=[
    (135, "🧰", "history-acceptance-source-laws-and-camera-helper", "History acceptance, source laws and camera helper: the history edit acceptance law, the editor source reader, the cursorized cohort route laws and the rendered camera helper."),
    (224, "🧮", "operations-and-command-envelope-laws", "Operations and command envelope laws: the node and example operations, the selection, nudge and undo laws, the fixed edit ledger and the command envelope round trip."),
    (378, "🎯", "board-event-and-ui-scope-laws", "Board event and ui scope laws: the camera and window transients, the board event commits and the ui dirty scope each action declares."),
    (587, "📜", "manifest-convergence-and-registry-laws", "Manifest, convergence and registry laws: the declared window kinds, actions and tools, the backbone convergence, the idempotent ingest and the registry view and context menu laws."),
    (813, "🧩", "engine-parse-and-handle-suggestion-laws", "Engine parse and handle suggestion laws: the board engine parse of the painted fixtures, the deleted handle laws, the hover scene and the suggestion popup laws."),
    (1122, "🏷", "display-label-inspector-edit-and-window-option-laws", "Display label, inspector edit and window option laws: the node display labels, the inspector patches and the window option verbs."),
    (1322, "🕹", "gumball-region-set-verb-and-snapshot-laws", "Gumball, region, set verb and snapshot laws: the transform gumball, the board region events, the idempotent set verbs, the shipped kind catalog and the initial snapshot cost and import."),
])
FRAG["pbh"] = dict(rows=[
    (10, "🧾", "manifest-descriptor-sync-and-style-laws", "Manifest, descriptor sync and style laws: the default manifest validation, the descriptor sync deferral while panning and dragging, the cached content and the element style colors."),
    (311, "🖱", "drag-selection-and-lod-hit-laws", "Drag, selection and lod hit laws: the node drag, the compact and bounded drags, the detail lod handle hit and the multi select and edge selection laws."),
    (626, "📐", "generation-area-select-and-minimap-laws", "Generation, area select and minimap laws: the scene generation stability, the background click deselect, the area select, the minimap hover and preselect and the board naming vocabulary."),
])
