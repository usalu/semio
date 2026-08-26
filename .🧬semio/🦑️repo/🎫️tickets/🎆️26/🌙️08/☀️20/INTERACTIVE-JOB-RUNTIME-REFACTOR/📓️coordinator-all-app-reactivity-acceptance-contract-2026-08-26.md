# All-App Reactivity Acceptance Contract

Date: 2026-08-26

## Scope

The Interactivity-First Refactor is complete only when every production app surface uses the shared worker scheduler for every interaction that can exceed one bounded semantic grant. No app, plugin, framework surface, import/export path, renderer, solver, pack decoder, mutation replay, or browser bridge is exempt because it predates the scheduler.

This contract strengthens the phase plan with the developer's explicit all-app requirement. Source/static acceptance alone never proves end-to-end completion.

## Required interaction shape

Every expensive interaction is decomposed into schema-first request, event, page, and control features with owned primitive identifiers and byte payloads. A feature must provide:

- fixed operation, item, byte, page, output, event, and control capacities;
- admission and exact-page preflight before producer allocation or copy;
- one bounded semantic unit per grant, with fuel and deadline checks;
- bounded latest-wins progress, checkpoint, and preview channels;
- lossless retained terminal pages with explicit acknowledgement;
- cancellation before and after every ownership transfer;
- generation, operation, base-revision, parent, and handle validation immediately before publication;
- retry, handle-loss recovery, interrupted incremental close, and idempotent terminal-empty closure;
- exact handback of every retained page, item, byte, event, control, child, GPU, listener, and host resource;
- production reachability from the real app entry point, with the former monolithic path unreachable.

A whole JSON/DSL/pack decode, whole snapshot clone, generic diff/apply, direct store mutation, unbounded collection/join, synchronous geometry or GPU batch, promise/callback object in a domain API, or compatibility wrapper that merely hides a monolithic call is a failing interaction.

## Test-driven evidence

Every feature needs a language-neutral schema fixture and deterministic ledger that cover empty, single, maximum, and maximum-plus-one input. Hostile fixtures cover malformed and omitted fields, stale/wrong/duplicate/ABA/exhausted handles, insufficient fuel, expired deadline, repeated rejected controls followed by a valid control, callback interruption, panic/fault, cancellation at every transfer, and interrupted close.

At least one test-only third-party oracle must produce the same semantic result as the owned implementation through an owned test interface. The oracle cannot become a runtime dependency or leak a third-party type through a public API. Native and Wasm ledgers must be byte-identical.

## All-app end-to-end gate

The final serialized matrix must discover production app entries rather than rely on a hand-maintained allowlist. For every discovered app it must prove:

1. the real app can start through its registered `launch.json` command;
2. representative expensive operations remain responsive under the 8 ms ceiling;
3. progress becomes visible before completion;
4. cancellation stops further publication and releases retained resources;
5. replacement and short connection shortage preserve the last valid view without freezing;
6. zero, maximum, maximum-plus-one, stale-generation, device-loss, and close-during-work scenarios are fail-closed;
7. terminal ledgers are empty and repeated runs are deterministic;
8. owned and third-party-oracle semantic outputs match;
9. English and German UI paths expose equivalent progress, cancellation, and accessible status semantics without a hard-coded default language;
10. desktop behavior passes first, followed by mobile and tablet surface verification where the app exposes those surfaces.

Static scans, focused wrappers, or one app's browser trace cannot satisfy this gate. The gate requires native, Wasm, replay, timing, browser, and repository-wide evidence after source work is quiescent.

## Closure rule

Phase tickets and the master ticket remain open until every discovered production app is represented in the final matrix, all monolithic interaction denials are zero, all required ledgers and oracle comparisons are green, launch registrations exist, and ticket evidence is present on disk. Missing repo ticket APIs or missing ticket metadata are closure blockers, not reasons to mark the implementation complete.
