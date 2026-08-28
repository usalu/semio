# Flow VCS Direct-Leaf Root Review

## Verified Source Boundary

Root read all ten direct Rust payloads and their aggregate, the ordered structural diff and its application helpers, the mounted native tests, the actual Flow package dependency declaration, and the source/neutral controller. The leaf descriptors are now also validated against the actual authoritative descriptor schema rather than only comparing selected fields.

Root strengthened the controller to capture the exact bytes it parses, reject repeat-read drift and symlink inputs, use portable relative paths, and re-read all captured inputs before success. The immutable original baseline remains retained. Each direct leaf and aggregate now has explicit source regions. The native tests now explicitly retire owned inverse/diff values where the earlier helper left them to ordinary drop.

The shared integer decoder dependency is separately native-verified in [its report](./📓️dsl-checked-integers-41.md). Flow itself was not compiled or executed.

## Schema-Adjacent Structural Diff

The review found the new ordered FlowDiff lacked a language-neutral schema. Root first added fifteen valid/invalid JSON contract vectors and an actual-file schema check. [The retained red run](./🧪️flow-vcs-direct-41/🧫️run-jXGSAr/🔣️result.json) passed275of291checks; all sixteen failures were the absent diff schema and its dependent vector checks.

The new schema and Rust source now sit together at `🌊️flow/🌿️vcs/🧬️schema/🔺️diff/{🔣️.json,🦀️.rs}`. The VCS root reexports the same FlowDelta/FlowDiff types. The implementation remains an ordered structural sequence, not a vector of mutation operations. The schema covers widget/synapse indexed fragments, layout assignments and explicit fixture import, including exact u32 bounds and closed envelopes.

The unchanged expectations plus the source mount/native-vector joins passed **295of295** in [the final root run](./🧪️flow-vcs-direct-41/🧫️run-N2sp3r/🔣️result.json), with every captured input stable. The actual Ajv schema gate, independent jsonc-parser edit oracle and source assertions are distinguished from Rust execution. The added native diff JSON contract test is authored but unexecuted; the packet now has eighteen new native tests including ten leaf-owned tests.

## Explicitly Unfinished Runtime Ownership

No Flow native acceptance is claimed. Existing collection replacement/removal can drop an owned Widget, partial serde failure can abandon an already-built ordered root, and the synchronous diff path still uses existing cold disposal. Those are real ownership questions, not fixed by the schema or by test-only cleanup.

Demonstrator explicitly handed off the exact FlowMutationRetirement struct/impl/factory for a test-first repair after root observed that it ignored maximumBytes and cleared the whole Option. The original full-retirement baseline is not overwritten. [The handoff capture](./🧪️flow-vcs-direct-41/🧫️mutation-retirement-handoff.json) records the exact authorized preimage and separate unchanged adjacent prefix/suffix hashes. The controller now protects FlowFixture/FlowSnapshot retirement and member-store registration while allowing only that authorized mutation-retirement interval to change. Its new writer uses actual FlowOwner/FlowRetirement primitives and is required to preserve ownership on refusal/fault and report Complete only at terminal-empty.

Plugin coherence and compiler slots remain separately coordinated. No Flow build, all-plugin publication, shared primitive edit, cleanup or evidence restoration occurred in this root review. Detection remains honestly apply-only: the existing detector does not prove reorder/camera/schema coverage, and its layout-removal ordering is still a separate issue. Full monorepo mutation acceptance remains open.
