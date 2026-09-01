# 📊️ Vocabulary-Gap Census — `semantic-stem-unresolved` / `semantic-stem-ambiguous` / `directory-kind-unresolved`

Source: `$T/🗑️temp/🔣️modules-plan.json` (`.unresolved`, scope `🧰️framework/🔨️modules`, baseline
`bb06c41f73f0122fbed315b7487428b976f99921`), cross-checked against `git ls-files` and the mechanism
in `🧹️normalization/🟦️.ts` (`canonicalFile` ~3089-3132, `matchDirectoryKind` ~2206,
`resolveFileKind` ~2239).

## 1. The dominant class was NOT missing vocabulary — it was a `fileKinds` emoji mismatch

`resolveFileKind` strips a file's leading emoji from its stem only when that emoji equals the
selected **file kind's own registered emoji** (🧹️normalization/🟦️.ts:2262,2265,2295,2300). If it
doesn't match, the emoji rides along as `semanticEvidence` into `matchDirectoryKind`, which then
requires a **directory** kind sharing that exact emoji — an unrelated, much stricter test than
"does this stem denote a domain concept".

Repo-wide count of the *actual on-disk* leading emoji per extension (`git ls-files`), vs. the
registered `fileKinds` emoji before this slice:

| extension(s) | registered emoji (before) | real on-disk emoji (count) |
|---|---|---|
| `.svg` | `svg` → 🎨️ | 🔣️ ×690, 🎨️ ×2 |
| `.3dm`, `.stp`, `.step` | `model-3d` → 🧊️ | 📐️ ×208+6+1 |
| `.glb`, `.gltf`, `.obj`, `.stl`, `.ply`, `.las` | `model-3d` → 🧊️ | 🧊️ ×~100 (consistent) |
| `.dxf`, `.dwg` | `model-3d` → 🧊️ | 🖊️ ×3 |
| `.ifc` | `model-3d` → 🧊️ | 🏗️ ×4 |

This single mismatch alone explains **569** rows under `🔣️icons`, **240** under `🎨️representation`,
**107** under `🧪️fixtures`, **106** under `👆️cursor`, **70** under `🪧️logos`, **24** under
`🖼️images`, **17** under `📛️badge`, **7** under `🗚️fonts` — the entire "one distinctly-named icon /
logo / cursor / 3D asset per file" long tail that made `semantic-stem-unresolved` look like an
enormous vocabulary problem. It isn't: once the file kind's emoji matches reality, the stem strips
cleanly and falls through to the already-registered `asset-subject` (🖼️) directory kind.

Fixed (`fileKinds`): `svg` emoji → 🔣️; `model-3d` split into `cad-source-model` (📐️: `.3dm` `.stp`
`.step`), `mesh-model` (🧊️: `.glb` `.gltf` `.obj` `.stl` `.ply` `.las`), `drawing-2d-model` (🖊️:
`.dxf` `.dwg`), `building-model` (🏗️: `.ifc`) — a clean partition of the extensions `model-3d`
already owned, matching each sub-format's real, already-consistent repo convention rather than one
emoji forced across four unrelated physical formats. `fileKindResolutionRules` remapped to match.

## 2. Second family: `asset-subject`'s pattern was ASCII-lowercase-kebab only

Real named content assets under `assets` use underscores and mixed case:
`capsule_J`, `tambour_first-storey`, `cylindric-tambour_first-storey`, `kit_horizontal`,
`cursor_ew-resize_dark_inkscape`, `emblem_dark_round_inkscape`. `asset-subject`'s `slugPattern`
(`^(?!assets$)[a-z0-9]+(?:-[a-z0-9]+)*$`) rejected all of them. Widened to
`^(?!assets$)[A-Za-z0-9]+(?:[-_][A-Za-z0-9]+)*$` — same concept, same emoji (🖼️), same
`parentKindIds: ["assets"]`, just accepting the naming the content actually uses. Combined with §1,
this resolves the **~313** `🌱️metabolism` 3D-asset rows in one pattern edit plus the file-kind fix.

## 3. Third family: real domain vocabulary — `directory-kind-unresolved` (111 rows, ~90 distinct words)

These are genuine nested domain concepts inside `🎭️actor`, `🎠️kernel`, `🖱️ui/🧬️contract/🧵️retained`
and similar module trees — an event-sourcing/actor-lifecycle type vocabulary
(`admission`, `activation`, `lifetime`, `composition`, `retirement`, `transaction`, `payload`,
`return`, `content`, `message`, `authority`, `credit`, `evidence`, `validation`, `binding`, `typed`,
`value`, `wire`, `nodes`, `graph`, …). Grouped every distinct stem's on-disk leading emoji:

