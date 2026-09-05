# Norm Post-MutationLeaf Compile Frontier — Live Audit

**Scope.** `semio-s-plugin-norm` after the split `MutationLeaf` work, with the native D1 fan-in as the downstream consumer. This is a read-only audit on 2026-09-04. No Cargo build was started: a concurrent owner is changing the same UI sources and the prior isolated target was removed during the audit.

## Verdict

**RED — the taxonomy packet is source-proved, but the first causal Rust contract is still direct-leaf-only.** The neutral taxonomy gate accepts all **392** current payloads (**371 direct + 21 split**) and was independently run uncached by this audit:

```text
bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-check --skip-nx-cache
norm mutation-leaf taxonomy is fresh: 392 payloads, AJV schema and hostile vectors passed
NX Successfully ran target mutation-leaf-taxonomy-check for project @semio-tech/norm-plugin
```

That is source/fixture proof only. It cannot prove the generated Rust aggregate contract. The 21 ISO-16757 split leaf sources are now treated as canonical by the taxonomy reader and by the derive-time leaf discovery, yet `MutationLeafSourceScope` still permits only `owner/sourceFilename`. Consequently `#[derive(Mutations)]` const validation rejects the split provenance before native D1 can reach its assertions.

The smallest correct first packet is **one framework source-authority extension**, not moving 21 canonical payload files back to a direct layout and not weakening aggregate validation.

## Evidence quality and the moving frontier

The coordinator-owned isolated Norm check that finished at 03:23:11 retained a terminal of **53 errors, 213 warnings** before its generated target was concurrently removed. Its Rust error buckets were:

| Code | Count | Current interpretation |
| --- | ---: | --- |
| `E0080` | 3 | Current source RED: one split provenance contract and two stale `ChangeAnnex` physical leaf identities. |
| `E0046` | 3 | Current source RED: generic artifact, shared config and shared presence mutations omit required descriptors. |
| `E0277` | 2 | Current source RED: EN 1990 hashes an unsized slice through `ToValue`. |
| `E0308` | 30 | Historical terminal during the UI conversion; live source has since changed its leaf builders. It must be recompiled, not re-counted as a current terminal. |
| `E0631` | 15 | Historical terminal during the viewer conversion; likewise needs a fresh terminal. |

The historical terminal is useful for dependency ordering, but not runtime evidence. In the live snapshot, all **30** `ArtifactEditor`/`ArtifactViewer` roots still declare synchronous `fn render`, while the framework contract is `async fn render` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11303-11310`. The body leaf migration is actively changing, so only the still-observable root and test defects below are current claims.

## P0 — close the split-leaf source-authority mismatch first

The source authority parser already recognizes exactly two canonical locations:

- a direct primary at `<owner>/🦀️.rs`;
- a split primary at `<owner>/🦠️mutation/🦀️.rs`.

That decision is explicit in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:59-98`; it obtains the taxonomy's `mutation_payload_facet` at :70-74 and accepts it at :79-91. The neutral Norm taxonomy applies the same rule at `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts:45-75`.

The generated aggregate drops that fact. `MutationAggregateSourceAuthority` has no payload-facet field (:49-57), and `derive(Mutations)` emits a `MutationLeafSourceScope` with only root, taxonomy, source filename and descriptor filename (:1841-1846). The kernel scope has the same omission at `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:624-632`; its validator calls the direct-only matcher at :680-684, whose implementation requires exactly `owner + '/' + filename` at :735-743. This is the immediate reason the ISO aggregate's first split leaf fails const evaluation.

### Minimal implementation boundary

1. Carry the single taxonomy-selected `mutation_payload_facet` from `MutationAuthorityCommon` through `MutationAggregateSourceAuthority`, the derive expansion, and `MutationLeafSourceScope`.
2. Make only `provenance.source_path` accept either exact canonical form: `owner/sourceFilename` or `owner/payloadFacet/sourceFilename`. Keep `descriptor_path` exact-direct at `owner/descriptorFilename`, and keep `descriptor.owner` an immediate child of `mutationRoot`.
3. Apply the existing lexical constraints to the facet: one nonempty portable segment; no separator, traversal, or ASCII-case-insensitive `compose`. Do not permit arbitrary depth, multiple facets, or a fallback path.
4. Keep workspace-token, root, owner, taxonomy and descriptor comparisons unchanged. This expands the authenticated/const source grammar by one taxonomy-owned segment; it does not relax identity.

