# Foundation Completeness Audit and Bounded Closure Packet

Date: 2026-08-27. Lane: TERRA-FND-COMPLETENESS-AUDIT-01. Read-only production audit after Sequence closure; no shared API/root-script/taxonomy/policy/test edits were made. Root, products, repo-product, and OS AGENTS were read completely. The referenced end-to-end plan was read. Real compose paths were not accessed.

## Decision

The current foundation is not complete against the brief. The direct taxonomy, lexical Rust facts, descriptor JSON schema, and high-severity policy layer are present, but scaffolding, compile-time derive enforcement, enriched inventory, and mutation apply terminality retain concrete gaps. Preserve coordinator ownership of the active policy/AST identity work; freeze the seams below before implementation.

## Authoritative Sources and Findings

### Scaffolding

Source: root `📜️script.ts`, `🧬️MutationScaffolding` around lines 20256–20358. Helpers: `newMutationSemanticParts`, `newMutationRustLeaf`, `newMutationDescriptor`, `newMutationUpdateAggregate`, `newScaffoldMutationTree`.

- The direct Rust template declares a free SEMANTICS constant and a unit `Mutation`; it does not implement MutationKind. Its serde unit representation is also not the object shape emitted by the optional payload schema.
- Tests, text, binary, GraphQL, and protobuf are comments-only; TypeScript is the generic empty facade. Optional facets are not mounted by the direct owner. The only meaningful payload schema accepts any object and specifies no fields.
- `--binary` still writes a null binary tag. Duplicate checks cover sibling folder emoji/name/variant only; descriptor opcode/tag collisions and whole-root declared language requirements are not used.
- Only Rust aggregate wiring changes. Existing root TS/GraphQL/protobuf/JSON surfaces, codec registries, and catalog vectors remain stale.
- Root mutation mounts are inserted at the `pub enum` token, after the enum's attributes. The actual helper was evaluated with an in-memory attributed aggregate: `#[derive]` and `#[serde]` then bind to the inserted module instead of the enum. The current test starts with an unattributed bare enum and cannot catch this.
- Aggregate edits use regex against raw source, so comments/string/attributes and unusual enum layout remain unsafe. Leaf files are written before aggregate validity is known, allowing partial output on failure.
- Owner checks use lexical resolve plus statSync, not the fail-closed symlink guard now used by scoped policy. No post-write scoped structural verification runs. No schema-first semantic behavior input exists, so a name alone cannot justify a claim that generated domain behavior is implemented.

Existing regression: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`, `scaffolds one direct mutation idempotently without overwriting implementation` around line 4721. It checks file creation, a partial descriptor, trivial object-schema acceptance, and non-overwrite; it does not compile generated Rust, parse real mirrors, assert executable tests/codecs, or require all-17 policy cleanliness.

### Derive and Protocol Contract

Built source: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`, `🔖️Mutations` around lines 1015–1183. Cargo.toml explicitly sets `[lib].path = "📦️glue.rs"`. A second implementation exists at `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` around lines 938–1101. Editing only the component would not change the currently compiled entry point; this duplicate-authority seam must be resolved or kept mechanically identical in the closure transaction.

