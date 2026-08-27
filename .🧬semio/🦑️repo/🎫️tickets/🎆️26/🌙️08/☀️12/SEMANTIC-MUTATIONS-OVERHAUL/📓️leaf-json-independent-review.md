# Leaf JSON Independent Review

## Accepted Private Boundary

FND-LEAF-JSON-08 is independently accepted only for the private full-descriptor parser and typed emitter. Public leaf/aggregate derive, mandatory protocol metadata, registry propagation, and production owner conversion remain separate work.

The coordinator ran the registered derive crate twice in `🧪️derive-contract-target`: both invocations exited0 with5 tests passing and0 skipped. The second materialized the dev-only serde derive dependency needed to compile unchanged metadata types. Logs: `🧪️leaf-json-registered.log` and `🧪️leaf-json-serde-registered.log`. Registered parser coverage includes72 neutral byte-level cases; the other tests cover existing source authority and strict attributes.

The coordinator then ran the ticket harness through an explicitly scoped workspace Nx invocation. It exited0 and passed72 actual-parser cases, emitted10 Rust values, compiled each value against the unchanged production descriptor region, and executed all fourteen serialized-field comparisons. Required null fields, all metadata enum values, integer spellings, duplicate raw keys, malformed/trailing JSON, and owner mismatch are represented. Adversarial client `Some`/`None` names do not capture generated code.

Final transcript: `🧪️leaf-json/🧪️root-final.log`. Exact parser/token sources, dependency inputs, compile/runtime logs: `🧪️leaf-json/🧫️run-1787821730933`. The concatenated derive/core source SHA-256 before and after was `8be118d06ea86b916ec83d919bf499449dca40dc8d0497934aa9978a7913a3bf`.

## Corrected Invocation Failures

The split Rust artifacts require both repeated `--extern name=<same-hash>.rlib` and `--extern name=<same-hash>.rmeta`, with their dependency directory. Choosing only one format produced opposite metadata/linkage diagnostics. These failures do not prove source incompatibility.

The executor's first successful wrapper omitted the Nx project selector and repeatedly ran the harness across projects. Its exact owned wrapper and child processes were cancelled after coordinator diagnosis; no other worker was targeted and all evidence remains. The independent acceptance run used `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun <ticket-harness>` and completed once with exit0.

## Source Release

The coordinator released the OS metadata source for FND-LOWER-METADATA-10 after this replay completed and the separate PDF gate had produced fresh protocol/kernel metadata. Moving the defining source invalidates the harness's hardcoded extraction location for future runs; subsequent verification must use the new canonical lower owner, without retaining duplicate production types.
