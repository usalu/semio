# Lower Mutation Contract Audit

## Ownership and Mandatory API

The read-only Luna audit located the actual base `Mutation<P>` contract in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`. The OS command module already reexports that lower contract. The complete fourteen-field descriptor, its five enums, and const validation belong beside the base trait; adding a reverse dependency from replication to OS is forbidden. `SemanticDescriptor` remains OS authoring vocabulary and is also used by inference.

The coordinator freezes the metadata API shape as follows. This is the required end state, not an assertion that the live traits already implement it.

```rust
pub trait MutationLeaf {
    const DESCRIPTOR: MutationLeafDescriptor;
    const PROVENANCE: MutationSourceProvenance;
}

pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: MutationDiff<P>;
    const DESCRIPTORS: &'static [MutationLeafDescriptor];
    fn descriptor(&self) -> &'static MutationLeafDescriptor;
}
```

The omitted behavior methods remain on `Mutation`. Neither metadata item receives a default. `MutationKind` and `CompositeMutationKind` inherit `MutationLeaf` and retain explicit `SEMANTICS`; there is no second independently declared `DESCRIPTOR`. Derives qualify the declaring trait as `<Payload as MutationLeaf>::DESCRIPTOR`. The coordinator's compiler prototype proved omission errors for both required aggregate items and the inherited-associated-constant qualification error. That prototype is not an actual production-trait test.

## Generic and Runtime Boundaries

The qualified audit counted194 base-mutation implementation lines in163 files, with no actual `dyn Mutation`, `dyn MutationKind`, or `dyn CompositeMutationKind` consumer. These are bounded lexical counts, not an exhaustive acceptance census.

`SetArtifactMutation<D>` in `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs` is a genuine generic ownership problem: it spans document owners and payload schemas. Its `commit_document<D>` consumer cannot receive an invented generic leaf identity. Specialize concrete document operations under their true owners, or retain only a non-`Mutation` generic helper below the public mutation/codec boundary.

The generic `CollectionMutation` helper is an internal diff/inverse engine, not permission for public semantic collection variants. `FlowMutation` must expose explicit widget/synapse Add/Remove/Move/Patch leaves. Config, presence, transient, session, and infrastructure operations remain in scope. `NormPresenceMutation::Noop` is absence, not a leaf; a config snapshot inverse is not automatically an allowed replacement operation. Public codec-registered `ProbeMutation` requires a leaf. `CreateBuildingStorey` is a composite planning leaf and need not become a base aggregate operation.

## Source Proof

The leaf derive must validate its actual compiler source, governing workspace/taxonomy, direct owner, and full descriptor. Aggregate expansion needs its own root-source validator; the private leaf-source validator cannot be reused unchanged for an aggregate. The aggregate must require matching leaf owner, mutation root, descriptor identity, and physical workspace identity, then validate roster uniqueness and variant correspondence.

Repo-relative paths alone do not distinguish two physical workspaces containing identical relative trees. Source provenance therefore needs a separate opaque build-time workspace identity, excluded from the fourteen serialized descriptor fields and from runtime registry identity. Absolute host paths must not leak into serialized descriptor metadata.

A proc macro cannot recover arbitrary referenced types' declaration spans through stable type introspection. Semantic aliases to a correctly derived same-root payload may preserve provenance; aliases to another owner/workspace must fail. Handwritten provenance literals can compile, so independent source policy must reject manual `MutationLeaf` implementations rather than treating trait conformance as source proof.

Required tests include same-workspace wrong-root rejection, same-relative-path foreign-workspace rejection, valid and invalid semantic aliases, missing metadata, and explicit compiler-pass/policy-fail evidence for forged manual implementations. Optional child-facet payload declarations require proven public ownership from the mandatory direct source; historical triads are not exemptions.

No production trait was changed by this audit. Actual `compose/**` was not accessed.

## Workspace Token Freeze

The read-only digest audit and coordinator source inspection confirmed the existing repository-owned `Sha256::digest(&[u8]) -> [u8; 32]` in `🧰️framework/🔨️modules/#⃣hash/🦀️component.rs`. Its package has no OS/derive dependency. Reuse it through the hash package from the proc-macro crate; do not copy its algorithm or use `DefaultHasher`. That package's existing Blake3 dependency is not used by this SHA-256 operation, and generated clients receive only constant bytes, not a new hashing dependency.

`MutationSourceProvenance` has `workspace_token: [u8; 32]` and static repo-relative `mutation_root`, `owner`, `source_path`, `descriptor_path`, and `taxonomy_path` fields. It is source/build metadata, not a serializable runtime descriptor, and has no default. The token input is domain-separated with UTF-8 `semio.mutation-source-provenance/v1` followed by NUL, then two length-prefixed byte strings: the canonical physical workspace-root spelling and normalized repository-relative taxonomy locator. Each length is an eight-byte big-endian unsigned integer. Native path separators are converted to `/` after safe canonicalization; path case is not guessed or rewritten. All unsafe/excluded/symlink paths must be rejected before this computation.

Changing workspace location changes only compile-time provenance, never mutation wire identity or runtime registry fingerprints. The implementation must verify same-root aliases and distinct physical workspaces with a compiler fixture, and compare digest output against a standard-library/third-party SHA-256 oracle. Reusing the actual hash source maintains one algorithm implementation.

## Public Metadata Derive Freeze

Use the existing derive crate and DSL facade, with a metadata-only `#[derive(dsl::MutationLeaf)]` and exactly one required `#[mutation_leaf(contract = ::protocol)]` argument. Existing clients may name the genuine OS facade instead. The contract is an absolute Rust module path without generic arguments; missing, repeated, relative, or unknown arguments fail. Owner/file/schema/source overrides and behavior attributes are forbidden. There is no default contract path.

The caller-supplied path selects owned public trait/type names only; all descriptor values and provenance come from validated compiler-source authority and the sibling JSON. Existing kernel clients need no new direct replication dependency solely for this macro. The emitter takes the path explicitly, and the OS DSL facade reexports the macro. Preserve payload generics and where clauses without imposing `T: 'static` merely for metadata. Generic metadata spanning multiple real owners must still be specialized.

Accept structs and enums as visible payload declarations; reject unions. Compile fixtures cover the genuine lower path, existing facade path, borrowed generics, strict attribute errors, and lookalike fake contracts failing the actual aggregate trait boundary. Source policy independently validates aliases and rejects manual implementations; syntax alone cannot prove a path's package identity. This metadata-only entry point precedes, but does not replace, the mandatory aggregate and registry transaction.