- `parse_mutations_attrs` discards parse errors and ignores unknown keys. The accepted container schema is only snapshot/diff/schema.
- `derive_mutations` rejects non-enums and non-single-field tuple variants, then checks only SEMANTICS.kind against variant kebab and SEMANTICS.verb against approved verbs.
- It does not prove direct file ownership, descriptor existence/owner, wire opcode/tag uniqueness, payload-schema/language declarations, or reject forbidden behavior metadata. It emits no From<Leaf> conversions.
- Generated delegation and leaf-derived kinds are already the correct architectural seam; retain them.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` around lines 180–241 defines SemanticDescriptor and MutationKind with semantic metadata only; lines 409–477 define the runtime MutationDescriptor registry model without direct owner/wire/payload/classification fields. The JSON direct descriptor is currently an independent tooling contract, not the compile-time leaf contract.
- Existing positive runtime test `derive_mutations_wires_mutation_and_semantic_mutation` lives in that command source around line 1276 and checks one inline fixture. It has no rejection matrix or From assertion. No tests or compile-fail section were found in the compiled derive source.

### Inventory and Plan/Apply Truth

Source: root `📜️script.ts`, `🧬️MutationTaxonomyWorkflow` around lines 19278–19465.

- `inventoryMutationTaxonomy` walks inside each mutation root only. `commandsEditorsViewers` is always empty; the three assignment fields are always `unassigned`.
- Catalog/registry matches are searched only inside the mutation root, so actual subset oracle catalogs, commands, editors, viewers, and outside registries cannot be represented. The field can instead list ordinary implementation files that happen to mention the identity.
- Every record receives the whole root's TS/schema file list. Cross-owner dependencies are source filenames containing crate/super, not resolved target ownership edges. Shared helpers are filename matches for support/helper; a sibling schema operations owner such as Sequence's is not included.
- State is only shape (direct/legacy/central-only), not an execution state machine. Evidence is only leaf file paths, not validation/assignment history.
- sourceTreeDigest hashes derived records/violations, not source bytes, so a behavior-only edit that preserves detected facts can leave the digest unchanged.
- `planMutationTaxonomy` immediately skips direct-shaped records, even if their violationClasses are nonempty. An in-memory call with a direct record containing behavior-ownership failure returned zero moves and zero unresolved items. This is executed evidence, not inference.
- `runMutationTaxonomyCli(..., "apply", ...)` validates the stored plan's own digest/baseline and requires zero moves/unresolved, then writes `state: committed`. It does not compare the plan inventoryDigest with fresh source, or run a terminal policy check. The empty-plan case above can therefore reach a false committed result by code inspection; the mutating apply command was NOT executed during this audit.
- The mutation branch returns before the generic `taxonomyCliInventoryOptions` setup; scope/progress/cancellation/resume facilities are not carried into the mutation inventory workflow. Its loop does not expose progress or cancellation.

Existing test: same TypeScript test file, `inventories direct and legacy records as stable language-neutral JSON validated by Ajv` around line 4756. Its JSON Schema only checks the outer inventory shell and arrays; it does not validate enriched records, consumer edges, assignments, byte digests, or clean terminality.

### Developer Entry Points

`📋️project.json:407` already declares `workspace:new-taxonomy-mutation`, and lines 371–405 declare the four taxonomy Nx operations. The checked-in `.vscode/launch.json` has taxonomy inventory/plan/apply/verify at lines 2686–2728 without `--kind mutation`, and artifact/standard/subset creation at lines 3847–3880. It has no mutation scaffold launcher or mutation-specific taxonomy selection. A focused search for new-mutation/new mutation/kind mutation confirmed the absence. The new launcher must use the existing Nx route and match group/order rather than inventing another executable script.

## Bounded Test-First Closure Packets

These packets are proposals, not authorization to edit coordinator-owned paths.

### FND-SCAFFOLD-02 — Complete Direct Output and Transactional Verification

Write set after freeze:

- Root `📜️script.ts`: only `🧬️MutationScaffolding`, the new-mutation CLI argument routing, and reuse of existing guarded/transactional primitives.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`: scaffolder tests only.
- New language-neutral `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-scaffolding/🧫️fixtures/🔣️.json`.
- `.vscode/launch.json`: one mutation scaffold entry and explicit mutation workflow entries/options using existing Nx targets. Root project target already exists; change it only if frozen arguments demand it.

Red tests first: attributed aggregate insertion, an existing mounted aggregate/glue layout, all eight requested surfaces, real payload-field validation, executable/mounted tests, deterministic unique binary identity, descriptor collision, root-declared surface auto-completion, full all-17 verification, no writes on invalid aggregate/schema/symlink/opaque selection, dry-run byte stability, idempotence with unchanged hand-authored code, cancellation before commit. Use a virtual/temporary non-production fixture for opaque exclusions only.

