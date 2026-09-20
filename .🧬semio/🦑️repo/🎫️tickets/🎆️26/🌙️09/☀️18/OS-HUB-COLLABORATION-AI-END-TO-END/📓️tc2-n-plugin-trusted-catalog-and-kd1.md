# TC2 — N-plugin trusted catalog + KD1 (kind vs dialect)

Slice TC2 (session 6, 2026-09-20). Owner of outcomes 1+2+3 **beyond gis**: can the hub's trusted
catalog carry a plugin that is not `gis`.

## 0. HUB HANDOFF (top of report)

| field | value |
|---|---|
| **second hub** | **NOT STARTED.** No three-package catalog was bootstrapped — §4 names the two blockers that make it impossible from this slice, both measured, neither of them the 3 h of cold wasm builds I had budgeted for. |
| **LIVE hub on 7611** | untouched. This slice never stopped, restarted, read-modified or wrote to `.🧬semio/🌐hub/gm1-boot`. No hub process was started or killed by TC2 at all. |
| **can a non-gis document be created and open-planned on a hub?** | **Not yet — but the reason changed.** Before this slice the answer was "no, the schema refuses it" (three validators, KD1). That refusal is now **gone at seven product sites** and the two-space pair is a proven, admitted shape. What remains is not a refusal but a *missing* piece: a plugin needs a statically linked native genesis factory in the hub binary, and only `stdio`, `gis` and `vcs` have one. |
| **needs a coordinator hub rerun** | **YES.** Rerun 21:03 gave `321 — 315 / 6`; the two new reds were mine (one fixture row, §8) and are fixed **product-first** — the index entry is now bound by the declared dialect-ownership predicate, which is strictly stronger than the equality I removed. `cargo check -p semio-hub --all-targets` green. |
| **§9** | the design the coordinator asked for: **catalog-carried genesis**, so the hub can create a document of any cataloged plugin kind without linking per-plugin Rust into `os-hub`. Not implemented. |

## 1. Inherited state (measured)

- No `🗑️generated/tc2-*` captures and no TC2 report existed: this slice had no predecessor.
- `git status` on my paths was clean of TC2 edits; the working tree carried GM1's/TC1's uncommitted
  `🌎️hub/📦️packages/🦀️rust/📜️script.ts` hunks (the `--features component-receipt-acceptance` fix and
  the `proveTrustedStdioGisCandidatePlan` rewrite). **Nothing of a peer's was reverted.**
- One transient peer breakage was observed and waited out, not killed: `cargo` reported `E0425` in
  `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` at one moment and compiled
  clean a minute later (preamble rule 3).

## 2. KD1 — decided by measurement: **gis is the outlier, not stdio**

### 2.1 The census

HT1 H13 framed KD1 as "stdio declares kinds in the manifest id space while its dialects are
`s.stdio.*`". The first thing I did was read how *every other* plugin declares the two, because if
stdio were the outlier the fix would be stdio-side. It is not. Every `ArtifactKindSpec::id` in
`✏️s/🔌️plugins`, and every `Dialect::artifact_kind`:

| plugin | `ArtifactKindSpec::id` (manifest / taxonomy space) | `Dialect.artifact_kind` (plugin space) |
|---|---|---|
| note | `2d.note` | `s.note.note` |
| draw | `2d.drawing` | `s.draw.*` |
| writer | `text.document` | `s.writer.writer` |
| raster | `2d.raster` | … |
| cad | `3d.cad` | … |
| flow | `computation.flow` | `s.flow.flow` |
| trinity | `text.rewriting`, `graph.trinity` | … |
| wfc | `2d.wfc2d`, `3d.wfc3d`, … | `s.wfc.wfc2d`, … |
| procedural | `2d.generation`, `3d.generation` | `s.procedural.generation2d`, … |
| fem | `computation.fem2d/3d` | … |
| block | `2d.block`, `3d.block`, `5d.block` | … |
| puzzle | `2d.puzzle`, `5d.puzzle` | … |
| layout / forms / process / remodel / lowpoly / energy / architect / dag / reasoning / playbook / imperative / sequence / mathematical / shooting / demonstrator / vcs | `2d.layout`, `form.dictionary`, `3d.process`, `3d.remodeling`, `3d.lowpoly`, `data.model`, `data.program`, `graph.dag`, `graph.wires`, `text.playbook`, `computation.procedure`, `computation.sequence`, `computation.equation`, `2d.shooting`, `playground.document`, `vcs.vcs` | `s.<plugin>.<artifact>` throughout |
| **stdio** | `stdio.json`, `stdio.png`, … (36 kinds) | `s.stdio.json`, `s.stdio.png`, … |
| **gis** | `id: GISMAP_DIALECT.artifact_kind` ⇒ **`s.gis.gismap`** | `s.gis.gismap` |