- **72 words carry exactly one emoji** everywhere they appear — safe to register as an unscoped,
  exact-word `semanticDirectoryKinds` entry reusing that already-established repo emoji (never
  inventing a new one). Registered **68** of them (see §5 for the 4 held back).
- **11 words carry 2-3 *different, conflicting* emoji** across different subsystems
  (`admission` 🏘️/📨️/🎟️, `handback` 🚪️/📮️, `hash` #⃣/🔢️, `instance` 🏘️/🚪️, `metadata` 📋️/🪪️,
  `output` 📤️/📥️, `patch` 📋️/🩹️, `root` 📄️/🪪️/🌳️, `transport` 📡️/🔒️, `typed` 🧾️/🌳️/🧬️,
  `wire` 📦️/📥️). One shared English word, several *different* domain roles per subsystem — minting
  one flat kind per word would be a wrong synonym, not vocabulary closure. **Not registered.**
  Would need per-subsystem `parentKindIds` scoping by someone who owns those trees (actor/kernel/
  retained-contract), which is outside this slice's visibility.

## 4. `semantic-stem-ambiguous` (167 rows) — a mechanism side-effect of §3, not a new gap

166/167 rows are `test-case` vs. `test-fixture-member` (both emoji 🧪️, near-identical patterns).
Root cause, read directly out of `matchDirectoryKind`: when a `🧪️<name>` file/dir's *immediate
parent* isn't literally `tests`/`mutation-test-subject` (`test-case`'s only allowed parents) nor an
already-resolved `test-case`/`test-fixture-member` (the other's only allowed parents), the function's
ambiguity fallback returns **both** emoji-🧪️ candidates instead of "no match" — regardless of how
implausible the context is. All 166 rows are co-located test files (`bytes.rs`, `contention.rs`,
`fixture.json`, `story.tsx`, `descriptor.json`, …) sitting directly inside one of the exact §3 domain
module directories (`builder`, `payload`, `admission`, `composition`, `page`, `clock`, …) — 15
distinct basenames repeated across ~30 directories.

**Fix chosen** (parentKindIds scoping, per the brief — never pattern-deletion): extended
`test-case.parentKindIds` with every §3 word that was actually registered, so a co-located test file
living directly in an ordinary domain-module directory now resolves as `test-case` (its own
`parentKindIds` list contextually wins over `test-fixture-member`, whose parents are `test-case`/
`test-fixture-member` only — no new overlap introduced). This is additive scoping, not a pattern
rewrite, and the existing "fixture-member nested under an already-resolved test-case" shape is
unaffected (regression-tested, see report).

The 29 rows parented by a §3 multi-emoji word (`admission`, `patch`, `typed`, `instance`, `root`,
`output`, `metadata`, `transport`, `handback`) stay ambiguous, for the same reason those words
weren't registered.

## 5. Two representative words could not be registered at all — a taxonomy-schema constraint, not a choice

The brief's own examples were `⏱️clock` (registered — single emoji, clean) and `#⃣hash`. Checked
`#⃣hash` and `🗚️fonts` against the taxonomy's own emoji-validity gate
(`🔍️discovery/🟦️component.ts`: `semanticDirectoryKinds[id].emoji` must match
`^\p{Extended_Pictographic}️(?:‍\p{Extended_Pictographic}️)*$`):

- `#⃣` is a keycap sequence (`#` + U+20E3) — `#` is not `Extended_Pictographic`. Fails.
- `🗚️` (U+1F5DA + VS16) is not `Extended_Pictographic` in the Unicode data the validator uses. Fails.

Verified directly with `bun -e` against the real regex — both fail, `numeric`'s 🔢️ passes. Since
`matchDirectoryKind`'s emoji-present branch requires the *candidate kind's own* emoji, and no
directory kind may ever carry an invalid emoji, **no `semanticDirectoryKinds` registration can ever
make `#⃣hash` or `🗚️fonts` resolve as they're spelled on disk today** — that would need an on-disk
rename to a valid pictographic emoji first (or a `semanticDirectoryMemberKinds` literal-name overlay
scoped to their one owner), both out of this slice's "vocabulary only, no file moves" mandate.
Left unregistered; flagged here for whoever eventually renames them.

## 6. One planned registration reverted: `input` collides with an existing overlay

`📥️input` already has a deliberate, different meaning at
`🖱️ui/🖥️host/📦️packages/🦀️rust/📥️input/...` via `semanticDirectoryMemberKinds["members-of-…
-modules"]` (a nested `members-of` chain, not a flat kind). A flat unscoped `input` kind would win
the `exact` match unconditionally (its `contextAllows` is always true) and silently override that
overlay, which an existing test (`UI-host metadata binds canonical source inputs…`) caught
immediately. Checked all 68 other candidate words against every `semanticDirectoryMemberKinds`
list — `input` was the only collision. Dropped from the registration batch.

## 7. Config-leaf family (`vitest.config.ts` ×7, `tailwind.config.ts` ×3, `postcss.config.ts` ×1,
   `eslint.config.ts` ×1) — correctly a `fixedFilenameContracts` case, blocked by an out-of-scope file

