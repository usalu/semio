# Exact Publication-Lane Admission

## Result

The official interactivity verifier now treats a retained reducer and its completion publication as one indivisible route proof. A route is not admitted merely because it has an app-owned factory, a bounded-first-step proof, and a `Migrated` manifest disposition.

Each `ArtifactOwnedToolJobFactory` route must additionally declare exactly one nonempty `ArtifactToolPublicationContract`. `HostOnly` is exclusive. Artifact, Config, Draft, Presence, and Transient declarations require the concrete app author surface to return `Some(...)` from the matching retained one-item preparation-factory hook. Child publication remains governed by the existing child-output and retirement authorities.

This prevents the global Store seam from falsely admitting every app route when a particular app cannot publish the lane its reducer emits.

The shared gate also covers Presence and Transient publication separately from durable Artifact/Config/Draft publication. Both ephemeral stores must retain their preparation owners, displaced roots, retry/ACK state, cancellation, and close state. An atomic root swap may not immediately `drop(previous)`, because that Arc can own an unbounded last root.

## Executable hostile laws

`bun ./📜️script.ts verify interactivity tool-jobs --self-test` covers:

- exact HostOnly acceptance;
- rejection of HostOnly mixed with a Store lane;
- rejection of Config publication without an app-owned preparation factory;
- acceptance of Config publication only with the exact preparation hook.
- rejection of immediate displaced-root destruction in either ephemeral store;
- rejection of a Transient fallback to the monolithic `apply_one` path;
- rejection of a resumed publication that advances before document revision/generation freshness is revalidated;
- acceptance and hostile mutation of the extracted production freshness guard as well as the inline form;
- rejection of per-publication factory reconstruction after the app's exact factories have been cached.

Result on 2026-08-27 after the durable, ephemeral, Drop-enforcement, root-retirement, cached-factory, freshness, and publication-contract hostile additions: `self-tests=486 clean`.

`git diff --check -- ./📜️script.ts` also completed without errors.

## Cohort consequence

The audit immediately corrected route claims: Space retains four genuinely HostOnly routes, Remodel retains two HostOnly import routes, and Shooting retains two HostOnly request routes. Their remaining routes are fail-closed where the app does not own the exact Store-lane preparation authority. Playbook's former retained view routes emitted Config without that authority and were therefore returned to fail-closed status.