**Every plugin in the repo spells the two differently. `gis` alone collapses them**, by writing
`ArtifactKindSpec { id: GISMAP_DIALECT.artifact_kind.into(), … }`
(`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:144`). stdio is **conformant**, not the outlier:
its kind ids are taxonomy ids exactly like note's `2d.note` and writer's `text.document`.

So "make stdio derive both from one constant like gis" would have been the wrong fix — it would have
put stdio out of line with ~40 sibling plugins, and it would still have left note, draw and writer
refused. The hub's equality is the defect: **it admits exactly one plugin, and only by accident.**

### 2.2 The equality was in seven product sites, not three

H13 named three. A full census (`parent_dialect.artifact_kind` / `parentDialect.artifactKind` against
`artifact.kind` / `kind_id`, product code only) found seven, and the two H13 did not name are on the
**open** path, so even a creation that succeeded could never have been open-planned:

| # | site | term | landed |
|---|---|---|---|
| 1 | `📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs` `SpaceArtifactCreationKindV1::validate` | `dialect.artifact_kind == kind_id` | replaced by `identity(&self.dialect.artifact_kind)` |
| 2 | same file, `SpaceArtifactCreationReadyV1::validate` | `parent_dialect.artifact_kind == kind_id` | replaced by `identity(&self.parent_dialect.artifact_kind)` |
| 3 | `📇️directory/🧬️schema/🦀️.rs` `DocumentOpenPlanV1::validate` | `parent_dialect.artifact_kind != artifact.kind` | dropped; the field joins the existing bounded/trim array |
| 4 | same file, `DocumentExecutionTargetLeaseFieldsV1::validate` | same | same |
| 5 | `🌎️hub/🏗️bootstrap/🦀️.rs` `DocumentOpenPlanAuthorityV1::validate` | `parent_dialect.artifact_kind == artifact.kind` | dropped; the field joins the existing bounded/trim array |
| 6 | `🌎️hub/📇️directory/🦀️.rs` `document_index_projection_v1` | `descriptor.artifact_kind != entry.dialect.artifact_kind` | dropped |
| 7 | TS twins: `🌱️space-artifact-creation-v1/🟦️.ts:100,:118`, `🧬️schema/🟦️.ts:1409,:1532` | `artifactKind === kindId` / `!== artifact.kind` | same three changes, twin for twin |

Site 6 is worth naming precisely: the indexed entry's dialect is **assigned** from the intent
(`entry: DocumentIndexEntryV1 { name, dialect: append.intent.parent_dialect }`,
`🌎️hub/📇️directory/🦀️.rs:1756`) while the descriptor's `artifact_kind` is assigned from
`intent.request.kind_id`. The projection then compared those two assignments — i.e. it compared the
two id spaces against each other one hop after the hub itself had written them from different fields.

**No authority is lost by any of the seven.** The binding between the two spaces is declared where it
actually exists, in the trusted catalog: `validate_descriptor_open_target` requires a manifest kind
with exactly `target.artifact_kind` **and** an owning app whose `Dialect` is exactly
`target.parent_dialect`; `artifact_creation_catalog`
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:371-399`) derives **both** emitted fields from
one `VerifiedDocumentOpenSelectionV1`; and `ArtifactCreationIntentV1::ready()` copies both out of that
accepted intent. Each edit records this in a `🪢` docstring naming both spaces and both spellings —
the same discipline HT1 §14 used when it deleted the same equality one layer earlier. HT1 §14's last
sentence ("if stdio is brought in line with GIS that deletion should be reconsidered") is answered:
stdio is already in line with everything except gis, so the deletion stands and its six siblings join it.

### 2.3 The laws: two refusals re-expressed, three new laws added

The refusals I removed were pinned in four places. None was weakened into nothing; each was
re-expressed as a bound the validator genuinely owns, **plus a new positive law** that the two-space
pair is admitted — which is the property the product needs and which nothing asserted before.

| law | before | after |
|---|---|---|
| `🌱️space-artifact-creation-v1/🔣️.json` `statuses` | `ready-kind-mismatch` (`kindId: s.gis.map`, dialect `foreign.kind`) refused | `ready-dialect-kind-unbounded` (dialect `""`) refused **+ new `ready-two-space-dialect`** (`kindId: 2d.note`, schema `note.document`, dialect `s.note.note`) **accepted** |
| same file, `catalogs` | `kind-dialect-mismatch` refused | `kind-dialect-unbounded` refused **+ new `two-space-dialect`** accepted |
| `🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json` `negativeMutations` (22 → 23) | `parent-kind-mismatch` → `denied` | `parent-kind-substitution` → `stale` (the code its sibling `parent-standard-mismatch` already expects, and the code the live ledger really answers: `record.authority != current`) **+ new `parent-kind-control`** (`""`) → `denied` |
| same file, `parentDialectNegativeMutations` (8 rows) | `parentDialect.artifactKind: "s.foreign.document"` must fail catalog encoding | `parentDialect.artifactKind: ""` must fail catalog encoding — and the generation-digest loop above it now substitutes **only** `parentDialect.artifactKind` (the `if (key === "artifactKind") rows[0].artifact.kind = …` coupling is gone), so it proves that field alone is framed into the generation id, which it could not prove while the two were forced equal |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` `document_open_plan_ledger_…` | one hostile `("artifactKind", "s.foreign.document")` ⇒ `Stale`; `public_parent_kind` push `.foreign` ⇒ `Denied` | two hostile rows (`""` and a leading-space spelling) ⇒ `Stale`; `public_parent_kind` gets a leading space ⇒ `Denied`; **new**: `parent_dialect.artifact_kind = "s.note.note"` with `artifact.kind = "2d.note"` ⇒ `Ok(())` |
| `📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` | `assert_eq!(valid_plan.parent_dialect.artifact_kind, valid_plan.artifact.kind)` — an assertion about the *fixture's* gis shape, not about the code | the same two-space plan asserted `Ok(())` **and** an empty dialect kind asserted `Denied` |

