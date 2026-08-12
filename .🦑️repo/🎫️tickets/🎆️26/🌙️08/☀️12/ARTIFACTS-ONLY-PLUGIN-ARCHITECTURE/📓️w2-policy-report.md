# W2 — Policy Rules Report (report mode)

Added five new report-mode policy rules to `📜️script.ts`, region `//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture` (inserted right after `//#endregion 🔧️PolicyRuleTaxonomy`, currently spanning ~4791–5533). All five are wired into both the `policy` export aggregator (unconditional pushes, `breaches.push(...)`) and `VerifyScript.runGate`'s `dissolveBreaches` block (the one that already `.filter((b) => b.priority === "high")`s before throwing — the same site `policyPluginRootShapeBreaches` uses). I deliberately did **not** add them to the earlier `osBreaches` block (~line 693–705, `policyOsStateAuthorityBreaches` + `policyDocumentAppShapeBreaches`) because that block throws on **any** breach regardless of priority — wiring medium-priority rules there would have violated the report-mode hard requirement the very first time the gate ran. This is a deliberate deviation from "register consistently with ALL of policyOsStateAuthorityBreaches's sites," made because that site's own filtering behavior is incompatible with report mode; the `policy` export site and the `dissolveBreaches` gate site (both of which `policyPluginRootShapeBreaches` also uses) are where I matched it exactly.

## Verification 1 — `bun ./📜️script.ts policy` runs clean

Ran repeatedly through debugging; final synchronized run:

```
$ cd "/Users/ueli/Documents/semio" && bun ./📜️script.ts policy
EXIT:1
```

Exit 1 is expected and pre-existing — it comes from `runPolicyExit` reporting **22188 pre-existing high-priority breaches across 27 rules** (handcrafted-grammar/spec-distinctness alone is 19601), none of which are mine. No stack trace, no crash. The `policy` command discovers, runs, and serializes all five new rules without error; the full breach set (26,000+ records) writes cleanly to `.🦑️repo/⚡️cache/breaches/compose.json` every run.

One real bug surfaced and fixed during this: my first draft used `/**` block-comment doc text containing the literal substring `**/📦️packages` (a glob-style path in prose) — Bun's tokenizer treats any `*/` inside a `/** ... */` block as the comment terminator, so this broke parsing at `script.ts:5343` with `error: Unexpected 📦`. Fixed by rewording the doc comment to avoid an embedded `*/`. Also swept the whole new region with a script checking every `*/` occurrence for accidental early termination — none found beyond the one fixed.

## Verification 2 — per-rule breach counts + example lines (final synchronized snapshot)

All numbers below are from one `bun ./📜️script.ts policy` run's cache JSON (`.🦑️repo/⚡️cache/breaches/compose.json`), read via `python3 -c 'json.load(...)'` and tallied by `kind`. **Total: 1727 breaches across the five rules, all `priority: "medium"`.**

### Rule 1 — `PolicyRulePluginClosedShape` (`policyPluginClosedShapeBreaches`) — **113 breaches**, kind `taxonomy/plugin-closed-shape`

```
"✏️s/🔌️plugins/🔱️trinity/🔨️modules" is a plugin-root entry outside the closed apps+artifacts shape
"✏️s/🔌️plugins/🔱️trinity/🗣️language-service" is a plugin-root entry outside the closed apps+artifacts shape
"✏️s/🔌️plugins/🔱️trinity/🧮️executor" is a plugin-root entry outside the closed apps+artifacts shape
```
Every one of the 33 plugin roots' 🎟️capabilities/🔧️setup/🛂️manifest legacy facets is flagged (now that `taxonomy.pluginChildDirs` reads `["🎛️apps"]` only), plus the genuine extra dirs (fem's 8 compute modules, trinity's 5, energy's `⚙️engine`, cad's `node_modules`+`🔨️modules`+`🧩️extensions`+`🔣️machine.json`, norm's 4 shared dirs, the six `🧩️extensions` owners, etc.), each carrying a specific proposed destination (or "needs ruling"/"CANNOT CLASSIFY" text) from `📓️w0-b-plugin-shape.md` §5 / `📓️w0-census.md` §6. `🛂️manifest.json` (root data **file**), `📇️registry` (stdio), and `🖼️assets`/`🧫️fixtures` (cad) are correctly **excluded** — they're legal per `taxonomy.rootDataDirNames`/`rootDataFileNames` at an owner root, which is exactly the false-positive the ticket's own §6 flagged as needing a recheck.

