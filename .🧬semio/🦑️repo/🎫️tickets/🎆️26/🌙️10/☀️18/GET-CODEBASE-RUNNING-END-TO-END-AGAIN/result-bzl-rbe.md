# result-bzl-rbe: local Buildbarn remote execution proof (executor bzl-rbe)

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`. Tools and staged set: `C/bzl-rbe-tools/`.

## Status: PREPARED, waiting for GO (strict phase rule: nothing outside `C/` written, no docker daemon started, no image pulled)

### Choice of the remote execution service
Buildbarn (bb-deployments `docker-compose`), official images only (`ghcr.io/buildbarn/bb-{storage,scheduler,worker,runner-installer}` and `docker.io/library/ubuntu:22.04` as the runner image instead of the upstream sample's third-party `catthehacker/ubuntu`). All five carry linux/arm64 and linux/amd64 manifests (checked against the registries' manifest lists, anonymous read-only API calls, nothing pulled). Every image is pinned by tag AND index digest:

| image | tag | index digest |
| --- | --- | --- |
| bb-storage | 20260906T092937Z-ae61334 | sha256:05760070ed56...02c7 |
| bb-scheduler | 20261008T101024Z-582d07f | sha256:3e61fd882470...b491 |
| bb-worker | 20261008T101024Z-582d07f | sha256:9188ee7ba21f...05c7 |
| bb-runner-installer | 20261008T101024Z-582d07f | sha256:50f1936a10d5...c7 |
| ubuntu | 22.04 | sha256:0a9887931b0e...8956 |

Topology (one storage shard instead of upstream's two, no portal/postgres, no FUSE): `frontend` (gRPC 8980 = CAS + action cache + execution, diagnostics `/-/healthy` on 8981, both published on 127.0.0.1 only because the cluster has no authentication), `storage` (CAS 6 GiB, AC 128 MiB, block-device files, no persistence), `scheduler`, `worker` (hardlinking build directories, no privileged mode, so it runs on Docker Desktop for Mac), `runner` (ubuntu, `network_mode: none`: hermetic actions get a real network namespace without network), `runner-installer`, `init` (one-shot: creates the storage and worker directories on named volumes). The configs are plain JSON (a Jsonnet superset; `bb_*` evaluates every config path as Jsonnet), one worker file per platform with exactly the contract properties (`OSFamily=linux`, `Arch=arm64|amd64`) and instance `main`. Field names were checked against the protos of the pinned commits (`key_location_map_on_block_device` exists at the storage commit, was folded away only on main).

### Staged set (`C/bzl-rbe-tools/`)
- `src/` authored sources, `targets.py` (src -> live path), `edits.py` (11 anchored edits), `apply.sh [--dry-run] [--allow-drift]` (all-or-nothing, drift check against `base-hashes.json`, never overwrites other content), `stamp.py`, `mkoverlay.py` (verification copy in `/private/tmp/rb`, copies only).
- New files (18): service schema `B/🏃️execution/🧬️schema/🐳️remote-execution-service.json`, descriptor `B/🏃️execution/🐳️service/🔣️.json` (images, ports, platforms), module `B/🏃️execution/🐳️service/🟦️.ts` (runtime interface, docker compose implementation, `serve|status|env`), stack `B/🏃️execution/🐳️service/🐳️stack/{compose.yaml,config/*.json}`, test + fixture under `🧪️tests`, hermetic probe script `B/🏃️execution/🧪️tests/🔬️probes/🟦️.ts`.
- Edits: operation `remote-execution serve|status|env` in the operations catalogue (EN+DE, flags `--platform`, `--port`, `--diagnostics-port`), the script branch in `B/📜️script.ts`, two dashboard tools in the root `📋️project.json` (`build-remote-execution-status`, `build-remote-execution-serve`, continuous with a ready probe), five hermetic probe targets in `L/⚡️caching/📋️project.json`, and `SEMIO_REMOTE_EXECUTION_INSTANCE` (+ `defaultInstance: main`) in the Bazel execution contract schema.
- `apply.sh --dry-run` against the live tree: "dry run ok: 18 file(s)".

### Verification done before GO (overlay copies in `/private/tmp/rb`, one process at a time)
- `bun test ./<overlay>/.../🐳️service/🧪️tests/🟦️.ts`: 11 pass, 94 expects (schema vs Ajv, compose anchors vs descriptor vs Python, `docker compose config --format json` as third-party resolver of the stack: images, loopback-only ports, `network_mode: none`, worker properties equal the execution contract per platform, rendering rows, refusal rows, container listing parser, serve/cancel/teardown and failure paths with a runtime double, status, ASCII runtime copy).
- `tsc` (strict, noUncheckedIndexedAccess) over module, test and probe: 0 errors.
- Probe script run locally for all five kinds (`platform`, `descriptor`, `compose`, `fanout`, `chain`): outputs as expected.

## Open (after GO)
Start Docker, pull, `remote-execution serve`, point `SEMIO_REMOTE_EXECUTION_*` (and the cache contract) at it, run the probe targets with `bun repo --tool=bazel run`, prove remote execution and a second-client cache hit, fix the adapter, README EN+DE, teardown.
