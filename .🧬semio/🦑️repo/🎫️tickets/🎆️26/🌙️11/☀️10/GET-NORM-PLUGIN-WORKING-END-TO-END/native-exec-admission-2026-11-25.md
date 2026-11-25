# Native Exec Admission

The ticket's `native-exec.ts` launched its target with an issued script envelope but no capacity reservation. The baseline mock reproduced the problem by entering its target with no grant directory or grant row.

The helper now uses the repository's command admission policy and exact current ledger reservation. The Play serving target uses the repository's end-to-end envelope. An existing reservation is accepted only when its live ledger row covers the requested memory and core count. Missing, stale or undersized carried authority stops the launch. Admission uses the normal host gate and warden state without loading a project graph.

Known Rust package and component driver entry points keep their own compiler admission. The new controller grant counts their process tree and carries its wait markers. It omits the synthetic one-job plan for these self-gated children. Valid enclosing plans are preserved. Interrupts cancel the admission wait or forward the original signal to the owned process tree, and the helper waits for graceful cleanup before returning its grant.

The focused `native-exec-law.ts` suite reproduced the unreserved launch before the edit and passed six cases after it, with 17 assertions. The mock verified fresh admission, live inheritance, refusal of an undersized Play reservation, refusal of stale authority, preservation of native compiler routing, and signal delivery with grant release. Only tiny mock processes ran.
