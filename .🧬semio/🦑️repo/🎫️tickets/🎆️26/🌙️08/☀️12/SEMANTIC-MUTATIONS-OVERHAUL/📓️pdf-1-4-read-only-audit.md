# PDF 1.4 Reserved Batch Read-Only Audit

## Hold Boundary

No PDF 1.4 source, fixture, schema, catalog, or consumer file was written. No Cargo or Nx task was launched. This ticket evidence records a read-only audit while the coordinator preserves the coherent PDF 1.7 snapshot for the shared runtime compile.

Applicable inherited repository/STDIO instructions were already read. Explicit checks under the PDF 1.4 standard/subset ancestors and the three reserved subset trees found no additional `AGENTS.md`.

## Exact Existing Roots and Counts

| Order | Subset | Aggregate | Old Variants | Existing Concrete Non-Fallback Kinds | Root Files | Direct Owners | Descriptors |
| --- | --- | --- | ---: | --- | ---: | ---: | ---: |
| 1 | any | `PdfMutation` | 2 | none | 31 | 0 | 0 |
| 2 | a | `PdfA1Mutation` | 4 | `set-page-text`, `clear-page-text` | 1 | 0 | 0 |
| 3 | x | `PdfX1Mutation` | 4 | `set-page-size`, `collapse-page-size` | 1 | 0 | 0 |

The exact roots are:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations`

There are ten old aggregate cases total: four concrete semantic cases, three identity sentinels, and three whole-snapshot fallbacks. Any contains one legacy `📄set-snapshot` owner with a complete nested Rust mutation/diff/inverse triad and a nested fixture; A and X are entirely central-only.

Any declares root Rust, TypeScript, GraphQL, protobuf, JSON Schema, text, and binary surfaces; its actual text/binary mutation implementation is centralized in the root while the physical codec facets expose grammar/protocol metadata. A and X currently declare only a Rust mutation root.

## Catalog and Test Correspondence

| Subset | Catalog | Old Kinds | Feature Rows | Registered Oracle |
| --- | --- | ---: | ---: | --- |
| any | `pdf-1-4-any` | 2 | 4 | `lopdf-pdf-1-4-mutate`, lopdf 0.44 |
| a | `pdf-1-4-a` | 4 | 8 | `lopdf-pdf-1-4-a-mutate`, lopdf 0.44 |
| x | `pdf-1-4-x` | 4 | 8 | `lopdf-pdf-1-4-x-mutate`, lopdf 0.44 |

The three exhaustive cases are `mutate-pdf-1-4`, `mutate-pdf-1-4-a`, and `mutate-pdf-1-4-x`. Any's reference currently only round-trips or rebuilds the entire page tree from `params.snapshot.pages`; it must gain independent operation-specific page editing when the fallback is removed.

## Dependencies and Cutover Risks

1. **Any cannot be converted by deletion alone.** Its only two cases are `NoMutation` and `SetSnapshot`; removing both would create the explicitly forbidden empty aggregate. No independent whole-snapshot semantic operation is modeled here, so renaming the fallback is not justified.
2. **The real base schema is a page vector, not the PDF 1.7 retained object graph.** `PdfSnapshot { schema, pages: Vec<PageDoc> }` and `PageDoc { width, height, text }` are owned by 1.4 Any. It borrows only the PDF COS lexical grammar from 1.7. Do not import 1.7 graph mutation payloads or claim full conformance properties absent from this schema.
3. **Approved naming direction, to implement only after release:** `insert-page`, `remove-page`, `move-page`, `resize-page`, and `replace-page-text`. The coordinator chose `resize-page` / `replace-page-text` over the proposed `set-page-size` / `set-page-text`; preserve the actual PDF 1.4 `PageDoc { width, height, text }` domain. These five explicit page operations can replace legitimate page-vector changes without a whole-document or generic collection escape hatch. Use index-aware payloads, exact inverse plans, minimum-page/bounds outcomes, and one greenfield tag table. Source remains held for the shared runtime snapshot; this is not a completed implementation.
4. **A then X depend on the base's snapshot/diff and builders.** A's two retained semantic operations affect only page 1's extractable text. X's two affect only page 1's width/height; `collapse-page-size` sets width to zero. Their `schema-gap-unverifiable` diagnostics must remain honest and unchanged. Neither should gain arbitrary object-graph mutations.
5. **Approved-verb risk:** `collapse` is not an approved semantic descriptor verb. Preserve the `collapse-page-size` identity/behavior but use a supported descriptor verb such as `resize`, as the accepted `embed-font-file` identity uses the supported `insert` verb.
6. **Root glue has a legacy mount.** `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` lines 4333–4350 mounts the 1.4 Any root, root text/binary facets, and the legacy nested set-snapshot triad. The triad routes must be removed during the future cutover, with canonical codec routes preserved.
7. **Artifact registration consumes root codec metadata.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🦀️component.rs` binds the 1.4 snapshot/mutation type and reads the root grammar/protocol constants. Grammar, protocol, ABNF, KSY, Spicy, ANTLR/EBNF, text schema wrappers, and TypeScript wire facets must match the new exact identity table.
8. **Do not confuse the editor alias with the base mutation type.** The physical 1.4 Any editor imports the artifact-level `crate::artifacts::pdf::{PdfMutation, PdfSnapshot}`, which intentionally denotes the canonical 1.7 model; it is not a stale 1.4 SetSnapshot consumer. True 1.4 consumers are the schema builders, its IO law tests, its legacy fixture, and the three exhaustive adapters/oracles.
9. The proposed concrete final owner count is five Any plus two A plus two X = nine, subject to the coordinator's reserved-batch release. No source changes begin during the runtime hold.

