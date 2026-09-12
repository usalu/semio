# Shared Paged List Ownership

## Result

The physical paged-list implementation now belongs to `🧰️framework/🔨️modules/🌱️value/📋️list`, exported by the existing first-party replication package. `UiFixedList` consumes that owner; numerical assembly can use it without a dependency on UI. There is one implementation and no compatibility alias or new external runtime dependency.

The generic owner preserves ordered logical indexing, fixed fanout, separate metadata/payload admission, exact rejected producers, and bounded page retirement. A caller can now pre-admit capacity across multiple pages without adding logical items. Each admission does at most one physical allocation. An empty-page release checks the actual backing byte count before taking the page and returns the actual released byte count. Insufficient grants retain the exact backing. UI typed retirement now forwards its byte grant and accounts for actual released backing instead of reporting zero.

## Validation

The registered `framework-paged-list-ownership` route runs the language-neutral JSON corpus through Ajv and fast-json-patch, strict TypeScript, generic native ownership tests, and UI consumer tests. The corpus includes 600 ordered items across page boundaries, exact maximum-capacity refusal, oversized admission, zero grants and exact backing retirement. Native generic order also agrees with Serde.

Native run one passed all four generic laws, then exposed UI tests that referenced generic `cfg(test)` probes across a crate boundary. Those probes stay inside the storage owner. UI tests now inspect public iteration and stable payload addresses; generic tests directly verify initialized slots after each push/pop and empty pre-admission.

Native run two exited successfully on the configured 2 MiB stack: four generic laws and eight UI fixed-list laws passed. Runtime debug witnesses cover allocation accounting, oversized producer retention, zero-sized values, mutable iteration, reuse, ordered wire output and typed retirement grants. Evidence: `🗑️generated/framework-paged-list-native-2.log`. This does not yet prove numerical assembly integration; that consumer migration remains open.

## Files

New shared owner and proof files:

- `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/🔬️counter/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list/🟦️.ts`

Shared export and UI consumers:

- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪞️copy/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧱️component/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️list/🧪️tests/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧪️tests/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs`

Removed duplicated physical owner files:

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧪️tests/🔬️counter/🦀️.rs`

Command registration: root `📜️script.ts`, `📋️project.json`, canonical `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (order `311.203`), and this ticket's `validation/📜️script.ts` and `validation/project.json`.

## Actual Signed-Total Audit Correction

The follow-up Terra audit found that requested allocation bytes were preflighted against `isize::MAX`, while actual allocator capacity only received a `usize` overflow check. Both metadata and payload paths now reject an actual total above the signed-addressable bound while retaining and reporting the exact physical owner for bounded release. The regression seeds accounting near the limit, uses the doubled-allocation seam under a sufficient grant, refuses a release one byte too small without changing its pointer, then releases the exact backing. Two language-neutral decimal-string vectors cover 32-bit and 64-bit totals with BigInt arithmetic and independent Ajv/JSONPatch output. Native validation is pending; the earlier 4+8 result predates this correction.

Changed files in the existing value/list owner: root Rust implementation, counter tests, neutral fixture/schema, TypeScript oracle, and this report.

Native3 is GREEN: five generic owner laws and eight UI laws pass on explicit 2 MiB, with the independent Ajv/JSONPatch counter vectors and strict TypeScript. Runtime diagnostics confirm both metadata and payload reject actual signed-total excess, retain the exact pointer under an insufficient release grant, and release the exact backing. The whole Nx route exited zero.
