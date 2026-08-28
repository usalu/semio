# Generated Mini Registry Preparation

The ticket-local `🧪️registry-mini-generated` harness mounts the permanent source-owned Mini
document, aggregate, leaf and both descriptor schema files directly. It neither copies Mini types
nor supplies manual source provenance. The client makes the external crate root expose
`semio_framework_os_kernel::os_spr` as `crate::os_spr`, so the mounted source keeps its actual
imports; `protocol`, `serde` and the real `dsl_derive` proc macro are explicit compiler inputs.

The frozen two-case matrix is schema-first:

- `ordinary-metadata-apply-inverse` proves generated `From`, all fourteen descriptor fields,
  actual provenance facts, aggregate descriptor roster, diff application and inverse.
- `generated-registration-conflict-propagation` first registers a deliberately unequal full
  envelope with the generated schema id, then calls the actual generated registration function.
  It requires an explicit conflict and proves the established entry was not replaced.

The permanent Mini aggregate has one direct leaf, so generated conflict propagation proves no
replacement for its single atomic batch; the separate immutable-32 client retains the multi-entry
same-batch and existing-entry atomicity laws. There is no source-owned generic Mini aggregate.
Creating a ticket-only generic leaf would require invented provenance or a duplicate direct-leaf
taxonomy, so this harness deliberately does not claim a generic derive law.

`📜️script.ts` refuses defaults and requires one `SEMIO_REGISTRY_MINI_ARTIFACTS` JSON map with
absolute paths to a single dependencies directory and coherent kernel/protocol/serde rlib+rmeta
pairs plus the derive dylib. It fingerprints the complete real Mini source closure and every
supplied artifact before and after compiling, listing and running exactly two tests. No Cargo is
used by this lane.

Root released a stable completed-framework compiler slot. The retained first source compile with
the current kernel and `protocol-2294…` but `serde-7fd…` is red in
`🧪️registry-mini-generated/🧫️run-w9mfBf/`: direct source derives and the protocol trait bounds
proved those serde artifacts were not coherent. The successful coherent selection is
`protocol-2294e75bdb5f4513`, `serde-9726de5488b8f586`, and
`dsl_derive-f44e247812382dc4`, paired with kernel rlib
`1894b38a9d4a0e52ae1ca44f9747593a18580cb7cbb86794d188c7273df2ee69` and rmeta
`9bde8070560b507ca15c74ef996cb9974a27726258b50958120c95e88e14a8da`.

`🧪️registry-mini-generated/🧫️run-4UxFWF/` compiled, listed and ran the actual source closure:
2 tests passed, 0 failed, every compiler/list/runtime process exited 0 with no signal or spawn
error, and every mounted source and artifact hash was unchanged before and after the gate. An
intermediate runtime red (`run-oIU9yx`) exposed that derived provenance `sourcePath` and
`descriptorPath` are exact workspace-relative full paths, not bare filenames. The client was
corrected to assert those actual full facts; no production source changed.

The final retained binary is SHA-256
`2137f04ef251ecbd23a8b463e05c03479a5363cb630c442e0d0b874b0ded2413`.
The final input boundary is the client SHA-256
`2a04479cf249a4dbc945250ed858700ba5d7dae9baffa2d0bf6d094b79d45b3b`, the five mounted actual
Mini inputs recorded in `run-4UxFWF/🔣️results.json`, and the six exact artifact hashes recorded
there. This is the final Mini compiler/runtime boundary; the compiler slot was released to root.