## Qualified Rust Reference Scan

A scoped STDIO scan found these fifteen files with explicit 1.4 mutation type/path references. This is exact for the command's literal patterns, not a claim to resolve every possible alias dynamically:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-x/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-4-a/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs`

## Exact Census Commands

```sh
bun -e 'const fs=require("fs");const roots=[{"subset":"any","base":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations"},{"subset":"a","base":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations"},{"subset":"x","base":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations"}];const walk=d=>fs.readdirSync(d,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(d+"/"+e.name):[d+"/"+e.name]);const rows=roots.map(({subset,root,base})=>{const source=fs.readFileSync(root+"/🦀️component.rs","utf8"),files=walk(root);const kinds=[...source.match(/pub const KINDS:[^\n]+/)[0].matchAll(/"([^"]+)"/g)].map(m=>m[1]);const dirs=fs.readdirSync(root,{withFileTypes:true}).filter(e=>e.isDirectory()&&!["📝️text","💾️binary"].includes(e.name)).map(e=>e.name);return {subset,root,aggregate:source.match(/pub enum (\w+)/)[1],declaredKinds:kinds,oldVariantCount:kinds.length,concreteNonFallbackKinds:kinds.filter(k=>!["no-mutation","set-snapshot"].includes(k)),files:files.length,directSemanticDirectories:dirs,directOwners:dirs.filter(d=>fs.existsSync(root+"/"+d+"/🦀️component.rs")).length,descriptors:dirs.filter(d=>fs.existsSync(root+"/"+d+"/🔣️component.json")).length,nestedMutationComponents:files.filter(f=>f.endsWith("/🦠️mutation/🦀️component.rs")),nestedDiffComponents:files.filter(f=>f.endsWith("/🔺️diff/🦀️component.rs")),nestedInverseComponents:files.filter(f=>f.endsWith("/↩️inverse/🦀️component.rs")),rootFiles:files.filter(f=>f.slice(root.length+1).split("/").length===1)};});console.log(JSON.stringify({rows,oldVariants:rows.reduce((n,r)=>n+r.oldVariantCount,0),existingConcreteKinds:rows.reduce((n,r)=>n+r.concreteNonFallbackKinds.length,0)},null,2));'
rg -l 'v1_4::subsets::(any|a|x)::schema::mutations|PdfA1Mutation|PdfX1Mutation' '✏️s/🔌️plugins/🗄️stdio' -g '*.rs' -g '!**/target/**'
```

Structured counts, catalogs, and qualified dependency paths are preserved in `🔣️pdf-1-4-read-only-audit.json`.