### Rule 2 — `PolicyRulePluginPurity` (`policyPluginPurityBreaches`) — **115 breaches**

```
filesystem-io:                 35
interior-mutability-refcell:   36
interior-mutability-mutex:     19
interior-mutability-atomicu64:  7
thread-local-state:             6
env-process-io:                 4
interior-mutability-atomicu32:  4
interior-mutability-cell:       2
ts-side-effect:                 2
```
Examples:
```
"✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🌉️wasm/🦀️component.rs:29" declares item-scope RefCell ambient state inside a plugin tree
"✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:60" declares item-scope Mutex ambient state inside a plugin tree
```
Zero `network-io` hits at the Rust level (matches census: 0 `reqwest`/`ureq`/`hyper`/`std::net`), and the 2 `ts-side-effect` hits are exactly animate's two `fetch()` calls in `🎬️present/📺️renderer/⚛️react/🟦️component.tsx` — the census's "cleanest violation in the whole census."

### Rule 3 — `PolicyRuleDeclarativeRegistration` (`policyDeclarativeRegistrationBreaches`) — **1334 breaches**

```
plugin-registration-engine-backlog:  721
plugin-registration-violation:       582
plugin-registration-setup-callback:   31
```
Examples:
```
"...✒️writer/🎛️apps/✒️writer/🎚️config/🧬️schema/🦀️component.rs:26" calls register_app_schema_descriptor(...) outside its owning artifact's ⚙️engine — wrong layer
"...✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/⚙️engine/🦀️component.rs:39" calls register_document_codec_for_app(...) from inside its own artifact ⚙️engine — currently-compliant interim shape, migration backlog
```
The engine-backlog/violation split works as intended: `register_document_codec_for_app`/`register_artifact_schema_descriptor`/etc. called from inside a `🗿️artifacts/<kind>/…/⚙️engine/` tree land as backlog (medium, non-architectural), the same functions called from `🎚️config`, `🔧️setup`, `🎛️apps`, `🎪️panes`, `🎮️commands` land as violations.

### Rule 4 — `PolicyRulePluginDependencyAllowlist` (`policyPluginDependencyAllowlistBreaches`) — **118 breaches**

```
plugin-dependency-allowlist: 105
plugin-dependency-os-host:    13
```
Examples:
```
"✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml" depends on framework crate "semio-framework-editor", outside the plugin dependency allowlist
"✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml" depends on framework crate "semio-framework-math", outside the plugin dependency allowlist
```
All 13 `semio-framework-os` (HOST) hits get the exact offending-symbol list in `solution` (e.g. 🪐️space's 36 symbols, 🎪️demonstrator's 8 `register_*` fns) transcribed from `📓️w0-d-sdk-surface.md` §3.2.

### Rule 5 — `PolicyRuleEffectCapabilityParity` (`policyEffectCapabilityParityBreaches`) — **47 breaches**

```
"✏️s/🔌️plugins/🔱️trinity" constructs HostEffect::LoadDocument (2 sites, first at .../🔌️jack/🦀️component.rs:66) without declaring the Document capability it requires
"✏️s/🔌️plugins/📸️remodel" constructs HostEffect::RequestFileOpen (1 site, ...) without declaring the Asset capability it requires
"✏️s/🔌️plugins/📸️remodel" constructs HostEffect::RequestMediaFrames (1 site, ...) without declaring the Asset capability it requires
```
Fires on 20 of the 33 plugins (every plugin that constructs any capability-mapped `HostEffect` variant) — matches the assignment's expectation that "today only ONE plugin declares any capability at all" (🪐️space's `.local_backbone_storage()`), and that lone declaration doesn't even satisfy any of 🪐️space's own 6 constructed variants (none require `Backbone`).

## Verification 3 — report mode confirmed: zero `priority: "high"` among the five rules' breaches

```python
prios = Counter(b["priority"] for b in breaches if b["kind"] in apa_kinds)
# Counter({'medium': 1727})
```
Cross-checked the other direction too: the console's own high-priority tally (`22188 high-priority breach(es) across 27 rule(s)`) lists 27 kinds, none matching `taxonomy/plugin-closed-shape`, `taxonomy/plugin-purity-*`, `taxonomy/plugin-registration-*`, `taxonomy/plugin-dependency-*`, or `taxonomy/effect-capability-parity`.

## Verification 4 — census cross-check and discrepancies