### Required proof

Extend the kernel's existing source-contract fixture/law near `mutation/🦀️.rs:825-846` with one positive direct row and one positive split row using the identical descriptor owner. Reject a wrong facet, a nested facet, altered token/root/taxonomy/owner, a direct descriptor below the facet, and a noncanonical filename. Add derive tests proving direct and split discovery emit the corresponding scopes. Then run the neutral taxonomy target above and an exact Rust law selected by full name before running the package check.

## P1 — repair the two real stale physical leaf identities

The En 1990 and En 1992 `ChangeAnnex` payloads still live in `🐷set-snapshot` and `🐝set-snapshot` mutation triads. Their own files describe this as a repurposed pre-migration directory while declaring `ChangeAnnex` / semantic `change-annex`; examples are:

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/…/🧬️mutations/🐷set-snapshot/🦀️.rs:1-35`;
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️🟢️en1992/…/🧬️mutations/🐝set-snapshot/🦀️.rs:1-35`.

This is a source identity defect, not a reason to rename the valid editor command `set-snapshot`. The latter remains a user-level document replacement command and can correctly dispatch a concrete `ChangeAnnex` mutation.

Atomically rename both physical triads to `change-annex`, updating their aggregate `mod`/`#[path]` wiring, component/diff/inverse imports, test fixture `include_str!` paths, descriptor/provenance source paths, and the committed taxonomy/oracle rows. The two triads have **11 files each** in the current tree. Do not create aliases or retain stale module names.

The neutral law must prove both canonical physical source identities and a separate command vector proving `set-snapshot` still emits `ChangeAnnex`, applies only the annex diff and inverts exactly once. It must reject a physical `set-snapshot` leaf whose descriptor claims `change-annex`.

## P2 — restore truthful mutation descriptors

`Mutation` requires `DESCRIPTORS` and `descriptor`; see `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145-151`. Three live manual implementations omit both.

| Subject | Anchor | Smallest honest repair |
| --- | --- | --- |
| `SetArtifactMutation<D>` | `✏️s/🔌️plugins/📕️norm/🗿️🛰️🐝️artifact/🦀️.rs:563-589` | Remove the generic whole-document mutation, its `commit_document` helper and DemoFamily-only tests if no concrete Norm caller exists. A static protocol descriptor cannot truthfully identify a generic `D` leaf. Do not invent `set-document` metadata. |
| `NormConfigMutation` | `✏️s/🔌️plugins/📕️norm/🎚️config/🦀️.rs:87-101,158` | Add two distinct static descriptors and `descriptor` matching the already concrete variants, with validated multi-segment semantic kinds, payload schemas and stable op ordinal/inverse behavior. Explicitly const-validate the manual descriptors. |
| `NormPresenceMutation::Noop` | `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️.rs:60-79` | Do not expose an externally named mutation that carries no state. The type is selected by all 15 editors. Either make a real, bounded `SetPresence { presence }` descriptor/diff/inverse and add actual publishing, or introduce a framework-supported no-presence capability after proving the association can be absent. A fake descriptor for `Noop` is a protocol bypass. |

For each retained manual mutation, the exact law must verify descriptor uniqueness/semantic schema, text and binary encode/decode, no-op behavior, apply and inverse. A language-neutral fixture must reject an undeclared variant, wrong descriptor kind, wrong payload and a malformed/oversized record.

## P3 — finish the actual UI assembly boundary, not an imaginary `Assembly::render`

There is no current Norm `Assembly::render` symbol. The load-bearing boundary is the 15 `ArtifactEditor` and 15 `ArtifactViewer` implementations, consumed by the framework's async `render` trait.

Live census:

