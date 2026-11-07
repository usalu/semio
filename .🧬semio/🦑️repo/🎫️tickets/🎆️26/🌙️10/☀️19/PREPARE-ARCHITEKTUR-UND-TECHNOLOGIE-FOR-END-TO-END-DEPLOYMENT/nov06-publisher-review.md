# Native Windows Publisher Review

Reviewed the current Windows publisher source and the native publication, source registration and extractor ownership reports. This is a finite source review; no build or publication was run.

## Actionable finding

The publisher creates its scratch directory and then creates its inputs and output directories before entering the cleanup try/finally. A normal failure or cancellation during either child directory creation leaves a charged, unreferenced stage without pending intent. Install cleanup immediately after capturing scratch identity, retaining the pending-intent exception once intent is published. Communicated to the publisher owner.

The ordinary success path deletes pending intent without comparing its physical identity, whereas recovery explicitly compares it. Guard deletion against the identity of the published intent under the same cooperative custody assumptions.

## Observations and limits

The source fingerprint checks the exact seventeen-file production roster, admitted Source bytes and physical file stamps before and after helper build and invocation. The helper itself is checked by digest and physical stamp. Recovery distinguishes stage from published tree, checks closed intent and receipt against allocation and pinned identities, and verifies inventory and full tree before completing publication. The consumer holds the shared provider lease and verifies physical receipt, inventory and full tree through its lifetime.

These observations do not establish race-free hostile same-user path mutation, actual issued-store publication, crash/fault recovery at every boundary, or successful canonical Cargo/cc consumption. Those require the pending governed integration and fault tests. The extractor findings require cooperative private stage custody.