The oracle in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` was changed with them:
`documentOpenNeutralDialect(value, artifactKind)` loses its second parameter and its `dialect-kind`
throw, keeping the per-field text and trim bounds (`:3835`); `documentOpenNeutralParentDialect`
(`:3845`) and `documentOpenNeutralStructure` (`:3994`) follow.

### 2.4 Verification (measured, with the captures)

- `cargo check -p semio-framework-os-kernel --all-targets` — **Finished**, warnings only
  (`🗑️generated/tc2-kernel-check.txt`). Warnings present, so expansion really ran.
- `cargo check -p semio-hub --all-targets` — **Finished**, 58 + 88 warnings, 0 errors
  (`🗑️generated/tc2-hub-check.txt`). Rule 26: no `cargo test`/`build` on `-p semio-hub` from here.
- `bun ./📜️script.ts open-plan-check --source` — **passes**
  (`🗑️generated/tc2-open-plan-source-check.txt`):
  ```
  verified parent dialect source:18 fields,3 digest substitutions,8 hostile rows,…
  document-open-plan-oracle: descriptor=1 catalog=2 receipt=1 independent-codecs=3 issuer=11
    consume=9 negative=23 exchange-negative=5 redaction=1 activation=catalog-gated-issuer+exchange passed
  document-open-plan-production-parity: codecs=3 rejected=17 exchange-rejected=5 passed
  open-plan-source-check: neutral fixture, independent framing and owned schema
  ```
  `negative=23` is the 22 rows plus my added `parent-kind-control`; `consume=9` is unchanged and
  `parent-artifact-kind-substitution` still answers `denied` — through the catalog-row match, which is
  the binding that actually exists, not through the deleted equality.
- **The Rust fixture rows themselves (the creation-contract laws and the bin-unit ledger law) are
  compile-verified only.** They live in `cargo test -p semio-hub`, which rule 26 reserves for the
  coordinator. That rerun is requested.

### 2.5 Two pre-existing reds found on the way, and repaired (neither is mine)

`open-plan-check` was **already failing before I touched anything** — both failures reproduce against
`HEAD`, and both are stale literals left by a peer's earlier refactor:

1. A source-text fence required `"checkpoint != authority.checkpoint"` in `🌎️hub/🏗️bootstrap/🦀️.rs`.
   The production code now spells it `checkpoint.as_ref() != Some(&authority.checkpoint)`
   (`🏗️bootstrap/🦀️.rs:4368`) and the string is absent from the file in `HEAD` as well
   (`git show HEAD:… | grep -c` ⇒ `0`). Fence updated to the current spelling; the property it pins is
   unchanged.
2. `catalogEncoding.expectedHex` / `expectedGenerationId` in `🧭️document-open-plan-v1.json` were
   computed with `appChannelVersion = 15` while the fixture's own `catalogRows` carry `17`. I located
   it by re-implementing the encoder (`🐍️tc2-catalog-encoding.mjs`, new, ticket-owned): first
   differing byte is at offset 236, `\x00\x00\x00\x11` vs `\x00\x00\x00\x0f`, everything else
   byte-identical (both 893 bytes). The descriptor golden beside it had already been refreshed, so
   this was a half-finished refresh. Both goldens recomputed:
   `2bb0378f…8ef2978f` → `7b3248e5…c54564ba`.

## 3. The N-plugin generalisation — **not landed**, and the reason is new

TC1 §5 and C1c's E2E run 4 both stopped at this function and both named the same cost: eight welded
places plus three source-text guard laws, gated behind cold `wasm-release` component builds. I
re-read the function end to end (`materializeTrustedStdioGisBundle`,
`🌎️hub/📦️packages/🦀️rust/📜️script.ts:9414-9612`) and confirmed all eight rows are still exactly
where TC1 put them. I did **not** land the rewrite, for a reason neither predecessor had measured —
§4. Landing a request-list parameter whose third row cannot be published would have been the
half-landed rewrite both predecessors refused, in a file three peers are editing right now.

What I can add to the hand-off, beyond TC1 §5's table:

- **Row 3/4 are harder than "a `Record<string, Codec[]>` keyed by `pluginId`".**
  `projectTrustedBootstrapCodecsV1` (`:8766`) is not one projector with two inputs; it is **two
  different schemas**. stdio's source is `semio.stdio.native-openable-catalog-provider/v1` with 26
  snake_case receipts carrying `pack_schema_sha256`; gis's is
  `semio.gis.native-codec-receipts/v1` with 2 camelCase receipts carrying a `packRecord` whose pack
  field table is re-hashed with blake3 per artifact family, inline in the projector. A third source
  (`✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🔣️.json`, `semio.vcs.native-codec-receipts/v1`) is a **third**
  shape: camelCase like gis but with **no `packRecord` at all**. So row 3 is "each request carries its
  own projector", not "each request carries its own path".
- **Row 8's `packageCount` is pinned in a second place**: the `🧬️stdio-gis-bootstrap` fixture asserts
  `limits = { …, packageCount: 2, codecCount: 28, openTargetCount: 1 }` and a 19-name `hostile` list
  (`:8462-8469`), and `Object.keys(fixture.sources)` is asserted to be exactly
  `["stdioReceipts", "gisReceipts"]` (`:8470`). That fixture is a ninth welded place TC1's table does
  not list.

## 4. Why a three-package catalog could not be bootstrapped from this slice

I planned to start the cold bootstrap early and wait it out. Reading the creation path first is what
prevented burning 3 h on a run that cannot succeed. Two measured blockers:

### 4.1 A creation target needs a **statically linked native genesis factory**, and only three exist

`artifact_creation_selection` (`🔏️trusted-catalog/🦀️.rs:351-368`) admits a target only if some codec
in the generation has `codec.genesis.is_some()` **and** matches the selection on package identity,
`artifact_kind`, `artifact_schema` and `pack_schema_hash`. `genesis` is a Rust function pointer
(`NativeArtifactGenesisFactoryV1`) supplied by
`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`, whose inventory is a `const` table:

```
NATIVE_OPENABLE_PROVIDER_SET_V1_ID = "stdio+gis+vcs/native-codecs/v1"   (:9)
linked() -> entries: stdio (semio:stdio), gis (semio:gis), vcs (semio:vcs)   (:28)
```

**`note` is not in that table, has no `📇️native-codecs` module at all** (only `gis` and `vcs` have one
in the whole repo), and the hub does not depend on `semio-s-plugin-note`. So "bootstrap a catalog
`stdio + gis + note`" is not a request-list change: it needs a new note-side codec+genesis module, a
new hub dependency, a new provider entry, **and a rebuilt `os-hub` binary**. That binary build is
`cargo build -p semio-hub`, which preamble rule 26 reserves for the coordinator. No note catalog can
be validated or published without it.

### 4.2 `vcs` is the only plugin that could be third today — and it is broken by one string

`vcs` is already linked, already has a genesis factory, already ships
`📇️native-codecs/🔣️.json` with one receipt. But its codec identity declares
`artifact_kind: "s.vcs.vcs"` (`📇️native-codecs/🦀️.rs`, the JSON, and the hub provider's literal at
`📇️native-openable-provider/🦀️.rs:105`) while its **manifest** kind is
`ArtifactKindSpec { id: VCS_DOCUMENT_SCHEMA }` = **`vcs.vcs`**
(`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs:23`, `VCS_DOCUMENT_SCHEMA = "vcs.vcs"`).
`artifact_creation_selection` requires `codec.identity.artifact_kind == selection.artifact.kind`, and
`validate_descriptor_open_target` requires `selection.artifact.kind` to be a **manifest** kind id. The
two cannot both hold for vcs.

Measured across the three linked providers, this is a *different* outlier from §2's:

| plugin | codec `identity.artifact_kind` | manifest `ArtifactKindSpec::id` | selectable? |
|---|---|---|---|
| stdio | `stdio.json`, `stdio.png`, … | `stdio.json`, `stdio.png`, … | ✔ (but no open target is declared for it) |
| gis | `s.gis.gismap`, `s.gis.gisterrain` | `s.gis.gismap`, `s.gis.gisterrain` | ✔ |
| **vcs** | `s.vcs.vcs` | `vcs.vcs` | **✘ — one string apart** |

**This is the KD1-shaped fix that really is plugin-side**, and it is vcs's, not stdio's. But it is
**not** the three-file change it looks like, and that is the last measurement of this slice.
`NativeVcsCodecReceiptV1::validate` requires
`definition.identity().as_str() == identity.artifact_kind`
(`✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🦀️.rs:74`), i.e. the codec kind must equal the **artifact's DSL
identity**, which is the envelope id persisted documents are written with. Reading that third
spelling across the three linked plugins settles the conformant rule and shows gis and vcs breaking
it in opposite directions:

| plugin | DSL artifact identity | manifest `ArtifactKindSpec::id` | codec `artifact_kind` | `Dialect.artifact_kind` |
|---|---|---|---|---|
| **stdio** (conformant, and the reference for ~40 plugins) | `stdio.json` | `stdio.json` | `stdio.json` | `s.stdio.json` |
| gis | `s.gis.gismap` | `s.gis.gismap` | `s.gis.gismap` | `s.gis.gismap` |
| vcs | `s.vcs.vcs` | **`vcs.vcs`** | `s.vcs.vcs` | `s.vcs.vcs` |

The rule stdio follows — DSL identity = manifest kind = codec kind, all in the taxonomy space, with
the Dialect alone in the plugin space — is what every other plugin's manifest kind agrees with.
**gis violates it by dragging its manifest kind into the dialect space** (which is exactly why the
hub's equality appeared to hold), and **vcs violates it by leaving its DSL identity and codec kind in
the dialect space while its manifest kind stayed in the taxonomy space** — which is the one-string
gap. Bringing vcs in line therefore changes a persisted envelope id, not three literals, so I did not
land it blind: it needs its own decision (change the DSL identity, or exempt vcs) and the same
`cargo test -p semio-hub` and fresh binary to be worth anything.

### 4.3 The bootstrap builds its own hub binary

`📜️gm1-hub-boot.sh` step 1 runs `trusted-stdio-gis-bootstrap`, which builds `os-hub` into
`⚡️cache/cargo/target-gm1/debug/os-hub` itself (GM1 §0: "built by the bootstrap itself"). That is a
`cargo build -p semio-hub` under a different name, so rule 26 forbids me from starting the chain at
all — independently of §4.1 and §4.2.

## 5. What the next owner should do, in order

1. **Coordinator**: `cargo nextest -p semio-hub` + a fresh `os-hub` binary with §2's changes. Nothing
   below is provable until the creation-contract and ledger laws are seen green at runtime.
2. **vcs codec identity** (§4.2): decide whether vcs's DSL artifact identity moves to `vcs.vcs` (the
   stdio rule, an envelope-id change) or its manifest kind moves to `s.vcs.vcs` (the gis exception,
   which puts a second plugin out of line with the taxonomy). The first is correct; the second is
   cheap. Either makes vcs the first non-gis creatable kind with no hub-side addition.
3. **Generalise the verb** over `[stdio, gis, vcs]` — TC1 §5's eight rows plus the two this report
   adds (per-request codec *projector*, and the `🧬️stdio-gis-bootstrap` fixture's `limits`/`sources`).
4. Bootstrap into a **new** data root through the fleet wasm mutex (three cold `wasm-release`
   components, ~45-60 min each), boot on a free port, create a `vcs.vcs` artifact, open-plan it.
5. Only then is `note` worth doing: it needs a note `📇️native-codecs` module with a genesis factory,
   a hub dependency and a fourth provider entry (§4.1).

## 6. Files changed

Product code (seven sites, one id-space defect):

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs` | `SpaceArtifactCreationKindV1::validate` and `SpaceArtifactCreationReadyV1::validate` bound `dialect.artifact_kind` / `parent_dialect.artifact_kind` as identities instead of comparing them to `kind_id`; both carry a `🪢` docstring naming the two id spaces, their spellings and where the binding is really declared. |
| `…/🌱️space-artifact-creation-v1/🟦️.ts` | the TS twins, `:100` and `:118`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` | `DocumentOpenPlanV1::validate` and `DocumentExecutionTargetLeaseFieldsV1::validate` drop the cross-space equality; `parent_dialect.artifact_kind` joins the existing bounded/trim array; both gain the `🪢` docstring. |
| `…/📇️directory/🧬️schema/🟦️.ts` | the TS twins, `:1409` (`document-open.invalid-parent-dialect`) and `:1532` (`document-execution-target-lease.invalid-parent-dialect`). |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `DocumentOpenPlanAuthorityV1::validate` — same change and docstring. |
| `🌎️hub/📇️directory/🦀️.rs` | `document_index_projection_v1` — the descriptor/entry cross-space comparison is dropped with a docstring recording that both come from one accepted intent and are already pinned by `validate_document_genesis_append_v1` and the descriptor digest. |

Laws and fixtures:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json` — two rows re-expressed, two positive two-space rows added (§2.3).
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json` — `negativeMutations` 22 → 23, `parentDialectNegativeMutations` row re-expressed, **and** the stale `catalogEncoding` goldens refreshed (§2.5, a pre-existing red).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` — the fixture-shape assertion becomes two laws (two-space accepted, empty dialect kind denied).
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — hostile dialect rows re-expressed and the two-space positive law added.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `documentOpenNeutralDialect` loses its `artifactKind` parameter and `dialect-kind` throw (`:3835`), its two callers follow (`:3845`, `:3994`), the generation-digest substitution loop stops coupling the two fields (`:4128`), and the stale `checkpoint != authority.checkpoint` fence is updated to the production spelling (`:4115`, a pre-existing red).

