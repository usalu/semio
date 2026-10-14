# Test Bootstrap Readiness Neutral Law Audit

Read-only new owning law audit, 2026-10-14. No test/native dispatch or producer edits. Actual owner `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; discovery-boundaries test124–140 and fixture bootstrapOrder were inspected while producer243 still lacked await.

The law is deterministic for the actual missing boundary: it copies the actual plugin source and uses its actual createNodesV2, preserving policy/taxonomy and feature discovery. Wrapper ready starts false and changes only through the exported libraryBootstrap thenable's then method; original real bootstrap promise may already be resolved but cannot spontaneously set wrapper readiness. Proxy intercepts the actual first relativeScriptInputs call and refuses if wrapper readiness was not received. Thus unchanged producer causes the canonical missing-await error, while importing and awaiting the exact exported bootstrap establishes ready before inputs. It does not rely on timer ordering, slow disk or an arbitrary delay. Revision-specific wrapper module import hashes its exact source with SHA256, matching actual plugin module identity, so event observation reads the same instance.

Independent Node oracle separately awaits a thenable and emits ready/inputs, compared to the same closed fixture expectation. It validates language ordering independently; it is not a Node runtime proof of the whole copied plugin. Actual copied plugin remains the first-party tested body, so this is stronger than checking a source substring or running only a mock helper.

Concrete type correction sent root: `type Vector` at11 does not yet declare bootstrapOrder but law136–137 reads fixture.bootstrapOrder. Add existing fixture type's readonly bootstrapOrder with readonly expected string sequence; otherwise strict TypeScript inclusion produces property error before the intended runtime RED. Fixture schemaVersion assertion alone is not schema validation for this new contract; an existing independent schema may be extended if one owns this fixture.

Prior resident-source wrapper92 exports only cacheInternals; matching dependency Rust inputs mock94 similarly omits readiness. Reexport real libraryBootstrap beside cacheInternals when updating actual boundary so these fixtures carry the same contract. Awaiting undefined itself resolves, so omission does not automatically cause a failure; it simply fails to model the readiness required by real owner. New explicit thenable law correctly prevents that false success.

No graph-cache or newer native-metadata diagnosis inferred. Prior graph-missing failures remain separate from this deterministic parser readiness boundary, and Root owns subsequent registered before/after execution.

Fresh follow-up readback: Vector line11 already includes `bootstrapOrder:Readonly<{expected:readonly string[]}>`; prior missing-property finding described an earlier capture and is resolved. No further type edit is required.
