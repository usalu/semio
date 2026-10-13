# Modular Monorepo Builds And Shared Caches

## Objective

Refactor shared build infrastructure and architecture so growth does not multiply graph construction, source scanning, cache invalidation, compiler work, or mutable output collisions. Every compiler invocation owns a prototype or release policy. Prototype optimizes iteration; release optimizes execution and artifact size. Keep Bun, Nx, canonical script entrypoints, existing resource admission, and cooperative same-checkout editing.

## Coordination

Maximum available fleet: coordinator plus three agents. Initial exploration uses GPT 6.1 Sol low reasoning (the available equivalent of requested Light); execution uses GPT 6.1 Sol high reasoning. The coordinator cannot change its own host-selected model/reasoning through the tools; no claim is made that its setting changed. Repo MCP rejects the current model identifier, so ticket metadata uses its generic codex identifier and this report records the actual requested fleet. Repo MCP was invoked through its installed stdio binary; repo://goals and repo://tickets were read before ticket_open. Existing build tickets cover narrower tasks and do not cover this refactor.

## Architecture Decisions

1. Own build-mode vocabulary and tool optimization policy in the domain-neutral process module, with JSON schema and language-agnostic test vectors. Admit only prototype and release; remove dev/ship mode aliases from actual policies and invocations.
2. Infer mode configurations in Nx before hashing and executing tasks. Compiler leaves inherit the selected mode from orchestration. Profile-specific outputs and cache identities must not replay prototype artifacts for release or vice versa.
3. Preserve existing immutable tool caches, profile-scoped Cargo leases, staging/publication leases, resource admission, cancellation, and graph seals. Extend their missing boundaries rather than create parallel infrastructure.
4. Partition mutable compiler stores by owner, platform, toolchain, and build mode. Shared immutable artifacts remain reusable across concurrent agents. Prevent all-core allocation by independent invocations from multiplying host resource demand.
5. Modularize command routing and source/hash ownership so unrelated product or audit changes do not slow compiler startup or invalidate native/web outputs.
6. Validate actual execution, warm-cache reuse, mode separation, cancellation, and concurrent invocations; record measured evidence and unverified platform limits. Preserve unrelated edits.

## Workstreams

- Coordinator: schema-first build policy, thin shared helpers, vocabulary migration and integration, launch commands, end-to-end acceptance.
- Native execution: Cargo native/WASM profiles and propagation, CMake all-core/mode/lease ownership, .NET and Python build contracts.
- Nx execution: mode-aware inferred task/output/input graph, cache policy, bootstrap routing, configuration inheritance, graph/cache behavioral tests.
- Web execution: Vite mode-sensitive optimization and artifacts, shared cache ownership, TypeScript incremental state ownership, prototype/release packaging.

## Acceptance

- No accepted compiler/build mode other than prototype or release; no accidental implicit production optimization in prototype.
- Warm builds reuse both task artifacts and compiler intermediates; unrelated source changes do not invalidate scoped tasks.
- Concurrent same-checkout builds avoid corrupting outputs or incremental stores and honor shared CPU/memory admission.
- New permanent commands are registered in launch.json and route through Nx and canonical scripts.
- Language-agnostic fixtures are checked by owned implementation and third-party validators/oracles.
- Required checks have real runtime logs, with temporary logs prefixed [DEBUG] and stored below the ticket generated directory.
- Reports remain in the ticket; generated command outputs are removed when the ticket closes. The goal remains active until all required work is achieved.