Ticket-owned: `🐍️tc2-catalog-encoding.mjs`, this report, and `🗑️generated/tc2-kernel-check.txt`,
`tc2-hub-check.txt`, `tc2-open-plan-check.txt`, `tc2-open-plan-source-check.txt`.

**Nothing of a peer's was reverted**, no `🗑️generated` file I did not create was touched, no process
was started or killed, and the live 7611 hub and its data root were never accessed.

## 7. Honest gaps

1. **No runtime observation at all in this slice.** Every claim in §2 is `cargo check` + the TS
   oracle. No hub was booted, no artifact was created, no plan was issued. The Rust fixture rows I
   changed are compile-verified only and need the coordinator's `cargo nextest -p semio-hub`.
2. **The N-plugin generalisation is still not landed** — a third consecutive slice that stopped at
   this function. §3 and §4 say why, and §5 is the ordering, but zero of TC1 §5's eight rows changed.
3. **The gis-only equality survives in two more places I deliberately left**:
   `🌎️hub/💡️inference/📇️catalog/🦀️.rs:90` and `🌎️hub/💡️inference/🧬️schema/🦀️.rs:216` (the latter
   compares against a `GIS_ARTIFACT_KIND` constant). They gate the gis inference service only, so they
   block no other plugin's creation or open — but a second inference service would hit them.
   `🌎️hub/📦️packages/🦀️rust/📜️script.ts:777` (`createCheckpointPublicationProcessGenesis`) also
   compares `status.ready.parentDialect.artifactKind !== fixture.artifact.kind` against a fixture whose
   type literally pins `kind: "s.gis.gismap"`; it will refuse any non-gis document and needs a
   `parentDialect` field on `CheckpointPublicationProcessFixtureV1` when that proof is generalised.
