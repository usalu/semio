# Container Capacity Boundary Audit

Read-only audit, 2026-10-30. No container, compiler, setup, network or graph operation was launched; no funding, limits, estimates, environment authority or ledger was changed.

## Established Boundary

The completed private dependency receipt is genuine provisioning acceptance (409493, 0/null at 06:02:09.460Z), independently qualified by private pinned Ajv and unchanged host endpoints. It does not by itself establish one shared host-capacity reservation domain.

The retained `native-private-dependencies-current.log` explicitly records outer registered Docker admission at 2048 MiB, then inner dependencies admission requesting 6144 MiB while reporting zero running grants. Retained process metadata at 05:51:14 identifies host issuer 409493 → podman client 409868, and container frontdoor host PID 409887 under a different parent 409884. The container workload is not a normal descendant of the client process in that snapshot. These are the exact observed facts; no historical inner grant identifier or namespace translation was recovered.

The retained `run-container-command.cjs` calls registered `repo tools exec docker -- exec` with explicit language, workspace, artifact directory and working directory. It does not pass a capacity reference or connect a funding broker. The Docker registry row at `process/🧰️tools/🔣️.json` declares ordinary `command/service` admission. It has no Docker-exec-specific funding rule or inner command request contract. Its declared adapter is container lifecycle configuration, not an execution funding adapter.

Generic `process/🧰️tools/🏃️run/🟦️.ts:154` preserves literal argv and merges the host grant into the environment of the client process. It does not translate that environment into Docker exec arguments. The admission interface at line 34 carries tool, operation, kind and label, with no container identity or declared inner operation. Therefore this actual route does not transfer the host reservation into the inner repository process. Docker/Podman exec does not implicitly inherit the client's environment.

## Existing Ledger Contract

`process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts:16` defaults to a per-user cache ledger (Linux: XDG cache or home/.cache/semio/host-capacity). Its inheritance branch at lines 176–181 requires the actual grant file to be current in that ledger; a plan in an environment alone is insufficient. Grant liveness uses local process identity, boot anchor and heartbeat.

The existing handover interface under `gate/🤝️handover/🟦️.ts` is for replacing a command: it authenticates an offer token, the actual parent process identity, operation, current grant and one-time takeover under the same ledger. It is not a container namespace delegation contract. No container/namespace bridge was found in the inspected tool, container lifecycle, capacity gate or devcontainer configuration paths.

The current devcontainer configuration does not declare capacity environment forwarding or a host ledger mount. The exact private container's complete initial mount configuration was not retained in the inspected evidence, so its ledger's physical backing cannot be proved merely from this configuration. The observed inner zero-grant view and actual argv are enough to establish the missing delegation on the retained command route, not the exact private mount topology.

Passing a host grant identifier or mounting the host ledger would not repair this safely: local PID liveness inside another PID namespace could classify a live host grant as dead, and the gate removes stale rows. A carried plan also cannot authorize 6144 MiB against an outer 2048 MiB reservation. No such workaround is proposed.

## Smallest Meaningful Next Work

The proven gap is the registered container execution boundary: it models only the client command and has no owned operation that reserves the actual inner declared work in the host domain, retains that reservation through actual container completion, and relates the engine's real workload identity to host accounting.

A repair should add that explicit owned boundary through the existing tool-independent container interface and a production schema contract, then use normal host admission for the actual inner request. Guest admission must consume or broker the held host reservation through an authenticated namespace-aware port; it must not trust a new environment claim. Cancellation, joined container completion and accounting of the engine-owned workload are part of that same lifetime.

Causal neutral tests belong beside the existing tool admission runner tests and host-capacity gate/handover tests. Required cases: plain argv grants only the client; insufficient outer reservation refuses larger inner work; unqualified ledger/namespace references refuse; host and guest PID numbers may differ; completion/cancellation settles exactly the owned workload and reservation; duplicate consumption fails. Use independent SQLite accounting or the existing Python capacity oracle for the same neutral rows. A schema-only or mocked environment green would not establish real cross-namespace acceptance.

Discovery ownership remains retained, with no rerun until the complete authorizer Source checkpoint.
