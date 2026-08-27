# PDF 1.7/E Direct Mutation Cutover

## Scope

- Root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🧬️schema/🧬️mutations`
- Catalog/test glue: the sibling PDF/E oracle and `mutate-pdf-1-7-e` case only.
- The old fourteen-case aggregate became twelve concrete semantic operations after removing `NoMutation` and generic `SetSnapshot`.
- No Cargo or Nx runtime target was started; the root coordinator owns the serialized shared-crate run.

## Final Roster

`insert-encryption-dictionary`, `remove-encryption-dictionary`, `insert-javascript-action`, `remove-javascript-action`, `insert-launch-action`, `remove-launch-action`, `insert-media-annotation`, `remove-media-annotation`, `set-output-intent`, `remove-output-intent`, `embed-font-file`, `remove-font-file`.

Each direct owner contains `🦀️component.rs`, schema-valid `🔣️component.json`, `🔣️payload.schema.json`, TypeScript, GraphQL, protobuf, text, and binary facets. Binary tags are the exact contiguous table 0–11. The root contains 103 files: 96 direct-owner files and seven transparent aggregate/registry/schema surface files.

## Executed Evidence

- Independent repository policy query: PDF 1.7/E prefix = **0 findings across 17 mutation policy classes**.
- Ajv draft-07 validation: `descriptors=12 payloads=12 surfaces=84 errors=0`.
- Exact surface/catalog parity: `leaves=12 rust=12 ts=12 graphql=12 protobuf=12 jsonSchema=12 text=12 binary=12 oracle=12 tags=0..11 errors=0`.
- Nightly parser: `E Rust=48 adapter=1 errors=0`.
- Bun TypeScript import parse: root plus 12 direct imports clean.
- Language-neutral feature parity: `feature rows=24 distinctKinds=12`.
- Sentinel/fallback/nested-owner scan: clean.
- Scoped `git diff --check`: clean.
- `[DEBUG]` scan: clean.

## Commands

```sh
bun -e '<Ajv descriptor, payload-schema, owner, and required-surface validation>'
bun -e '<folder/variant/root-surface/oracle/tag parity validation>'
find '<PDF/E root>' -type f -name '🦀️component.rs' -print | while read file; do rustc +nightly -Z parse-crate-root-only --edition 2021 --crate-type lib "$file"; done
bun -e 'await import("./<PDF/E mutation root>/🟦️component.ts")'
git diff --check -- '<PDF/E subset>' '<mutate-pdf-1-7-e case>'
rg -n 'NoMutation|SetSnapshot|no-mutation|set-snapshot|unclassified|\[DEBUG\]' '<PDF/E subset>' '<mutate-pdf-1-7-e case>'
```

## Runtime Hold

The structural and parser cutover is complete. Behavioral Cargo/oracle execution remains deliberately deferred to the coordinator's combined STDIO run so concurrent owners do not repeatedly rebuild or fail the shared crate while other roots are in flight.

## Canonical Codec Reachability Follow-Up

The E root now explicitly mounts `📝️text/🦀️component.rs` and `💾️binary/🦀️component.rs` exactly once. Both physical targets exist. The shared E/H mount-only check independently parsed both roots and all four codecs: `roots=2 canonicalMounts=4 parserFiles=6 errors=[]`. No Cargo runtime was launched. Exact commands are in `📓️pdf-1-7-ua-validation-commands.md`.
