# Staged doc batches, round 2 (docs4)

Each batch: `python3 -I staged2/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, exit 2 on drift, writes nothing then). Laws: `DOCS4_STAGED=staged2 python3 -I verify.py <mirror root>`.

| # | batch | edits | adds |
|---|---|---|---|
| 1 | `memo-serve` | 4 | The graph memo across generations, edits and derived facts: the three info lines, what is served and what still plans, output directories. |
| 2 | `plan-walk` | 4 | Bazel plan witness: the git row digests only admitted files, the source walk asks git about excluded names. |
| 3 | `dashboard` | 2 | The dashboard no-op: installed binary returned when Cargo's own source list holds, the profile policy in the digest, the measured numbers. |
| 4 | `rust-lanes` | 4 | Rust test path: incremental workspace crates, lane quota, startup of a test action; the lease-order sentence loses its remaining pairs. |
| 5 | `disk-memo` | 2 | The graph memo's 1 GiB budget and the sweep of abandoned journal state directories. |
| 6 | `sealed-capacity` | 4 | The sealed bullet loses its stale 'gate batch 8'; the grown-reservation row mentions the jobs that memory carries. |
| 7 | `status` | 14 | Replaces the 10-26 status, the G34 note and the 10-26 baseline with today's measured numbers; updates the measured line of 'Measuring a build' and the graph-builder row. |
