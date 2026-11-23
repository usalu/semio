# Boot Matrix 02:00 Causes (Overlay, en)

Probe: overlay port 8775, modules demonstrator (01:55) and procedural (02:3x) built with all fixes through 00:25. Result 2/8 (energie, statik pass).

| Pane | Root error |
| --- | --- |
| generator | `member.publication: original page allocation attempt is occupied` on `toolRunStart`, contributions re-arm, typed-operation progress refresh and completion effects. Shell reaches `error`. |
| koordinator | `[DEBUG] one-item publication blocked` then `interactive-job.publication-stalled` on `setContributions` and `setActiveExample` (`pending_artifact=true publication=true`). |
| aussuchen | `setContributions` publication-stalled plus other sessions' `[DEBUG] session advance`/`page admit`/`recv refuse` noise. |
| aggregator | Only other sessions' `[DEBUG] session advance`/`page admit`/`recv refuse` noise. |
| bearbeiten | `member.publication: seal canonical second pass length changed` (bundle `err Some("validation failed: …")`) on `setContributions`. |
| verfolgen | One-item publication preparation refused with `InvariantViolated: canonical original child receipt exceeds caller grant` (canon kind 3 phase 4, demand copy_bytes 3336 depth 3, progress copied_bytes 3975), silently retried until `setActiveExample` stalls. |

Raw log: `🗑️generated/ov-0200.log` (deleted at close).