Per the brief, tool-mandated filenames belong in `fixedFilenameContracts`, not the semantic
vocabulary — confirmed these are genuinely external-tool-owned names (Vitest, Tailwind, PostCSS,
ESLint each require their own literal config basename or an explicit `--config` flag we don't pass).
Drafted the contracts and matching `packageSourceDispositions` entries, but
`packageSourceDispositions[id].validator` is validated **twice**: once in
`🔍️discovery/🟦️component.ts` (mine — generalized cleanly) and once, independently, in
`🧹️normalization/🟦️.ts::parseTaxonomy` with the *same* hardcoded three-value whitelist
(`package-glue` / `command-router` / `vitest-configuration`, the last pinned to the single id
`vitest-config-entry`) — a file explicitly out of this slice's ownership (assigned to the peer
working the normalization plumbing). Any new validator token fails to parse at all, at
`clean taxonomy plan` runtime, not just in the discovery-side test. Reverted both the
`fixedFilenameContracts`/`packageSourceDispositions` entries and the `component.ts` generalization
rather than ship a change that can never load. **Flagged, not closed** — needs the same three-value
whitelist generalized on the `🧹️normalization/🟦️.ts` side too, by whoever owns that file next.

## 8. Directly registered (68 `semanticDirectoryKinds`)

`activation` 🪪️ · `assembly` 🎟️ · `authority` 🪪️ · `base64` 🔤️ · `binding` 🔗️ · `bindings` 🔗️ ·
`bootstrap` 🏗️ · `budget` ⏱️ · `bytes` 🔢️ · `cancellation` 🚫️ · `causal-add` ➕️ · `clock` ⏱️ ·
`commit` 🔗️ · `compare` ⚖️ · `composition` 🏘️ · `content` 📦️ · `cooperative` ⏱️ · `copied` 📋️ ·
`copy` 📋️ · `credit` 🎟️ · `enqueue` 📥️ · `entries` 📚️ · `evidence` 🧾️ · `fault` 🧯️ · `fixed` 🗃️ ·
`framing` 📄️ · `graph` 🔬️ · `inbound` 📨️ · `inbox` 📥️ · `index` 🗂️ · `json` 🧩️ · `lifetime` 🚪️ ·
`list` 📋️ · `local-interaction` 🏠️ · `message` 💌️ · `mutation-leaf-contract` 🧬️ ·
`mutation-leaf-source-contract` 🧬️ · `nodes` 🗂️ · `numeric` 🔢️ · `operations` 🩹️ · `ordered` 🗂️ ·
`ownership` 📏️ · `pack` 🧩️ · `page` 📄️ · `pages` 📄️ · `payload` 📦️ · `pending` 📨️ · `poll` 📥️ ·
`read-lease` 📖️ · `reader` 📖️ · `release` 🧾️ · `resident` 🎟️ · `response` 📨️ · `retirement` ♻️ ·
`return` 📤️ · `set` 🧺️ · `slot` 📨️ · `source` 🏠️ · `string` 🔤️ · `tail` 🏁️ · `transaction` 🔄️ ·
`tutorial` 🎬️ · `update` 🩹️ · `validation` 🛡️ · `value` 🧾️ · `whole` 📄️ · `wire-retirement` 🧹️ ·
`writer` ✍️.

Plus `asset-subject`'s widened pattern, `test-case`'s extended `parentKindIds` (+68 domain words +
`builder`), and the four `fileKinds` above.

## 9. Not attempted (record only)

- The 11 multi-emoji words (§3) and 2 invalid-emoji words (§5) — need domain-owner or on-disk-rename
  follow-up, not vocabulary registration.
- `input` (§6) — needs subsystem-scoped `parentKindIds` from whoever owns the UI-host package tree,
  not a flat global kind.
- Config-leaf `fixedFilenameContracts` (§7) — needs the `🧹️normalization/🟦️.ts` validator whitelist
  generalized first.
- `reference-syntax-unsupported`, `frozen-coordinate-evidence-unowned`, `collision-*`,
  `package-implementation-destination-unresolved` classes are unrelated mechanism/evidence
  categories, not vocabulary — out of this slice by the brief's own framing.
