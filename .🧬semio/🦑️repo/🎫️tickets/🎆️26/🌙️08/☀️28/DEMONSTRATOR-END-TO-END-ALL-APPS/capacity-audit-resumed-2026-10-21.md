# Resumed Host Capacity Audit — 2026-10-21

Independent read-only investigation. No policy, lease, application, or foreign process mutation; no process enumeration. Source and naturally refreshed cached host receipts were inspected directly. Old terminal execution handles were not polled.

## Current Evidence

Initial filesystem snapshot at 2026-10-21T00:53:58.537Z: available16157MB of32768MB; pressure normal; swap3835MB, swap-out0MB/s, load15.13/10cores. One run grant PID79940 admitted00:53:49.547UTC, jobs4, reserved9728MB, observed not yet sampled. One cargo waiter PID80070, preferred4/minimum1, fixed1536MB+2048MB/job, arrived00:53:49.523UTC (24ms before run admission). These records do not encode parent PID, task target, ticket or cwd; independence/ancestry cannot be inferred from their timestamps alone.

Second naturally refreshed snapshot at 2026-10-21T00:54:43.310Z: available14103MB, pressure normal, swap3811MB, swap-out0, load11.68; same run grant fresh at00:54:35.853UTC observed1504MB, same cargo waiter fresh. Floor6553MB. During authored300000ms ramp, unrealized reservation=max(0,9728−1504)=8224MB. Available-bound room14103−6553−8224=−674MB; cargo minimum3584MB therefore waits. Ignoring ramp reservation would report7550MB room, but doing so would change the intentional future-growth reservation rather than correct arithmetic. After ramp, current implementation uses observed footprint instead of full reservation. This is a short-lived conservative reservation, not confirmed double-accounting.

## Source Findings

- Sampler: 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts. macOS available=(Pages free+Pages inactive+Pages speculative)×OS-reported vm_stat page size; pressure maps kern.memorystatus_vm_pressure_level1/2/4→normal/warn/critical. swap used is parsed from sysctl vm.swapusage with binary unit conversion; rate is cumulative Swapouts delta/time. Current cached sample is explicitly normal, so no present false-pressure claim is supported. No raw independent OS invocation was used, since all commands must enter repo.
- Decision: sibling 📏️decision/🟦️.ts. It deducts only reserved-minus-observed during ramp, avoiding counting the already-observed grant footprint twice in free-memory availability. Total budget separately deducts full reserve while ramping, then observed footprint; final room is min(availability,budget), not their sum. Swap above ceiling alone is not a hard refusal when rate0; positive/unknown swapping is required. No confirmed bug in these computations.
- Sizing/classification: repo library 🏗️build/🌡️host-capacity/🟦️.ts derives operation/target kind and requested jobs. Generic sizeHostRequestV1 deliberately includes build estimate as a lower bound across kinds because outer task graphs can compile prerequisites; hence test1536MB/job rises to build2048 plus fixed1536. The current run9728MB is exactly1536+4×2048. Native parallel controls can request fewer jobs through supported --parallel, but that is scheduling intent, not permission to remove the reserve or gate. A sustained historical wait is not proof that these estimates are false.
- Inheritance: 🎫️gate/🟦️.ts checks SEMIO_HOST_CAPACITY_GRANT and optional SEMIO_HOST_CAPACITY_PLAN against a current grant (pid/boot/mtime) and returns inherited handle before joining queue. Thus a correctly inherited child should not make an additional host reservation. Current receipts lack parent relation and environment, so queued cargo alongside run cannot establish lost inheritance. A deterministic regression would need a genuine owner-launched descendant whose grant env is known, not infer ancestry from adjacent PIDs.

## Confirmed Diagnostic Ingress Mismatch

Attempted only supported owner spelling from existing TS source: repo capacity status --json. Actual native repo returned immediately: unknown command "capacity" for "repo" (exit1). repo --help succeeded and omits capacity from native build command list. No raw fallback command or sampler execution was used. TS owner exists at repo library 🏗️build/🌡️host-capacity/⌨️command/🟦️.ts and is explicitly dispatched in 🏗️build/📜️script.ts. This is a concrete source/native ingress mismatch preventing an agent from obtaining a fresh official diagnostic sample through the advertised command. It does not itself create false memory pressure. If fixing it, register the existing owned command at native ingress; do not add an alternate unaudited sampler or bypass admission.

