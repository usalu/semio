# PDF 1.7/H Direct Mutation Cutover

## Scope and Result

The PDF 1.7/H mutation root now contains 87 files: 80 files under ten concrete direct owners and seven transparent root aggregate/registry/schema surface files. The old twelve-case aggregate lost `NoMutation` and generic `SetSnapshot`.

Final roster: `set-info-title`, `set-info-author`, `insert-javascript-action`, `remove-javascript-action`, `insert-launch-action`, `remove-launch-action`, `insert-signature-field`, `remove-signature-field`, `embed-font-file`, `remove-font-file`.

Each direct owner contains Rust behavior/tests, a mutation descriptor, `🔣️payload.schema.json`, TypeScript, GraphQL, protobuf, text, and binary facets. The exact binary identity table is 0–9.

## Executed Evidence

- Shared STDIO library check immediately before the mounted H cutover: exit 0, zero errors.
- Ajv: `descriptors=10 payloads=10 surfaces=70 errors=0`.
- Surface/catalog parity: `leaves=10 rust=10 ts=10 graphql=10 protobuf=10 jsonSchema=10 text=10 binary=10 oracle=10 tags=0..9 errors=0`.
- Nightly parser: `H Rust=42 adapter=1 errors=0`.
- Bun TypeScript import parse: root plus ten direct imports clean.
- Language-neutral feature parity: `feature rows=20 distinctKinds=10`.
- Scoped `git diff --check`, `[DEBUG]`, sentinel, fallback, nested-owner, and classification hygiene: clean.
- No new Cargo/Nx runtime command was started after the cutover.

## Independent Policy Gate Status

The initial independent query was blocked before evaluation by unrelated taxonomy generator-contract drift. The coordinator subsequently reran the exact existence-checked query successfully: PDF 1.7/H returned zero violations across all 17 structural policy classes. Transcript: `🧪️accepted-roots-existence-checked-policy.log`.

## Canonical Codec Reachability Follow-Up

The root now explicitly mounts `📝️text/🦀️component.rs` and `💾️binary/🦀️component.rs` exactly once. Both physical files exist. The E/H mount-only check independently parsed both roots and their four codecs: `roots=2 canonicalMounts=4 parserFiles=6 errors=[]`. No Cargo runtime was launched. Exact commands are in `📓️pdf-1-7-ua-validation-commands.md`.