- **30 / 30** roots still use synchronous `fn render`; none use `async fn render`.
- **15** viewer tests still serialize the un-awaited trait call at each viewer root `🦀️.rs:111` (EN 1990 is representative at `…/📘️en1990/…/👁️viewer/🦀️.rs:107-113`).
- The framework helpers deliberately admit a fallible `BuiltNode` once into `ComponentTree`: `built_to_component_tree` at `…/🔌️plugin/🦀️.rs:333-355`.

The current editor/viewer sources already show a partial body-leaf conversion to `UiAssemblyResult<BuiltNode>`; do not roll it back or reintroduce `UiNode`. Complete the packet root-inward:

1. change every root implementation to `async fn render` with `UiAssemblyResult<ComponentTree>`;
2. retain fallible body renderers and convert with `built_to_component_tree` exactly once at the root;
3. propagate table/panel admission failures with `?`/`map`, never `unwrap` or a silently empty tree;
4. await the 15 viewer test calls and assert the returned `Result<ComponentTree>`; and
5. make the unknown-body path exercise the existing bounded text/error contract.

Use one editor, one table viewer and one non-table/view-family representative as exact render laws. The neutral `surface-render-contract/v1` corpus should name `{dialect, role, bodyKey, expectedRootKind}` and include unknown body plus oversized text/admission failure; its Node reader must not import Rust. The Rust law renders real apps and compares the component-tree projection without claiming GPU presentation.

## P4 — independent small source repair

`en1990_qk_scene_id` passes `&[En1990QkEntry]` to `ToValue`, which is implemented for the owned vector rather than the unsized slice: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️.rs:71-76`. Encode an owned `entries.to_vec()` at that boundary. The law must establish same order/content produces the same scene id and that empty/nonempty and changed-row vectors differ as expected. This has no connection to `MutationLeaf` and can land independently.

## Dependency order and acceptance

| Order | Owner packet | Acceptance boundary |
| --- | --- | --- |
| 0 | P0 split source-authority bridge | exact kernel + derive direct/split source laws and taxonomy check green |
| 1 | P1 stale `ChangeAnnex` physical identities | exact source/semantic neutral corpus and corresponding Rust law green |
| 2 | P2 manual descriptor ownership | real descriptor/text/binary/inverse law green; no fake generic or noop protocol leaf |
| 3 | P4 EN 1990 owned-vector codec repair | focused deterministic scene-id law green |
| 4 | P3 async ComponentTree boundary | real editor/viewer render laws green, then a fresh package compiler terminal |
| 5 | Norm package and native fan-in | `bun nx run @semio-tech/norm-plugin:check --skip-nx-cache`, then the registered uncached WGPU native frontier and finally native D1 gate |

The current `mutation-leaf-taxonomy-check` is a good neutral freshness/hostile gate, but it selects no Rust law. Extend the existing `📜️script.ts` with a registered exact-one contract command: list every intended suffix, require exactly one fully-qualified law for each, then run them `--exact` after the neutral oracle. Do not count test selection or compilation as execution.

## Nonclaims

- This audit did not run Cargo, WGPU, native D1, or a real renderer.
- The 53-error compiler terminal is coordinator historical evidence; its output directory was removed and UI source changed afterwards. A fresh package terminal is required before claiming a residual error count or a closed UI lane.
- The green 392-row Bun/AJV gate proves current source/fixture agreement and hostile fixture rejection, not Rust aggregate/source validation or native readiness.
- No compatibility alias, fake descriptor, blanket leaf implementation, or source-path fallback belongs in these packets.

## Follow-up — post-root-repair canonical-test and WGPU frontier

**Fresh source snapshot.** After the report above was filed, only `✏️s/🔌️plugins/📕️norm/{🗿️🛰️🐝️artifact,🎚️config,👥️presence}/🦀️.rs` changed in the Norm lane. There is still no fresh Norm Cargo terminal. The source changes mechanically supply the three formerly absent `Mutation::{DESCRIPTORS, descriptor}` implementations, but do **not** provide canonical source authority:

| New descriptor owner | Source declaration | Physical owner directory now | Verdict |
| --- | --- | --- | --- |
| `…/🗿️🛰️🐝️artifact/🧬️mutations/📤️set-document` | `artifact/🦀️.rs:575-594` | absent | RED |
| `…/🎚️config/🧬️mutations/{📄️snapshot,☑️selected-check}` | `config/🦀️.rs:161-200` | both absent | RED |
| `…/👥️presence/🧬️mutations/🫧️noop` | `presence/🦀️.rs:72-90` | absent | RED |

The generic `SetArtifactMutation<D>` is still used only by its DemoFamily/test path and the unused `app_surface::commit_document` helper (`artifact/🦀️.rs:568-609,909-980`; `🖥️app-surface/🦀️.rs:287-288`). It cannot truthfully be one direct `set-document` leaf for every `D`. The manual descriptor validator verifies lexical field shape, not filesystem/provenance existence (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:385-419`), and these new manual entries have no explicit `validate()` assertion or source-identity law. Thus they may remove `E0046` in a future compile while making a false canonical descriptor claim.