Implementation seam: prepare a complete schema-first file map and validated aggregate edits before writing; derive mirrors and registries from actual payload/descriptor data; use structured source boundaries rather than the regex insertion point; validate the proposed tree, then publish through the existing guarded transaction mechanism and verify the exact root. Do not fabricate apply semantics from a verb-noun. Freeze a required schema/behavior contribution input or an explicit incomplete status with failing semantic tests; a comments-only scaffold must never be reported as completed behavior.

Independent proof: Ajv for descriptor/payload fixtures; existing nightly rustc for generated module/enum/trait structure and compile checks; existing schema/language parsers where available. No new artifact runtime dependency.

### FND-DERIVE-02 — Compile-Time Direct Descriptor and From Contract

Write set after freeze:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` and its current duplicate `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`, or a single-authority extraction agreed with the assembly owner.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`: only the frozen direct-leaf metadata contract, const validation helpers, and the existing derive-law test region.
- New neutral `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mutations/🧫️fixtures/🔣️.json`.
- Derive test module and existing `📜️script.ts`/Nx test route if needed; do not add a separate executable script.

Red tests first: valid direct leaf produces From and delegates all existing methods; inline/unit/multi-field/non-direct wrapped payloads reject; missing/wrong owner descriptor rejects; duplicate kinds/opcodes/tags reject; malformed/unknown/behavior attributes reject; generated registries exactly equal leaf descriptors. Extract a pure `expand_mutations(&syn::DeriveInput)` seam so tests can invoke codegen without the proc_macro entry API, then parse output with existing syn and validate the same language-neutral identity matrix with nightly rustc. Add real compile-fail cases for const-only checks after the Cargo gate is serialized.

Freeze boundary: the current MutationKind has no direct owner/wire descriptor obligation. Decide the one authoritative Rust representation and its correspondence to `🔣️component.json` before adding required metadata. Whether a required leaf descriptor trait or explicit descriptor-path metadata is chosen, rejecting absent metadata requires updating every deriving consumer; do not hide that fan-out behind a default, compatibility layer, or partial opt-in. This propagation is a coordinator-assigned root wave, not an unannounced small shared edit.

### FND-INVENTORY-02 — Consumer Graph and Assignment Evidence

Write set after freeze:

- Root `📜️script.ts`: only MutationTaxonomyRecord/Inventory and inventory helpers/workflow routing.
- Same TypeScript test file: inventory tests only.
- New neutral `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️.json` and a schema for the enriched inventory if the existing outer-shell schema is formalized.

Red fixture: two owners in separate source roots; one external command, editor, viewer, catalog, and registry; sibling operations; a foreign leaf reference; identity-like comments that must not create edges; shuffled input order; byte-only behavior change; explicit ticket assignment rows; empty/missing/conflicting assignment data; excluded virtual paths. Expected JSON names exact source and target owner paths, per-leaf surfaces, assigned agents, independent structural shape and execution states, and evidence provenance.

Implementation seam: build one compose-excluding taxonomy source index and reuse the frozen lexical/AST identity facts to resolve imports/mounts/reexports. Join assignments from a schema-validated ticket ledger keyed by exact mutation root/leaf path; never invent agent names. Keep source facts, byte fingerprints, and human execution/assignment metadata separately deterministic. Thread the existing progress/cancellation context through scan and graph construction; preserve requested scope before reading files.

Independent proof: fast-glob's no-symlink source roster plus Ajv on the full expected records; nightly/syn for Rust mounts and identity facts. Existing package test dependencies already supply these oracles.

### FND-TERMINALITY-02 — No False Committed Apply

Write set after freeze: root MutationTaxonomyWorkflow and its TypeScript tests/neutral inventory fixture only.

Red tests first: direct-but-violating records remain unresolved; a changed source byte invalidates an old plan; mismatched baseline/inventory digest rejects; cancellation leaves no committed artifact; a zero-move plan with current violations cannot commit; only fresh zero-violation terminal verification produces committed. Reuse the generic taxonomy transaction state/result model instead of the present special-case committed literal.

## Gate Order and Commands

