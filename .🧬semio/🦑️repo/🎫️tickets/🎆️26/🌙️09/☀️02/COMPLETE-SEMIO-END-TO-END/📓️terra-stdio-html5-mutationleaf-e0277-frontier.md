# Stdio HTML5 `MutationLeaf` E0277 Frontier

## Verdict

The `E0277` class is a deliberately useful **leaf-derive closure** failure.  It
is neither a reason to remove `MutationLeaf` from `MutationKind`, nor evidence
that a generated implementation should be added by a blanket impl.  The
current HTML5 owner is source-closed: every one of its nine direct payloads
derives `dsl::MutationLeaf`, declares the same `::protocol` contract, and
implements the `MutationKind<HtmlSnapshot, HtmlMutation>` required by the
aggregate.

The native-eight compiler fingerprint itself was reported to this audit lane,
but its raw diagnostic was not present in the current ticket-generated Native8
directory.  Therefore this report does **not** claim a current native pass.
It does establish that an HTML5 `MutationLeaf` E0277 persisting against the
bytes read here would have to be preceded by a failed derive/source-authority
diagnostic or come from a different compilation snapshot.

## Current source evidence

| Boundary | Evidence | Result |
| --- | --- | --- |
| Public trait contract | [`command.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs#L215) makes `MutationLeaf + Clone + ToValue + FromValue` supertraits of `MutationKind`. | This is the intended expansion; do not weaken it. |
| Individual derive | [`derive.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs#L735) resolves a sidecar descriptor and emits the only `MutationLeaf` impl at lines 762–765. | No separately generated Rust impl exists or should exist. |
| Aggregate closure | [`derive.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs#L1897) requires one direct leaf type per enum variant, then explicitly references it as both `MutationKind` and `MutationLeaf` at lines 1914–1915 and validates semantic/descriptor/source agreement at lines 1927–1935. | A failed leaf derive naturally cascades into E0277 at the aggregate/leaf impl; it is not a missing manual implementation. |
| HTML5 aggregate | [`mutations/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs#L27) mounts exactly nine direct owner modules; lines 47–70 declare exactly nine wrapped variants; line 77 has the matching nine kinds. | Source-closed. |
| HTML5 leaves | Each leaf's lines 8–16 carries `ToValue`, `FromValue`, `dsl::MutationLeaf`, `#[mutation_leaf(contract = ::protocol)]`, and the typed `MutationKind` impl: [`set-snapshot`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs#L8), [`set-doctype`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜set-doctype/🦀️.rs#L8), [`insert-node`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕insert-node/🦀️.rs#L8), [`remove-node`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-node/🦀️.rs#L8), [`set-element-name`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-element-name/🦀️.rs#L8), [`set-attribute`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖set-attribute/🦀️.rs#L8), [`set-text`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✍️set-text/🦀️.rs#L8), [`set-comment`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬set-comment/🦀️.rs#L8), [`set-raw-text`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⌨️set-raw-text/🦀️.rs#L8). | Nine of nine satisfy the required shape. |

`protocol` is not a distinct competing trait in this crate: the Stdio crate
aliases `semio_framework_os_kernel` as both `dsl` and `protocol` in
[`🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs#L21).  Thus the aggregate's hard-coded kernel reference and the leaf's
`contract = ::protocol` resolve to the same trait identity.

A current static census found 809 Stdio source files implementing
`protocol::MutationKind`; all 809 contain both the leaf derive and the explicit
contract attribute.  It also found 105 `dsl::Mutations` aggregates in Stdio and
185 across first-party plugins.  This is a source-token census, not a native
result, but it rules out a broad current Stdio missing-derive packet.

## Taxonomy and generated-surface result

The HTML5 payload taxonomy is coherent:

- The nine leaf sidecars have exact physical `owner`, `semanticKind`, and
  `aggregateVariant` values.  The text and binary siblings are representation
  facets, not payload leaves; in particular [`📝️text/🔣️.json`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🔣️.json#L1) deliberately lacks leaf metadata and must stay out of the roster.
- The cross-language oracle catalog lists the same nine kinds at
  [`🔮️oracle/🔣️.json`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🔮️oracle/🔣️.json#L92); its nine manifest rows agree with the nine aggregate variants.
- The aggregate's native kind law constructs all nine variants and checks their
  text-kind order at [`mutations/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs#L679).

There is one bounded documentation drift: the standalone adapter still calls
the nine-kind loop a “10-kind” sweep at
[`mutate-html-5/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️tests/🟠️mutate-html-5/🦀️.rs#L10), lines 317–318, and lines 324–326.  The actual loop is over the correct
nine-element `KINDS` at lines 26 and 330–337.  Correct those claims with the
next HTML5 test-source update, but do not add a phantom tenth leaf or restore
the prohibited `NoMutation` variant.

## Smallest clean repair / validation packet

### P0 — diagnose the exact compiler snapshot before touching production

1. Preserve `MutationKind`'s four supertraits and leave both derive macros
   unchanged.  Do not write blanket `MutationLeaf` impls and do not generate
   leaf Rust files from descriptors.
2. On the actual Native8 fingerprint, retain the *first preceding diagnostic*
   for the affected type.  Classify it as (a) sidecar/source-authority failure,
   (b) missing direct derive/contract attribute, or (c) one of `Clone`,
   `ToValue`, or `FromValue` genuinely missing from its payload.  Only then
   repair that concrete owner.  An E0277 after a derive expansion failure is
   secondary.
3. For a real leaf gap, update its source and its existing sibling
   `🔣️.json` together, preserving physical owner path, semantic kebab case,
   aggregate variant, and the declared non-serde value codec.  This is a
   source-owned leaf packet, not a framework compatibility change.

### P1 — make the HTML5 closure independently observable

Add one schema-first neutral roster fixture below the existing owner, for
example `…/🧬️schema/🧬️mutations/🪪️leaf-roster/{🧬️schema,🧫️fixture}/🔣️.json`.
It must contain exactly these nine rows and no representation facet:

`set-snapshot`, `set-doctype`, `insert-node`, `remove-node`,
`set-element-name`, `set-attribute`, `set-text`, `set-comment`,
`set-raw-text`.

Each row must pin owner path, semantic kind, aggregate variant, payload schema,
and required Rust surface.  Include hostile rows for a descriptor owner with a
stale emoji/path, a mismatched aggregate variant, duplicate semantic kind,
missing leaf, inclusion of `📝️text`, and a missing/extra kind.  A Node/AJV
reader must read raw JSON sidecars independently of Rust.  The Rust law must
read the real `<HtmlMutation as Mutation<HtmlSnapshot>>::DESCRIPTORS` roster,
the nine `MutationKind::SEMANTICS` values, and real `OpText`/`OpBinary`
round-trips; it must not parse the Rust source or merely duplicate `KINDS`.

The present native kind law is useful but does not expose the descriptor roster
to a neutral oracle.  The present direct fixture mount is only `set-snapshot`
([`mutations/🦀️.rs`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs#L715)); retain it, but do not call it proof of every leaf.

### P2 — registered proof

Extend only the existing Stdio Rust router
[`📜️script.ts`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts#L466), its matching
[`📋️project.json`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📋️project.json#L23), and the ordered
`.vscode` launch seed with an `html5-mutation-leaf-check` source/native pair.
The source half runs the AJV/Node roster oracle.  The native half must use
`runExactCargoLaws` with `{ package: "semio-s-plugin-stdio", target: { kind:
"lib" } }`, exact-discover the one new roster law, and require exactly one
selected / one passed assertion.  It must use the ticket-generated target and
must not be folded into a broad Stdio lib test.

Suggested later commands, once registered (not run by this audit):

```sh
bun ./📜️script.ts nx run @semio-tech/stdio-plugin:html5-mutation-leaf-check --skip-nx-cache -- --oracle-only
bun ./📜️script.ts nx run @semio-tech/stdio-plugin:html5-mutation-leaf-check --skip-nx-cache
```

## Scope and ownership

The P0/P1 owner is the Stdio HTML5 mutation owner.  The derive/framework owner
only acts if an actual, current fingerprint proves a macro expansion defect;
current evidence does not.  A missing leaf elsewhere should be repaired by its
own artifact owner with the same packet, not through an HTML-specific or global
compatibility layer.  No native gate was run for this report.
