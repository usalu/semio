# Retained Command Child Close Packet

## Outcome

The generic artifact completion handoff now returns the exact submitted Emit, EphemeralEmit, and rejection Fault owner when admission is busy or duplicate. Framework retained-command and full-operation jobs preserve that rejection owner, never replay the handler, and close nested child emits before releasing their enclosing result.

This packet fixes the prepublication child-owner loss boundary. It does not claim deep retirement of arbitrary app-defined mutation/effect/event/task/presence/transient fields; that broader RED frontier is documented separately in `📓️terra-artifact-tool-completion-owner-returning-rejection-census.md`.

## Contract and implementation

- `ArtifactToolCompletionRejection<A>` is the owner-returning completion error.
- `Emit::close_child_one` is public so concrete artifact jobs can use the same bounded child authority.
- `ChildEmit::close_one` provides the owned nested close primitive inside the plugin SDK.
- Retained command close invokes child retirement before parent result and lane release.
- Language-neutral fixture and strict schema pin child identifiers, byte ownership, retirement order, zero-item behavior, and duplicate/busy owner return.
- The native law uses reserved raw vector capacity and child bytes to prove zero-item preservation, byte-bounded progress, LIFO child close, no handler replay, and exact duplicate/busy ownership.

## Verification

| Check | Result |
|---|---|
| Root tool-job source self-tests | GREEN, 100 checks |
| Plugin runner/admission/completion oracles | GREEN, 6 / 15 / 10 cases |
| Pinned nightly Rust parser | GREEN |
| Scoped diff hygiene | GREEN |
| Exact native child-close law | Pending isolated exact-Cargo execution |
| Plugin-registry generated launch freshness | GREEN |

## Executable surfaces

- Nx target: `@semio-tech/framework-plugin:retained-child-close-check`.
- The target is registered in `.vscode/🧩️launch.seed.jsonc`; `.vscode/launch.json` was regenerated from that source.

