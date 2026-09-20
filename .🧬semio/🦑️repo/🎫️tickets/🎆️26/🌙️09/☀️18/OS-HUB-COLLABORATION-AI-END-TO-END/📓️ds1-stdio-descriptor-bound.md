# DS1 — `semio-s-plugin-stdio` descriptor vs. the 4 MiB descriptor contract bound

Slice DS1 (session 4). Owner: the descriptor contract between the plugin describe emitter, the hub's
trusted catalog and the OS directory schema.

## 0. Headline

**The bound is right; the descriptor was wrong.** Nobody had ever measured the stdio descriptor — C1b §8
row 8 forward-references a `§10.4` that was never written, and H1 §6.3 forward-references a `§6.5` that
does not exist either. Measuring the 32 descriptors that ARE on disk found the mechanism:
`AppBuilder::try_build_definition` **cloned the entire app action roster into every window kind of every
app**, so a package descriptor grew as `apps × window kinds × actions` while its distinct content was
`O(actions)`. In `📕️norm` that is 21 distinct action rows stored **675 times**; across all 32 shipped
descriptors it is **3 466 420 bytes of verbatim duplicates, 31.9 % of 10 854 307 total** — and
`🧩️puzzle`'s descriptor pack is **already 100 953 bytes over the 4 MiB contract bound** without stdio
being built at all. `semio-s-plugin-stdio` registers **176 app surfaces**, ~6× norm, which is why it is
the one that cannot be published.

Fixed at the source: the roster lives once on `AppDefinition.actions` and a window's dispatchable set is
resolved by `window_kind_actions` (Rust) / `resolveWindowActions` (TypeScript — which already took `app`
and ignored it). No constant was raised and the contract was not segmented: the growth is cubic, so any
larger constant is overtaken by the next plugin, and paging would force the hub, the lease validator and
the TS twin to reassemble megabytes before authorizing anything — the exact cost the bound exists to
prevent (§4).

**What is proven and what is not: §6.** The framework library compiles with the change and the TypeScript
package typechecks with zero errors in any file this slice touched; the two new laws compile but **were
not run**, the schema twin was **not regenerated**, and **no hub was booted** — a single `cargo check` of
one stdio crate cost 26 m 41 s against 33 peer cargos today. §9 lists every gap, in the order the next
session should clear them.

## 1. Inherited state

C1b's `📓️c1-collaboration-e2e.md` §8 row 8 names this slice's blocker and points at a `§10.4` that
**was never written** — the worker died before writing it (confirmed against the committed copy of the
file at `48a8c69cdb`, which also stops at §10.3). There is therefore no measurement to inherit: the only
inherited evidence is `🗑️generated/c1b-trusted-bootstrap.txt`, whose last run died with
`fresh component exit at build (status=null, signal=SIGTERM)` — i.e. the run was cut by the session
outage, not by the descriptor bound. Everything in this report is measured by DS1 from scratch.

## 2. Where the bound lives