Supported status implementation refreshes sample through currentHostProbeV1 and invokes ledger liveness pruning under owner control; the read-only audit did not call its TS function directly. Supported resource-control interfaces include withHostCapacityV1 (grant/inheritance), readHostCapacityStatusV1 (owned diagnostic), createCapacityControlV1 (cancelable admission without running-task deadline), and finite requested ResourceLeaseControl downstream. No blanket environment opt-out, floor override, receipt edit or foreign cancellation is justified by findings.

## Conclusion

Present evidence shows recovered physical memory and an already-admitted real run, with one cargo waiter held by its active5min ramp reservation. No confirmed false pressure, arithmetic double accounting, stuck stale lease or lost-inheritance bug. The actionable proven defect is unavailable native capacity diagnostic ingress. High capacity owner was informed for independent implementation judgment. Source-level counterexamples or actual failing owned execution should precede any change to scheduling behavior.

## Receipt Snapshots

```json
{
  "initial": {
    "at": "2026-10-21T00:53:58.537Z",
    "sample": {
      "host": {
        "version": 1,
        "atMs": 1792544037147,
        "platform": "darwin",
        "cores": 10,
        "totalMegabytes": 32768,
        "availableMegabytes": 16157,
        "pressure": "normal",
        "swapUsedMegabytes": 3835,
        "swapOutMegabytesPerSecond": 0,
        "load": 15.13,
        "openFiles": 6193,
        "openFilesMaximum": 245760,
        "attachedImages": 10
      },
      "swappedOutMegabytes": 38637
    },
    "grants": [
      {
        "name": "HS12jBGLOP11kAUskSPc7A.json",
        "mtime": "2026-10-21T00:53:49.548Z",
        "record": {
          "id": "HS12jBGLOP11kAUskSPc7A",
          "operation": "run",
          "jobs": 4,
          "cpu": 4,
          "reservedMegabytes": 9728,
          "admittedAtMs": 1792544029547,
          "observedMegabytes": null,
          "pid": 79940,
          "bootAtMs": 1792504810000
        }
      }
    ],
    "queue": [
      {
        "name": "001792544029523-B6aSOVSwUTGG3bSLtJ9tPg.json",
        "mtime": "2026-10-21T00:53:57.935Z",
        "record": {
          "request": {
            "id": "B6aSOVSwUTGG3bSLtJ9tPg",
            "operation": "cargo",
            "priority": "batch",
            "minimumJobs": 1,
            "preferredJobs": 4,
            "fixedMegabytes": 1536,
            "perJobMegabytes": 2048,
            "cpuPerJob": 1
          },
          "arrivedAtMs": 1792544029523,
          "pid": 80070
        }
      }
    ]
  },
  "followup": {
    "at": "2026-10-21T00:54:43.310Z",
    "sample": {
      "host": {
        "version": 1,
        "atMs": 1792544081484,
        "platform": "darwin",
        "cores": 10,
        "totalMegabytes": 32768,
        "availableMegabytes": 14103,
        "pressure": "normal",
        "swapUsedMegabytes": 3811,
        "swapOutMegabytesPerSecond": 0,
        "load": 11.68,
        "openFiles": 6248,
        "openFilesMaximum": 245760,
        "attachedImages": 8
      },
      "swappedOutMegabytes": 38637
    },
    "grants": [
      {
        "name": "HS12jBGLOP11kAUskSPc7A.json",
        "mtime": "2026-10-21T00:54:35.853Z",
        "record": {
          "id": "HS12jBGLOP11kAUskSPc7A",
          "operation": "run",
          "jobs": 4,
          "cpu": 4,
          "reservedMegabytes": 9728,
          "admittedAtMs": 1792544029547,
          "observedMegabytes": 1504,
          "pid": 79940,
          "bootAtMs": 1792504810000
        }
      }
    ],
    "queue": [
      {
        "name": "001792544029523-B6aSOVSwUTGG3bSLtJ9tPg.json",
        "mtime": "2026-10-21T00:54:42.385Z",
        "record": {
          "request": {
            "id": "B6aSOVSwUTGG3bSLtJ9tPg",
            "operation": "cargo",
            "priority": "batch",
            "minimumJobs": 1,
            "preferredJobs": 4,
            "fixedMegabytes": 1536,
            "perJobMegabytes": 2048,
            "cpuPerJob": 1
          },
          "arrivedAtMs": 1792544029523,
          "pid": 80070
        }
      }
    ]
  }
}
```