# Execution Contract (WP1–WP7) — binding for every worker

Coordinator decisions, derived from the WP0 reports in this folder (`📓️wp0-*.md`, `📊️wp0-*.json`).
Workers implement against this contract; deviations are reported in the worker's own `📓️wp*-*.md`, not
silently applied.

## A. Identity model

- **Scope** = an eligible owner directory that carries (or will carry) one `🧬️schema/` module. Eligible
  levels: plugin root (`✏️s/🔌️plugins/<p>`), artifact standard subset
  (`…/🏅️standards/🔖️N/🪆️subsets/<s>`), surface (`✏️editor/🎚️config|👥️presence|🫧️transient`), framework
  module (`🧰️framework/🔨️modules/<m>` and nested module dirs), product module
  (`🧰️framework/🛍️products/<p>/🔨️modules/<m>`), hub area module (`🌎️hub/<area>`), mutation leaf
  (`…/🧬️schema/🧬️mutations/<leaf>`, declared authority per `mutationPayloadSchemaAuthority`).
  NOT eligible: `🧱️elements/*` (kind ui), `🎯️targets/*`, `📦️packages/*`, any `🧪️*`/`🧫️*` directory,
  any single contract directory (`🧬️contracts/<x>`). A nested module directory that is a crate root
  (e.g. `🖱️ui/🧬️contract/`, the `ui-contract` crate) is an eligible nested module scope; only the plural
  per-contract `🧬️contracts/<x>` shape is forbidden. Scope ids must be derivable from the path: sibling
  directories whose ASCII tails collide are renamed, never mapped through an override table.
- **Scope id** = dotted id derived from the module `$id` path after `https://semio.tech/schema/`, e.g.
  `s.writer.writer`, `hub.inference`, `os.directory`, `framework.actor.return`. It is declared in the
  module's `🔣️.json` as `$id` and must equal the id the Rust `ArtifactSchemaDescriptor` registers where
  one exists. Never derive ids by stripping emoji from paths.
- **Export id** = a PascalCase key of the module's `$defs` (JSON Schema), the same-named `message`
  (proto), `type` (GraphQL), `struct/enum` (Rust), exported `interface/type` + `parse<Export>` function (TS).
- **`$id` grammar (settled after WP2)**: `https://semio.tech/schema/<scope path>/<facet>.json`. The scope
  path is every segment before the filename; the filename is the facet (`schema`, `artifact`, `snapshot`,
  `diff`, `mutations`, `inferences`, `text`, `binary`, …). Scope id = scope path with `/` → `.`. Facet
  documents of one module (e.g. `📸️snapshot/📝️text/🔣️.json` → `…/<scope path>/snapshot/text.json`) keep the
  root scope path and only vary the facet filename; they never create a deeper scope. A **mutation leaf is
  its own scope**: `<leaf>/🧬️schema/🔣️.json` declares `$id …/<root scope path>/mutation/<semanticKind>/schema.json`
  (scope id `<root>.mutation.<semanticKind>`, facet `schema`, exports `Payload`/`Wire` or the `title`).
  The catalog generator recognizes a `🧬️mutations/<leaf>/🧬️schema/` module as a scope of its own.
- **Format ids** = taxonomy `schemaFormats` keys (`🔣️jsonschema`, `🛰️protobuf`, `🔗️graphql`, `🦀️rust`,
  `🟦️typescript`). Normative format for `🧬️data` facets is JSON Schema.
- **Resolution key** = `(scope id, export id, format id)`. Fixture binding URI: `schema://<scope id>/<ExportId>`
  (format chosen by the consumer). No nearest-parent search, no glob, no fixture-local fallback.

## B. Physical layout

```
<scope>/🧬️schema/🔣️.json      draft-07, $id, $defs.<ExportId>, root may be a oneOf of exports or a $ref
<scope>/🧬️schema/🛰️.proto     package derived from scope id
<scope>/🧬️schema/🔗️.graphql
<scope>/🧬️schema/🦀️.rs        pub types + decode/validate entry points, include_str! of siblings
<scope>/🧬️schema/🟦️.ts        exported types + parse<Export>() functions
<scope>/🧬️schema/📸️snapshot|🔺️diff|🧬️mutations|💡️inferences/…   existing child facets unchanged
<fixture collection>/<case>/🔣️.json (+ inputs, expectations)     data only, never a schema
```

- JSON Schema dialect everywhere: `http://json-schema.org/draft-07/schema#`. Migrate 2020-12 documents
  (`prefixItems` → `items` array + `additionalItems:false`, `$defs` stays, `unevaluated*` removed).
- `$id` everywhere: `https://semio.tech/schema/<scope path>/<facet>.json` (replace `urn:` and
  `semio.hub.*-fixture` ids). Cross-scope `$ref` uses the target `$id` + `#/$defs/<ExportId>`.
- Filenames: inside a `🧬️schema/` module only the canonical five (+ facet dirs). `*.schema.json`,
  `🧬️schema.json`, `📋️.schema.json`, `🧬️contracts/` cease to exist after WP7.
- Mutation leaves: the leaf stays the authority; every leaf payload schema moves to the taxonomy default
  `<leaf>/🧬️schema/🔣️.json` and the descriptor `payloadSchema` is updated to that relative path.
  Module aggregates `🧬️mutations/🔣️.json` are pure `$ref` unions (G-B shape), never inline payloads.
- Fixture directories are named per taxonomy: `🧫️fixtures` (`testFixturesDirName`) for data collections,
  `🧪️tests` (`testsDirName`) for test cases. `🧪️fixtures` is not a taxonomy name; wave-2 partition owners
  rename it and rewire readers. Dev-tool output graphs that are serialized and consumed are scope contracts.
