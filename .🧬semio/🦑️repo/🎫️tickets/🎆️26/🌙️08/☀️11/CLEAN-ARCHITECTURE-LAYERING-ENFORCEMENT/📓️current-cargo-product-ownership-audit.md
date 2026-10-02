# Current Cargo Product Ownership Audit

Read-only inspection on 2026-10-01. No source, schema, fixture, manifest or expectation edits; no runtime, test, Nx or Cargo launch. Findings describe the live files inspected, not a successful build. No temporary generated files were created.

## Physical Dependency Edges

General Cargo manifests currently contain 15 product dependency declarations: 11 explicit path declarations and 4 workspace declarations. Optional and dev declarations remain architectural edges. The actor manifest's product path occurs in a comment and is not a dependency.

| General owner | Product dependency | Concrete source use | Ownership correction |
| --- | --- | --- | --- |
| `📦️packages/🦀️rust` | OS kernel | Entry lines 3–6 alias kernel as DSL/protocol/store; lines 9, 20–32 export localization, I/O and snapshot vocabulary | Extract neutral value, localization, protocol, registry and snapshot owners; remove product facade imports rather than rename kernel |
| `🔨️modules/✍️editor` | OS kernel; OS infinite | `🦀️.rs` exports infinite canvas wholesale; `EditorError::Pack` uses store error; lines 701–708 decode wire/pack values to JSON | Neutral editor needs value/pack codecs and camera/text contracts; move OS board integration outward, stop exporting all infinite |
| `🔨️modules/🖱️ui` | OS kernel, optional under `wgpu` | Package entry aliases kernel as DSL because derive expansion defaults there; component wire tests instantiate kernel `Viewport2d` | Bind derives to neutral value owner and viewport vocabulary; optional feature does not justify product ownership |
| `🔨️modules/🧬️schema` | OS kernel | Validator takes `DslValue`; component exports `StateClass`, composition descriptors and kernel artifact/app registration; document HTTP imports `os_directory::client` and `DocumentScope` | Split neutral validator/composition from OS registration and document HTTP integration; move latter to directory/product |
| `🔨️modules/📚️compiler` | OS kernel as `dsl_core` | Package exports `os_dsl`; compiler facade `Syntax` wraps `crate::os_dsl::TextError` | Neutral diagnostic/parser owner, with OS DSL family catalog/registration remaining product-owned |
| `🔨️modules/🕸️graph` | OS kernel; OS neural engine | Engine binds graph storage to DSL value; manifest imports `neural_engine::Value`, exports `ValueType`, converts graph literals to neural atoms at lines 658–661 | Neutral graph value/schema vocabulary; neural atom conversion belongs to neural integration, not graph core |
| `🔨️modules/🗺️surface` | OS infinite; OS kernel; infinite DAG artifact, workspace | Paint and tiled map export infinite wholesale; node graph exports board directed-DAG and imports `DomainHover`, `DomainSelection`, `SelectionMethod`, `Viewport2d`; line 25 imports artifact `DagCamera`, `DagHostSnapshot`, `DagHostSnapshotEdge`, `DagNodeKind`, `DagNodeSpec`, `IoPortSpec` | Extract camera/text/surface contracts; relocate board/artifact assembly to infinite product and interaction contracts to general owners |
| `🔨️modules/🌱️value/✨️derive` | OS kernel, dev dependency | Expansion and retained-clone generators hardcode `::semio_framework_os_kernel`; dev tests exercise concrete generated code | Neutral value and retained-clone owners must be actual emitted paths and test runtimes; moving only dev tests leaves emitted reverse dependency |
| `🔨️modules/◻️2d` | OS kernel, workspace | Package entry reexports `os_spr`; geometry wire encoding uses value derive | Depend on neutral replication/value directly; remove OS facade exposure |
| `🔨️modules/⏯️tool-run` | OS kernel, workspace | Owner lines 8–9 take record layouts and exact codec functions from `os_dsl` and `os_pack` | Bind to neutral record/value/pack owners; tool-run is domain neutral |
| `🔨️modules/🕸️graph/⏯️layout-run` | OS kernel, workspace | Glue aliases kernel; owner imports `FromValue`/`ToValue` and neutral job/tool-run ledger | Bind to neutral value owner; retain layout job at graph owner |