`NormPresenceMutation::Noop` is similarly now advertised as `presence-noop` even though its diff always produces default presence and no current Norm command publishes a presence payload (`👥️presence/🦀️.rs:65-99`; the 15 editors merely select it as their associated type). It is not an accepted collaborator-presence surface.

### Coherent remaining repair order

1. **P0 source authority.** Implement the direct-or-one-taxonomy-facet provenance bridge described above. It is still unchanged: `MutationLeafSourceScope` and its generated initializer contain no payload facet, while the derive parser does. This is the first actual aggregate blocker.
2. **P1 canonical source identities.** Rename the two `ChangeAnnex` physical `set-snapshot` triads atomically. Both stale directories remain live; do not let a future compile erase the identity mismatch with aliases.
3. **P2 truthful manual mutation ownership.** Remove the unused generic whole-document bridge and its Demo-only protocol claims, or make any retained mutation concrete and source-owned with a real descriptor/schema/provenance fixture. For config, materialize the two real operations as canonical source-owned leaves and const-validate their descriptors. For presence, either use the framework no-presence type for all 15 editors or implement a real bounded `SetPresence` publication path; do not publish a `Noop` leaf. This is a semantic acceptance condition, even if the mechanical methods compile.
4. **P3 renderer completion.** The live census is still **30 synchronous root renders, zero async roots, and 15 un-awaited viewer trait calls**. The framework requires `async fn render` at `…/🔌️plugin/🦀️.rs:11303-11310`; EN 1990 remains representative at `…/✏️editor/🦀️.rs:110-120` and `…/👁️viewer/🦀️.rs:107-113`. Finish the existing fallible `BuiltNode → ComponentTree` conversion by making all roots async and awaiting all 15 test calls. Do not restore `UiNode` or hide errors.
5. **P4 independent codec.** Replace EN 1990's unsized slice handed to `ToValue` at `…/📘️en1990/🦀️.rs:71-76` with the owned vector boundary and add deterministic scene-id vectors.

Only after P0–P4 has a fresh compile terminal should another plugin's old diagnostic be considered. No source-only audit can truthfully predict the first post-Norm external crate error.

### Registered-gate defect: test-level targets are currently aliases

`@semio-tech/norm-plugin:test-quick`, `:test-long` and `:test-exhaustive` invoke `bun ./📜️script.ts test <level>` (`📋️project.json:16-45`), but `TestScript.run(_segments)` discards `<level>` (`📜️script.ts:90-93`). `runCargoTestBudgeted` instead reads `SEMIO_TEST_LEVEL` (`🧰️framework/…/📚️library/📦️packages/🟦️typescript/🟦️.ts:1635-1645`). The shared `resolveTestLevel` exists precisely to set that environment from the first segment (:1150-1160), but Norm does not call it. Therefore the three named targets all silently run whichever ambient level is set (default `fundamental`); they cannot currently prove the named canonical level.

The smallest gate packet is to call `resolveTestLevel(segments)`, pass its remaining arguments through to `runCargoTestBudgeted`, and add a script-level test that spies the spawned invocation for each target. Add a separate `mutation-leaf-contract-check` registered target which first runs the existing independent AJV fixture, then lists every intended Rust suffix, requires exactly one fully-qualified name, and runs each `--exact`. The existing 392-row target remains valuable but contains no Rust execution.

### Exact proof ladder after the repair packets