- Wrapper schemas that describe a fixture file (hostile lists, `maximumBytes`, state-machine "spec of one
  example") are NOT contracts: keep the example data, move the real contract into the owner module, and
  express hostile cases as fixture expectations (stage + reason), not as schema.

## C. Catalog and tooling (WP2)

- Derived catalog (generated, never hand-edited, provenance header with source hashes):
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`
  `{ scopes: { <scope id>: { path, formats: {…: file}, exports: { <ExportId>: { file, facet } }, dependsOn: [scope id…], hashes } } }`
  (settled after WP1: each export names the module-relative file that carries it, since facet children
  such as `🔺️diff/`, `📸️snapshot/`, `💡️inferences/` hold exports of the same scope).
- A schema document inside a `🧪️*`/`🧫️*` tree is a finding unless the enclosing case declares
  `inertSchemaData` (parser test inputs). No blanket fixture exemption.
- Taxonomy additions (`🔣️taxonomy.json`): `schemaScopeOwnerLevels` (eligible levels above),
  `schemaExportResolution` (`schema://` scheme, catalog path), plugin-root `🧬️schema` slot in
  `scopedFileKinds`/`fixedDirectoryContracts`, `schemaJsonDialect` single value.
- Commands in root `📜️script.ts`: `schema audit | check | generate | verify | test | docs`
  (`project.json` targets call `📜️script.ts schema <sub>`, `package.json` calls nx, `.vscode/launch.json`
  entries follow the existing naming/grouping in `📓️wp0-mechanism.md` §3.2).
- Heuristics retired (`📓️wp0-mechanism.md` §5 rows 1–8): POLICY_ARTIFACT_SCHEMA_PREFIXES table, dual
  payload-schema convention, `rustTypeIndex` + three nearest-parent call sites, first-declaration fallback,
  facet-kind disk probing.
- Harness: `resolveFixtures` gains `schema://`; pipeline stages are explicit: metadata → contract
  resolution → parse → structural validation → domain → assertions; negative fixtures assert stage + reason.
- Rust framework: `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs` — named exports are registered by
  a sibling declaration `register_scope_schema_exports(ScopeSchemaExports { scope, exports })` (accepted
  deviation, see `📓️wp2-rust-schema-registry.md` §3.1: the descriptor round-trips through the replication
  wire type and has 235 external literals). `resolve_schema_export(scope, export, format)` unifies fixed
  facets and named exports; registration conflicts remain fatal. Draft-07 structural validation is
  `semio_framework_schema::structural_validator_for` / `✅️validator.rs` (owned, ajv-oracle-verified).

## D. Ownership decisions already settled by WP0

| Contract | Owner scope | Notes |
|---|---|---|
| `semio.hub.inference-approval/v1` + receipt | `hub.inference` (`🌎️hub/💡️inference/🧬️schema/`) | os/mcp duplicates become re-exports or explicit conformance-checked mirrors, see WP4-os |
| all `🌎️hub/🧪️fixtures/*-v1` contracts | `hub.inference` | `-v1` is a fixture naming convention, not a protocol version |
| trusted-catalog publication receipt/command | `hub.artifact-authority.trusted-catalog` | three competing impls → one `pub` module |
| ShellHost invite-capability-transfer, artifact-creation, directory-bootstrap | `os.directory` | ShellHost/SpaceAdministration/backbone-worker are consumers |
| ShellHost presence-scope | `os.store.presence` | |
| ShellHost dialog-origin, host-bootstrap | `os.renderer` (new `📺️renderer/🧬️schema/`, renderer is a `🔨️modules` member) | UI-presentation contracts get facet `🖼️presentation` if taxonomy needs it |
| retained-command-limits / -routes | per artifact subset scope (six owners), shared *shape* exported by `framework.ui` | |
| plugin-identity, artifact-identity | plugin root scope (`<plugin>/🧬️schema/`) | requires the plugin-root slot from WP2 |
| flow/note action-cohort | `s.flow.action-cohort` owns; note references | note's drifted copy deleted |
| mutation leaf payloads | the leaf | aggregates become `$ref` unions |
| coordinator REST API (auth/diff/event/repo/ticket) | `repo.server.coordinator` (`🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/`) | zod-shaped validator becomes a consumer |
| os `🖥️shell` JsonSchema-derived structs | `os.shell` (new `🧬️schema/` module) | typegen reads from it |
| os `🌉️mcp` raw `json!` literals | `os.mcp` registry (`🌉️mcp/🧬️schema/🦀️.rs`) | no schema outside the registry |

## E. Rules for every worker

- Work only inside your assigned file partition; other workers edit the rest concurrently. Do not revert
  others' changes. If a file you need is outside your partition, write the exact needed change into your
  report under "cross-partition requests" instead of editing it.
- No git-modifying commands, no worktrees, no ticket open/close/reopen, never delete `🗑️generated/`.
- No compatibility layers, aliases, redirects, fallbacks, deprecations or migration scripts left in the
  tree. Delete superseded files in the same change. Fix every reference (Rust `include_str!`, TS imports,
  script.ts paths, nx inputs, launch.json, docs).
- Temporary logs carry the `[DEBUG] ` prefix. Build and test output goes to `<ticket>/🗑️generated/`.
- Verify: run the affected tests/commands yourself (bun/vitest/python/ajv; `cargo check -p <crate>` only
  for the crate you touched, never a workspace build) and paste the real result into your report.
  Do not claim a test passes without running it.
- Deliver `<ticket>/📓️<wp>-<slug>.md`: what changed (file list), decisions, verification output, open
  cross-partition requests. Keep it factual.
