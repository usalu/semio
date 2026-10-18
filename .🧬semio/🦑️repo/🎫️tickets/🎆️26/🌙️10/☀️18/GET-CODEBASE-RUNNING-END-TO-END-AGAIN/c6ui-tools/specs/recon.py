from specs.lib import *
F = U + '/🧠️runtime/♻️reconcile/🦀️.rs'
CH = [
    ch('🧮', 'surface-reconciler', 'surface_reconciler', [(18, 142)],
       'Surface reconciler: the `SurfaceReconciler` that drives a surface document reconcile and its drop.'),
    ch('🎫', 'reservations-and-registry', 'reservations_and_registry', [(150, 277)],
       'Reservations and registry: the reconcile admission limits, the reservation and its refusals, the registry census and the job phase.'),
    ch('🤝', 'handback-registry', 'handback_registry', [(279, 425)],
       'Handback registry: the handback slots, keys, reservations and registry with the reserve, release, rebind and acquire operations.'),
    ch('🎟', 'retained-authority', 'retained_authority', [(427, 597)],
       'Retained authority: the `SurfaceReconcileRetained` custody of a reconcile across turns and the handback of a finished reconcile.'),
    ch('📄', 'document-producer-and-patches', 'document_producer_and_patches', [(599, 892)],
       'Document producer and patches: the job step, the document fault, the document producer and outcome, and the ready, published and acknowledged patches.'),
    ch('🏃', 'reconcile-job', 'reconcile_job', [(894, 1183)],
       'Reconcile job: the `SurfaceReconcileJob` with its debug, step and drop.'),
    ch('🏁', 'rejected-and-terminal', 'rejected_and_terminal', [(1185, 1448)],
       'Rejected and terminal: the rejected reconcile and the terminal with their retirement, and the terminal take and handback close.'),
    ch('🧪', 'registry-test-support-and-drain', 'registry_test_support_and_drain', [(1451, 1548)],
       'Registry test support and drain: the registry test guard, the isolation, the orphaned slot reclaim and the drain until idle.'),
]
SPEC = dict(orig='recon.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='recon.rs'))
