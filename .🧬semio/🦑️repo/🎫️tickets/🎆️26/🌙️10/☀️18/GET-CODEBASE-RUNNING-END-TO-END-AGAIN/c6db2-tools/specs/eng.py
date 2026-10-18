from specs.lib import *
F = DB + '/⚙️engine/🦀️.rs'
CH = [
    ch('🩺', 'database-spec-health-and-shutdown', 'database_spec_health_and_shutdown', [(73, 261)],
       'Database spec, health and shutdown: the artifact spec, the health report, the shutdown control with its interruption witness, the live guards and census and the shutdown phases and progress.'),
    ch('🚫', 'database-open-rejections', 'database_open_rejections', [(263, 329)],
       'Database open rejections: the retained activity rejection and the document open rejection with its conversions.'),
    ch('🧷', 'database-handle', 'database_handle', [(331, 452)],
       'Database handle: the database, the open-at rejection and the handle operations that do not depend on the emit runtime.'),
    ch('🔓', 'database-open-and-mount', 'database_open_and_mount', [(454, 796)],
       'Database open and mount: the open operations, the catalog read and bootstrap retained opens, the document mount and its adoption over the emit runtime.'),
    ch('🛠', 'database-catalog-shutdown-and-hello', 'database_catalog_shutdown_and_hello', [(798, 1060)],
       'Database catalog, shutdown and hello: the document creation, the catalog and health queries, the shutdown steps, the compaction and the sync hello admission over the emit runtime.'),
]
SPEC = dict(orig='eng.rs', file=F, children=CH, pre=split_impl("impl<E: Emit + 'static> Database<E> {", [796]), expect_extra=split_extra("impl<E: Emit + 'static> Database<E> {", [796]), wiring='block', external=mounted(F, orig='eng.rs'))
