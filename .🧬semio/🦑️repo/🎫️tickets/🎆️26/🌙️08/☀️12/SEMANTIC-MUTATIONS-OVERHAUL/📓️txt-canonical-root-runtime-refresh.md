# TXT Canonical Root Runtime Refresh

The ticket-local `🦀️actual-source.rs` harness mounts the actual TXT snapshot, diff,
mutation-support, and canonical aggregate sources. It contains no copied production
implementation. The exact source inputs before the attempted compile were:

- `🧬️mutations/🦀️.rs`: `eb5a8ce98775955618a32f00ef017a5aed4aba140791588bd2884c1157fcd309`
- `📦️glue.rs`: `668fc633ca88aaf39de8939209572ec96e336c84e4e7212d6d1b6fc94992c89b`

The fresh artifact inputs were present as both `rlib` and `rmeta`; the compiler uses
metadata for source checking. Selected metadata SHA-256 values:

- kernel: `886fe9be3e25b86a9c066ce88b2ce5512b94a04cab6c48fa3c39d0969cc8a7f4`
- schema: `f1e59248090a6e09cf31039025235030e94a6227a0db52e0590837a1b8ccff3a`
- serde: `d115e862ef4359ed0f9cccb22dee96f65da89b6550c380b9a36ae8e924601702`
- serde_json: `daf83952e6de838d9a7bb23ed0e157ed377a75c6e24b6fdfabc9ad23d480442c`

`rustc --test --emit=metadata` correctly reaches the actual canonical source but stops
with E0460: `semio_framework_schema-529c308cc4e44832` was built against a different
`semio_framework_os_kernel` metadata identity than the supplied fresh kernel pair.
This is an artifact-coherence blocker, not a source failure. Rebuild schema after its
matching kernel pair is selected, then the same harness can compile and run the canonical
roster and codec tests.

The compiler named the incompatible input paths in its diagnostic: the supplied
`libsemio_framework_os_kernel.rmeta` and `libsemio_framework_os_kernel.rlib` versus
`libsemio_framework_schema-529c308cc4e44832.rmeta`. A source SHA-256 recheck after the
attempt matched both recorded source hashes above.

## Feature-Coherent Actual Source Runtime

The full STDIO test compilation supplied the coherent replacement set: unhashed kernel
metadata SHA-256 `886fe9be3e25b86a9c066ce88b2ce5512b94a04cab6c48fa3c39d0969cc8a7f4`,
schema `d2283d0c57ab378a` metadata SHA-256
`73ebf956d57a891ff41cddd33f0647ada1078ad7b80b9c8383cd3fc43c584995`, and
serde_json `0caf27179e7b9139` metadata SHA-256
`381a296946558179d2ffd1f03289a0f10ba08f928284dec3c84692be44d334de`.

With those exact `rmeta` and matching `rlib` pairs, the ticket-local actual-source
harness compiled and ran successfully. It mounts the real snapshot, diff,
mutation-support, and aggregate sources; applies all five real `TxtMutation` variants
with non-error outcomes, checks the final CRLF snapshot, and confirms the exact five-kind
semantic roster. Runtime output:

```text
[DEBUG] actual TXT canonical aggregate applied all roster identities
```

The post-runtime source SHA-256 values still match the two source hashes recorded above.

The same source harness was then compiled with `--test`, the exact
`semio_framework_async_macros-4945efd0a40a2b35` proc macro, and the feature-coherent
`rmeta`/`rlib` pairs. It listed 34 tests and executed all 34 successfully (`0 failed`,
`0 ignored`). The roster includes aggregate exactness; all five leaf metadata,
provenance, semantic identity, inverse, and codec paths; the generic five-frame binary
tag/parse/walk/decode test; malformed text/binary framing; and TXT snapshot/diff laws.

## Regenerated Replay Artifacts

I mistakenly removed the two generated executables after the first successful runs. They
were rebuilt from the unchanged harness and retained; they are regenerated outputs, not
the original binary artifacts. Independent replay paths are:

- `🧪️txt-canonical-root-runtime/🧪️actual-source`
- `🧪️txt-canonical-root-runtime/🧪️actual-source-tests`
- `🧪️txt-canonical-root-runtime/📓️actual-source-tests-rebuild.log`

The rebuild log contains the 34-test roster, run result, and post-rebuild source and
selected dependency fingerprints. The source hashes match the values recorded before the
mistaken removal and before this regeneration.

The runtime source was compiled against the earlier coherent kernel/schema metadata
fingerprints recorded above. A concurrent root build changed those mutable target files
after the regenerated test run (the retained log records its later observed fingerprints),
so the regenerated executables are retained for replay but must not be presented as a
fresh recompilation against the later target state. The production source fingerprints
remained unchanged across both observations.

The independent AST and schema command passed:

```text
[DEBUG] canonical root AST/schema/vector gates passed operations=5
```

That gate validates more than schema compilation. It loads the file-kind and payload
location identities from the current taxonomy, validates the five-case neutral matrix
against a strict matrix schema, and asserts unique ordered tags 1–5. For every leaf it
checks the taxonomy-derived primary files, absence of the two former owner directories,
the exact fourteen-field descriptor object (including forward-slash provenance owner),
the direct aggregate Rust mount, exact TypeScript leaf interfaces/imports/discriminated
root union, GraphQL input AST, and proto message AST. Ajv accepts each representative valid payload and rejects its invalid
counterpart, then validates the aggregate one-of after resolving every payload reference.
The root GraphQL enum/input and proto imports/oneof are compared exactly, and the binary
protocol is checked for `field tag u8` plus the five-frame runtime test declaration.

## Semantic-Parity Gaps

Canonical layout validation is not complete language-semantic parity. The refreshed
neutral matrix retains three adversarial cases and the gate confirms the relevant JSON
Schema result through Ajv plus the declared GraphQL/proto field roots:

- `insert-line.index = 2147483648` is accepted by JSON Schema and the proto `uint64`,
  but GraphQL's `Int!` is 32-bit.
- `set-line-ending.value = "cr"` is rejected by JSON Schema and the TypeScript literal
  union, but the GraphQL and proto declarations both use unrestricted strings.
- an injected `set-line` payload field is rejected by JSON Schema and GraphQL's input
  envelope, while proto unknown-field handling and structural TypeScript assignment do
  not provide equivalent rejection.

The monorepo initially had no GraphQL/protobuf parser. A ticket-isolated oracle install
now supplies `graphql` 16.11.0 and `protobufjs` 7.5.4 without changing monorepo package
or lock files. The refreshed verifier uses their real parser APIs for the SDL/proto AST
checks and their actual runtime validation for all three adversarial payloads. The
ticket-local `🟦️semantic-cases.ts` is type-checked by the
installed TypeScript compiler: it accepts the large index and variable-bound malicious
envelope, and emits its sole expected diagnostic for the invalid line-ending literal.
No production repair is proposed while these distinct semantic contracts remain
unresolved.