4. **`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:1383` was deliberately not
   changed**: it compares the descriptor app's own `dialect` against the plan's `parentDialect` — two
   copies of the *same* coordinate, which is a real binding — and it already checks the manifest kind
   space separately (`artifactKinds.some((kind) => kind.id === fields.artifact.kind …)`). It was
   already correct about the two spaces.
5. **I did not verify the TS twins by running a TypeScript suite.** The hub script's own oracle covers
   `🧬️schema/🟦️.ts`'s plan and lease parsers only indirectly (it has its own independent codec); the
   `🌱️space-artifact-creation-v1/🟦️.ts` twins are unexercised here.
6. **The two pre-existing reds I repaired (§2.5) belong to a peer's refactor.** I fixed them because
   they blocked my only runnable law, and both are mechanical (a fence spelling, a golden recomputed
   from the fixture's own rows), but neither was diagnosed with its author.

## 8. Coordinator rerun 21:03 — the two reds in my lane, fixed

`321 — 315 / 6`. Both new reds are one fixture row, `"foreign-kind"` in
`📇️directory/🧬️schema/📇️document-index-v1/🔣️.json` (`backendAccepted: false`), read by
`artifact_authority::creation::tests::directory_document_index_is_ordered_idempotent_and_backend_descriptor_bound`
and `directory::sqlite::tests::document_index_neutral_transactions_survive_projection_rebuild`. It
forges a `DocumentIndexed` whose `entry.dialect.artifact_kind` is `s.foreign.kind` while the announced
descriptor is `s.gis.map`/`gis`. **The coordinator's instinct was right and my §2 site 6 was a real
authority loss**: the deleted equality was the *only* thing binding `entry.dialect` in that projection
(the entry is not covered by `descriptor_digest_v1`), so a forged event page could carry any dialect.

**Product first.** The binding is restored in the space where it is actually declared, not by
comparing two id spaces: a `Dialect`'s artifact kind is canonical `s.<plugin>.<artifact>` grammar
(`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:104`, `ArtifactKindId::parse`) and is **owned by exactly
one plugin** — the same predicate `🔌️plugin/🛂️describe/🦀️.rs:83` (`owns_artifact_kind`) already
applies. `document_index_projection_v1` now requires `entry.dialect.artifact_kind` to be
`s.<owner.plugin_id>` or to start `s.<owner.plugin_id>.`. It refuses `s.foreign.kind` for a `gis`
document (the row stays red-by-design), admits `s.note.note` for `note` and `s.stdio.json` for
`stdio`, and is **strictly stronger than the deleted equality**: it also refuses a foreign plugin's
dialect that happened to equal the kind, which the equality could not express.

Exposed by it, and fixed with it: `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`'s synthetic
`document_descriptor_for_test` was itself collapsing the two spaces
(`artifact_kind: "test.artifact"`, `owner.plugin_id: "test.plugin"`, and six call sites deriving the
parent dialect as `descriptor.artifact_kind.clone()`). It is now product-shaped: owner `test`, a new
`TEST_ARTIFACT_PARENT_DIALECT_KIND = "s.test.artifact"` used by all six, so the hub's own fixture now
exercises a two-space document instead of a gis-shaped one. One of those six was not synthetic —
`publish_openable_document_for_test`'s catalog path derived the dialect from `descriptor.artifact_kind`
and now takes `selection.parent_dialect.artifact_kind`, which is correct for any plugin.

New positive law: `📇️document-index-v1/🔣️.json` gains `two-space-dialect` — the `ordered` case with
`entry.dialect.artifact_kind = "s.gis.gismap"` against an `s.gis.map` descriptor, `backendAccepted:
true`, `clientRows: 1`. It reuses `ordered`'s descriptor and digest unchanged (the entry is outside
the digest domain), so it costs no recomputation and proves admission across the two spaces at the
projection gate.

