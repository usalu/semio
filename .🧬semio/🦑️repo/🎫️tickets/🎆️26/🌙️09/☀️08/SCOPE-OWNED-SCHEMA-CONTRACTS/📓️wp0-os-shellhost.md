# WP0 — os / ShellHost Schema-Ownership Audit

Partition: `🧰️framework/🛍️products/💻️os/**` (~500 schema-named tracked files), special focus on
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/*`
(🌱️artifact-creation, 🎟️invite-capability, 👥️presence-scope, 📇️directory-bootstrap, 🗨️dialog-origin,
🪪️host-bootstrap). Read-only audit, repo MCP unavailable this session; ticket folder managed on disk.
Census built from `git ls-files -z` + `git ls-files -s` (index/tracked snapshot, not working tree) on
2026-09-08. One concurrent in-flight edit was detected mid-audit inside the partition (§7, §8) and is
called out explicitly rather than silently absorbed into the findings.

Companion deliverable: `📊️wp0-os-shellhost.json` (82 per-file records: `{path, blob, role, classification,
owner, intendedOwner, exports, consumers, decision, evidence, open}`).

## 1. Taxonomy basis (vocabulary read from `🔣️taxonomy.json`)

- `schemaFormats`: `🦀️rust`, `🟦️typescript`, `🔗️graphql`, `🔣️jsonschema` (fileKind `json`, emoji `🔣️`),
  `🛰️protobuf`, `📜️wit`.
- `schemaFacetKinds["🧬️data"]`: normative format `🔣️jsonschema`, full format set
  `[jsonschema, rust, typescript, graphql, protobuf]`.
- `artifactSchemaSpecFileKinds["🧬️schema"] = "json"` → the canonical JSON Schema filename inside a
  `🧬️schema/` directory is the **bare** `🔣️.json` (fileKinds.json.emoji = `🔣️`). Any file spelling the
  word "schema" into its own name (`🧬️.schema.json`, `🔣️.schema.json`, `<slug>.schema.json`) is
  off-taxonomy by construction, regardless of which directory holds it.
- `mutationPayloadSchemaLocation`: `{directoryKindId: "schema", directoryName: "🧬️schema", fileKindId:
  "json"}` → a mutation's payload schema belongs at `<mutation-slug>/🧬️schema/🔣️.json` (one level nested
  under the mutation leaf), confirmed empirically as the dominant, live pattern (§3).
- `mutationPayloadSchemaAuthority`: `targetAuthority: "owner-relative-regular-json-schema"`,
  `jsonSchemaDialect: "http://json-schema.org/draft-07/schema#"` — draft-07 is canon; several ShellHost
  contracts instead declare `https://json-schema.org/draft/2020-12/schema` (§4).
- `semanticCollections["🧱️elements"] = {"kind": "ui"}` vs `semanticCollections["🔨️modules"] = {"kind":
  "module"}` — **directories inside a `🧱️elements/` collection are UI-kind members, not module/artifact/
  product-kind scopes.** `semanticAllowedOwnerLevels = [subset, standard, artifact, app, plugin, product,
  s, framework]` contains no "element" or "ui" level. This is the load-bearing rule for §2: ShellHost (and
  every other member of `📺️renderer/🧑‍🎨engine/🧱️elements/`) is structurally ineligible to own a schema
  contract, by the taxonomy's own collection-kind vocabulary, independent of any judgment call.
- `artifactComponentDirs = ["🧬️schema", "🚪️io"]` — any artifact/module-leaf that owns a schema is expected
  to nest it under a child `🧬️schema` directory, not place a schema file directly at its own root.
- `osChildDirs = ["🎮️commands", "🎚️config"]` (required: `🎮️commands`) — matches the real top-level layout
  of `💻️os/` (`🎚️config`, `🎮️commands`, `🔨️modules`, `📦️packages`, `📚️examples`, `🧪️fixtures`,
  `🧫️fixtures`, `🖥️host`); `🎚️config` is confirmed as a genuine **product-level** eligible scope (it
  already owns a full 5-format `🧬️schema/` at its own root). `🖥️host` also owns a `🧬️schema/` but is not
  listed in `osChildDirs` — a taxonomy-declaration gap noted for completeness, out of this audit's scope
  to resolve.

## 2. Is ShellHost an eligible scope owner? — No, with direct evidence

`🏛️ShellHost` lives at `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/`, one of 34 siblings
under `🧱️elements/` (⚙️EngineCanvas, 🐚️Shell, 🕸️NodeGraph, 🛂️SpaceAdministration, 🛠️ShellHelpers, …). It
has no `project.json`, no `Cargo.toml`/component metadata, and no taxonomy entry of its own beyond being a
named member of the `🧱️elements` semantic collection (`kind: "ui"`, per §1). Renderer elements register
through the `📺️renderer/🧑‍🎨engine/📦️packages/{🟦️typescript,🦀️rust}` target packages, not individually.

ShellHost is the **only** location in the entire `💻️os` partition (and, per the sibling ticket's
master-plan census, in the whole repo) that uses a bespoke `🧬️contracts/<slug>/` directory shape instead
of a `🧬️schema/` module — a pattern that exists nowhere else. That uniqueness is itself strong evidence
this is an ad hoc convention invented locally rather than a recognized taxonomy scope shape.

Every one of the six seed contracts, once traced to its actual issuer/verifier/consumer, points to a
different, already-existing, taxonomy-eligible module (mostly `💻️os/🔨️modules/📇️directory`, one to
`🏪️store/👥️presence`), never to hub, never fully self-contained inside ShellHost except for
`🗨️dialog-origin` and `🪪️host-bootstrap`, whose correct home is a genuinely open question (§4).

## 3. os product scope model

`💻️os` (`🎮️commands`, `🎚️config`, `🔨️modules`, `📦️packages`, `📚️examples`, `🧪️fixtures`, `🧫️fixtures`,
`🖥️host`) is organized as:

- **Product-level scopes** (`🎚️config`, `🖥️host`): own a `🧬️schema/` directly at product root. `🎚️config`
  has the full 5-format set (`🔣️.json`+`🟦️.ts`+`🦀️.rs`+`🔗️.graphql`+`🛰️.proto`) plus 5
  `🧬️mutations/<slug>/` leaves (all correctly *located* one level too shallow — see the config-mutations
  finding below).
- **Module-level scopes** (everything under `🔨️modules/<name>/`): the dominant, taxonomy-compliant
  pattern. 143 distinct `🧬️schema/` directories were found under the partition (`🎚️config`, `♾️infinite/
  🎲️board/…/dag`, `🌉️mcp`, `🌊️flow/🌿️vcs`, `🌊️flow/🕸️wasm`, `🏪️store` (≈20 sub-scopes), `📇️directory`
  (+`🔌️client/🪪️runtime`), `📡️spr` (≈10), `🔁️workflow` (+`🗿️artifacts/🏃️run`), `🔌️plugin` (≈20,
  including many self-testing mutation-framework fixture trees), `🗣️dsl`, `🖥️host`). The overwhelming
  majority of the ~825 schema-shaped files in the partition are `🧬️mutations/<slug>/🧬️schema/🔣️.json`
  payload schemas following `mutationPayloadSchemaLocation` exactly — these are classification **(a)
  authoritative** and were bulk-recognized by directory-shape rather than individually re-verified (see
  §9 coverage note).
- **Renderer element scopes** (`📺️renderer/🧑‍🎨engine/🧱️elements/<Name>/`): UI-kind, not schema-eligible
  (§2). ShellHost is the only member with a `🧬️contracts/` per-contract-directory tree; siblings
  (🕸️NodeGraph, 🛠️ShellHelpers) instead have a single stray `🧪️fixtures|🧫️fixtures/🧬️.schema.json`
  (classification **b**, same violation family as the rest of the partition's fixture-owned schemas).
- **Fixture-owned schemas** (`🧪️fixtures/<slug>/🧬️.schema.json` or `🧫️fixtures/<slug>/🧬️.schema.json`
  beside a `🔣️.json` example): 58 instances found via the dotted-filename scan, spread across nearly every
  module (`🌉️mcp`, `🌊️flow`, `🎒️pack`, `🏪️store`, `🔌️plugin` (largest concentration, ≈20), `🛢️db`
  (largest single sub-tree, ≈17), `🧠️neural`, `🧑‍💻dev`, plus 4 at the `💻️os` product root). Pattern is
  systemic and repo-wide, not ShellHost-specific: in ~50 of 58 cases, the *same module's own*
  `📜️script.ts` reads the co-located `{🔣️.json,🧬️.schema.json}` pair via `readFileSync(join(...))` +
  Ajv2020/Ajv (draft-07 or 2020-12 inconsistently) and asserts fixture shape plus source-substring
  behavior against sibling `.rs`/`.tsx` files — a real, wired oracle, just structurally in the wrong place
  per the goal's rule ("fixtures provide examples only and never define contracts").
- **Filename-convention chaos, independent of location**: within the single `📇️directory/🧬️schema/`
  directory alone there are *three* different JSON-Schema filenames in concurrent use — `🔣️.json`
  (`📇️document-index-v1`, canonical), `🔣️.schema.json` (`🌐️browser-actor`), and `🧬️.schema.json`
  (`🌱️space-artifact-creation-v1`, `📣️checkpoint-publication-command-v1`). `🏪️store/👥️presence/🧬️schema/`
  adds a *fourth* style, `<slug>.schema.json` (`📌️peer-commit.schema.json`, no emoji prefix at all). None
  of these four is a location violation by itself; all four are filename violations.

## 4. Seed case — the six `ShellHost/🧬️contracts/*` entries (deep dive)

Full evidence for every file below (with source line numbers, consumer lists, and exports) is in
`📊️wp0-os-shellhost.json`. Summary:

| Contract | `$id` / schema string | Real domain | Recommended owner |
|---|---|---|---|
| 🎟️invite-capability | `.../directory/invite-capability-transfer.v1.json` (2020-12) | Renderer clipboard-copy FSM for the directory-administration invite flow. **Not** the same thing as hub's `InviteCapability` token (a different, already-correctly-owned type at `🌎️hub/🔐️auth/🧬️schema/{🔣️.json,🟦️.ts}`, minted/verified/redeemed in `🌎️hub/📇️directory/🦀️.rs`). | `💻️os/🔨️modules/📇️directory/🧬️schema/` |
| 🌱️artifact-creation | `.../os/artifact-creation-progress-ui-v1.json` (2020-12) | en/de progress-UI labels for the same creation flow `📇️directory/🧬️schema/🌱️space-artifact-creation-v1` already models structurally; hub owns the real creation business logic (`🌎️hub/🗿️artifact-authority/🌱️creation/*`). | `💻️os/🔨️modules/📇️directory/🧬️schema/` |
| 👥️presence-scope | `semio.shell.presence-scope-fixture.v1` (2020-12) | spaceId/documentId-scoped runtime-key derivation + surface routing — a presence-domain concern; `💻️os/🔨️modules/🏪️store/👥️presence/🧬️schema/` already owns adjacent peer-commit/peer-admission/retirement contracts. | `💻️os/🔨️modules/🏪️store/👥️presence/🧬️schema/` |
| 📇️directory-bootstrap | `semio.space.home.directory-projection-receipt.v1` (2020-12) | directory event-page/receipt handshake bootstrapping ShellHost's home view. | `💻️os/🔨️modules/📇️directory/🧬️schema/` |
| 🗨️dialog-origin (+`🚪️opening`, `🎥️tutorial`, `🎥️tutorial/⏩️seek`) | no `$id` (draft-07, the one dialect the family gets right) | Modal/dialog-origin admission FSM for the renderer shell chrome. | **Open** — every consumer (🐚️Shell, 🏛️ShellHost, 🛠️ShellHelpers) is itself a `🧱️elements` member; no cross-module or cross-app consumer exists. No natural non-element home currently exists (§4.5). |
| 🪪️host-bootstrap | `.../react/host-identity-bootstrap-v1.json` (2020-12), filename `🔣️.schema.json` (a third naming style within the same tree) | React-target host/landing-app resolution (`resolveRequiredHostApps`). | **Open** — namespaced under a rendering *target* ("react"), not a domain; `semanticAllowedOwnerLevels` has no "target" level (§4.6). |

### 4.1 🎟️invite-capability — full trace

- Schema: `$id: https://semio.tech/schema/directory/invite-capability-transfer.v1.json`, dialect
  2020-12 (not draft-07). Fixture `🔣️.json` mirrors the schema 1:1 (a "schema of one example" document,
  per the coordinator's seed note).
- **Issuer/verifier of the real token**: `🌎️hub/📇️directory/🦀️.rs:805` (`capability_type!(InviteCapability,
  CapabilityKind::Invite)`), `:1048` (`InviteCapability::mint()`), `:1081`/`:1118`/`:2501`
  (`verify_invite_redemption_scope_hint`/`invite_redemption_preflight`/`redeem_invite`). The actual wire
  type is `🌎️hub/🔐️auth/🧬️schema/🔣️.json:23` (`"InviteCapability": {"pattern":
  "^invite\\.v1\\.[0-9a-f]{32}\\.[0-9a-f]{64}$"}`) + `🟦️.ts:4,17`. This is a fully-formed, correctly-owned,
  cross-target hub contract — **unrelated** to the ShellHost schema in question except by shared English
  name.
- **Consumer of the ShellHost schema**: `directoryInviteCapabilityOracle` in
  `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts:160-206`
  (`join(repoRoot, ".../ShellHost/🧬️contracts/🎟️invite-capability")` + `readFileSync` + `new
  Ajv2020({strict:true}).compile(...)`), wired to nx target `directory-invite-capability-check`
  (`📋️project.json:21-25`) and gate `⚖️gate🎟️directory-invite-capability🌐️browser-worker`
  (`.vscode/launch.json:6952-6955`). The oracle additionally greps `🧵️backbone-worker.ts`,
  `ShellHost/🟦️.tsx`, and `🛂️SpaceAdministration/🟦️.tsx` source for literal behavior strings — real,
  exercised infrastructure, not dead code.
- **Verdict**: the fixture's `shellRoute.actionId` is `"os.directory.open-administration"`; the `$id`
  itself says `directory`. Owner = `💻️os/🔨️modules/📇️directory/🧬️schema/🎟️invite-capability-transfer-v1/`
  (new, draft-07, sibling to `🌱️space-artifact-creation-v1` and `📣️checkpoint-publication-command-v1`).
  ShellHost, `🛂️SpaceAdministration`, and `🧵️backbone-worker.ts` become pure consumers.

### 4.2–4.4 (artifact-creation, presence-scope, directory-bootstrap)

See table above and `📊️wp0-os-shellhost.json` for full oracle line ranges
(`directoryHomeBootstrapOracle` script.ts:99-149, `directoryInviteCapabilityOracle` 160-206,
`scopedPresenceOracle` 219-254) and consumer lists. All three name their domain in the schema string
itself (`semio.directory.*`, `semio.space.home.directory-projection-receipt.v1`, `semio.shell.
presence-scope-fixture.v1`) and are consumed only by ShellHost + one or two sibling renderer files —
none crosses into hub.

### 4.5 🗨️dialog-origin — the genuinely unresolved case

Unlike the other three, `🗨️dialog-origin` (plus its nested `🚪️opening`, `🎥️tutorial`, and
`🎥️tutorial/⏩️seek` variants) has **no cross-module consumer at all**. Every reference is inside
`📺️renderer/🧑‍🎨engine/🧱️elements/{🐚️Shell,🏛️ShellHost,🛠️ShellHelpers}` — three UI elements exchanging a
shared "who currently owns this modal/effect" state machine
(`shellDialogOriginV1`/`shellEffectSourceIsCurrentV1`/`createAdmittedShellInstanceV1`). It is the one
contract of the six where the taxonomy's "elements can't own schema" rule and "every contract needs one
eligible owner" rule pull in different directions: there is no existing module this naturally belongs to.
Two honest options: (a) create a new eligible module (e.g. `🔨️modules/🐚️shell-ui/🧬️schema/`) purely to
hold renderer-shell-internal FSMs like this one, or (b) have the taxonomy explicitly carve out an
exception for element-internal, single-target UI machinery that never crosses a module boundary. This
audit does not resolve that choice; it is flagged as an open question for WP3/WP4.

*Caveat*: at audit time this directory showed **uncommitted local changes** (`git status`: `M` on
`🎥️tutorial/{🔣️.json,🟦️.ts,🧬️.schema.json}`) plus two brand-new **untracked** subdirectories,
`🎥️tutorial/⏩️seek/` and `🎥️tutorial/🧵️serial/` — a concurrent session appears to be actively extending
this exact contract right now. Per the ticket's ground rules this audit is based on the tracked
(`git ls-files -s`) blob snapshot, not the live working tree; the untracked `⏩️seek/` directory was
excluded from the enumeration and record count for that reason (see §9). `🧵️serial/` was not inspected.

### 4.6 🪪️host-bootstrap — the other unresolved case

`$id: https://semio.tech/schema/react/host-identity-bootstrap-v1.json` namespaces itself under a
*rendering target* ("react"), not a domain. Content (`resolveRequiredHostApps`, `landingAppId`/
`hostAppId` resolution) is intrinsic to how the React target boots ShellHost's own chrome; no consumer
outside `⚛️react`'s test suite and ShellHost itself was found. `semanticAllowedOwnerLevels` has no
"target" level, so there is no obviously-correct destination module — recommend the ticket owner decide
whether target-scoped bootstrap contracts are in scope for schema-ownership at all, or whether this one
should simply stop being modeled as a JSON Schema. Also notable: this is the only one of the six using
`🔣️.schema.json` as its filename (every other ShellHost contract uses `🧬️.schema.json`) — a third,
internally-inconsistent naming convention inside the same `🧬️contracts/` tree.

### 4.7 Naming collision worth flagging

`🏛️ShellHost/🧭️opening/🧪️fixtures/📍️scope/` (top-level sibling of `🧬️contracts`, consumed by
`🧵️backbone-worker.ts` and `⚛️react/🚪️opening.test.ts`) and
`🏛️ShellHost/🧬️contracts/🗨️dialog-origin/🚪️opening/` (nested inside the dialog-origin contract) are two
**unrelated** "opening" concepts sharing a name inside the same element directory. Neither reasonably
belongs to ShellHost as final owner (§4.5, and `🧭️opening/📍️scope`'s consumer is the product-root
`🧵️backbone-worker.ts`, itself evidence the contract already crosses the element boundary). Flagged for
the relocation review to resolve the naming collision, not just move files.

## 5. Fixture-owned duplicate: `os/🧪️fixtures` vs `os/🧫️fixtures`

`💻️os/🧪️fixtures/↩️gis-map-approval-history-v1/{🔣️.json,🧬️.schema.json}` and
`💻️os/🧫️fixtures/↩️gis-map-approval-history-v1/{🔣️.json,🧬️.schema.json}` are near-duplicate copies of the
same fixture living in two differently-emoji'd top-level `os` product directories. `diff` shows they have
**diverged**: the `🧫️fixtures/` copy requires 9 hostile-input cases (including `oldOwnerRetired`), the
`🧪️fixtures/` copy only 8 and lacks that field. The only live consumer,
`💻️os/📦️packages/🟦️typescript/📜️script.ts:163`, reads exclusively from `🧫️fixtures/` — the `🧪️fixtures/`
copy appears **orphaned and stale**, not merely redundant. This is a self-contained finding worth a
dedicated cleanup pass independent of the ShellHost relocation work.

## 6. Dynamic schema-path construction (task 5)

No `**/🧬️.schema.json`-style glob or `readdirSync`-based schema discovery was found anywhere in the
partition's `📜️script.ts` files (readdirSync usages found are unrelated: staging-dir cleanup, plugin
enumeration, icon-asset listing, taxonomy walking). The actual, repeated pattern is **hardcoded
`join(repoRoot, "<literal contract path>")` + `readFileSync` per contract**, once per oracle function:

- `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts:99, 161, 219`
  — `directoryHomeBootstrapOracle`, `directoryInviteCapabilityOracle`, `scopedPresenceOracle`, each
  building `contractRoot = join(repoRoot, ".../ShellHost/🧬️contracts/<slug>")` then reading both
  `🔣️.json` and `🧬️.schema.json` from it.
- `💻️os/📦️packages/🟦️typescript/📜️script.ts:163, 217, 307, 384, 477, 538` — six more oracles at the
  product root, each with its own literal `join(repoRoot, "💻️os/🧫️fixtures/<slug>")` or
  `.../ShellHost/🧪️fixtures/<slug>` path (line 538 is `proveMountedGisMapProbe`, the ShellHost
  gis-map-probe oracle).
- Every one of the ~50 module-local fixture oracles found in §3 follows the identical shape:
  `<module>/📜️script.ts` hardcodes `join(repoRoot, "<module path>/🧪️fixtures|🧫️fixtures/<slug>")` once,
  local to that module — i.e. the "dynamic path construction" is a repo-wide idiom (one literal join per
  contract, no wildcards), not something unique to ShellHost or a genuine glob-based risk.

Also present: `🔬️index.test.ts` (react target) and `⚡️quick.test.ts` statically `import` the fixture and
schema JSON files as ES modules (bundler-resolved, not `readFileSync`) — a second, parallel harness
mechanism for the same ShellHost contracts (index.test.ts lines 26-39 for dialog-origin family +
artifact-creation; quick.test.ts lines 8-15 for host-bootstrap).

## 7. Coverage and methodology

- Full enumeration command: `git ls-files -z '🧰️framework/🛍️products/💻️os' | grep -iE
  '(schema|🧬️|🔣️\.json$|\.proto$|\.graphql$)'` → 825 candidate paths.
- Of those, **78** are literally named `🧬️.schema.json` (the goal's stated violation-pattern filename);
  plus one `🔣️.schema.json` (host-bootstrap) = **79 tracked violation-pattern files**, all individually
  classified in `📊️wp0-os-shellhost.json` with path, blob id, role, owner trace, consumers, and a
  relocation decision. One additional untracked `🧬️.schema.json` under
  `🗨️dialog-origin/🎥️tutorial/⏩️seek/` was found mid-audit (§4.5) and excluded from the tracked
  enumeration/count per the ticket's tracked-only scope.
- The remaining ~746 candidates are `🧬️mutations/<slug>/🧬️schema/🔣️.json` (or equivalent
  `🧬️schema/<other-format>`) files matching `mutationPayloadSchemaLocation` exactly — **bulk-classified
  as (a) authoritative** by directory-shape pattern-match (143 distinct `🧬️schema/` roots enumerated,
  §3) rather than individually re-opened and content-reviewed one by one; four representative samples
  (os/config root, ♾️infinite/board move-node, directory root, store/presence peer-commit) were opened
  and are included as positive-control records in the JSON. This is a depth/breadth tradeoff: full
  per-file consumer citation for all ~825 candidates was out of reach in one pass; the six ShellHost seed
  contracts (the ticket's explicit special focus) and all 79 dotted/off-convention filenames received
  full individual treatment, while the large, structurally-uniform, already-compliant majority did not.
- Fixture-consumer citations: obtained via `rg -l <slug> --glob '*.ts' --glob '*.tsx' --glob '*.rs'`
  against each of the 58 fixture directory names; 56 of 58 returned at least one hit (module-local
  `📜️script.ts` in the large majority). Two — `🌊️actor-import` (browser-bundle) and `🚀️browser-startup`
  (flow/wasm) — returned no `.ts`/`.tsx`/`.rs` hit in this pass; `🚀️browser-startup`'s consumer may be
  the Rust `.rs` component/protocol files that don't literally spell the slug, or it may be genuinely
  orphaned — flagged as `open` in the JSON rather than guessed at.
- Not opened in this pass, out of scope for time: the ~70 flat `<name>🧬️schema.json`-style files and
  ~43 `🔣️.schema.json` files the master-plan's repo-wide census mentions outside this exact partition's
  dotted-filename scan (this audit's partition already captured every `🔣️.schema.json` inside it — only
  one, host-bootstrap — and the `🧬️.schema.json` scan is exhaustive for the partition).
- `.DS_Store` and the untracked `🧫️fixtures/⏳️asyncprobe/…/target/` Cargo build directory (confirmed via
  `git ls-files` returning zero tracked entries under it) were excluded as build litter, not schema
  definitions.

## 8. Unresolved questions for the ticket owner / WP3-WP4

1. **🗨️dialog-origin family**: no eligible non-element owner exists today (§4.5). Needs either a new
   module or a taxonomy exception for renderer-shell-internal UI FSMs.
2. **🪪️host-bootstrap**: target-scoped (React), not domain-scoped; `semanticAllowedOwnerLevels` has no
   "target" level (§4.6).
3. **🎟️invite-capability / 🌱️artifact-creation**: is UI-facing state/label data (as opposed to wire/data
   contracts) appropriately co-owned by `📇️directory/🧬️schema/`, or does the taxonomy need a distinct
   facet for UI-presentation contracts? No `schemaFacetKinds` entry currently covers this.
4. **👥️presence-scope**: is the ShellHost runtime-key/surface-routing concern the same contract as
   `🏪️store/👥️presence`'s peer-commit/peer-admission, or a genuinely distinct sub-contract needing its
   own versioned entry there?
5. **🧭️opening / 🚪️opening naming collision** (§4.7) needs a decision, not just a mechanical move.
6. **`os/🧪️fixtures` vs `os/🧫️fixtures`** (§5): confirm the `🧪️fixtures/↩️gis-map-approval-history-v1`
   copy is dead before any deletion — this audit did not delete anything (read-only).
7. **Filename convention**: this audit found four concurrent JSON-Schema filename styles in the
   partition (`🔣️.json` canonical, `🔣️.schema.json`, `🧬️.schema.json`, `<slug>.schema.json`) and two
   JSON Schema dialects (draft-07 canon vs 2020-12, used by 5 of 6 ShellHost contracts). Both are
   independent of the location/ownership question and should be fixed uniformly during relocation, not
   contract-by-contract.
8. **Concurrent edit in `🗨️dialog-origin/🎥️tutorial/`** (§4.5): re-verify against the committed state
   before any relocation work lands, since this audit's read of that one directory reflects a
   moving working tree, not a stable blob.
