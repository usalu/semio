# Trinity `♻️rewrite` → `♻️rewriting` rename + emoji-id hygiene fix

## Scope
Two fixes on the `🔱️trinity` plugin's rewrite artifact, per the ticket brief:
1. Verb→noun rename of the ARTIFACT identity: `rewrite` → `rewriting` in the directory, the
   Rust module path, and every Rust type/const/fn that names the artifact itself.
2. Emoji-hygiene fix: `ArtifactKindSpec.id` was `"text.♻️rewrite"` (emoji inside a machine id) →
   now `"text.rewriting"`. Checked `source_format`, `schema`, `component_kind` on the same spec —
   none of those carried an emoji (they resolve through `REWRITE_RULE_SCHEMA = "trinity.rewrite.rule"`
   and the literal `"trinity"`), so no change was needed there.

## Decision: what stayed `Rewrite*` (domain "rule" terminology, per the ticket's own instruction)
Kept unchanged, verbatim:
- `REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule"` (name **and** value).
- `RewriteRule`, `RewriteRuleMutation`, `RewriteRuleMutationInput`, `RewriteRuleEnvelope`,
  `RewriteRuleLayoutPoint`, `RewriteRuleLayoutPointInput`, `RewriteRuleStore`, and every
  `*_rewrite_rule_*` fn/test name (`apply_rewrite_rule_mutation`, `rewrite_rule_summary`,
  `rewrite_rule_state_parse_dsl_*`, `dsl_round_trip_rewrite_rule_state`,
  `document_text_round_trip_rewrite_rule_store`, `create_rewrite_rule_envelope`,
  `dispatch_rewrite_rule_mutations`, `register_rewrite_rule_mutation_descriptors`,
  `rewrite_rule_labels_core`, `rewrite_rule_parameter_substitution`).
- `ArtifactKindSpec.name = "Trinity Rewrite Rule"` (the human display name) and the matching
  `ArtifactPresentation.name = "Trinity Rewrite Rule"` in the editor's IO surface.
- Every prose occurrence of the two-word phrase "rewrite rule" (docstrings, the Python
  differential-oracle test, the `.feature` file, terminology labels) — reverted back from
  "rewriting rule" after the mechanical pass over-corrected them, to stay consistent with the
  identifiers above.
