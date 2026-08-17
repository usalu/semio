# glTF Mutation Transport Taxonomy Readiness

## Lease

Central taxonomy/discovery readiness only. No glTF mutation leaf, mutation schema, fixture, generated artifact, or `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` path was edited. The protected repo-library TypeScript index remained untouched.

## Implemented Contract

- `🚪️io/🧬️mutations` is an I/O semantic collection with the explicit `transport` direction.
- `ioSemanticCollectionDirNames` is the exact, nonempty registry of I/O semantic collection directories: `💡️inferences` and `🧬️mutations`. The old singleton `ioInferenceCollectionDirName` is rejected.
- `SemanticIoDirection`, `SemanticCollectionSpec`, and `SemanticMember` now model `transport` alongside `import` and `export`.
- `🚪️io/🧬️mutations/{📝️text,💾️binary}` are declared artifact representation leaves. The former `🧬️schema/🧬️mutations/{📝️text,💾️binary}` paths are explicitly invalid and have no normative-spec mapping.
- `artifactSpecFilenames` and `artifactSchemaSpecFilenames` require the grammar, protocol, and JSON schema at the new I/O boundary paths.
- A semantic census fixture proves two bidirectional mutation codecs pass only with `io.direction: "transport"`; changing one member to `import` yields `io-contract-missing`.

## Exact Handoff

Let `G` be `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any`.

The mutation SCC source lease must atomically move these complete representation components:

- `G/🧬️schema/🧬️mutations/📝️text` → `G/🚪️io/🧬️mutations/📝️text`
- `G/🧬️schema/🧬️mutations/💾️binary` → `G/🚪️io/🧬️mutations/💾️binary`

The destination collection manifest must be `G/🚪️io/🧬️mutations/🔣️component.json` with exactly these members:

```json
{
  "x-semio": {
    "kind": "collection",
    "members": [
      {
        "directory": "💾️binary",
        "id": "s.stdio.gltf.mutation.io.binary",
        "kind": "io",
        "responsibility": "Deterministic frozen-tag glTF mutation binary transport.",
        "io": { "format": "s.stdio.gltf.mutation.binary", "direction": "transport" }
      },
      {
        "directory": "📝️text",
        "id": "s.stdio.gltf.mutation.io.text",
        "kind": "io",
        "responsibility": "Deterministic glTF mutation text transport.",
        "io": { "format": "s.stdio.gltf.mutation.text", "direction": "transport" }
      }
    ]
  }
}
```

Both canonical Rust/TypeScript leaves and every language/schema representation file move with their respective codec. The source lease must update all source-level imports to the owned I/O boundary and delete the empty former schema codec directories; it must not retain a forwarding export.

After the source owner reports its referrer sweep and manifest/tree check, submit one central registrar request for `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`: remove the two former `schema::mutations::{text,binary}` mounts and add exactly one `io::mutations::{text,binary}` mount each. The registrar must make no compatibility alias and must be sequenced with the aggregate mutation-dispatch mount change in the SCC packet.

## Validation

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --test-name-pattern 'declares canonical semantic collection and module ownership contracts|accepts mutation codecs only as explicit bidirectional I/O transport boundaries'` | Pass: 2 tests, 15 expectations. |
| `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` | Completed: 27 components, 64 errors, 0 warnings. The 64 findings are the already assigned glTF mutation SCC/root work; no transport taxonomy finding was added. |
| `git diff --check -- <central lease paths>` | Pass. |
| `bun nx run @semio-tech/repo-lib:check --skip-nx-cache` | Not run as a quality gate: Nx reports no `check` target for this project. |

The broader focused taxonomy/census run has one unrelated failure: `reports a completeness dir missing from the structural set` still removes `📡️spr`, but the concurrently current artifact component/child vocabularies no longer contain that retired directory. The run otherwise passed 32 tests, including the two new transport assertions; this lease intentionally did not alter that unrelated stale test.