`cargo check -p semio-hub --all-targets` — **Finished, 0 errors**. **Needs hub rerun.**

Watch on the rerun: every law that indexes a document now requires an owned dialect, so any remaining
synthetic producer that still spells the dialect as the manifest kind will surface as
`"document index descriptor binding differs"`. **This paragraph was wrong** and the 21:22 rerun proved it in twenty laws — see §8b.


## 8b. Rerun 21:22 — my §8 binding was TOO STRONG, and the measurement that proves it

`321 — 297 / 24`: 20 new reds, all `Conflict("document index descriptor binding differs")`. The
coordinator's option (b) was the right one, and it is not a test artefact — **a live hub would have
refused real documents.**

**The measurement.** I read the `dialect.artifactKind` of every app in all 32 shipped plugin
descriptors (`✏️s/🔌️plugins/*/🔣️.json`, `manifest.apps[].dialect`) against that descriptor's own
`manifest.pluginId`: **119 of 127 are owned by their own plugin, and 8 are not** — every one of them
`demonstrator`, which ships apps whose `Dialect` belongs to another plugin:
`s.gis.gismap`, `s.cad.cad`, `s.puzzle.puzzle3d`, `s.process.process3d` ×2, `s.sourcing.curation` ×2,
`s.procedural.generation3d`. One package hosting another plugin's dialect is a **shipped product
shape**, so `entry.dialect.artifact_kind` must never be required to name `descriptor.owner.plugin_id`.
The hub's own fixtures said the same thing once I looked: the five dialect literals in its tests are
`s.gis.gismap`, `s.stdio.semio`, `s.fixture.document`, `s.test.artifact` and the catalog's own
`selection.parent_dialect` — three of them deliberately unrelated to their descriptor's owner.

