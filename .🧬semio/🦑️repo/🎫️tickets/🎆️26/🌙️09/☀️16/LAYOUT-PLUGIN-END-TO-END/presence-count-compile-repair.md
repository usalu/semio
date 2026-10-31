# Presence Count Compile Boundary

## Actual RED And Current Source

Full Layout original74693 is terminal EXIT1/inner101 before any selected test binary ran. Its frozen compiler input reported four errors from presence-round-trip-law39/44: E0599 on peer_presence.len() and two cascading E0308 comparisons. The current Source already has `.len` FIELD at both exact assertions when this repair task first read it. Current helper SHA256 is68520ee8644a5ffd0ad1e4a268b52abc795dc4d5edc9cf9518f8ab4ad3f9f1c0. This packet made NO Source edit and does not claim authorship of that peer correction. No stale code was restored and no cosmetic patch was manufactured.

## Defining Semantic Count

Canonical type is declaration/view/peer-presence/🦀️.rs:36–99, PeerPresenceRoot. It owns fixed `[Option<Arc<PeerPresenceEntry>>; PEER_PRESENCE_SLOTS]` storage and public `len:usize`. Empty root initializes len0; insert shifts the active prefix and increments len; iter reads entries[..len]; is_empty compares len0. The count is the active logical roster size. Using entries.len() would read fixed capacity64 and fail the one-peer/departure law.

The same type exposes a len() method ONLY under its own #[cfg(test)]. Compiling semio-framework-plugin-testing-laws or a downstream Layout cfg(test) binary does not enable cfg(test) for the declaration-view dependency. Thus the stale method calls cannot work through this boundary; the current public len field is the defining canonical observation. No compatibility method, extra feature, trait bound, visibility widening or production API is required.

Current assertions still require exactly1 live metadata entry after admission and0 after the empty departure roster, alongside the existing typed-presence/source-actor/document-unchanged checks. No assertion or helper body semantics was removed.

## Bounded Related Review

All current Rust files under plugin/testing were inspected for peer_presence.len usage; only these two field accesses occur. Dependency owner independently reviewed the actual shared artifact-app-laws helper files and agrees there is no additional stale count call there. Existing pages Vec/slice len calls refer to their real containers and were not changed.

Outside this current testing-laws owner, old plugin/runtime contract part3 retained-operation-close-and-checkpoint still contains live root len() calls270/277, and part5 operation-action-stack-and-peer-presence contains live len()136 plus captured.len()135. Those roots are not the selected Layout15 test binaries and were not edited or presented as compiler-verified by this bounded repair. Their other APIs/mounts require owning-context review before any broader change. No unrelated old test cleanup is included.

## Verification Options, Not Launches

A minimal normal compile verification for this exact shared dependency is:

```text
repo --service=off cargo --store=shared --configuration=prototype -- check --manifest-path /Users/ueli/Documents/semio/Cargo.toml -p semio-framework-plugin-testing-laws --lib
```

That is an OPTION only, not executed and not a test pass. The package manifest workspace resolves to the actual repository-root Cargo.toml. Root may instead release the same full Layout15 request directly to reuse its actual compiled dependencies and continue integration. Either choice must use the genuine current Source/store/toolchain/capacity owners and preserve one native lane. A focused check cannot replace the full mandatory Layout15 runtime, private test roots, codec/media records/oracles or the remaining full-goal gates.

No Cargo, setup, GC, preview, cache edit, cap/deadline/profile change or peer interruption was launched by this task. Root owns the next release decision.
