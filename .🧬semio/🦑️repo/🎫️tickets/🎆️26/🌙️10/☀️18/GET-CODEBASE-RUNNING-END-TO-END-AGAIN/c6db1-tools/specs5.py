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
