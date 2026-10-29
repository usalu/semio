# Native Search Validation

Current Source candidate: final Go, independent-oracle, platform-compilation, and read-only review gates pass. Fresh installed-ELF validation is pending the container owner. See [native-discovery-repair.md](native-discovery-repair.md) for the final cache boundary and exact proofs; older statuses below describe earlier candidates. The old authored target waiters have been retired and have no live handles.

The Go candidate is frozen. The bounded search pipeline, both tree callers, canonical CLI, duplicated client, source-opacity boundary, and generated entity catalog passed their focused and complete native test gates.

| Gate | Terminal Result |
| --- | --- |
| Search module | 25 top-level tests passed, 6.173 s |
| Complete tree module | 39 top-level tests passed, 10.770 s |
| Complete client module, short mode | 472 root-package tests and 6 internal-package tests passed; root package 18.015 s, internals 0.002/0.003 s |
| Complete canonical CLI, short mode | 114 top-level tests passed, 1.758 s |
| Native source opacity | 28 neutral vectors and schema-derived policy behavior passed in two tests, 0.010 s |
| Independent SQLite ranking oracle | Two tests, twelve assertions passed |
| Client projection ownership | Existing neutral Ajv/projection ownership proof passed |
| Native policy projection | Exact canonical emitter freshness passed |
| Windows amd64 | Search module test binary cross-compiled successfully |
| macOS arm64 | Search module test binary cross-compiled successfully |

The complete CLI test uses an explicit bounded artifact directory owned by this ticket. An earlier direct invocation omitted that required test input; the assertion stayed intact and the corrected complete run passed. The initial opacity regression reached the native predicate and failed ten expected reserved-path vectors before implementation. Both new native checks passed after the schema-derived component matcher was added.

The client uses its existing CLI-owned entity-catalog producer, with 58 entries and source SHA256 `f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242`. No Go module, workspace, compatibility layer, or framework producer ownership change was required.

The existing pure source-owned emitters and standard generated-file writer refreshed both Go assets through registered Repo tooling. Their authored Source targets remain live and require terminal integration results; the pure refresh does not claim a Script Original invocation.

The final native rebuild, real monorepo cold/warm `repo search linux`, and authenticated installer route are delegated to the existing bootstrap owner after this frozen candidate. These integration results are not yet claimed.

A canonical overlay coverage check found two further client test packages, `internal/command` and `internal/eventstore`. Both internal packages passed their explicit native test request. The client module coverage is complete, with 472 root-package tests plus 6 internal-package tests. No new source changes were needed.

The bootstrap owner reports that the isolated container rebuilt and installed the final native ELF successfully from 969 verified current Source files with zero drift. The enclosing setup later ended with terminal one after earlyoom sent SIGTERM to its 4178 MiB captured Bun process. The installed final ELF remains available; the bootstrap owner is validating it directly without another setup or rebuild. Authenticated installer testing and cold/warm real-monorepo search are still pending. No competing container setup or installation was launched by this owner.

A read-only process/argv audit confirmed that repeated authored-target heartbeats come from the supported `runWithStaleReexecV1` wait-yield protocol: the same request ticket advances through at most three yield rounds. This is not recursive script execution. The existing requests remain in place; their terminal results are still required.

The frozen CLI accepts `--format=json search linux --limit=8`; leaf eligibility adds `--only-file`. Scoped search uses `tree monorepo linux --scope=<repository-relative-path> --limit=8 --only-file`. Cursor paging is the neutral Go `SearchRequest.After` API, covered by the Go/SQLite corpus; there is no CLI cursor flag. MCP stdio accepts newline-delimited JSON, with `initialize` protocol `2025-06-18`, an optional initialized notification, and ordinary `tools/list` or `resources/list` requests.

## Authored Target Integration Failure

The registered search oracle request ended with terminal one before reaching the oracle tests. The enforced lease-order guard reported `SEMIO_LEASE_ORDER`: one graph request held facts plus generation custody while waiting for custody admission; another held admission while waiting for an older generation. The guard selected the first request to fail and allowed its peer to proceed. This is a graph lifecycle failure, not a Go or SQLite assertion failure. No guard was disabled and no blind retry was issued. The existing client and native-policy generator requests remain live. The parent owner and read-only auditor received the exact cycle and are investigating the graph acquisition order.