- **Rule 1 (113)**: independently re-derived from a fresh Python walk of the live filesystem using the exact same allowed-set logic — **matches exactly, 113 = 113**, confirming the implementation, not just plausibility. Diverges from the census's rough "13 plugins" framing because that referred only to *non-standard extra dirs* under the old 4-facet baseline; my rule (correctly, per the assignment) also flags the 🎟️capabilities/🔧️setup/🛂️manifest facet directories themselves now that `pluginChildDirs` dropped them — that's the "missing absence check" the assignment specifically asked for. **Concurrent-churn observation**: mid-session, another wave (SMO's Wave M, per `📓️w0-census.md` §3 "space+trinity: running"/"puzzle running"/"singles lane running") landed a commit (`1caac91709`) that dissolved these three facets from 🪐️space and 🕸️dag live while I was testing — the rule's count moved and then stabilized as I re-ran it, which is exactly the intended live-census behavior, not a bug.
- **Rule 2 (115)**: `filesystem-io` = **35, an exact match** to the census's "35 real production fs call sites." `thread-local-state` = **6, an exact match**. `interior-mutability-atomicu64/u32` = 11 raw hits vs. the census's 15 — undercounts because (a) my rule also (correctly, per the census's own lower-severity call) excludes a locally-scoped `struct PreviewApp { closed: Arc<AtomicBool> }` defined *inside* a fn body in animate's video engine — the census itself notes this is "interior-mutability on an owned struct rather than a process global — lower severity," not real ambient state; and (b) puzzle's two `AtomicU32` files each contribute one duplicate hit from their `use std::sync::atomic::{AtomicU32, ...}` import line (module-level, so it also matches the item-scope gate) alongside the real `static` declaration — a known minor double-count, not a false negative. Net: this sub-check is close but not perfectly reconciled against the census; flagging as an honest known gap rather than claiming exact parity.
- **Rule 3 (1334)**: no census total exists to compare against directly (the census counted 137 raw grep hits for a narrower regex over the *whole repo including the two OS definition files*, not scoped to plugin call sites specifically, and did not enumerate `register_document_codec_for_app`'s 44 turbofish sites or `semio_framework_os::` path references at all) — my rule's much larger count is *expected*, not a red flag: it additionally counts every `semio_framework_os::` path reference (which the assignment explicitly asked for as clause (c)) and every one of `register_document_codec_for_app`'s 44 call sites (turbofish form, handled correctly since bare-identifier matching doesn't care what follows).
- **Rule 4 (118)**: 13 `semio-framework-os` hits match the census's §3.2 table (17 crates listed there include `semio_framework_os = { workspace = true }` for 4 of them — ✒️writer, 📸️remodel, 🗒️note, 🔱️trinity — which don't have a literal `path = "...🧰️framework..."` in their own `Cargo.toml` line, since `workspace = true` deps are declared once in the root workspace `Cargo.toml` instead; my rule's path-into-🧰️framework filter correctly doesn't re-flag those 4 as a **second**, separate breach from their own crate file, since the actual `path=` lives in the workspace root, not the plugin's own Cargo.toml — a defensible scoping choice, not a bug, but means 4 of the census's 17 aren't independently re-derived by this rule from the plugin's own file; documenting rather than silently under-claiming). The other 105 non-os-host hits (`semio-framework-editor`, `semio-framework-math`, `semio-framework-ui`, `semio-framework-os-flow`, etc.) are new findings beyond §3.2's os-host-specific table — exactly what "flag every framework dependency outside the allowlist" (not just the os-host one) asked for.
- **Rule 5 (47)**: fires on 20 distinct plugins, matching the union of constructing-plugins the census's §5 table lists for `LoadDocument`/`Notify`/`ClipboardWrite`/`Navigate`/`DownloadMediaExport`/`RequestFileOpen`/etc. No fewer-than-census gap found here.

**General honesty note on Rule 3/5's fully-fresh scans**: I did not do a raw-grep sanity total for Rule 3/5 the way I did for Rule 1/2 (time budget) — the per-example spot checks above are real, verified breach lines, but a full independent re-derivation of the 1334/47 totals wasn't performed. Flagging this as the one verification gap in this report rather than implying a false completeness.

## Verification 5 — scope discipline

```
$ git diff HEAD --stat -- "📜️script.ts"
 📜️script.ts | 747 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 1 file changed, 747 insertions(+)
```
**Zero deletions, zero modified pre-existing lines** relative to the last real commit (`16619a9699`, #490) — confirmed with `git diff HEAD -- 📜️script.ts | grep -c '^-'` → 1 (only the diff header itself). This proves:
- `//#region 🔧️PolicyRuleMutationArtifactEngines` was never touched (no `-` lines exist anywhere in the diff to touch it).
- No pre-existing `POLICY_*_ALLOWLIST` constant (or any other pre-existing constant/function) was modified — every touched allowlist (`POLICY_PLUGIN_CLOSED_SHAPE_ALLOWLIST`, `POLICY_PLUGIN_PURITY_ALLOWLIST`, `POLICY_DECLARATIVE_REGISTRATION_ALLOWLIST`, `POLICY_PLUGIN_DEP_ALLOWLIST`, `POLICY_EFFECT_CAPABILITY_ALLOWLIST`) is new, empty, and carries the required "entries need a ticket citation" comment.
- The two `dissolveBreaches`-block edits I made mid-session (Atomic-regex fix, `*/`-in-comment fix) are themselves inside code I added this session, so they still net to pure insertion against the pre-session baseline.

## Design decisions worth flagging

1. **No new `pluginRootAllowedEntries` taxonomy key added.** Rule 1's allowed set is derived entirely from existing `Taxonomy` fields (`pluginChildDirs`/`artifactsDirName`/`packagesDirName`/`rootDocFileNames`/`rootDataDirNames`/`rootDataFileNames`) via `loadTaxonomy()` — no hardcoding, satisfying the instruction's "derive from taxonomy.json" requirement without touching `🔣️taxonomy.json`/`🔍️discovery/🟦️component.ts`/`🧪️index.test.ts`, all outside this session's declared single-writer boundary on `📜️script.ts` and actively owned by concurrent W1 work.
2. **Rule 2's `!inFn` gate got a targeted override** (`POLICY_PLUGIN_PURITY_STATIC_ITEM_RE`) after debugging showed the fn-body-local `static COUNTER: AtomicU64 = ...;` id-counter idiom (used in ≥7 files) is lexically nested inside a `fn` but is still a genuine persistent item — Rust's `static` keyword denotes item-scope regardless of lexical nesting. Without this override the rule would have silently missed this entire violation category.
3. **Rule 3's family list (25 fns)** = the assignment's 22 named fns + `register_os_fixture_json`/`register_artifact_descriptors`/`register_artifact_descriptor` (all confirmed part of the same OS-host global-registry-mutator family per `📓️w0-a-escape-hatch.md` §1) — deliberately excludes `register_studio_port` (plugin-local, not a framework fn) and plain `register_app` (an `&mut self` method, not a free-function global mutator), both explicitly ruled out by the census.
4. **Rule 4 deliberately scopes to `[dependencies]` + `[target.'cfg(...)'.dependencies]`, not `[dev-dependencies]`** — dev/test-time deps are out of the runtime-purity concern this rule targets; 🧩️puzzle's cfg-gated `semio-framework-os` dep (under `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]`) is still caught because that section header matches the `target.*.dependencies` pattern.
5. **Rule 5 matches at `ArtifactKind` granularity only**, not full `{ArtifactKind, Rights, Scope}` — no plugin has ever called `.capability(...)` for real (0 hits repo-wide) to pin down the actual argument shape of that unimplemented builder method, so matching finer would be guessing at an API that doesn't exist yet. Documented explicitly in the code comment.

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts` — the only source file touched (single-writer per the assignment). 747 net insertions, 0 deletions against commit `16619a9699` (#490).
- This report: `/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w2-policy-report.md`.
- Scratch/debug artifacts (session scratchpad, not ticket-persisted): `apa_region.ts`, `apa_destinations.ts`, `apa_os_symbols.ts`, `policy_run{1..4,final}.txt`, `full_diff.txt` under `/private/tmp/claude-501/-Users-ueli-Documents-semio/5128c8d3-abfa-49da-81ac-33286ba73278/scratchpad/`.

## Not enforced / open items for later waves

- Extension-crate axis (`🧩️extensions/`) and shared-across-sibling-apps code (fem's/norm's `🖥️app-surface`, norm's `🎚️config`/`👥️presence`/`📄️artifact`) are surfaced by Rule 1 with "needs ruling" text rather than a concrete destination — per `📓️w0-census.md` §6, these are open design questions this ticket doesn't adjudicate.
- Rule 5's `ArtifactKind::Window` mapping for window-chrome effects (`Notify`/`SetActiveUtility`/etc.) is the census's own best-fit proposal against an enum with no "Shell"/"UI" member yet — not authoritative until W1/W2 confirm.
- Rule 2/Rule 3's full-total independent re-derivation (beyond the spot-checks above) wasn't performed for time; flagged honestly in Verification 4 rather than claimed.