**Weakened to exactly what the product guarantees**: `entry.dialect.artifact_kind` must be canonical
`s.<plugin>[.<artifact>]` grammar — `is_canonical_artifact_kind`
(`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:143`) widened by the bare `s.<plugin>` IO form that
`🔌️plugin/🛂️describe/🦀️.rs:83` documents — mirrored as
`canonical_document_index_dialect_kind` beside the projection, the way
`📇️document-index-v1` already mirrors `SubsetId::ANY`. The plugin segment is explicitly **not**
compared to anything; the docstring records why, and names the two gates where the cross-package
binding really is declared (`validate_descriptor_open_target`, `validate_document_genesis_append_v1`).

**The `foreign-kind` row is re-derived through its producer, and stays refused**: its dialect goes
from `s.foreign.kind` (canonical, and now correctly admissible — `demonstrator` proves a foreign
plugin segment is legal) to **`foreign.kind`**, which is not a dialect at all. I checked every case in
the fixture against the new predicate: `foreign-kind` is the only one whose refusal depends on it, and
all eleven others keep the reason they already had (ordering, scope, document, author, name, digest).
`two-space-dialect` (`s.gis.gismap` against an `s.gis.map` descriptor) still passes.

**Honest gap this leaves.** With the kind↔dialect equality gone and ownership shown to be wrong, there
is **no product-guaranteed binding left between the announced descriptor and the index entry's
dialect** at this gate beyond the grammar — the entry is outside `descriptor_digest_v1`'s domain. That
is a real hole, it predates me in substance (the old equality only closed it for `gis`), and the fix
is a wire change: put `DocumentIndexEntryV1` inside the descriptor digest domain, or carry the
selection's `parent_dialect` in the descriptor. It belongs with §9 and I did not start it.

`cargo check -p semio-hub --all-targets` — **Finished, 0 errors**. **Needs hub rerun.**

## 9. DESIGN — catalog-carried genesis: creating a document of ANY cataloged plugin kind

**The defect.** `VerifiedNativeArtifactCodec.genesis` is a Rust `fn` pointer
(`NativeArtifactGenesisFactoryV1`), so `materialize_selected_genesis` can only mint the empty document
of a kind whose plugin is *linked into `os-hub`*. The inventory is a `const` table of three
(`📇️native-openable-provider/🦀️.rs:28`). Every other plugin's document is uncreatable **by
construction**, whatever the trusted catalog says — the catalog is data, the genesis is code, and the
two are in different worlds. Outcome 1 ("every plugin hosted") cannot meet outcome 3 until they are in
the same world.

