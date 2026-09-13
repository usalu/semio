# Hub Foundations Extraction

## Outcome

The first bounded Hub foundation slice now has ten anonymous TypeScript implementation owners rather than semantic bodies in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`. The package command imports those owners directly and retains routing only for this slice. The graph contains 47 declared owner bindings, 7 internal edges, 33 exact package-root imports, 26 taxonomy contexts, and no owner-to-command back edge or cycle.

The extracted behavior covers Cargo stage environments, shared fixture expectations, local-bootstrap framing/authentication/process lifecycle/credential issuance, direct-child credential delivery, ordered directory publication verification, current WGPU/MCP credential-source proofs, and the route-only source-test command. `GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES` remains in the existing Hub inference schema owner and is imported rather than copied.

## Exact Owners

1. `🌎️hub/🏗️build/🛂staging-environment/🟦️.ts`
2. `🌎️hub/🧪️tests/🧬️schema/🛂expectation/🟦️.ts`
3. `🌎️hub/🚀️local-bootstrap/📡️framing/🟦️.ts`
4. `🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts`
5. `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`
6. `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts`
7. `🌎️hub/🔐️auth/📤️credential-delivery/🟦️.ts`
8. `🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts`
9. `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts`
10. `🌎️hub/🧪️tests/🧱️foundation-source/🏃️execution/🟦️.ts`

The test-only installed Rust parser oracle is the anonymous leaf `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🔮️oracles/🦀️.rs`. It is mounted only under `#[cfg(test)]` by the Hub library and uses the Hub package's `syn` dev dependency.

The seven internal owner edges are authentication → framing; execution → framing and authentication; credential issuance → framing, authentication, and execution by type; and credential delivery → framing. The fixture records every edge and exact root-import subset.

## Context And Source Contract

The 26 exact context identifiers are:

1. `hub-build`
2. `hub-build-staging-environment`
3. `hub-tests`
4. `hub-tests-schema`
5. `hub-fixture-expectation`
6. `hub-local-bootstrap`
7. `hub-local-bootstrap-framing`
8. `hub-local-bootstrap-authentication`
9. `hub-local-bootstrap-execution`
10. `hub-local-bootstrap-credential-issuance`
11. `hub-auth`
12. `hub-auth-credential-delivery`
13. `hub-directory`
14. `hub-directory-publication`
15. `hub-directory-publication-tests`
16. `hub-directory-ordered-append-broadcast`
17. `hub-foundation-source-test`
18. `hub-foundation-source-execution`
19. `hub-auth-tests`
20. `hub-auth-credential-source-order`
21. `hub-inference`
22. `hub-inference-schema`
23. `hub-schema`
24. `hub-foundation-source-schema`
25. `hub-fixtures`
26. `hub-foundation-source-fixture`

The schema/fixture/test authorities are:

- `🌎️hub/🧬️schema/🧱️foundation-source/🔣️.json`
- `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json`
- `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`

The fixture records 27 exact Nx inputs. They include all ten owners, the Rust oracle, GIS bound owner, package router/project/Cargo/library mount, root lockfile, taxonomy, launch seed/derived files, local-bootstrap/auth/directory schemas and fixtures, the directory Rust source, repository process-routing owners, and the exact current WGPU and MCP source populations.

The command registration is `foundation-source-check`; the Nx target is `os-hub:foundation-source-check`; the seed and derived launch entry is `⚖️gate🧱️hub-foundations📐️source`, group `4_gate`, order `411.10755`. Both launch catalogs contain exactly one matching record.

## Behavior Evidence

The portable contract proves:

- the fixture against draft-07 through Ajv and independently through `jsonc-parser`;
- all 47 exported declarations, the seven exact internal edges, TypeScript semantic diagnostics, the acyclic graph, 26 owner-to-full-ancestry bindings, no command back edge, all 33 exact root imports, and absence of the moved declarations from the package router;
- canonical HMAC bytes against Node HMAC and Web Crypto, split-frame reads, bounded writes, and zero/oversize/malformed/deadline/EOF/outstanding-read rejection;
- injected private run-root allocation, pure readiness classification, idempotent finish, and secret zeroing without starting a Hub;
- protected inherited-environment removal, native/MCP stdio selection, fd-3 delivery, client-class rejection, injected spawn rejection, failure termination, and capability erasure on success and rejection;
- all four ordered-publication fixture decisions and five source hostiles while retaining append/fanout under one writer guard;
- the current WGPU `RunScript.run → runNativeSession → runNativeBinary → runTool` source chain and its one credential-bearing Rust `main`;
- the one current MCP runner and actual `main` call order, while allowing only the pure `schemas` return preflight before credential claim;
- TypeScript AST parity for comments, strings, nested braces, and declaration-only syntax; and lexical handling of Rust raw strings, comments, nested blocks, and declarations;
- default/override Cargo stage environments and the inference-owned 256-byte GIS control-frame bound.

