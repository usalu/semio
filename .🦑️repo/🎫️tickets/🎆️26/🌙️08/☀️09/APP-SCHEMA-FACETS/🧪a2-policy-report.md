# 🧪 A2 Policy Report — App Schema Facet Scanners

Ticket `26/08/09/APP-SCHEMA-FACETS`. Wave A2 owns root `📜️script.ts` only:
region `🔧️PolicyRuleAppSchemas` (immediately after `🔧️PolicyRuleArtifactSchemas`) +
`VerifyScript.runGate()` / `policy` export registration beside `policyArtifactSchemaBreaches`.

## Region `🔧️PolicyRuleAppSchemas`

### Reused extractors (unchanged — not copied)

Calls the five artifact extractors + `policyFindSchemaDeclaration` from
`PolicyRuleArtifactSchemas` via `policyLoadAppSchemaFacetLeaves` / config-fidelity:

| Function | Role |
| --- | --- |
| `policyExtractRustSchemaFields` | rust leaf + real `XConfig` fidelity |
| `policyExtractTypescriptSchemaFields` | typescript leaf |
| `policyExtractGraphqlSchemaFields` | graphql leaf |
| `policyExtractJsonSchemaFields` | normative jsonschema leaf |
| `policyExtractProtobufSchemaFields` | protobuf leaf |

Leaf filenames come from `taxonomy.schemaFormats`; normative leaf names from
`taxonomy.appSchemaSpecFilenames` (already present — taxonomy not edited in A2).

### Owner discovery

`policyDiscoverAppSchemaOwners(repoRoot)` walks plugin app `🦀️component.rs` files,
parses `type Config = XConfig;`, resolves owner (`🎚️config` → legacy `🧮️config` → plugin-level
`🎚️config` declaring `pub struct XConfig`), presence sibling `👥️presence`, dedupes by owner path.

Validated against `🧪owner-table.json`: **39 owners / 53 apps**, zero path/type/app mismatches.

### Scanners → breach kinds

| Scanner | `kind` |
| --- | --- |
| `policyAppSchemaFacetCompletenessBreaches` | `app-schema/facet-completeness` |
| `policyAppSchemaFieldParityBreaches` | `app-schema/field-parity` |
| `policyAppSchemaConfigFidelityBreaches` | `app-schema/config-fidelity` |
| `policyAppSchemaStatePurityBreaches` | `app-schema/state-purity` |
| `policyAppSchemaTypeNameParityBreaches` | `app-schema/type-name-parity` |
| `policyAppSchemaConfigRelocationBreaches` | `app-schema/config-relocation` |

Export: `policyAppSchemaBreaches(repoRoot)`.

## Gate probe

```bash
bun -e 'const m=await import("./📜️script.ts"); console.log(m.policyAppSchemaBreaches(process.cwd()).length)'
```

### Breach count by kind (current tree)

| kind | count |
| --- | ---: |
| `app-schema/facet-completeness` | 76 |
| `app-schema/field-parity` | 0 |
| `app-schema/config-fidelity` | 0 |
| `app-schema/state-purity` | 0 |
| `app-schema/type-name-parity` | 0 |
| `app-schema/config-relocation` | 0 |
| **total** | **76** |

> Re-probed after A2 path fix: lowpoly pilot is clean (0 breaches). Remaining 76 are unmigrated owners (2 facets × 38).

All kinds are under `app-schema/*`. Completeness covers the 38 unmigrated owners × 2 facets.
Lowpoly pilot already has both facets; the 12 field-parity hits are fixedList camera shape
disagreements across TS/GraphQL/proto vs normative JSON. No `🧮️config` / `🕸️wasm` dirs remain
on disk (relocation clean). Launch seed: no artifact-schema entry in
`.vscode/🧩️launch.seed.jsonc`, so none added for app-schema.

## Files edited

- `📜️script.ts` — new region + `runGate` + `policy` export hook
- ticket `🧪a2-policy-report.md` — this report

Taxonomy, registry, kernel, and plugin files were not touched.
