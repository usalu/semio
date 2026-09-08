# Nx Validation Dispatch

The installed Nx implementation in `node_modules/nx/dist/src/project-graph/project-graph.js` explicitly supports `NX_FORCE_REUSE_CACHED_GRAPH=true`. Ticket validations now reuse the existing graph only to dispatch `nx exec` against the existing workspace project, avoiding repeated graph reconstruction while unrelated projects change. The validations themselves read current source and fresh Cargo metadata; no compiler results or package lists are taken from the cached graph. Permanent project configuration and Nx dependency behavior remain unchanged.

The first Stdio handler audit completed through normal graph construction. Subsequent standalone ticket commands may use this supported environment switch. A successful dispatch will be recorded separately; reading this implementation alone is not execution evidence.

Pass 610 successfully dispatched the current-source parser and ticket-selector validation through Nx with graph reuse enabled. Fresh source was read by each check.
