# Scope of speculative owner looks

Read 67785 reported 38 of 4,808 decided owners and 4,770 wanted from the fleet, with nine farm children busy and individual owner waits above 800 seconds. The wanted count was recorded once the bulk take path returned and announced its remaining owners. The read subsequently adopted seven arrivals from the fleet and reached at least 49 decided owners. These observations establish a slow transition and continued work.

The queue previously kept dispatching speculative owner looks after the bulk take expired or finished. It would fill a newly freed worker slot before an exact inference or adoption request arrived. Once a speculative job was active on that worker, bringing an exact request to the queue front could not free it.

A native worker fixture reproduced this on both deadline and normal-completion signals. In each case the active worker finished, immediately started the unrelated unasked owner's look and would have held the exact next request behind its gate. The queue now gates speculative dispatch on the bulk pass's existing signal. Normal completion closes that signal. Running jobs retain custody, and exact later requests can promote the same queued entries. The post-pass fleet-adoption poll has its own lifetime.

The native green cases accepted the original running answer, dispatched the later exact request ahead of the unasked entry, and reused that queued entry when it was explicitly demanded. The neutral fixture and SQLite agreed on dispatch order. The look queue, take, adoption, isolation and release suites passed 37 tests across eight files. The tests used owned local fixture processes. The active graph read was observed through its log.
