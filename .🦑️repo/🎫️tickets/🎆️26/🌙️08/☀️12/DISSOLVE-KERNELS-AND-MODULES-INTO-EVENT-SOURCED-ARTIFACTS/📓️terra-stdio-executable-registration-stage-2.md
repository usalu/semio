# Stdio Executable-Registration Stage 2

## Baseline

- Packet reread: `📓️sol-stdio-executable-registration-ordered-central-lease.md`; applicable `✏️s` and `stdio` instructions reread.
- Stage 1 is structurally settled: `bun ./📜️script.ts stdio quick` passed with 36 artifacts, 40 dialects, and 6 codecs.
- The protected root-validator fingerprint advanced after the packet from `c35904…` to `8ca44f9c328343bf7f6bd55f6a8596dcd12b8b9bbdb5ba47a96088f2983eff56`. It is a dirty external referrer and is not edited in this lease.
- Registry: `a6f251f5b789b53b2cb6d74cffdcbab67b2d185602005b94738f787bbf204596`; plugin root: `9b4199621ce2cbd3df15a0fb1604a5f22e9a122ee5d9b47b289112b2f9cd2845`; manifest referrer: `d4d3bcc9455ed43e3ef871df784a6a98f0d297dbb0f403b474144df7c9cdc9b4`.
- Ordered all-root source/definition cohort fingerprint: `b56b15073523b083a84ede58ec6dedf56d23e3cd2f1e9a32784f1e58baa3e872` (the roster and individual path hashes were emitted from the catalog before editing).
- The catalog holds 36 roots: 26 call `runtime_assembly`, while binary, txt, ifc, gif, bmp, semio, wav, epw, tsv, and html are definition-only. At baseline all 36 definitions had `runtime_capabilities: []`; all codec/mutation/inference rows had `executable_registration: false`.
- The runtime roots and registry are already dirty predecessor work. The catalog, root validator, framework capability files, Cargo files, generated paths, taxonomy, and launch paths remain outside this lease.

## Decision

- The shared schema owner populated the exact runtime capability records after the baseline. This lease consumes the representation records as source of truth rather than duplicating MIME or extension claims in runtime roots.
- Every runtime root derives its `.formats(...)` facet through `registry::format_descriptors_for(artifact)`. The registry validates the full catalog before finding the matching representation claim set, so each declaration is assembled from the same schema-owned records used for capability identity.
- Keep the ten definition-only artifacts definition-only. Do not fabricate executable mappings: with zero true registration rows, the only valid mapping table is empty.

## Implementation

- Updated `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` so the per-artifact descriptor resolver presents assembly failures as `ArtifactDefinitionError`, matching every root declaration's return contract.
- Final registry fingerprint: `2f505d2286b3027cc7b11c75075df967925813ddc5075f0664bf479f037eacb7`. The protected root and manifest referrer fingerprints remain exactly the baseline values.
- Updated the 26 executable roots — xml, deflate, zip, json, csv, md, gltf, obj, stl, ply, las, step, dwg, dxf, svg, png, jpg, tiff, pdf, docx, pptx, xlsx, bcf, mp4, avi, and mp3 — to pass their exact schema-derived formats to `ArtifactDeclaration::builder`.
- Added a bounded registry proof for glTF's `mime-model-gltf-json-extension-gltf` representation capability. It checks the exact MIME/extension claims, the derived format descriptor, all 36 `artifact_assemblies`, and the real `crate::plugin()` / `PluginBuilder::try_library` boundary.
- Held all ten definition-only roots as referrers only; neither their source roots nor their capability mappings are given executable behavior. The canonical mapping table remains `BTreeMap::new()` because every declared codec, mutation, and inference has `executable_registration: false`.
- No root plugin, manifest, root validator, Cargo, taxonomy, launch, generated, or framework file was changed. No registrar request is required.

## Validation

- `bun ./📜️script.ts stdio quick` passed: `36 artifacts, 40 dialects, 6 codecs`.
- `bun nx run @semio-tech/stdio-js:test-quick --skip-nx-cache` passed: the TypeScript facade exposes 36 schema-owned artifact definitions.
- A direct source/JSON bijection sweep parsed all 36 definition files: 26 runtime roots have exactly one canonical format-derivation consumer, the ten definition-only roots have no runtime capabilities, and all executable mappings remain empty.
- `rg -l 'format_descriptors_for\\(' ... -g '🦀️component.rs'` returned exactly 26 direct runtime-root consumers. The registry contains the sole empty executable mapping table.
- `RUSTC_WRAPPER= cargo check -p semio-s-plugin-stdio --lib` is blocked before stdio assembly by the frozen quarantined framework owner: `semio-framework-plugin` fails with `E0063`, missing `dialect` and `role` in `AppDefinition` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4665:34`. No framework bypass or modification was attempted.
- `git diff --check` is clean for the registry and all stdio artifact source/schema paths. The ticket evidence file is also whitespace-clean.
- The packet orders taxonomy report/enforce after framework stability; it is deliberately deferred pending the quarantined owner's release rather than run against an unavailable framework boundary.

## Release Boundary

Stage 2 source, static, and structural work is complete. The only unresolved validation is the intentionally quarantined framework capability API compile gate. Once that owner restores its `AppDefinition` contract, the bounded `crate::plugin()` test, `cargo check`, and the post-framework taxonomy report/enforce remain the ordered final acceptance checks.
