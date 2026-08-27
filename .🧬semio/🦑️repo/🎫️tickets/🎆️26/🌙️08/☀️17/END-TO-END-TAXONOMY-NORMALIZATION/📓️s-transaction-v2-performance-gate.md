# Transaction v2 Performance Gate

## Result

Performance acceptance remains withheld at `0/3`. No trial was started because both preconditions for valid acceptance evidence failed on 2026-08-27:

1. The shared host was severely saturated rather than reasonably quiet.
2. One of the four files frozen by the independent Transaction v2 disposition no longer had its frozen byte identity before the first trial.

No unrelated process was terminated. No normalization source, test, golden, runner, Git state, or intentionally deleted opaque tree was read through an excluded path, modified, restored, or otherwise touched by this verification lane.

## Exact command

The frozen report requires three complete uncached executions of:

```text
bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache
```

The Nx target still routes through the package-local permanent script as `bun ./📜️script.ts test transaction-v2`. The explicit `--skip-nx-cache` flag is the required cache-bypass authority. Because no trial was started, there is no runtime cache transcript to claim.

## Frozen identity preflight

| Boundary | Frozen SHA-256 | Observed SHA-256 | Result |
| --- | --- | --- | --- |
| normalization source | `86f90f2e954e8082e0a6f9b0f5432a1e0131f86137624312e945849a602dc76f` | `fce97b0724482ae27c52070d9b2bd4ac121bb779f9b1cf615ca74e28403e9ff2` | changed before trials |
| dedicated aggregate | `22099778a38e0107cdadae4762010ba4f001bd484efb924ca350ee6c51b0539c` | `22099778a38e0107cdadae4762010ba4f001bd484efb924ca350ee6c51b0539c` | exact |
| exact golden | `e3c9dbad890beda23b7ed8233cb027ccd9374dc77ed72beb077c55ba2fd4138d` | `e3c9dbad890beda23b7ed8233cb027ccd9374dc77ed72beb077c55ba2fd4138d` | exact |
| aggregate runner | `e5e205edf9bf00643ed29bb05b5ba3f9a92363186f31f5b21f7bebfae92fd1f4` | `e5e205edf9bf00643ed29bb05b5ba3f9a92363186f31f5b21f7bebfae92fd1f4` | exact |

The normalization source was last modified at `2026-08-27T02:39:14+0200`; the identity preflight was captured at `2026-08-27T02:57:29+0200`. This gate is read-only and therefore did not attempt to restore or rewrite the signed source.

## Host-load preflight

The machine exposes 10 logical CPUs.

| Observation | Load averages | Relevant concurrent work |
| --- | --- | --- |
| approximately 02:55 CEST | `106.15 / 105.26 / 90.12` | multiple Nx lint jobs, dependency-cruiser, workspace tool-job verification, Git enumeration/status, and high-CPU application renderers |
| approximately 02:57 CEST | `118.64 / 111.54 / 94.33` | the same work plus an Nx quick test, a live Rust compiler using `-Z threads=8`, and additional repository/MCP activity |

The second observation was worse than the first and exceeded the logical-CPU count by more than an order of magnitude. Starting the four-shard aggregate would not have produced defensible performance evidence and would have increased contention for other developers.

## Trial ledger

| Trial | Started | Uncached evidence | Exit | Pass/fail/assertions | Internal time | Wall time | Qualifies |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | no | command specifies `--skip-nx-cache`; not executed | n/a | n/a | n/a | n/a | no |
| 2 | no | command specifies `--skip-nx-cache`; not executed | n/a | n/a | n/a | n/a | no |
| 3 | no | command specifies `--skip-nx-cache`; not executed | n/a | n/a | n/a | n/a | no |

## Acceptance disposition

The required acceptance condition remains exactly three complete uncached runs, all on identical source, test, golden, and runner bytes, each with wall time below 15 seconds. This checkpoint supplies no qualifying run. Retry only after the four intended identities are frozen again and the host is reasonably quiet.
