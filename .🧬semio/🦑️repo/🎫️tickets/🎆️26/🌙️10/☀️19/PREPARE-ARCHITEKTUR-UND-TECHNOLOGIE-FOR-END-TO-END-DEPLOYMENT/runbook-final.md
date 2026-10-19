# Final Course Runbook Coherence

The current course README and deployment workflow comments were compared with the actual package router, `.vscode/launch.json`, package preflight metadata, E2E port selection, image producer and rollback tag helper. Only two authored files changed:

- `🎓️teaching/🏛️architecture/📚️course/README.md`
- `.github/workflows/architecture-course.yml` (comments only)

Concrete repairs:

- Developer instructions now start through Run and Debug and the existing sixteen course launch configurations. Dashboard task creation, dashboard cancellation and standalone CLI developer examples were replaced with actual launch references. E2E selections are arguments to its registered launch. The unrelated standalone proctor target rows were removed from the course launch table; their owner documentation remains intact.
- E2E ports now say fresh OS-selected loopback ports, matching `endToEndPorts()` rather than obsolete fixed pairs.
- Original release mode is documented as the existing launch configurations' and workflow's `SEMIO_BUILD_MODE=release`; managed CI may explicitly select `--configuration=release`.
- The current Debian 12 x64 image producer rejects ARM64, so the stale ARM64 canonical-build suggestion was removed.
- The runbook and workflow comments agree with actual authored preflight metadata: Linux x64, Debian 12, 77,309,411,328 available bytes (72 GiB), at least seven writable unused loop devices, process capabilities and actual ext4 mount admission. Precise historical fixed-ledger sizing was replaced by the actual authored preflight threshold and per-store admission requirement.
- The existing immutable rollback format remains `sha-<12 commit hex>-<64 image-config hex>`; the complete example is 81 characters. No obsolete `--jobs` or raw Cargo operator build route remains in the scoped instructions. Docker host operation commands remain host instructions.
- The current material/catalog taxonomy, host values and legal placeholders were preserved. A supplied E2E executable is explicitly described as diagnostic input, without claiming production of the checkout's native artifact.

## Validation

The focused diagnostic completed with exit 0: **two files, 36 tests passed**, using the current course config, native config loader and genuine teaching workspace resolver. Actual output is in `🗑️generated/runbook-final.log`. Deployment tests read the edited README and workflow, parse workflow YAML and validate deployment drift and runbook step order; release-tag neutral cases and the SHA256 oracle check immutable naming.

The command also requested the launch suite, but that suite did not execute. A final read found concurrent source changes after this audit's initial inspection: `.vscode/launch.json`, the course launch suite and its fixture are absent, and the course config now includes a dashboard contract suite. The initial inspection actually read sixteen registered launch entries. This final absence is reported to root for reconciliation; this diagnostic does not establish the current launch contract, and the new launch-specific runbook wording cannot be called coherent until that concurrent change is resolved.

Root confirmed the latest human instruction explicitly requires `launch.json`, and owns restoring the sixteen launch entries and their neutral fixture/tests while preserving the new dashboard contract. The new dashboard suite's first two tests are compatible; its final `has no separate IDE command catalog` test explicitly forbids `.vscode/launch.json` and was reported to root as the conflicting assertion requiring correction. This report preserves the actual 36-test result rather than claiming the missing launch test ran.

No native compilation, Docker production, registry push, site publication or deployment was performed by this documentation audit.

## Final Shared-State Reconciliation

The intermediate absence and contradictory dashboard assertions described above were subsequently resolved while preserving the new dashboard metadata, compound, readiness and original mode defaults. The README now documents the required editor launches and their shared canonical dashboard owner. The neutral documentation schema and independent regex check positively require those instructions; the dashboard launch case verifies target/mode parity. See launch-coexistence.md for observed RED/GREEN evidence and concurrent reversals.

The coordinator's final complete configured course diagnostic exited zero with **13 files, 603 tests passed**, including deployment, release-tag, launch and all four dashboard/documentation tests. Its final current-source TypeScript diagnostic also exited zero with **zero diagnostics, 3,176 reads**. Logs are `🗑️generated/course-unit-coexistence.log` and `current-source-typecheck-coexistence.log`. These supersede this report's narrower earlier 36-test observation without adding overlapping totals. The course runtime release remains unverified pending native admission.