**The move: the genesis pair becomes catalog DATA, emitted by the plugin's own component, under the
same provenance the catalog already publishes.** Nothing per-plugin is linked into `os-hub`.

1. **Production.** Inside `produceFreshComponentV1` — the cold `wasm-release` build that *is* the
   provenance guarantee — the freshly built component is instantiated once more and asked, for each
   creatable artifact kind it declares, for its empty document: the `(pack, spr)` pair. It is the same
   call `native_artifact_genesis_for_editor` makes today, made against the wasm guest instead of a
   linked rlib. Output is staged as `packages/<plugin>/genesis/<kindId>.{pack,spr}`.
2. **Provenance.** Each row is recorded in `trusted-catalog.json` beside the package's `browserActor`
   row and framed identically: `{ kindId, pack: {path, byteLength, sha256}, spr: {…},
   sourceComponentSha256, sourceDescriptorByteSha256 }`. `sourceComponentSha256` must equal that
   package's `component.sha256`, so a genesis pair is admissible only if the component the catalog
   publishes emitted it. The rows join `trustedBootstrapProfileEncoding`, so they are inside the
   generation id and cannot be swapped without rotating the generation.
3. **Consumption.** `NativeCodecBinding::genesis: Option<fn…>` becomes
   `Option<TrustedGenesisPairV1>` read out of the retained generation;
   `materialize_selected_genesis` (`🌱️creation/🦀️.rs:63-70`) reads the two files, verifies both
   sha256s and the byte lengths, and hands them to the existing
   `ArtifactCreationPreparedV1` path unchanged. No new trust: the pair is hash-pinned and the
   descriptor digest already binds `bootstrap_snapshot_hash == sha256(pack)`.

**What the linked native codec table is still for** — it does not go away, it loses one job:
- **stdio**: `ArtifactCodec` decode/encode for real file import/export on the hub, and the
  `validate_native_codec_artifact_kinds` / `…_catalog_contributions` manifest cross-checks.
- **gis**: `🌎️hub/💡️inference/*` runs a native gis inference service (`gis_map_inference_service`) —
  genuinely linked Rust, and out of scope here.
- **every package**: `codec.schema` / `extension` / `pack_schema_hash` identity, which
  `artifact_creation_selection` still matches. What it stops carrying is `genesis`.

**What changes where**
- `SpaceArtifactCreation*`: **no wire change**. `kind_id` and `dialect` are already two independent
  spaces after §2, which is the prerequisite this design stands on.
- `artifact_creation_selection` (`🔏️trusted-catalog/🦀️.rs:351`): `codec.genesis.is_some()` becomes
  "the selection's package carries a verified genesis pair for `selection.artifact.kind`". A package
  with no genesis row is still openable and still not creatable — the same refusal, from data.
- **Open plan**: unchanged. It already names the package and its component hash; a document created
  from a catalog-carried genesis is indistinguishable downstream.
- **Checkpoint genesis**: unchanged — `ArtifactCreationPreparedV1::validate` already pins the pair
  byte for byte to the descriptor, the digest and the zero frontier.
- `NATIVE_OPENABLE_PROVIDER_SET_V1_ID` shrinks to its codec-only role and its `29`-receipt count law
  stays; `preview_*_bindings` stop calling `into_codec_and_genesis`.

**Laws that pin it** — one new, three extended, none weakened:
- new: for every open target in the bundle, its package carries exactly one genesis pair for that
  `kindId`, whose `sourceComponentSha256` equals the package's `component.sha256` and whose
  `sha256`s match the staged bytes (the exact shape TC1 §5 proposed for the browser actor).
- `proveTrustedGisPublicationFixture`'s fences gain that row; `🧬️stdio-gis-bootstrap`'s `limits`
  gains `genesisPairCount` beside `openTargetCount`; the hostile list gains
  `foreign-genesis-component`, `missing-genesis`, `genesis-hash-substituted`.
- the creation-contract fixture gains an accepted row for a catalog-genesis kind and a refused row
  for a selection whose package has no genesis pair.

**Migration-free order of landing**
1. Land §2's id-space fix and §8's index binding (**done here**, pending the rerun) — without them
   nothing but `gis` survives the first validator.
2. Teach `produceFreshComponentV1` to emit the pairs and the bundle schema to carry them, with the new
   law, while the hub still uses the linked `fn` pointers. The bundle grows; nothing reads it yet.
3. Switch `materialize_selected_genesis` to the catalog pair and delete `genesis` from
   `NativeCodecBinding`. Because step 2 already publishes the pairs for `stdio`+`gis`, the *existing*
   published generation shape is a superset and `gis` creation keeps working through the new path —
   no compat branch, no migration, one generation rotation.
4. Only now is the N-plugin request list (§3) worth landing: the third package needs no hub change.