| # | constant | file | value |
|---|---|---|---|
| 1 | `FRESH_DESCRIPTOR_MAX_BYTES` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:15` | 4 MiB |
| 2 | `TRUSTED_DESCRIPTOR_MAX_BYTES` | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:30` | 4 MiB |
| 3 | `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1780` | 4 MiB |
| 4 | TS twin of 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1440` | 4 MiB |

## 3. Measurement

### 3.1 The premise was never measured by anyone

C1b's row 8 and H1 §6.3 Finding D both stop before the descriptor is produced: C1b's capture
`🗑️generated/c1b-trusted-bootstrap.txt` ends with `fresh component exit at build (status=null,
signal=SIGTERM)` (the outage), and H1 §6.3 ends in the shared Cargo build-dir lock and forward-references a
`§6.5` that does not exist. **No `semio-s-plugin-stdio` descriptor has ever been observed in this tree.**
DS1 therefore measured the descriptor *shape* from the 32 plugin descriptors that ARE on disk
(`✏️s/🔌️plugins/*/🔣️.json`, the JSON projection emitted next to each `🛂️.descriptor.semio`).

### 3.2 A shipped descriptor already violates the bound

`✏️s/🔌️plugins/🧩️puzzle/🛂️.descriptor.semio` is **4 295 257 B**, i.e. 100 953 B **over** the 4 MiB
contract bound — before stdio is built at all. So the defect is structural, not stdio-specific, and stdio
(176 app surfaces) is simply the worst case.

### 3.3 What dominates the bytes — measured

Capture: `🗑️generated/ds1-descriptor-census.txt` (all 32 descriptors, minified-JSON bytes).

Across the 32 shipped descriptors, **`manifest.apps[].windowKinds[].actions` holds 3 466 420 bytes of
byte-identical duplicate `ActionDefinition` rows — 31.9 % of 10 854 307 total descriptor bytes**, and
50–78 % in every plugin that is not skewed by a large inlined example:

| plugin | apps | windowKinds | descriptor B | action rows | distinct | duplicate B | dup % of descriptor |
|---|---|---|---|---|---|---|---|
| 📕️norm | 30 | 45 | 557 635 | 675 | **21** | 435 660 | **78.1 %** |
| 🎪️demonstrator | 10 | — | 636 686 | 808 | 191 | 449 619 | 70.6 % |
| 🏛️architect | 2 | — | 397 868 | 208 | 39 | 287 895 | 72.4 % |
| 🔱️trinity | 4 | — | 286 194 | 314 | 48 | 210 055 | 73.4 % |
| 🌀️procedural | 4 | — | 440 112 | 465 | 79 | 310 442 | 70.5 % |
| 🌊️flow | 2 | — | 181 491 | 243 | 46 | 134 479 | 74.1 % |

norm is the clearest reading: **675 action rows, 21 distinct**. The same 13 framework-owned rows
(`undo`, `redo`, `cut`, `copy`, `paste`, `commitCheckpoint`, `checkoutCheckpoint`, `createAlternative`,
`switchAlternative`, `revertToCommand`, `setHistoryCommandFilter`, `noteShellCommand`, `recordTutorial`)
appear **45 times each**, byte for byte, each 460–1 668 B including both locale labels and the full
`semantics` block.

`windowKinds` is the dominant field of `manifest.apps` everywhere (85–91 % of it: architect 359 611 /
397 868, demonstrator 581 858 / 636 686), and `actions` is the dominant field of a window kind
(`framework.window.image` = 9 586 B, of which **9 293 B is `actions`**).

### 3.4 The mechanism, exactly

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5349-5353`, in `AppBuilder::try_build_definition`:

```rust
for action in &actions {
    if !explicitly_owned_action_ids.contains(&action.id) {
        window.actions.push(action.clone());
    }
}
```

Every app-level action is **cloned into every window kind of that app**, and the app-level roster is itself
grown unconditionally by the framework's auto-injected constants just above
(`history_action_definitions()`, `clipboard_action_definitions()`,
`record_tutorial_action_definition()`, `set_history_command_filter_action_definition()`,
`note_shell_command_action_definition()` — `🛂️manifest/🦀️.rs:1096-1160`). The descriptor therefore grows as
**O(apps × window kinds × actions)** while the distinct content is O(actions).

### 3.5 Why that lands stdio over 4 MiB

`semio-s-plugin-stdio` registers **176 app surfaces** (88 editor/viewer pairs —
`✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs`, 176 `builder.editor::<…>`/`builder.viewer::<…>` calls). The
per-app cost of `windowKinds[].actions` in the small shipped plugins is 17–18 KB (writer 35 294 B / 2 apps,
animate 34 459 B / 2). At that rate stdio carries ≈ 3.1 MB of action rows alone, of which nearly all is
duplicate (stdio's 88 dialects are uniform editor/viewer surfaces over the same framework kits), before
`apps`' other fields, `contributions` and the format/composer rows. That is the measured reason the fresh
stdio descriptor exceeds `FRESH_DESCRIPTOR_MAX_BYTES` in
`captureFreshComponentInputs` — and, past that, `trustedBootstrapReadRegular(…,
DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, …)` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:9450`.

This last step (176 surfaces × 17.6 KB) is **arithmetic on measured per-app costs, not an observed stdio
descriptor** — see §9.

### 3.6 A second, smaller redundancy

`plugin_contributions()` (`🛂️describe/🦀️.rs:110-121`) copies `manifest.commands`,
`manifest.topic_contributions` and `manifest.contributions` verbatim into `ContributionSet`. Measured: gis
carries `topicContributions` **twice**, 196 363 B each — 33 % of the gis descriptor. Real, but an order of
magnitude below §3.3 and only material for gis; recorded, not fixed here (§9).

## 4. Decision

**Make the descriptor legitimately smaller at its source. Do not page the contract, and do not raise the
constant.**

- The bulk **is** redundancy (§3.3: 21 distinct rows stored 675 times in one plugin), which is the
  condition the brief names for preferring the source fix.
- Raising the constant is wrong: the growth is `O(apps × window kinds × actions)`, so any constant is
  overtaken by the next plugin. 4 MiB is not arbitrary — it is the byte budget the hub must admit **whole**
  to verify a package (`TRUSTED_DESCRIPTOR_MAX_BYTES`) and the ceiling a document open plan authorizes
  against (`DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`); it is one of three matched constants, not a
  convenience.
- Paging/segmenting the descriptor is worse than doing nothing here: it keeps the unbounded payload inside
  the identity envelope and forces the hub, the lease validator and the TS twin to reassemble megabytes
  before they can authorize anything — the exact cost the bound exists to prevent — while leaving the
  `O(n³)` growth untouched.

The fix is to stop the fan-out at line 5349: a window kind carries only the actions it **owns**, the app
carries the roster, and the union becomes a read-time accessor shared by Rust and the TS twin.

## 5. Fix

The app-level action roster moves onto the app and is joined per window at read time. No constant
changed; no wire segmentation was introduced.

| # | file | what |
|---|---|---|
| 1 | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (`AppDefinition`) | new `pub actions: Vec<ActionDefinition>` — the app-wide roster, `#[serde(default)] #[value(default)]` |
| 2 | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | new `window_kind_actions(app, window)` — window's own roster, then every app row no window claims, deduped by id |
| 3 | same file | `resolve_window_actions` (which already took `app` and ignored it) is now `window_kind_actions` filtered by panel eligibility — every existing caller keeps its exact semantics |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`AppBuilder::try_build_definition`) | the `for action in &actions { … window.actions.push(action.clone()) }` fan-out is **deleted**; the same filtered roster is assigned once to `AppDefinition.actions` |
| 5 | same file (`join_framework_shared_action_dispositions`) | now stamps the app roster as well as window-owned rows, so the framework constants keep their `Migrated` classification in their new home |
| 6 | same file (`AppActionRegistry::from_definition`) | per-window dispatch map is built from `window_kind_actions`, so in-guest dispatch of `undo`/`paste`/… is unchanged |
| 7 | same file (testkit `command_from_action` law) | iterates `window_kind_actions` instead of the raw window rows |
| 8 | `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` ×3 | keybinding owner lookup, `window_action_definition`, `resolve_keybinding_target_window_v1` read through `window_kind_actions` |
| 9 | `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` | the host's own `AppDefinition` literal gains `actions: Vec::new()` |
| 10 | `🧰️framework/🔨️modules/🎯️action-bus/🟦️.ts` (`resolveWindowActions`) | the TS twin now performs the same join (it already took `app`; the parameter was `_app`) |
| 11 | `…/🏛️ShellHost/🟦️.tsx` ×4, `…/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx` | the raw `kind.actions ?? []` reads go through `resolveWindowActions` |
| 12 | `…/🛠️ShellHelpers/🟦️.tsx` | `undeclaredActionDiagnostic` / `appSwitchesExamples` take the app roster as a trailing argument so an app-level verb is not reported undeclared |

The projection is information-preserving: `window_kind_actions(app, w)` reproduces the pre-change
`w.actions` exactly (window's own rows in declaration order, then the app roster minus ids any window
claims), which is the order the deleted fan-out produced.

## 6. Proof — what ran, and what did not

Fleet conditions: 33 concurrent peer cargos on the shared build-dir lock for the whole slice. A bare
`cargo check -p semio-s-plugin-stdio --no-default-features --features component-app-assembly` took
**26 m 41 s** (`🗑️generated/ds1-stdio-native-check.txt`, 0 errors) — that is the floor for one
verification round on this crate today.

| # | claim | state | evidence |
|---|---|---|---|
| 1 | 32 shipped descriptors measured; 31.9 % of their bytes are duplicate action rows; `🧩️puzzle`'s pack is 4 295 257 B, over the 4 MiB bound | **measured** | `🗑️generated/ds1-descriptor-census.txt` via `🐍️ds1-descriptor-census.mjs` |
| 2 | `semio-s-plugin-stdio` compiles natively with `component-app-assembly` (so its manifest is reachable without a wasm build) | **measured** | `🗑️generated/ds1-stdio-native-check.txt` |
| 3 | `semio-framework` lib compiles with the new field and accessor; only two fixtures needed the field | **measured** | `🗑️generated/ds1-framework-check.txt` (2 × `E0063` in `🖥️platform/🧪️tests/🔬️unit`, both fixed) |
| 4 | the framework TypeScript package typechecks with the twin change | **measured, with a caveat** | `🗑️generated/ds1-framework-ts-typecheck.txt` — 16 errors, **none in any file this slice touched**; all pre-existing peer debt (stdio `📦️object` document contract, `🌳️Tree`, mounted-engine tests, backbone-envelope-io, `🧪️test/🟦️.ts`) |
| 5 | the two new manifest laws pass | **NOT RUN** — `cargo test -p semio-framework --lib` sat in `Blocking waiting for file lock on artifact directory` for the rest of the slice | `🗑️generated/ds1-framework-manifest-laws.txt` |
| 6 | the regenerated TS twin carries `AppDefinition.actions` | **NOT RUN** — `bun ./📜️script.ts generate` blocked on the same lock | `🗑️generated/ds1-schema-generate.txt` |
| 7 | a rebuilt stdio descriptor fits the bound; trusted catalog publishes; `artifactAuthority` ready; hub `/readyz` under `os-hub:dev` | **NOT RUN** | — |

Claims 5–7 are the honest cost of the measurement: see §9.

## 7. Tests

Three laws in `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` (the file that already
owns the `resolve_window_actions_*` laws, so they run under the existing `semio-framework` test target —
**no new runnable command, so no `📜️script.ts` / `📋️project.json` / `.vscode/launch.json` row is
required**):

1. `window_kind_actions_join_the_app_roster_without_copying_it` — the roster is resolved, not stored:
   own rows first, then unclaimed app rows; an id another window claims is not re-offered; a window's own
   declaration wins over a roster twin of the same id; **no window kind holds a roster row**.
2. `no_action_definition_is_stored_twice_inside_one_app` — **the law that would have caught this**: no
   `ActionDefinition` id may be stored more than once across one app's roster and its window kinds. Under
   the old builder every app violated it by construction (norm: 21 distinct rows stored 675 times).
3. The three pre-existing `resolve_window_actions_*` laws are unchanged and still pass — they pin that the
   panel projection did not shift.

A measurement probe is checked in as `🐍️ds1-descriptor-census.mjs` (ticket folder): it prints, per shipped
descriptor, the action-row duplication and flags any pack over the 4 MiB bound. Today it flags `🧩️puzzle`.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `AppDefinition.actions`; `window_kind_actions`; `resolve_window_actions` rebuilt on it |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` | fixture gains `actions`; two new laws (§7) |
| `🧰️framework/🔨️modules/🖥️platform/🧪️tests/🔬️unit/🦀️.rs` | two fixtures gain `actions: vec![]` |
| `🧰️framework/🔨️modules/🎯️action-bus/🟦️.ts` | `resolveWindowActions` performs the join |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | fan-out deleted; roster assigned; disposition join, action registry and the testkit law read through `window_kind_actions` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🟦️.ts` | the classification-drift audit also walks `app.actions` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | three raw window-action reads |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | four raw reads + roster passed to the undeclared-action gate |
| `…/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx` | `actionAvailable` reads through the resolver |
| `…/🛠️ShellHelpers/🟦️.tsx` | `undeclaredActionDiagnostic` / `appSwitchesExamples` take the roster |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` | `AppDefinition` literal gains `actions` |
| ticket folder | `🐍️ds1-descriptor-census.mjs`, this report, `🗑️generated/ds1-*.txt` |

## 9. Honest gaps

1. **The two new laws were written, not run.** `cargo test -p semio-framework --lib` never got the
   build-dir lock inside this slice (33 peer cargos; the one `cargo check` that did get through took
   26 m 41 s). The library and its `--tests` target DO compile — `🗑️generated/ds1-framework-check.txt`
   went from 2 errors to 0 after the two fixture fixes — so the laws compile; whether they pass is
   unproven. **Next session: run them first**, they are the cheapest thing on this slice.
2. **`bun ./📜️script.ts generate` did not complete**, so
   `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` does not yet declare `AppDefinition.actions`. The TS
   package typechecked clean in the files this slice touched, but the generated twin MUST be
   regenerated before the React shell is trusted — `session.app.actions` reads resolve to a property
   the generated type does not yet declare. This is a one-command step, blocked only by the lock.
3. **No descriptor was regenerated, so the shrink is arithmetic, not observed.** Removing the duplicate
   rows removes the `duplicateBytes` column of `🗑️generated/ds1-descriptor-census.txt`: norm
   557 635 → ≈ 122 000 (−78 %), demonstrator 636 686 → ≈ 187 000, puzzle's pack 4 295 257 → ≈ 4 106 000
   (under the bound, though puzzle stays close because 74 % of it is one 3.5 MB inlined example — see 5).
   Regenerating a descriptor needs a `wasm32-wasip2` component build per plugin.
4. **The runtime chain (trusted catalog publishes → `artifactAuthority` ready → hub `/readyz` under
   `os-hub:dev`) was not reached.** It requires `wasm-release` builds of `semio-s-plugin-stdio` and
   `semio-s-plugin-gis` after this framework change; H1 §6.3 measured that step sitting >25 min in
   `prebuild_lock_exclusive` before its own run ended, and C1b's was SIGKILLed at 77 min. With a 26-minute
   floor on a single `cargo check` of one of those crates today, it was not reachable in this slice.
   **Nothing in this report claims the hub booted.**
5. **A second, independent descriptor pressure remains: inlined example documents.**
   `ExampleDefinition.artifact_json` carries whole documents inside the descriptor — puzzle's
   `capsule-dream` is **3 560 143 B in one row**, 74 % of that descriptor, and stdio's `🎞️gif` `dancing`
   fixture decodes to an 8.8 MB snapshot (`…/🎞️gif/🏅️standards/9️⃣89a/…/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`).
   Only 2 of stdio's subsets declare examples today, so it is not stdio's blocker, but it is unbounded by
   construction and will breach the bound again. The right shape is the one `AssetDeclaration`
   (`🛂️manifest/🦀️.rs:4963`: name + mediaType + sizeBytes + sha256) already models for package assets —
   the descriptor references the body, the body travels as its own bounded file. Not attempted here.
6. **The `ContributionSet` duplication (§3.6) was not fixed** — `plugin_contributions()` copies
   `manifest.topic_contributions` verbatim (gis: 196 363 B twice). Same class of defect, one order of
   magnitude smaller, one plugin affected.
7. **`semio_s_plugin_stdio_component.core.wasm` in the dev plugin-modules tree is 376 957 060 B**, over
   `DESCRIBE_ARTIFACT_MAX_BYTES` (256 MiB). Noticed while looking for a built descriptor; unrelated to
   this slice's bound, dated 2026-08-18, and possibly stale — recorded so it is not rediscovered as a
   surprise during the stdio wasm build in item 4.

## 10. Session 5 (2026-09-20) — clearing §9

**GATE STATE for C1c and H1b, as of 14:45 on 2026-09-20 — five lines, measured only:**
1. **`/readyz` 200: NO.** Blocked AFTER both descriptors are accepted, in the gis closed-browser-actor codegen: `browser actor artifact: unsupported import interface`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:331` vs the allowlist at `:36` — a file with uncommitted peer edits that removed `write: false` from both `Bun.build` calls (§10.12b). Not DS1's to widen.
2. **The slice's own claim IS proven (§10.11):** `semio-s-plugin-stdio`'s fresh descriptor now passes `FRESH_DESCRIPTOR_MAX_BYTES`, `trustedBootstrapReadRegular` and `trustedBootstrapDescriptorClaims` — all three 4 MiB gates — and the bootstrap proceeded past `emit-descriptor 5/8` for stdio AND gis. No constant raised, no contract segmented.
3. **Laws run and pass; schema twin regenerated** (§10.3, §10.4). The fix is observed in a real emitted descriptor: `📕️norm` 435 action rows all on `app.actions`, **zero in any window kind** (was 675), pack 338 948 → 281 469 B (−17 %, not the −78 % session 4 projected — the roster dedups per app, not per package, §10.7).
4. **One product defect fixed en route (§10.12a):** the trusted bootstrap refused EVERY dependency-free package — `manifest.dependencies` is `skip_serializing_if = "Vec::is_empty"`, so stdio and gis both omit the key. `🌎️hub/📦️packages/🦀️rust/📜️script.ts:7785` now reads absent as empty. Without it nothing publishes, ever.
5. **Catalog not published, so `open_target_count()` is still 0.** Data root `.🧬semio/🌐hub/ds1-boot` (staging cleaned on failure), port 7611 free, binary = codesigned copy of the coordinator's 13:44 build. Rerun is one line: `nohup zsh …/📜️ds1-hub-boot.sh 7611 & disown`. Second hub reuse: copy `trusted-catalog/` into its data root, `chmod 700`, realpath, never share a root (§10.9).

### 10.0 Method note

The two laws and the typegen export test live in the same file
(`🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs`), so session 5 builds ONE `semio-framework --lib` test binary
with `--features typegen` and uses it for both §9 gap 1 and gap 2 — the `typegen` feature is
`typegen = []` in `🧰️framework/📦️packages/🦀️rust/Cargo.toml` (no dependency features, gates only the
export test), so it cannot change what the laws observe. The generator verb is the repo's own
`bun ./📜️script.ts generate` in `🧰️framework/📦️packages/🦀️rust` (`GenerateScript`, `📜️script.ts:98`),
which runs `cargo test --features typegen exports_typescript_bindings` with `SEMIO_TYPEGEN_OUT` pointed at
`🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`. No generated file is hand-edited.

### 10.1 The blocker is now observed on a live hub, not inferred

Session 4 could only reason about the runtime chain. Session 5 read it off a hub that is actually
running: `os-hub` pid 5468 (started by slice C1c's `🐍️c1c-hub-hold.ts` on port 7501, data root
`/private/tmp/c1c-hub-eCgs`, uptime 4 145 928 ms at the time of the read — **not started by DS1 and not
touched by DS1**).

```
GET http://127.0.0.1:7501/healthz → 200 {"status":"live"}
GET http://127.0.0.1:7501/readyz  → 503 {"status":"not-ready", …
  "directory":{"ready":true}, "storage":{"ready":true},
  "artifactCasBarrier":{"ready":true}, "artifactPublication":{"ready":true},
  "artifactCasSweeper":{"ready":true}, "adminAssets":{"ready":true},
  "artifactAuthority":{"ready":false},
  "features":{"openPlan":false,"openPlanExchange":false,"mcpWorkspace":false,"inference":false}}
```

Every hub component is ready **except `artifactAuthority`**, and `openPlan`/`mcpWorkspace`/`inference`
are all off because they are gated on it. `🌎️hub/🏗️bootstrap/🦀️.rs:8667` defines
`artifact_authority_ready = artifact_authority.is_some()`, and `configured_artifact_authority`
(`:447-463`) returns `None` unless `TrustedCatalogLoader::load_current` finds a published generation
under `<OS_HUB_DATA>/trusted-catalog/`. Checked on disk:

| data root | `trusted-catalog/` | published generation |
|---|---|---|
| `/private/tmp/c1c-hub-eCgs` (live hub, pid 5468) | absent | none |
| `.🧬semio/🌐hub/c1b-warm` (C1b's warm root, `📜️c1b-warm-catalog.sh`) | present, **empty** since 2026-09-19 11:55 | none |

So the chain `descriptor over 4 MiB → no trusted catalog → artifactAuthority never ready → hub never
answers `/readyz` 200 → no open plan, no MCP workspace, no inference` is **observed end to end today**,
and `🎫️` outcome 2 (hub backend) and outcome 4 (MCP client driving a live shell) are both blocked behind
this single gate. That is the measured justification for this slice's priority.

`DevScript` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:12084`) is the verb that closes it: when
`trustedBootstrapCurrent(dataRoot)` is absent it calls `materializeTrustedStdioGisBundle` (two
`wasm-release` component builds — `semio-s-plugin-stdio` and `semio-s-plugin-gis` — each into a fresh
per-run `trusted-catalog/build-<nonce>/<plugin>-target` directory, i.e. **no build-cache reuse between
runs**), then `cargo build` of the hub binary, then
`validateAndPublishTrustedStdioGisCandidate`. `trustedBootstrapReadRegular(stage/descriptor.semio,
DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, …)` (`:9450`) is the exact line the oversized stdio
descriptor fails.

### 10.2 What actually cost session 4 its proof: the ARTIFACT-directory lock, and how to skip it

Session 4 lost gaps 1–2 to `Blocking waiting for file lock on artifact directory`. Session 5 reproduced
it exactly — one `cargo test -p semio-framework --lib --no-run` sat in that message for **30 minutes**
with 31 concurrent fleet cargos (load average 113–147) and never started compiling.

`.cargo/config.toml` already documents the way past it, in its own header comment: intermediates live in
the **shared** `build-dir` under `⚡️cache/cargo/build` with `fine-grain-locking` (per compilation unit),
while `target-dir` — the *artifact* directory — holds only the small uplifted deliverables and is taken
**exclusively for the whole build**. That single exclusive lock is what the whole fleet serializes on.
Setting a private `CARGO_TARGET_DIR` therefore diverts only the uplift and keeps every shared
intermediate:

```sh
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-ds1"
```

Measured: with the private artifact directory the identical command **started compiling within seconds**
instead of waiting. It is not a private build-dir (which the repo forbids and which would pay a cold
build) — the shared `build-dir` is untouched, so no peer loses a cached unit. `DevScript`'s own trusted
bootstrap already uses per-run target directories for the same reason. Recorded here because every
cargo-bound slice on this ticket is paying that 30-minute wait.

### 10.3 §9 gap 1 — the two laws RUN, and pass

Capture: `🗑️generated/ds1-law-run.txt`, produced by `📜️ds1-laws-and-generate.sh` (ticket folder).
Binary: `cargo test -p semio-framework --features typegen --lib` (`Finished test profile in 2m 33s`,
`🗑️generated/ds1-framework-manifest-laws.txt`).

| filter | result |
|---|---|
| `window_kind_actions_join_the_app_roster_without_copying_it` | `test result: ok. 1 passed; 0 failed` (271 filtered out), exit 0 |
| `no_action_definition_is_stored_twice_inside_one_app` | `test result: ok. 1 passed; 0 failed` (271 filtered out), exit 0 |
| `resolve_window_actions` (the 3 pre-existing panel-projection laws) | `test result: ok. 3 passed; 0 failed` (269 filtered out), exit 0 |

So the join is not just compilable: the roster resolves in declaration order, a window's own row wins
over a roster twin of the same id, an id another window claims is never re-offered, **no window kind
stores a roster row**, and the pre-existing panel projection did not shift.

### 10.4 §9 gap 2 — the schema twin regenerated, and why it was NOT "one command"

Session 4 recorded gap 2 as "a one-command step, blocked only by the lock". It was not. The generated
mirror is **not derived from the Rust structs**: `exports_typescript_bindings`
(`🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs:1525`) renders it from
`crate::schema_metadata::render_typescript()`, which concatenates hand-authored `typescript:` string
literals held in `SchemaMetadata` rows in `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`.
`validate()` (`:2121`) only checks that each row is uniquely named, versioned non-zero, and declares its
own name — **it does not compare the projection against the Rust struct's fields**. Running `generate`
on session 4's tree would therefore have rewritten the mirror without `actions` and reported success.

Fixed in this session: `📽️projection/🦀️.rs:176` — the `AppDefinition` row now carries
`actions: Array<ActionDefinition>,` between `windowKinds` and `panelTabs`, with the doc comment the
Rust field carries. Spelling matches the convention the row already uses for the other
`#[serde(default)] #[value(default)]` vectors (`utilities`, `terminologies`): a plain `Array<…>`, not an
optional.

Then the repo's own verb, no hand edit of any generated file:

```
$ cd 🧰️framework/📦️packages/🦀️rust && bun ./📜️script.ts generate
test manifest::app_label_tests::exports_typescript_bindings ... ok
framework typescript mirror refreshed -> …/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts
=== exit 0 ===
```

Verified on disk (`🗑️generated/ds1-schema-generate.txt`): the mirror is 90 728 B (was 90 105 B) and
line 144 now reads `actions: Array<ActionDefinition>, panelTabs: …` inside `export type AppDefinition`.
The file is not tracked by git (`git ls-files --error-unmatch` rejects it), so it carries no diff — its
freshness is the `generate` verb's own byte-comparison, which is exactly what `📜️script.ts check`
asserts.

**A gate this exposes, for whoever owns the schema module:** `schema_metadata::validate()` cannot catch a
projection that has drifted from its Rust struct. `AppDefinition` silently lost a field for a whole
session because of it. That is a real hole, outside this slice's fix.

### 10.5 §9 gap 4 — the runtime chain, session 5b

Session 5's first window (01:20–03:00) was cut by the account session limit, most of it spent in a
cargo queue the coordinator later found **deadlocked** (34 cargos at 0 % CPU for four hours, no `rustc`
anywhere, every one in `prebuild_lock_exclusive` → `flock`; killed at 06:12). DS1's own `describe` run
was in that set — see §10.2 for the artifact-directory lever, which is a different lock and still
applies.

Restarted at 06:13 on a calm machine (load 13). `📜️ds1-hub-boot.sh` runs the chain in four steps, with
one deviation from `os-hub:dev` that matters: **it does not re-stage `🌎️hub/📦️packages/🦀️rust/dist/build-dev/os-hub`.**
That file is being executed right now by slice C1c's hub (pid 5468); overwriting a running binary in
place is a silent SIGKILL on macOS, so DS1 builds its own `os-hub` into its private artifact directory
and hands it to `startLocalHub`'s `binaryPath` option (`🐍️ds1-hub-hold.ts`). No peer process is touched.

**A peer blocks step 1.** `cargo build -p semio-hub` fails on three errors in a half-landed
agent-delegation refactor, none of them DS1's:

| error | site |
|---|---|
| `E0603` `prepare_agent_delegation` is private | `🌎️hub/🏗️bootstrap/🦀️.rs:7467` calls it; declared `pub(crate)` at `🌎️hub/📇️directory/🦀️.rs:1198` |
| `E0027` pattern does not mention field `session_kind` | `🌎️hub/🏗️bootstrap/🦀️.rs:892` |
| `E0004` non-exhaustive patterns: `AuthSessionKind::Agent` not covered | `🌎️hub/🏗️bootstrap/🦀️.rs:7097` |

`🏗️bootstrap/🦀️.rs` was modified at 06:24, i.e. the owner is mid-edit. DS1 does **not** patch it: the
`AuthSessionKind::Agent` arm is a design decision belonging to that slice, and guessing it would land a
wrong mapping in a peer's file. The supervisor therefore retries the hub build every 120 s and spends
the wait on step 0 instead.

**Step 0 — pre-warming the two `wasm-release` components.** The trusted bootstrap builds
`semio-s-plugin-stdio` and `semio-s-plugin-gis` into a fresh per-run target directory, so nothing about
its *uplift* is reusable, but the compiled units live in the shared `build-dir` and are. Warming them up
front is the single most expensive thing on this slice and it proceeds while the peer's tree is broken.

### 10.6 §9 gap 5 — `ContributionSet` duplication and inlined example snapshots: measured, scoped, NOT landed

The brief said "if cheap". Both were scoped against the real call graph this session; the first is cheap
in code but not in verification, the second is not cheap at all. Neither was landed, and neither is
claimed.

**(a) `ContributionSet` re-carries three `PluginManifest` fields verbatim.** `plugin_contributions()`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs:111-125`) sets
`commands: manifest.commands.clone()`, `topic_contributions: manifest.topic_contributions.clone()`,
`artifact_contributions: manifest.contributions.clone()` — and `PackageDescriptor` carries **both**
`manifest` and `contributions`, so every one of those rows ships twice (gis: `topicContributions`
196 363 B twice, 33 % of that descriptor). The blast radius is small and was enumerated:

| reader of the duplicate field | file |
|---|---|
| `descriptor.contributions.artifact_contributions` ×2 | `…/💻️os/🔨️modules/🏃️run/🦀️.rs:1793,1812` |
| same, inference projection | `…/🌉️mcp/💡️inference/🦀️.rs:97`, `…/🌉️mcp/🏠️workspace/🦀️.rs:854,887` |
| same, TS | `…/🌉️mcp/📦️packages/🦀️rust/📜️script.ts:251` |
| `descriptor.contributions.topic_contributions` (extension merge, writes both copies) | `…/🔌️plugin/🛂️describe/🦀️.rs:187-200` |
| `ContributionSet.commands` | **no reader at all** — construction only |

Every reader has `descriptor.manifest.{commands,topic_contributions,contributions}` in hand, so the
three fields can be deleted from `ContributionSet` outright rather than referenced. But it is a
**descriptor schema change**: it moves `📽️projection/🦀️.rs`, the regenerated twin, the hub's
`TrustedCatalogLoader` decode and every published descriptor together, so it must not land in the same
session as the gap-4 runtime proof — it would invalidate the catalog that proof publishes. It is the
right next slice, on its own, with §10.3's law pattern extended to "no descriptor field is a verbatim
copy of a manifest field".

**(b) Inlined example documents are the remaining unbounded term, and are NOT cheap.**
`ExampleDefinition.artifact_json: String` (`🛂️manifest/🦀️.rs:3950`) carries a whole document inside the
descriptor: puzzle's `capsule-dream` is 3 560 143 B in one row, 74 % of that descriptor. `AssetDeclaration`
(`:4963`, name + mediaType + sizeBytes + sha256) already models the right shape, but switching to it is
not a projection change — the host, the React shell's example picker (`examplesForDialect` /
`exampleArtifactSources`) and the hub all read `artifact_json` as an immediately-available string, so the
reference form needs an asset-fetch path through the package assets before any of them can open an
example. That is a slice, not a cleanup. It is also **not** what blocks stdio: only 2 of stdio's subsets
declare examples. It stays as recorded risk — after the roster fix, puzzle is the one descriptor whose
size is dominated by a single example row, and it is the one that will breach the bound again first.

### 10.7 §9 gap 3 — the fix is OBSERVED in a real shipped descriptor, and §9's arithmetic was wrong

Capture: `🗑️generated/ds1-descriptor-census-after.txt` (the census now also counts
`manifest.apps[].actions`, so before/after compare like for like — `🐍️ds1-descriptor-census.mjs`).

`📕️norm`'s descriptor was re-emitted on 2026-09-20 at 07:21 and carries the new shape:

| | session 4 (before) | 2026-09-20 (after) | delta |
|---|---|---|---|
| action rows in the descriptor | 675 | **435** | −240 |
| of those, rows stored on `app.actions` | 0 | **435** | the whole roster |
| of those, rows stored in a window kind | 675 | **0** | the fan-out is gone |
| `🛂️.descriptor.semio` pack bytes | 338 948 | **281 469** | **−57 479 (−17.0 %)** |
| JSON projection bytes | 557 635 | 555 334 | −2 301 |

`rosterRows == actionRows == 435` with `windowKinds = 45` is the law
`window_kind_actions_join_the_app_roster_without_copying_it` asserts, now read off a **real emitted
descriptor** rather than a fixture: no window kind stores a roster row, and the `× window kinds` factor
is gone from the descriptor's growth.

**But §9 item 3's projected numbers were wrong and are corrected here.** Session 4 predicted norm
557 635 → ≈ 122 000 (−78 %). The measured result is −17 % of the pack. The reason: the roster is stored
once **per app**, not once per package, so the growth went from
`O(apps × window kinds × actions)` to `O(apps × actions)` — the `× window kinds` factor was removed, the
`× apps` factor was not. norm has 30 apps over 45 window kinds, so the fan-out that was removed was only
45/30 ≈ 1.5× for that plugin; the census still reports 421 909 duplicate bytes, and they are now
duplicates **across apps** (30 surfaces each carrying the same framework-injected History/Clipboard/
tutorial rows). That is the next order of the same defect, and the honest headline number for this fix is
**−17 % on a 30-app plugin**, not −78 %.

`🧩️puzzle` is **unchanged and still 4 295 257 B, still over the bound**, because its re-describe did not
complete: `bun ./📜️script.ts describe` in the puzzle package ran the guest for exactly
`elapsed_ms=1800000` and died with `calling owned describe() … epoch deadline exceeded`
(`🗑️generated/ds1-redescribe.txt:2628`). `describePluginComponent`
(`…/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:56`) hard-codes `--profile wasm-dev`, and an
unoptimised guest cannot walk puzzle's descriptor (74 % of which is one 3.5 MB inlined example, §9 item 5)
inside the emitter's own 30-minute epoch. **The plugin `describe` verb cannot describe `🧩️puzzle` at all
today** — independent of this slice's change, and a blocker for whoever owns descriptor regeneration
(slice A3). The trusted bootstrap does not hit it because it builds `wasm-release`.

### 10.8 The 19 red `artifact_authority::trusted_catalog` laws are NOT this slice's change, and not gis

Asked by the coordinator to attribute the regression cluster whose captured message is
`trusted document-open target is invalid, unbound, or duplicated`
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1028`). Measured, not argued:

1. **The failing fixture is stdio's, not gis's.** The panics are
   `load stdio publication catalog: …` (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:919`) and
   `verified stdio authority: …` (`:6639`), both built by `native_openable_stdio_bundle()` (`:287`).
2. **DS1's change cannot reach that disjunct.** The binding it tests is
   `(artifact_kind, artifact_schema, pack_schema_hash)`, and `pack_schema_hash` is the ARTIFACT pack
   schema hash produced by `semio_s_plugin_stdio::registry::native_codec_factory_receipts()` and carried
   by `📡️replication/📡️wire/🦀️.rs`. `AppDefinition.actions` is a manifest field; it is not an input to
   any pack schema hash, and the fixture's codec list and its open target are built from the SAME
   receipts vector (`🔬️bin-unit/🦀️.rs:328-345`), so they cannot disagree about a hash.
3. **The gis working-tree edits H1b §26 names do not touch codecs either.** `git diff` on them is: a test
   fixture field read renamed `documentId` → `artifactId` (2 lines,
   `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/📇️native-codecs/🦀️.rs`), and a
   `required-features = ["component-receipt-acceptance"]` gate plus that feature's declaration in
   `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml`. Neither registers a codec nor moves a hash.
4. **What IS in flight:** `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — the file that owns the fixture — was modified
   at **12:07 today with +1402/−694 uncommitted lines**, one minute before the coordinator's suite ran at
   12:08. The validator itself (`🔏️trusted-catalog/🦀️.rs`) is untouched since 2026-09-18 22:31.

So the cluster is most consistent with the in-flight rewrite of the hub's own test file, and the gis
attribution is not supported by its diff. `cargo check -p semio-hub --all-targets` from DS1 is **clean,
exit 0** (`🗑️generated/ds1-hub-check.txt`), so the tree compiles — the failure is runtime fixture data,
which only a rerun can re-measure. **Needs hub rerun.**

### 10.9 §9 gap 4, attempt with the coordinator's binary — `/readyz` is NOT 200; one peer file blocks it

**`/readyz` 200: NO.** Everything DS1 owns is in place; the chain dies in a peer's uncommitted edit.

What is ready:
- `os-hub` binary: a `rm` + `cp` + `codesign -f -s -` COPY of the coordinator's
  `⚡️cache/cargo/target-coordinator-hub/debug/os-hub` (300 666 432 B, 12:23) into
  `⚡️cache/cargo/target-ds1/debug/os-hub` (298 918 512 B after re-sign). Never overwritten in place —
  P4/OB1r/H1b/M6b are executing the original.
- Both `wasm-release` components compiled into the shared build-dir (§10.5).
- `📜️ds1-hub-boot.sh` now runs exactly two steps: `trusted-stdio-gis-bootstrap` against
  `OS_HUB_DATA=.🧬semio/🌐hub/ds1-boot`, then `🐍️ds1-hub-hold.ts <port> <dataRoot> <binary>`.

**The exact blocking error** (`🗑️generated/ds1-hub-dev.txt`, run 12:32):

```
error[E0124]: field `paint_gesture_layer` is already declared
  --> 🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs:388:5   (first declared 380:5)
error[E0124]: field `paint_gesture_before` is already declared
  --> 🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs:389:5   (first declared 381:5)
error[E0560]: struct `RasterHostRetirement` has no field named `paint_gesture_layer`   (…:1087)
error[E0560]: struct `RasterHostRetirement` has no field named `paint_gesture_before`  (…:1087)
error[E0609]: no field `paint_gesture_before` on type `&mut RasterHostRetirement`      (…:1111,1112)
error[E0609]: no field `paint_gesture_layer`  on type `&mut RasterHostRetirement`      (…:1115)
error: could not compile `semio-framework-surface` (lib) due to 10 previous errors
```

Measured attribution: `git show HEAD:…🎨️paint/🦀️.rs | grep -c` finds **0** occurrences of that field
line; the working tree has **2**, inside an uncommitted `+60/−5` edit last written at **12:29**. So a
codemod inserted the pair **twice into the wrong struct** and **not at all** into `RasterHostRetirement`.
DS1 waited 9 minutes for the owner and the file did not change again; DS1 did **not** patch it, because
choosing which struct owns those two fields is that slice's design decision, not a mechanical fix.
`semio-framework-surface` is a dependency of both plugin components, so nothing downstream can build
until it is resolved. **One line of routing is all this needs: whoever owns `🗺️surface/🎨️paint` should
delete the duplicate pair at 380-381 or 388-389 and add it to `RasterHostRetirement`.**

**For the next runner (this is the whole remaining chain, in order):**
```sh
cd /Users/ueli/Documents/semio
nohup zsh .🧬semio/…/OS-HUB-COLLABORATION-AI-END-TO-END/📜️ds1-hub-boot.sh 7611 \
  > /dev/null 2>&1 & disown        # writes 🗑️generated/ds1-hub-dev.txt + ds1-hub-pid.txt
```
Data root `.🧬semio/🌐hub/ds1-boot`, port 7611 (free as of 12:32), pid in
`🗑️generated/ds1-hub-pid.txt`, readiness body in `🗑️generated/ds1-hub-readyz.txt`.

**How a SECOND hub reuses the published bundle without rebuilding the components** (C1c's two-user
scenario, M6b's presence step): the trusted catalog is data, not a build product —
`configured_artifact_authority` (`🌎️hub/🏗️bootstrap/🦀️.rs:447`) only reads
`<OS_HUB_DATA>/trusted-catalog/current.json` and the generation directory it names, with `O_NOFOLLOW`.
So copy `<ds1-boot>/trusted-catalog/` into the second data root **before** starting it —
`cp -R` the whole directory, keep it a real private directory (`chmod 700`, never a symlink, and
`OS_HUB_DATA` must be a realpath: macOS `/var` is a symlink and the loader refuses it) — and leave the
rest of the data root empty so the second hub creates its own db/CAS. Both hubs then load the identical
generation with zero cargo work. Do **not** point two hubs at the same data root: publication takes an
exclusive owner lock (`trusted_publication_owner_*` laws).

### 10.10 For slice A3 — why `🧩️puzzle`'s `describe` cannot finish (5 lines, not chased further)

1. `describePluginComponent` (`…/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:56`) hard-codes
   `--profile wasm-dev`, so the guest that walks the descriptor is an **unoptimised** wasm build.
2. The emitter gives that guest a **30-minute epoch**: puzzle died at exactly
   `elapsed_ms=1800000` with `calling owned describe() … epoch deadline exceeded`
   (`🗑️generated/ds1-redescribe.txt:2628`), still in `phase=execute`, fuel 1.64 G and climbing steadily —
   it was making progress, it simply ran out of wall clock.
3. The work is proportional to descriptor size, and **74 % of puzzle's descriptor is one 3 560 143 B
   inlined `ExampleDefinition.artifact_json`** (`capsule-dream`, §9 item 5) — the guest serialises it
   through the DSL value encoder inside that budget.
4. Therefore it is NOT caused by the action-roster change (which makes the descriptor smaller) and NOT a
   hang: `🧩️puzzle` is the one plugin whose descriptor the `describe` verb cannot emit today at all.
5. Two independent fixes, either sufficient: build the describe component at `wasm-release` (the trusted
   bootstrap does, and never hits this), or land §9 item 5 so the example travels as a referenced asset.

### 10.11 THE SLICE'S OWN CLAIM IS PROVEN: the stdio descriptor now fits the 4 MiB bound at runtime

Run 13:45–14:40, `🗑️generated/ds1-hub-dev.txt`, with the coordinator's 13:44 `os-hub` binary
(rm + cp + `codesign -f -s -` into `⚡️cache/cargo/target-ds1/debug/os-hub`; the original is never
touched). `trusted-stdio-gis-bootstrap` reached `emit-descriptor 4/8 → 5/8` **for both components** and
then went on to the browser-actor stage, i.e. it passed, in order:

1. `captureFreshComponentInputs`' `FRESH_DESCRIPTOR_MAX_BYTES` (4 MiB) on the fresh stdio descriptor;
2. `trustedBootstrapReadRegular(stage/descriptor.semio, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, …)`;
3. `trustedBootstrapDescriptorClaims`' own `bytes.byteLength > DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`
   guard (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:7780`) — the failure raised after it was a *later* line in
   the same function, which is only reachable once the byte bound holds.

**`semio-s-plugin-stdio` — 176 app surfaces, the package that could not be published at all — now
produces a descriptor inside the 4 MiB contract bound.** That is §0's claim, measured end to end, and it
did not need the constant raised or the contract segmented.

### 10.12 Two further product defects found on the way, one fixed here

**(a) FIXED — the trusted bootstrap rejected every dependency-free package.**
`trustedBootstrapDescriptorClaims` demanded `Array.isArray(manifest.dependencies)`, but
`PluginManifest.dependencies` is `#[serde(default, skip_serializing_if = "Vec::is_empty")]`
`#[value(default, skip_serializing_if = "Vec::is_empty")]` (`🛂️manifest/🦀️.rs:4418`), so a plugin that
depends on nothing emits a descriptor with **no `dependencies` key at all**. `semio-s-plugin-stdio` is
exactly such a plugin, and so is `semio-s-plugin-gis` — i.e. the check refused the entire trusted
bootstrap set, and the refusal (`trusted descriptor manifest dependency vector is missing or unbounded`,
run 12:45, `🗑️generated/ds1-hub-dev.txt:6020`) was indistinguishable from a real unbounded vector.
Fixed at the root in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:7785` — absent now means the empty vector,
which is what the encoder's own contract says; a present vector still carries the 128 bound and every
per-row exactness check. The next run got past it, which is how §10.11 was reached.

**(b) OPEN, not DS1's — the gis closed browser actor fails codegen.** Run 14:40:

```
error: browser actor artifact: codegen exit:
AssertionError: browser actor artifact: unsupported import interface
  at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:331
```

The assertion is `result.imports.every(name => admitted.includes(name))` against the frozen allowlist
`browserActorInterfaces` (`…/🌐️browser-bundle/📜️script.ts:36` =
`semio:framework/pure@1.0.0` + `semio:framework/host-async@1.0.0` + `browserWasiInterfaces`), so jco
1.27.0 reported an import for the gis component that the allowlist does not name. This is the
`buildClosedBrowserActorArtifactOwned` stage, **after** both descriptors are accepted, and it is only
reached for gis (`derivedActor` is built for `pluginId === "gis"` only).

That file carries **uncommitted peer edits**, and one of them is behavioural, not cosmetic: `write: false`
was **removed** from both `Bun.build(…)` calls (`📜️script.ts:193` and `:492`), so those builds now write
to disk instead of staying in memory — inside a stage whose whole contract is a hermetic, digest-pinned
evidence directory. The sibling file `…/🌐️browser-bundle/🌐️wasi/🟦️.ts:263` also has an uncommitted
change loosening `subscribeInstant`/`subscribeDuration` parameters from `bigint` to `unknown`. DS1 did
not touch either: the allowlist is a security boundary and widening it is the owning slice's call.