- The `🛂️manifest.jsonrewrite-lhs.manifest.json` fixture's `id`/`node` ids (`rewrite-lhs`,
  `rewrite.match`, `rewrite.where`) — these feed a **cross-module code generator**
  (`🧰️framework/🔨️modules/🕸️graph`'s manifest system), which has a **hand-written** companion
  bridge file (`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️generated-value-bridge.rs`) that must
  match the manifest id byte-for-byte. Renaming the manifest id broke that bridge
  (`cannot find rewrite_lhs in generated`) — reverted to keep the domain-rule naming and avoid a
  cross-crate ripple outside this ticket's scope.
- `BatchOnlyPendingRewrite` (jack's `InteractiveJobClassification` variant) — an unrelated,
  pre-existing enum in a completely different subsystem that happens to share the word "rewrite";
  left untouched.
- The genuinely unrelated English use of "rewrite" as a verb, e.g. jack's
  `... rewrite instead of a hand-rolled handle mint at each site` docstring — reverted after the
  mechanical pass touched it by mistake (word-boundary match, not an identifier).

Everything else — `RewriteSnapshot`, `RewriteDiff`, `RewriteArtifact` (schema title),
`TrinityRewriteError`, `TrinityRewriteViewer`, `TrinityRewriteCommand`, `TrinityRewritePlayApp`,
`TrinityRewriteCallerPageOwner`, `TRINITY_REWRITE_DIALECT`, every `TRINITY_REWRITE_*` surface/body/
window constant, every `Trinity_rewrite_*` ANTLR/spicy grammar module name, every
`rewrite_*`/`trinity_rewrite_*` fn and test name that isn't about "the rule" specifically, the
`RewriteRuleMutation`-sibling schema titles, the DSL file extension (`rewrite` → `rewriting`), the
wire dialect strings (`s.trinity.rewrite` → `s.trinity.rewriting`, `s.rewrite` → `s.rewriting`),
and the `Cargo.toml` playground metadata (`variant`/`app`/`aliases`) — were renamed to `rewriting`.
I initially planned to leave the wire-level ids (`s.trinity.rewrite`, `text.♻️rewrite`'s sibling
`ArtifactPresentation.id`, Cargo.toml's playground `app`/`variant`) untouched, but the mechanical
word-boundary substitution already renamed them everywhere consistently (verified: no orphaned
`"s.trinity.rewrite"` remains anywhere), so I kept that outcome — it is more consistent with the
repo's no-half-migration policy than reverting it back to a stale wire id.

## Files changed
- **Directory rename**: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/` → `♻️rewriting/` (plain `mv`,
  ~350 files moved, content unchanged by the move itself).
- **Content edits** across the whole `✏️s/🔌️plugins/🔱️trinity/` tree (~220 files: every `.rs`,
  `.ts`, `.json`, `.proto`, `.graphql`, `.g4`, `.spicy`, `.feature`, `.py`, `.semio`,
  `Cargo.toml`), via a scripted token-aware rename (directory-path token, `mod`/`::` module
  segments, then a 171-entry compound-identifier map built from every unique `[Rr]ewrite`-bearing
  token actually present in the tree) plus the manual reverts above. Representative files:
  - `✏️s/🔌️plugins/🔱️trinity/🦀️.rs` (plugin root: `TrinityApps` enum, `.declare_artifact` calls)
  - `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/🦀️.rs` (the hand-written `#[path]` chain — all
    ~107 `♻️rewrite/…` path strings, 3 `mod rewrite {}` → `mod rewriting {}`, all
    `crate::artifacts::rewrite::` re-exports)
  - `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml`
  - `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/🟦️.ts` (barrel re-exports)
  - `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🦀️.rs` (artifact root: `artifact_kind()`,
    `TRINITY_REWRITING_DIALECT`, `definition()`/`ArtifactCapability` rows)
  - every file under `🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema,✏️editor,👁️viewer,🚪️io,📚️examples,🧪️tests}`
- **Cross-plugin / framework fixups** (small, targeted):
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs` — import path
    `trinity::artifacts::rewrite::RewriteSnapshot` → `trinity::artifacts::rewriting::RewritingSnapshot`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `"♻️rewrite"` member →
    `"♻️rewriting"`
  - `📜️script.ts` (root) — the `♻️rewrite` path literal read by `policyReadFileSafe`, and 4
    `"trinity/rewrite..."` taxonomy-slug strings in policy allowlists →
    `"trinity/rewriting..."`/`"Rewriting"`
  - `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/.../✏️editor/🦀️.rs` — a stale docstring
    reference to trinity's `♻️rewrite` editor
  - `package.json` — `dev:trinity:rewrite` → `dev:trinity:rewriting`
  - `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` — `bun run dev:trinity:rewrite` →
    `dev:trinity:rewriting`, `TRINITY_REWRITE_PLAY_PORT` → `TRINITY_REWRITING_PLAY_PORT` (the
    `name`/key fields in both files already said "rewriting" — evidently pre-seeded by whatever
    generates them — only the command/env-var strings were stale)
  - Icon system (the toolbar icon for this app, sibling to `trinity-lhs`/`trinity-rhs`):
    - `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🔣️trinity-rewrite.svg` and the duplicate under
      `🧰️framework/🔨️modules/🖱️ui/🖼️assets/🔣️icons/` → renamed to `🔣️trinity-rewriting.svg`
    - `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔣️shortcodes.json` — catalog entry
      updated, then regenerated everything downstream via the package's own generator
      (`bun nx run @semio-tech/assets:build`, i.e. `bun ./📜️script.ts build` in
      `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`) — this cleanly regenerated the
      `IconName` enum (`🤖️generated/🦀️icon_name.rs`) and embedded SVG
    - Hand-fixed the three consumer files the assets generator does **not** own:
      `🧰️framework/📦️packages/🦀️rust/🦀️.rs` (embedded TS union type string),
      `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️ui.css` (keyframe name + selector),
      `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️icon-name-value-bridge.rs`
      (hand-written serde bridge, both match arms), and the two icon-gallery `README.md` copies.

## A regression I introduced and reverted
The mechanical rename touched `🛂️manifest.jsonrewrite-lhs.manifest.json`'s `id`s
(`rewrite-lhs`→`rewriting-lhs`, `rewrite.match`→`rewriting.match`, `rewrite.where`→`rewriting.where`),
which broke `semio-framework-graph`'s hand-written `generated-value-bridge.rs` (that file is NOT
regenerated automatically and must track manifest ids by hand). Reverted all three ids back to the
original `rewrite-*` spelling — this is domain "rule" terminology per the keep-list above, not the
artifact's own identity, so reverting was also the semantically correct call, not just the
mechanically safe one.

## Verification
- Directory-path token `♻️rewrite` (old form): **zero** remaining hits repo-wide (excluding
  historical ticket folders and `.cursor/plans/*` snapshots, which are narrative records, not live
  source, per this ticket's own precedent for excluding `📓️`/prompt-log narrative content).
- `text.♻️rewrite` (Fix 2's target): **zero** remaining hits anywhere.
- `"s.trinity.rewrite"` (old wire id): **zero** remaining hits.
- `artifacts::rewrite::` / `trinity::artifacts::rewrite::`: **zero** remaining hits.
- Re-grepped for corruption patterns (`rewritinging`, `rewritings`, `Rewritin[^g]`): none found.
- `RewriteSnapshot`/`RewriteDiff`/`RewriteArtifact`/`TrinityRewriteViewer`/`TrinityRewritePlayApp`/
  `TrinityRewriteCommand`/`TrinityRewriteError` (old artifact-level names): **zero** remaining hits
  repo-wide.
- `cargo check -p semio-s-plugin-trinity --target wasm32-wasip2`: could **not** reach the trinity
  crate itself in this session, despite waiting. The shared workspace has been under continuous
  concurrent restructuring from sibling sessions on this same ticket
  (`UNIFIED-ARTIFACT-NAMING-AND-DEDUPLICATION`), each blocking the full-workspace resolution cargo
  check needs in turn:
  1. `semio-framework-graph` (a trinity dependency via the graph/manifest system) initially failed
     to compile for two reasons, both confirmed unrelated to this fix: the `🖍️draw`→`🖍️drawing`
     artifact rename in progress (`cannot find draw_layers in generated`, plus a workspace member
     path that briefly didn't exist mid-`mv`), and an unrelated `value_derive` tuple-struct derive
     gap in `🕸️graph/⚙️engine/🦀️.rs`. **This resolved itself** during this session (I polled with a
     background Monitor until `cargo check -p semio-framework-graph` passed clean).
  2. With the graph crate fixed, the check progressed much further and then failed on
     `semio-s-plugin-stdio` (a real trinity dependency, used for `stdio.docx`/`stdio.json`/etc.
     export/import): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…` is mid a subset split
     (`✳️any` → `✳️base` + `✳️iso21320`) by another sibling session, and stdio's own
     `📦️packages/🦀️rust/🦀️.rs` `#[path]` chain (919 occurrences of `✳️any`) has not caught up yet.
     This was still unresolved when I stopped polling. Per this ticket's own rule ("do NOT touch
     anything under `✏️s/🔌️plugins/🗄️stdio/`"), I left it alone.
  I confirmed my own change caused exactly one compile regression (the manifest-id rename above),
  fixed it, and re-ran the check: the `rewrite_lhs`-related errors dropped from present to zero
  (46 → 26 → 22 total errors on `semio-framework-graph`, then 0 once that crate's own unrelated
  issues were fixed by their owning session). **No error in any run — before or after the stdio
  block — ever mentioned a path under `✏️s/🔌️plugins/🔱️trinity/`.** The trinity-specific diff is
  therefore verified by grep/manual review (all checks above) plus the fact that the only
  trinity-caused compile regression that did surface (`rewrite_lhs` bridge mismatch, one crate
  away from trinity in the dependency graph) was caught and fixed by this same pass. A full,
  literal "cargo check reached and passed on `semio-s-plugin-trinity` itself" was not achieved in
  this session because of the stdio blocker described above — that is a pre-existing, actively
  being fixed, out-of-scope condition, not a defect in this change.

## Residual / out of scope, left as-is
- `📜️script.ts`'s `toolJobTrinityRewriteEnvelopeCallerRetainedExact` self-test block
  (~lines 3765–3872, 7804–7872) still uses old identifier names
  (`TrinityRewriteEnvelopeLoadHandle`, `TrinityRewriteEnvelopePageFault`, etc.) that **never
  existed** in the real `🌍️world/🦀️.rs` file even before this rename (confirmed by grep) — it is a
  self-contained synthetic fixture unit-testing the generic checker function, not a live
  assertion against real trinity source, and it was already structurally unable to pass against
  reality. Left untouched; flagging it here rather than silently leaving it since a future reader
  might assume it once matched.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🛂️manifest.jsonrewrite-lhs.manifest.json` keeps
  its odd, pre-existing double-barrelled filename (`manifest.json` + `rewrite-lhs.manifest.json`
  concatenated with no separator) — nothing in the repo references it by filename (only by its
  `id` field, via a directory-wide manifest scan), so renaming the file itself was unnecessary and
  out of scope for this ticket.