1. `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-check --skip-nx-cache` — neutral source/schema/hostile proof only.
2. The new exact source-authority/mutation contract target — direct and split positive vectors plus hostile facet, owner, token and stale-route rejection.
3. `bun nx run @semio-tech/norm-plugin:check --skip-nx-cache` — first fresh package compiler terminal.
4. `bun nx run @semio-tech/norm-plugin:test-quick --skip-nx-cache`, then the corrected long/exhaustive targets as appropriate — package assertions at the named level, not selection/compilation alone.
5. `bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker --skip-nx-cache` and `bun nx run @semio-tech/framework-renderer-wgpu:native-build --skip-nx-cache -- --scale` — the latter compiles the real native binary and its transitive fan-in, but its source expressly skips the plugin WASM program, catalog, asset server and GPU/winit path in scale mode (`…/📜️script.ts:273-309`). It is a compilation frontier, not Norm activation/runtime proof.
6. `bun nx run os-hub:native-document-open-check --skip-nx-cache` — current D1 orchestration independently runs plan/origin proof, frame-worker freshness, the scale build, seven exact kernel laws and the SocketGrant actor proof (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:3069-3091`). It still does not activate a Norm app because its scale stage intentionally has no catalog.
7. Add a registered Norm smoke matrix before claiming native Norm runtime: for each of the 15 catalogue variants, drive the existing real runner as `bun nx run @semio-tech/framework-renderer-wgpu:native --skip-nx-cache -- <variant> --smoke`. That runner builds the selected WASM program, resolves the matching catalogue app and boots the native binary headlessly (`…/📜️script.ts:323-350`). One EN 1990 smoke is a useful representative, but it cannot prove all fifteen variant/app rows. The matrix must assert selected app identity, bounded component-tree output, no secret carrier, and one unknown-body failure per role; it is distinct from document SocketGrant runtime.

The first possible qualified result is therefore **Norm package + canonical test green** at step 4, then **native compile green** at step 5. A full native D1 acceptance additionally requires step 6; an honest claim that all Norm catalogue apps activate requires the missing step-7 matrix. None has been executed by this audit.

## Current source correction — canonical config and ChangeAnnex rename

**Current verdict: RED, but no longer for the former split-leaf or async-render claims.** This is a
read-only source audit after the payload-facet authority, physical `ChangeAnnex` moves, and
`NoPresence` cutover landed. No Cargo or renderer process was run.

### Superseded observations

- **Split-leaf P0 is source-closed.** `MutationLeafSourceScope` now carries
  `mutation_payload_facet`, validates it as one canonical segment, and accepts only the direct or
  one-facet source form at
  `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:626-685,749-765`.
  The derive authority now retains and emits that same fact at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:49-57,102-115,1849-1858`.
  Its hostile fixture accepts the single split form and rejects a wrong or nested facet at
  `…/🎮️mutation/🦀️.rs:854-865`. The earlier direct-only finding is superseded; a fresh Rust
  terminal is still required for runtime/compile credit.
- **The Norm roots use the correct synchronous contracts.** `ArtifactEditor::render` and
  `ArtifactViewer::render` are synchronous at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26140,26329`; the async method at
  `:11303-11310` belongs to the older/general `PluginApp` trait, not these root traits. The live
  census is 15 editors and 15 viewers, all with synchronous `ComponentTree` result roots. The
  earlier async/await remediation is withdrawn.
- **NoPresence is source-correct.** All 15 editor roots and all 15 viewer roots select
  `NoPresence`/`NoPresenceMutation`; there is no
  `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️.rs` runtime producer. The retained five leaves are an
  honest empty app-schema facet (`additionalProperties: false`, zero properties) used by
  `config/schema/🦀️.rs:22-42`; they do not claim collaboration presence.
- **The level alias defect is source-closed.** The Norm Rust script now calls
  `resolveTestLevel` at `📦️packages/🦀️rust/📜️script.ts:90-94`; that helper sets
  `SEMIO_TEST_LEVEL` for child processes at the shared script library `:1150-1160`.

### Primary remaining config boundary

The live `NormConfigMutation` remains an inline, hand-authored two-variant protocol at
`✏️s/🔌️plugins/📕️norm/🎚️config/🦀️.rs:90-221`. Its descriptor owners name
`🎚️config/🧬️mutations/{📄️snapshot,☑️selected-check}`, but that physical collection does not
exist; the actual config schema has only the five config facet leaves. This is still a false source
ownership claim.

The proposed replacement topology is valid, with two non-optional details:

1. Put the generated aggregate directly at
   `🎚️config/🧬️schema/🧬️mutations/🦀️.rs`. The source authority permits its single
   `ChangeSelectedCheckIndex` leaf either at `<owner>/🦀️.rs` or at
   `<owner>/🦠️mutation/🦀️.rs`; `🔣️.json` must remain directly below that leaf owner.
2. `dsl::Mutations` generates `Mutation`, not `OpText` or `OpBinary` (the derive P6 boundary is
   explicit at `…/🗣️dsl/✨️derive/🦀️.rs:1-6,1646-1655`). The new aggregate therefore also needs
   schema-owned canonical text/binary codec code. It must publish only
   `change-selected-check-index`; it must not retain `Snapshot`, old ordinals, or a compatibility
   decoder.

The concrete leaf must require a bounded `newSelectedCheckIndex` value or explicit JSON `null` to
clear it, derive its descriptor from its physical `🔣️.json`, construct the corresponding one-field
`NormConfig` diff, and inverse to the same concrete leaf using the base value. The current config
JSON schema accepts an absent property but only an integer when present
(`🎚️config/🧬️schema/🔣️.json:1-13`), whereas the Rust field is `Option<u32>`; the new neutral
fixture must declare and prove the intended missing-versus-null clearing contract rather than leave
that cross-language rule implicit.

Atomic consumers are bounded: there are exactly **17** direct
`NormConfigMutation::SetSelectedCheckIndex` references — the inline config, the shared
`🖥️app-surface/🦀️.rs:288-290`, and one selected-check command/test in each of 15 editors. No
external `Snapshot` constructor remains. Wire the schema module from
`📦️packages/🦀️rust/🦀️.rs:46-52`, re-export the generated aggregate to the existing config
callers, rename those 17 sites atomically, and delete the old byte-baseline test rather than pin a
wire format that the greenfield cutover intentionally removes.

The existing `mutation-leaf-taxonomy-check` cannot validate this new leaf: its reader deliberately
walks only `🗿️artifacts` in `📦️packages/🦀️rust/📜️script.ts:31-75`. Add a separate config-leaf
AJV/oracle target instead of weakening that artifact-row schema. The new target must reject missing
descriptor/schema, a wrong owner/facet, unknown variant, negative/overflow index, ambiguous
missing/null encoding, and stale `Snapshot`; then select exactly one Rust law for apply, no-op,
clear, inverse, text and binary rejection. The existing artifact taxonomy target is still useful,
but it is not evidence for a shared config leaf.

### EN 1990 / EN 1992 ChangeAnnex: physical move complete, identity cutover RED

The production physical rename itself is correct: neither artifact retains a
`…/🧬️schema/🧬️mutations/*set-snapshot` directory; the leaf descriptors carry
`owner`, `semanticKind`, display name, and variant `ChangeAnnex` under respectively
`🐷change-annex/🔣️.json` and `🐝change-annex/🔣️.json`. The generated 392-row taxonomy also names
`change-annex` with those current source paths. The user-facing editor command
`🎮️commands/📤️set-snapshot` remains separate and is intentionally **not** part of this rename.

The semantic identity is nevertheless split in the committed oracle and differential suites:

| Scope | Current source evidence | Required bounded correction |
| --- | --- | --- |
| EN 1990 oracle | `…/📘️en1990/…/🧪️oracle/🔣️.json:168,180,387,394` still uses `set-snapshot` as `mutationId`, `kinds` entry, catalog `id`, and `productionDispatch.operation`; its two directory fields are already `🐷change-annex`. | Change exactly those four mutation-vocabulary fields to `change-annex`; do not alter the editor command catalog. |
| EN 1992 oracle | `…/📘️🟢️en1992/…/🧪️oracle/🔣️.json:443,455,1149,1156` has the identical four stale semantic values while its directory fields already say `🐝change-annex`. | Make the same four-field change. |
| EN 1990 differential feature | `…/🧪️🧪️🏔️🦋️tests/🐸️mutate-en1990-1/🥒️.feature:70-73,93-96` points fixture assets at nonexistent `🧪️🧪️🏔️🦋️tests`; the physical triad uses `🧪️tests`. Its examples at `:78,101` still call the row `set-snapshot`. | Replace only fixture path components with `🧪️tests` and both example ids with `change-annex`. |
| EN 1992 differential feature | `…/🧪️🧪️🏔️🦋️tests/🍋️mutate-en1992-1/🥒️.feature:57-60,105-108,65,113` has the equivalent stale paths and two stale example ids. | Same direct substitution. |
| Rust differential subjects | EN 1990 `…/🐸️mutate-en1990-1/🦀️.rs:91-148` has **40** nonexistent `include_str!` paths; EN 1992 `…/🍋️mutate-en1992-1/🦀️.rs:114-321` has **140**. Both already use `change-annex` and the renamed directory for the annex row. | Replace the bad fixture component in all 180 strings, not just the annex four. This is a fixture-root repair, not a new source layout. |

Both Python adapters are already coherent: their `KINDS` and `VECTORS` name `change-annex` and the
new directory. The accidental test weakness is that the aggregate checks at
`…/📘️en1990/…/🧬️mutations/🦀️.rs:395-404` and
`…/📘️🟢️en1992/…/🧬️mutations/🦀️.rs:470-479` merely search the oracle text for each `KINDS` word.
That passes even while the semantic catalog remains `set-snapshot`, because `change-annex` occurs
in `sourceMutationDirectoryName`. Parse the oracle and require equality among aggregate descriptor
kind, `mutationId`, `kinds`, catalog `id`, dispatch operation, and the feature/Python row; reject a
directory name containing `change-annex` paired with any different semantic id.

No direct launch/script/project reference to either `mutate-en1990-1` or `mutate-en1992-1` exists
in the current tree. Before counting either feature as runtime proof, register one exact,
non-vacuous repo-test subject target that discovers the two feature files, requires the expected
expanded example count, and runs the independent Python role plus Rust subject. The correction is
complete only when that gate executes after the 180 fixture includes and eight stale oracle ids are
fixed; the 392-row taxonomy check alone does not inspect any of these values.

## Current source audit — Norm public ComponentTree roots and native smoke matrix

**Verdict: source-PASS for the synchronous fallible `ComponentTree` migration; RED for localized
body rendering, a public all-30-root render proof, and viewer native activation.** This is a
read-only source census; no Cargo or WGPU command was run.

### Public return boundary

The framework contracts are synchronous: `ArtifactEditor::render` is at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26140` and `ArtifactViewer::render` at
`:26329`. `EditorApp` and `ViewerApp` are the public `ArtifactApp` adapters at `:26434-26456` and
`:26701-26720`; their asynchronous host call delegates to those synchronous author roots. The
fallible transfer is real rather than a test alias: `built_to_component_tree` and its text fallback
consume a `BuiltNode` into `ComponentTree` at `:326-358`.

The live Norm census finds exactly fifteen `impl ArtifactEditor` and fifteen `impl ArtifactViewer`
roots. Every editor maps its five declared body keys (`inputs`, `results`, `document`, `catalogue`,
`inspection`) through `built_to_component_tree`; every viewer maps `report` through the same helper
and its unknown-body fallback through `built_text_to_component_tree`. `📕️norm/🦀️.rs:9-43` gives
the corresponding thirty concrete `NormApps` variants and `:46-` registers each artifact/editor/
viewer pairing. Thus the ComponentTree propagation itself is source-complete.

This does **not** amount to public-root execution coverage. Each editor's existing all-body test is
inside a `#[cfg(test)] pub(crate) mod testkit` (for example EN 1990
`…/📘️en1990/…/✏️editor/🦀️.rs:195,310-326`), so an integration target cannot use it; there are
fifteen such editor tests. The fifteen viewer root tests exercise only the unknown `"nope"` fallback
(for example `…/📘️en1990/…/👁️viewer/🦀️.rs:107-113`), not the public `report` success root.

### Language and accessibility boundary

Norm's manifest chrome has English/German pairs — e.g. View/Ansicht and Edit/Bearbeiten at
`📕️norm/🖥️app-surface/🦀️.rs:35,120` — but the body builders use literal English headings, empty
messages, summaries, catalogue labels, and unknown-body text at `:53-113,188-198`. This cannot be
fixed or proved by a root test alone: the host `ViewModel` locale is used only for framework history
at `🔌️plugin/🦀️.rs:24577-24655` and is not passed into `ArtifactEditor::render` or
`ArtifactViewer::render`; `NormConfig` carries only `selected_check_index`
(`📕️norm/🎚️config/🦀️.rs:23-27`). Native German can therefore localize host chrome but cannot
localize a Norm body. A host-owned render-context/language axis must precede an EN/DE body claim.

The source census also found no accessibility projection, accessible-name/role bridge, or native
accessibility gate for these ComponentTrees. A typed UI tree is useful structure, but is not runtime
evidence of an assistive-technology surface. Treat accessibility as unproven until the renderer
projects and checks this information.

### Bounded honest public API proof

Add one external Norm integration target, not another root `cfg(test)` kit. Mirror the existing
explicit `[[test]]` registration in `📕️norm/📦️packages/🦀️rust/Cargo.toml`, with a physical
`🧪️tests` source that imports only public framework/Norm APIs. It must create every concrete
`VcsArtifactApp<EditorApp<T>>` and `VcsArtifactApp<ViewerApp<T>>` through `new`; that invokes the
real `initial_snapshot` and `initial_config` path (`🔌️plugin/🦀️.rs:19083-19110`). Obtain the
declared keys from each public app definition, then call the public async app render boundary for
all five editor keys and the single viewer report key: **30 real wrappers / 90 successful root
renders**. Require a nonempty structural ComponentTree and no fallback/error, and separately make
an unknown-key negative assertion. This is production-library compilation plus an external harness;
it must not depend on the private testkits or blanket-enable test-only behavior.

Register `surface-render-check` in `📕️norm/📦️packages/🦀️rust/📜️script.ts`: first run a neutral
corpus that asserts the 15 variants, 30 role/app identities, and 90 known `{role,body}` pairs,
rejecting duplicate ids, an editor missing a declared panel, viewer/editor key confusion, and an
unknown role/key. Then list the Rust test, require exactly one fully qualified law, and run it with
`--exact --test-threads=1`. The neutral corpus validates inventory/selection only; the external
Rust target supplies the public execution proof. After a render-context repair, extend the corpus
and target with EN and DE semantic-label vectors; do not call today's identical English body a
bilingual pass.

### Native runtime matrix

The WGPU native runner is real: `📺️renderer/…/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:299-350`
builds the selected native binary and plugin program, resolves the catalog app, and passes `--smoke`
for a headless widget-tree dump. Its Nx target delegates `native` at the matching
`📋️project.json:114-120`. Current playground metadata in
`📕️norm/📦️packages/🦀️rust/Cargo.toml` contains fifteen variants, **all `#editor`**; generated
`.vscode/launch.json` consequently has fifteen `🛠️dev🧩️<variant>🧊️wgpu🖥️native` entries (the
EN 1990 entry begins at line 7045) and no Norm `#viewer` entry. The authoritative generation rule
is explicit: `🔌️plugin/📇️registry/🖥️launch.ts:2-6,193-227` derives entries from the seed plus
these playground rows; generated launch JSON is not an owner.

Consequently, once package fan-in is green, the registered editor smoke is
`bun nx run @semio-tech/framework-renderer-wgpu:native --skip-nx-cache -- <variant> --smoke` for
each of the fifteen metadata variants. There is no current registered viewer selector or 30-root
native matrix. Add viewer catalog/playground identities only if the intended runtime product really
supports a standalone viewer; regenerate and freshness-check launch output, then run the selected
native matrix. Until then, the bounded public integration target is the only truthful proof covering
all thirty roots, and the existing native matrix can claim at most the fifteen editor roots.
