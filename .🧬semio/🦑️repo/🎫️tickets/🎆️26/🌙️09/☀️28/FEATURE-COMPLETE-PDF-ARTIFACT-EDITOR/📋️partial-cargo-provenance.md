# Closure-Scoped Partial Cargo Provenance

The live driver limits a failed Cargo invocation's temporary provenance observation to the combined dependency closure of the components Cargo actually linked. It joins the admitted graph to the pass's actual compiler-artifact and build-script-executed announcements using the same identities as Merkle leaf generation. All selected identities, including build-script runs, must be present. A linked component's observed output digest must also match its staged WASM. Per-component provenance still uses its own closure within the selected union.

The driver keeps the unfiltered Cargo JSON stream, compiler diagnostics, run telemetry, failure files, resource budgeting and cleanup inventories. Prefetch continues speculatively under the existing custody, deduplication, and cancellation controls. Its speculative digest and dep-info reads are not themselves Merkle evidence. The selected invocation observation still runs the existing compiler checksum and retained-source checks for every selected row and settles its invocation custody. A successful pass continues observing the full roster and follows the same session seal and receipt publication. A failed pass still returns a failure, discards its temporary provenance receipt and provides no sealed compilation capture. Linked post-link stages can receive their established per-component evidence.

The input was reproduced in the repository's admitted native-artifacts fixture driver with one shared compiler dependency, a shared build-script run, two independent links and a failure in Stdio. The neutral graph's component leaf counts come from an independent SQLite recursive reachability query. The old driver rejected the unrelated drift dependency because no retained source witness matched its compiler checksum. Neither linked stage received provenance. Under the staged driver both independent components received all four expected leaves. The stale source and malformed resource journal from the unsuccessful branch fell outside both closures. A stale shared source and a missing shared announcement each stopped the linked provenance stages. The full-success case observed and sealed all three components.

A read-only sample of the real w8jCBpl Cargo graph and event log joined 770 of the PDF root's 772 distinct graph identities. Its two outstanding identities were the PDF component and its immediate PDF API library. The captured BREP compiler-error packages were outside the PDF closure. The staged driver was exercised in the repository fixture harness. The real w8jCBpl process was already running the old driver.

## Review and replay

- `📜️partial-provenance-stage.py --prepare` generates the three reviewed paths, and `--verify` checks all exact live and staged SHA-256 digests. `--apply-reviewed` is available for the coordinated release.
- `📜️partial-provenance-run.ts <pinned rustc path>` runs the exact staged component-driver fixture. The runner uses the repository test owner and an issued script process envelope.
- `📜️partial-provenance-stage.py --baseline` and the runner with the additional arguments `' unrelated source' --baseline` demonstrate the expected failure against the original driver.
- The neutral input is `🔣️partial-provenance.json`. The fixture source is `📜️partial-provenance-law.ts`. Receipts and test output are under `🗑️generated/partial-prov`.

The three reviewed files are live and matched their staging digests immediately after the write. No broad Cargo compilation was started for this patch.