All paths in this table are relative to `🧰️framework`. Root `Cargo.toml:182` resolves workspace OS-kernel dependencies physically to `🛍️products/💻️os/📦️packages/🦀️rust`; line 127 resolves the infinite DAG artifact to the OS infinite product's artifact package.

## General Owner Versus Misplaced Specific Owner

Extract upward: shared `DslValue`/`ToValue`/`FromValue`, record layouts and codecs, localized labels, viewport/camera/text, diagnostic/span, component composition, retained cloning. Existing neutral implementations in replication/value/pack should own the concrete types once; product exports must not become intermediate facades. Kernel entry itself already reexports general protocol diagnostic/span and pack codec; these prove the product crate is currently an aggregation point, not the necessary semantic owner.

Move outward: schema document HTTP transport/scope; kernel app/artifact descriptor registration; surface/editor OS board assembly; graph-to-neural atom adaptation; OS DSL family registration. Do not extract the entire kernel, infinite canvas or neural engine upward merely to silence the policy: that promotes product responsibilities into general framework.

Recommended order: (1) neutral value/record/derive ownership removes the widest shared blocker; (2) separate schema document HTTP and graph neural adapters, which have identifiable product imports; (3) split infinite camera/text contracts from board assembly; (4) delete manifest edges only after every source import/mount and consumer is redirected to its final owner.

## Three Small TS Test Moves With Unchanged Assertions

Source: `🧰️framework/🧪️tests/🧱️fixture-ownership/🟦️.ts`. Keep its pixel-lock hierarchy oracle general. Move each product test intact to the fixture/schema product owner below; retain Ajv as independent oracle for schema cases. Adjust only root/path plumbing required by location.

### Directory Lease

Target owner: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🔏️document-execution-target-lease-v1/🟦️.ts`.

Original test: `the lease corpus binds both peers to one framework contract witness`.

```ts
expect(vectors.schema).toBe("semio.os.document-execution-target-lease-corpus/v1");
expect(vectors.plan.package.componentSha256).toBe(vectors.manifest.package.componentSha256);
expect(vectors.plan.package.descriptorByteSha256).toBe(vectors.manifest.package.descriptorByteSha256);
```

Reads directory `🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json`.

### MCP Approval

Target owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/✅️approval-request/🟦️.ts`.

Original test: `approval intents are closed under the framework-owned contract`.

```ts
expect(validate(intent)).toBe(true);
expect(validate({ ...intent, proposal: "private bytes" })).toBe(false);
expect(validate({ ...intent, jobId: "invalid" })).toBe(false);
```

Retain original Ajv compile and intent `{ schema: "semio.hub.inference-approval/v1", version: 1, jobId: "ab".repeat(16), proposalHash: "cd".repeat(32) }`. Reads MCP `🧬️schema/✅️approval-request/🔣️.json`.

### Flow Slider

Target owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧪️tests/🏷️slider-labels/🟦️.ts`.

Original test: `slider descriptors require authored labels across schema and native contracts`.

```ts
for (const row of vectors.cases) {
  expect(validate(row.widget)).toBe(true);
  const missing = { ...row.widget };
  delete missing.label;
  expect(validate(missing)).toBe(false);
}
```

Retain Ajv compile of `$defs.FlowInputSliderDescriptorV1`, the artifact schema and `🧬️schema/📸️snapshot/🧫️fixtures/🏷️slider-labels.json`. Existing unused `expectedDagName` type annotation does not establish a native assertion; this test validates JSON schema only, so preserve its scope without claiming native runtime verification.

## Clean Deletion Boundary

No deletions performed. Outward test moves must preserve the exact original assertions, fixture bytes and oracle calls. The general fixture-ownership file remains because its shared pixel-lock test stays. Future source extraction must remove product imports, facade exports, mounts and manifest dependencies together; adding a policy allowlist or renaming a product crate is not a completed ownership correction.
