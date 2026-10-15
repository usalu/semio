# Loaded Toolkit Root Propagation

The real public Nx exec planner refused before entering the ticket planner: `Original loaded daemon toolkit root required`. The terminal failure is retained at `🗑️generated/final-all-bazel-plan-public.log` (session 45645). SDK preflight authenticated generation `db49db21d1298457ae5b0278faf7968b87a5e84eb21eeafca6d79686e24e8f77` before this refusal. This is a production environment propagation defect, not a planner result or a graph publication failure.

`NxScript.runWithPreparedToolingStore` already acquires a live SDK witness for `dirname(tooling.modulePath)`. Its child environment supplied `NODE_PATH: tooling.modulePath` and the immutable identity, but omitted `SEMIO_NX_TOOLING_ROOT`. The daemon startup's service-birth guard correctly requires that loaded physical root and subsequently checks exact immutable SDK custody. An inherited environment variable could mask the omission; the regular clean public path cannot rely on it.

The single production correction adds `SEMIO_NX_TOOLING_ROOT: dirname(tooling.modulePath)` beside `NODE_PATH` inside the existing witness body. The actual loaded value replaces any ambient value. The SDK identity, graph digest, original invocation wire, cancellation, deadline and awaited progress are unchanged. No service guard was weakened and no new API, fallback or clock was introduced. An immediate reversible string edit preserved all other source bytes.

This bootstrap source is part of graph Source authority. The accepted ALL16 pin stays immutable and historically accepted, but current-source qualification must run genuinely again; this patch does not bless reuse as current authority. The graph module's separate nine-file fingerprint is not a substitute for the changed Source facts.

The ticket-only `--nx-bootstrap-gate` selects the existing dependency test file with `loaded Nx toolkit root` under the unchanged authentic 120-second SDK/Nx gate. High owns focused test execution and the harmless public exec witness. No runtime was launched by Core for this fix. High actual focused law passed: 1 test / 9 assertions / 321 ms. The actual harmless public exec then exited 1 (session 93589), past the old root guard, with only `Failed to process project graph` and no child receipt. This is not a public exec success. Root released one same-control verbose diagnostic to identify the next cause; no speculative fix or graph retry is authorized by this result.

Current bootstrap SHA-256: `fb409ade30c9fcb96fb153144578aadd4357868a83cd08a8827506a1a66b297c`.

Low reviewed the exact production bootstrap SHA above as clean. Focused evidence: `🗑️generated/nx-loaded-root-public-green.log` is a historically named log containing the actual public failure, not proof of GREEN runtime.

Final actual public route: after the separately documented private Nx dependency projection and implementation-identity corrections, High harmless exec session 18466 exited 0 under the original 120-second envelope. Exact evidence and qualification limits are in `nx-implementation-current.md`; the earlier generic project-graph failures remain failed attempts, not relabeled successes. Root owns current ALL17 qualification.
