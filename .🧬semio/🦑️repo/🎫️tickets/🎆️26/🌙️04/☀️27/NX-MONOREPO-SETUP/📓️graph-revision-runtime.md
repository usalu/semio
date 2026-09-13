# Graph Revision Runtime

## 2026-09-13

The full cache-contract retry stopped advancing after native preparation while its Bun process continued consuming CPU. A native one-second sample showed repeated file reads; the next fixture had not reached its Nx subprocess. The graph implementation had changed after the process started.

A bounded reproduction compared native Bun 1.3.14 and Node 24.15.0. Changing an imported file and only changing its URL query returned the first module in Bun, whose import.meta.url omitted the query. Node loaded the changed module and retained its revision query. The graph wrapper recursively calls the revised module, so Bun repeatedly returned the old wrapper and never reached graph work. The owned stalled Bun process was terminated; no foreign process was stopped.

A second reproduction used the public Node-compatible require.cache interface before importing. Both runtimes then observed the changed module, including a module using top-level await. This provides a direct runtime witness for the required cache admission boundary.

The permanent neutral fixture compares actual graph results under both runtimes after policy and implementation changes, and verifies rejection and recovery for changes in both dynamically loaded graph helpers. Red/green results and the final implementation are recorded below.

Reference: [Bun module systems](https://bun.com/docs/runtime/module-resolution). Cache behavior here was measured on the installed runtimes rather than assumed from query-string behavior.

The actual-graph red probe reached the Bun reload loop and was cancelled at its 20-second boundary (20.4 seconds total). An initial fixture-key typo had first failed separately in 906 ms and was corrected before that red witness. The first green graph proof passed in 2.3 seconds. The final proof also covers invalid taxonomy and restoration.

The graph loader now evicts its own canonical runtime cache entry before importing a content revision, coalesces reloads for the same revision, releases failed admission promises and explicitly rejects a runtime that returns its obsolete module. Both dynamic helper imports use the same mechanism. The graph revision now includes the taxonomy it loads. Native graph policy/code changes and helper failure/recovery agree between Bun and Node. No source copies or unbounded cache namespaces are created in the repository.

The extended taxonomy proof passed in 5.1 seconds. The next full actual-root run progressed past the previously hanging generator ownership boundary and failed after 5m18s at the stale OS Dev class extraction; its graph-revision test passed. The subsequent 9.4s run also passed graph revision and failed a separately relocated Cargo class assertion. See 📓️wgpu-runtime-isolation.md for those test-owner repairs.