Executed results:

- Schema-first absent-owner phase: 1 pass, 1 fail, 8 assertions. The first semantic owner was absent.
- Materialized before registration: 7 pass, 3 fail, 184 assertions. Taxonomy, the stale MCP predicate, and route registration remained red.
- Ordinary package command `bun ./📜️script.ts foundation-source-check`: final 11 pass, 0 fail, 259 assertions, 3.05 seconds in the final direct run.
- Actual isolated Nx route `bun nx run os-hub:foundation-source-check --skip-nx-cache`: final 11 pass, 0 fail, 259 assertions; target 3.9 seconds, cache skipped.
- `loadCatalogTaxonomy()` on the current concurrent tree returned successfully with 651 semantic directory kinds.
- The exact permanent Rust law `oracle::hub_credential_source_order_syn_parity`, included from the anonymous oracle by a ticket-private minimal Cargo harness and pinned to the workspace-lock `syn` 2.0.117, passed 1 test, 0 failed, 0 ignored, 0 filtered; 0.01-second test body. It independently parsed the hostile fixture and current MCP/WGPU Rust entrypoints.
- Prettier reported the extracted TypeScript/schema/fixture files formatted; `rustfmt --edition 2021` formatted the Rust oracle.

The final Nx attempt initially emitted no-space warnings while writing its private project-graph cache, continued without that cache, executed the target, and reported success. After completed private Nx directories were removed, a fresh final isolated run completed without the no-space warning and produced the final 11/259 result above.

## Native Limits

The ordinary Hub Cargo selection was attempted offline and locked. It stopped in the unrelated current plugin window-config dependency before `semio-hub` or this law compiled: one `E0308` (`String` versus `Option`) and three `E0616` private-field errors in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`. Therefore this report does not claim the three planned existing local-bootstrap Rust laws or a normal Hub-package Cargo test. The minimal `syn` harness executed the exact permanent oracle source, while the normal `#[cfg(test)]` Hub mount remains statically and input-bound.

Although a private Cargo target coordinate was requested, repository Cargo configuration reported the minimal harness binary under the existing repository Cargo build cache. No shared Cargo cache was reset or cleaned.

No Hub binary, browser, database, listener, native WGPU child, MCP child, service, Git mutation, live workspace operation, or security journey ran. Process lifecycle and credential-child behavior in the portable suite use injected boundaries. The source-order checks establish exact syntax/source relationships, not native security or runtime delivery.

## Exact Attribution

Created implementation/test authorities:

- `🌎️hub/🏗️build/🛂staging-environment/🟦️.ts`
- `🌎️hub/🧪️tests/🧬️schema/🛂expectation/🟦️.ts`
- `🌎️hub/🚀️local-bootstrap/📡️framing/🟦️.ts`
- `🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts`
- `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`
- `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts`
- `🌎️hub/🔐️auth/📤️credential-delivery/🟦️.ts`
- `🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts`
- `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts`
- `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🔮️oracles/🦀️.rs`
- `🌎️hub/🧪️tests/🧱️foundation-source/🏃️execution/🟦️.ts`
- `🌎️hub/🧬️schema/🧱️foundation-source/🔣️.json`
- `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json`
- `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`

Updated product/registration files, limited to this lane's named changes:

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — removed/rebound the bounded foundation statements and registered the route.
- `🌎️hub/💡️inference/🧬️schema/🟦️.ts` — exported the existing 256-byte GIS control-frame authority.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — added the exact named input and target.
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml` — added test-only `syn` with `full` and `visit`.
- `🌎️hub/📦️packages/🦀️rust/🦀️.rs` — mounted the oracle under `#[cfg(test)]`.
- `Cargo.lock` — added only `syn 2.0.117` to the `semio-hub` dependency row; concurrent font/package changes in this file are not attributed here.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — added the 26 Hub contexts only; concurrent Flow/configuration edits are not attributed here.
- `.vscode/🧩️launch.seed.jsonc` — added only the foundation source launch record.
- `.vscode/launch.json` — added only its derived launch record; concurrent launch changes are not attributed here.

Retained reports created by this lane:

- `📓️sol-hub-foundations-owner-plan-2026-09-13.md`
- `📓️sol-hub-foundations-extraction-2026-09-13.md`

All disposable files under `🗑️generated/sol-hub-foundations-extraction` are removed after retaining this report.