1. Freeze owner/descriptor metadata and structured edit boundaries with the coordinator.
2. Land test-first scaffolder and terminality changes under one root-script writer; inventory can be prepared independently but not edit the same section concurrently.
3. Land derive codegen tests/contract and explicitly assigned consumer propagation; serialize Cargo.
4. Re-run the focused existing regression target with all new fixture selections, then the scoped generated-owner gate and global mutation verification.

Existing commands to use after implementation (not run by this audit):

```sh
bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership|direct mutation taxonomy'
bun nx run @semio-tech/dsl-derive-rs:test-quick
bun nx run @semio-tech/framework-os-kernel:test -- derive_mutations_wires_mutation_and_semantic_mutation
bun nx run workspace:clean-taxonomy-verify -- --kind mutation --ticket 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL
```

Confirm the OS router's filter forwarding before relying on the final positional test filter. No Cargo command was run in this audit. No test pass beyond the in-memory probe is claimed.

## Executed Audit Evidence

Read-only rg/sed inspection covered all exact sources named above, Cargo/project metadata, and launch registration. A first attempted historical SPR package path did not exist; the actual owner mount was resolved at `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs:177` and no conclusion relied on the missing path.

The following real helper/plan probe executed with Bun, uses in-memory filesystem substitutes, and made no production writes. It returned missing MutationKind, unit payload, null requested binary tag, enum-attribute displacement, and a direct-but-violating empty plan. Output is saved in `🧪️foundation-completeness-probe.log`.

```typescript
import {readFileSync}from"node:fs";import{join}from"node:path";import{planMutationTaxonomy}from"./📜️script.ts";const source=readFileSync("📜️script.ts","utf8");const extract=(name)=>{const start=source.indexOf("function "+name+"("),end=source.indexOf("\n}\n",start)+2;return source.slice(start,end);};let aggregate="#[derive(Clone, Debug)]\n#[serde(tag = \"mutation\")]\npub enum ProbeMutation {}\n";const factory=new Function("join","existsSync","readFileSync","writeFileSync","POLICY_RS_COMPONENT_LEAF_NAME","NEW_SCAFFOLD_MARKER","NEW_SCAFFOLD_TICKET_PATH",new Bun.Transpiler({loader:"ts"}).transformSync([extract("newMutationRustLeaf"),extract("newMutationDescriptor"),extract("newMutationUpdateAggregate")].join("\n"))+"\nreturn {newMutationRustLeaf,newMutationDescriptor,newMutationUpdateAggregate};");const helpers=factory(join,()=>true,()=>aggregate,(_p,value)=>aggregate=value,"🦀️component.rs","probe","ticket");const parts={emoji:"➕️",semanticKind:"insert-page",moduleName:"insert_page",variantName:"InsertPage",verb:"insert",entity:"page"};const leaf=helpers.newMutationRustLeaf(parts),descriptor=JSON.parse(helpers.newMutationDescriptor("test/🧬️mutations/➕️insert-page",parts,{text:true,binary:true,jsonSchema:true}));helpers.newMutationUpdateAggregate("/virtual","test/🧬️mutations","➕️insert-page",parts,false);const inventory={schemaVersion:1,kind:"mutation",sourceTreeDigest:"0".repeat(64),roots:["test/🧬️mutations"],records:[{state:"direct",mutationRootPath:"test/🧬️mutations",targetMutationDirectoryName:"➕️insert-page",violationClasses:["mutation/behavior-ownership"],evidence:[]}],violations:[{kind:"mutation/behavior-ownership",scope:"test/🧬️mutations/➕️insert-page"}]};const plan=planMutationTaxonomy(inventory,"d03b1fdb6da7c4ea97043e5618d8f4098a43dff7");console.log(JSON.stringify({leafHasMutationKind:leaf.includes("impl protocol::MutationKind"),unitPayload:leaf.includes("pub struct Mutation;"),binaryRequestedTag:descriptor.binaryTag,aggregate,invalidDirectPlan:{moves:plan.moves,unresolved:plan.unresolved,inventoryViolationCount:inventory.violations.length}},null,2));
```
