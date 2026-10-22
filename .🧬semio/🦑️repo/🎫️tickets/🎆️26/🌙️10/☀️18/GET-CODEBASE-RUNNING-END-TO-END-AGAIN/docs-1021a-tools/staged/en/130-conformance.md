### 📐️ The conformance suite

One language-agnostic suite judges every build backend with the same scenarios, so that a swap cannot silently lose a capability. {{?conformance::::The command below belongs to the staged set. }}It is level 1 and runs in seconds to minutes; the real-repository harnesses `repo incremental`, `repo verify build-parity` and `repo concurrency` stay level 2. State: {{@conformance}}.

```bash
repo conformance list
repo conformance plan --scenario=<id>
repo conformance run --implementations=semio
repo conformance run --implementations=nx,bazel,semio --file=<report> --json
```

- **Workspace.** 15 stub projects (a diamond, a cut-off pair, multi-output, failing, failing test, slow, single-flight, three parallel). The same data becomes an Nx workspace, a Bazel workspace or the neutral graph of the in-house engine. Ground truth for "executed" is a marker file written by a stub task body, never the backend's own record.
- **17 capabilities in 20 scenarios**: plan, run (cold, ordered, lazy closure), cache restore (also after deleting outputs), invalidation, early cutoff, uncached tasks, nested outputs, failure isolation (a failure is never cached), test results, affected, cancellation (stops within 5 s, nothing published, the next run completes), progress, parallelism (bound and overlap), concurrent invocations, single flight, remote cache (fresh checkout, empty local cache, shared server) and run record. Each scenario cites the existing evidence it generalises (incremental scenario ids, parity sample ids, concurrency workloads).
- **Verdicts** combine the registry claim with the measurement: `pass`, `gap` (claimed missing and failing: documented), `regression` (claimed and failing), `stale-claim` (claimed missing but passing), `unobserved` (the binding cannot observe it: counted apart, never a pass) and `error`. The judge has an independent Python oracle; sensitivity tests show that a backend that forgets its cache, lies about exit codes, ignores cancellation or serialises work fails exactly the scenarios that should catch it.

| Capability | nx | bazel | semio |
|---|---|---|---|
| 18 of the 20 scenarios (plan, run, cache restore, invalidation, early cutoff, uncached, outputs, failure isolation, test results, affected, cancellation, parallelism, concurrent invocations, remote cache, run record) | pass | pass | pass |
| single flight: two simultaneous invocations execute the body once | gap, as declared (Nx processes do not coordinate) | pass | pass |
| progress: a start and an end per executed task | unobserved (Nx publishes no per-task events) | unobserved | pass |

Facts the runs established: unmodified Nx also gets early cutoff when dependents declare `dependentTasksOutputFiles`; a Bazel action tagged `local` can never use the remote cache, so a backend adapter that tags tasks `local` loses sharing; Bazel keeps its products outside the workspace, so every binding must copy them back, read-only files included. The in-house engine passes all 20 scenarios through its driver and through the real CLI (`repo conformance run --implementations=semio` and `repo --tool=semio tools setup semio`); real Nx 23.2.0 and real Bazel 8.4.2 give 19 of 20 satisfied plus the one gap or unobserved row above. Not run: the scenarios through `repo --tool=semio run` of a real repository target (the graph build waited in the capacity queue), and the level-2 harnesses with `--implementations=semio`. Procedure after the apply: `repo --tool=semio run <cheap project:target>` twice (the second must restore), then `--dry-run`, `affected` and `cache gc --area=semio-actions`.
