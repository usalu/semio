# 📓️ WASM facade wiring — note/cad diff·mutations·snapshot text+binary leaves

## Scope

The 10 files listed in `/private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/leaves-wasm-throw.txt`:

- cad (`🧬️schema/{🔺️diff/📝️text, 🧬️mutations/{📝️text,💾️binary}, 📸️snapshot/{📝️text,💾️binary}}/🟦️component.ts`)
- note (`🚪️io/{🔺️diff/📝️text, 🧬️mutations/{📝️text,💾️binary}, 📸️snapshot/{📝️text,💾️binary}}/🟦️component.ts`)

A prior attempt had deleted the exported `parseDsl`/`printDsl`/`encode`/`decode` functions from these files
and replaced them with copy-pasted `export type … = string;` aliases whose docstrings all named `stdio.json`
regardless of which artifact/facet they actually lived in. That was reverted before this pass started. This
pass **kept every exported function**, fixed each docstring/message to name the real artifact and facet, and
did not invent a binding that does not exist.

## Step 1 — what actually exists (verified before touching anything)

**No TypeScript-callable WASM binding exists for per-artifact `parseDsl`/`printDsl`/`encode`/`decode` today.**
Evidence:

1. **The plugin WASM ABI is the WIT Component Model, not wasm-bindgen.** `wasm-bindgen` is a
   `cfg(target_arch = "wasm32")` dependency in every plugin `Cargo.toml` but is never invoked with
   `#[wasm_bindgen]` anywhere in `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs` (grepped, zero hits) — the
   real guest boundary is `semio-framework-plugin`'s `component-guest` feature, compiled against
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`.
2. **`world actor`'s only exports are `reactor` (`poll`), `jobs`, `checkpoint`, `describe`**
   (`📜️component.wit:1307-1313`). The WIT doc comment on `interface reactor` (`📜️component.wit:~1043`) states
   explicitly this single `poll` turn-loop **replaces** `exchange`, the poll-backbone heartbeat, and *every*
   per-verb surface including `apply-mutations[-text]` and `read/load-app-document-{text,pack}` — i.e. a
   stateless codec-call export that could have backed these facades **used to exist conceptually and was
   deleted** in what the WIT calls the "B1 world-collapse".
3. **The real TS-side host code (`🌐plugin-web-materialize.ts`, jco-generated `🟨️host-shim.js` under
   `🧑️‍💻️dev/…/fixture/dist/…`) implements *host imports* the guest calls** (`readDocument`, `writeDocument`,
   `openWindow`, `writeBlob`/`readBlob`, …) — the opposite direction from what these facades need, and several
   of those host stubs themselves still throw `"…unsupported"`. There is no host code anywhere that
   instantiates a plugin component and calls an exported codec function.
4. **Nothing calls these 10 files today.** `grep -rn "parseDsl\|printDsl"` across `✏️s/🔌️plugins/🗒️note` and
   `✏️s/🔌️plugins/📐️cad` (excluding the files' own definitions) returns zero hits. The only consumers are the
   plugins' `📦️index.ts` barrels, which just `export * as … from "…component.ts"` — no call sites.

### Taxonomy inconsistency (as flagged in the brief)

At the identical tree position, `🕸️dag`'s `🚪️io/📸️snapshot/📝️text/🟦️component.ts` is `export type
JsonSnapshotText = string;` while `🗒️note`'s is a function facade. Both are legitimate, but for **different
reasons**, confirmed by reading each facet's sibling `🦀️component.rs`:

- **dag's `.rs` at that exact position is a real native codec** (`impl store::ArtifactDsl for DagSnapshot`,
  `parse_dsl`/`print_dsl`) — the `.ts` there is a **pure wire-type mirror** (matching the JSON-schema
  `$id: stdio.json/snapshot/text.json`, `title: JsonSnapshotText`), because nothing in TS calls the codec —
  Rust callers use the Rust function directly, natively.
- **note's / cad's `.rs` at the snapshot/mutations positions are equally real native codecs** (`parse_dsl`/
  `print_dsl`, `encode`/`decode`, `encode_op`/`decode_op`) — so the type-alias pattern would be equally valid
  there in principle. The type alias is **not** wrong on its own; what made the prior attempt's version wrong
  was (a) deleting real exported functions no design doc asked to delete, and (b) mislabeling every file as
  `stdio.json` regardless of its real artifact/facet.
- **cad's 5 files sit under `🧬️schema/`, not `🚪️io/`** — a real deviation from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/📓️design.md` §1, which is
  explicit that `🧬️schema/` is "TYPES + pure transforms ONLY (no codecs)" and all codecs — including the
  *native* one — belong under `🚪️io/<facet>/<representation>/`. `cad`'s `🚪️io/` tree only has
  `📥️import`/`📤️export`/`🗺️geometry-import` — no native `📸️snapshot`/`🔺️diff`/`🧬️mutations` codec dirs — so cad's
  snapshot/mutation codecs (Rust and TS alike) still live at their pre-`design.md`-correction location. This
  is a real, reportable inconsistency but **out of scope to fix here**: `📐️cad` has an active concurrent
  ticket (`CAD-END-TO-END`) touching this same tree (`git status` shows `M`/`A` on
  `📦️packages/🟦️typescript/📦️index.ts`, `🫀️core.ts`, and several `✏️editor/⚙️engine/*` files I did not touch),
  and moving the whole `🧬️schema` codec tree to `🚪️io` is a structural taxonomy migration, not a stub fix.

### A second, more important finding: `🔺️diff/📝️text` has *no* Rust codec at all, for either plugin

`✏️s/🔌️plugins/🗒️note/…/🚪️io/🔺️diff/📝️text/🦀️component.rs` says outright: *"`NoteDiff` has no `impl
store::ArtifactDsl`/`ArtifactPack` anywhere in this plugin — this facet exists purely to register
`note.diff`'s handcrafted text grammar for LSP/verification tooling… design.md §2's `LanguagePair` doc: 'a
registered grammar with no literal runtime parser backing it' [is] equally legal."* Grepping cad's
`🧬️schema/🔺️diff/📝️text/🦀️component.rs` confirms the identical situation there (only `apply_to_artifact`/
`impl MutationDiff<CadSnapshot> for CadDiff`, no `ArtifactDsl` impl), just without the explanatory comment.
So for these 2 of the 10 files, the honest state is not "needs a WASM binding" — it is "there is nothing on
either side to call, by design." The 8 remaining files (snapshot ×2, mutations ×2, per plugin) all have real,
callable native Rust codecs (`parse_dsl`/`print_dsl`, `encode`/`decode`, `encode_op`/`decode_op`) that
genuinely lack only the WASM/JS bridge.

## Step 2 — what was changed

All 10 files **still export the same functions with the same signatures** and **still throw** — nothing was
deleted, nothing was faked. Two changes per file:

1. **Docstring** now names the real artifact (`note`/`cad`) and facet, cites the real backing Rust function(s)
   in the sibling `🦀️component.rs` by name, and states precisely what's missing:
   - For the 8 snapshot/mutations files: *"no stateless WASM codec-call export exists on `world actor`
     (poll/jobs/checkpoint/describe only — deleted in the B1 world-collapse); needs a new WIT export in
     `component.wit` plus a TS host loader (jco-generated bindings, cf. `🧫️fixtures/🔌️jcoprobe`)."*
   - For the 2 diff/text files: *"`{Note,Cad}Diff` never implements `ArtifactDsl` — the grammar is registered
     for tooling only, never parsed; there is nothing to wire this to."*
2. **Return/parameter types upgraded from `unknown` where a real mirrored TS type already exists** in this
   repo (found via `grep -rl "NoteSnapshot\|CadSnapshot"` / `NoteDiff\|CadDiff` across `*.ts`, excluding
   ticket scratch files):
   - `📸️snapshot/📝️text` and `📸️snapshot/💾️binary`: `unknown` → `NoteSnapshot`/`CadSnapshot` (real hand-mirrored
     interfaces at `🧬️schema/📸️snapshot/🟦️component.ts`, one level up).
   - `🔺️diff/📝️text`: `unknown` → `NoteDiff`/`CadDiff` (real hand-mirrored interfaces at
     `🧬️schema/🔺️diff/🟦️component.ts`).
   - `🧬️mutations/{📝️text,💾️binary}`: left as `unknown` — the parent `🧬️schema/🧬️mutations/🟦️component.ts` for
     both plugins is itself just `export {};`, i.e. no `NoteMutation`/`CadMutation` TS mirror exists anywhere
     to reference honestly.

No file's docstring names an artifact other than the one it lives in.

## Verification (actually run)

```
bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler --esModuleInterop --skipLibCheck <all 10 files>
```
→ **zero output, exit code 0.**

Also ran the same command against each plugin's `📦️packages/🟦️typescript/📦️index.ts` barrel (the only
consumers, transitively) to check for regressions: both report `TS5097: An import path can only end with a
'.ts' extension when 'allowImportingTsExtensions' is enabled` on their `export * as … from "…component.ts"`
lines. **This is pre-existing and repo-wide, not caused by this change** — confirmed by running the identical
command against `🕸️dag`'s `📦️index.ts` (untouched by this ticket), which produces the same class of error on
its own `.ts`-extension re-exports. The project's real `tsconfig.json` (root) also has no
`allowImportingTsExtensions`, so this is an artifact of invoking raw `tsc` outside the project's actual build
pipeline (bun's bundler, not `tsc`, resolves these `.ts`-extension imports in practice), not a defect in any
file this ticket touched. The 10 target files' own `import type { … } from "…component.ts"` statements are
**type-only** and are exempt from `TS5097` (verified directly with a 2-file isolated repro in the scratchpad),
which is why the direct 10-file `tsc` run above is clean.

No round trip was run — there is no code path to exercise. Nothing calls these functions (confirmed above),
and the functions still throw by design.

## `git status --porcelain`, scoped to the 10 files

All 10 show as `M` (modified in place), nothing added/deleted/renamed, no stray files:

```
 M .../📐️cad/.../🧬️schema/🔺️diff/📝️text/🟦️component.ts
 M .../📐️cad/.../🧬️schema/🧬️mutations/📝️text/🟦️component.ts
 M .../📐️cad/.../🧬️schema/🧬️mutations/💾️binary/🟦️component.ts
 M .../📐️cad/.../🧬️schema/📸️snapshot/📝️text/🟦️component.ts
 M .../📐️cad/.../🧬️schema/📸️snapshot/💾️binary/🟦️component.ts
 M .../🗒️note/.../🚪️io/🔺️diff/📝️text/🟦️component.ts
 M .../🗒️note/.../🚪️io/🧬️mutations/📝️text/🟦️component.ts
 M .../🗒️note/.../🚪️io/🧬️mutations/💾️binary/🟦️component.ts
 M .../🗒️note/.../🚪️io/📸️snapshot/📝️text/🟦️component.ts
 M .../🗒️note/.../🚪️io/📸️snapshot/💾️binary/🟦️component.ts
```

An earlier bash typo (`🏅️标准` instead of `🏅️standards`) briefly created a stray directory/file under
`📐️cad/🗿️artifacts/📐️cad/`; both were `rm -rf`'d immediately and `git status` confirms nothing stray remains.

`📐️cad` also shows unrelated `M`/`A` changes from the concurrent `CAD-END-TO-END` ticket
(`📦️packages/🟦️typescript/📦️index.ts`, `🫀️core.ts`, several `✏️editor/⚙️engine/*` files, `🧩️extensions/*`) —
not touched by this pass, reported here only as a collision note per the brief.

## Implementation plan (for whoever builds the real binding)

1. **Add a stateless codec-call WIT interface** to
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`, exported by `world actor` alongside
   `reactor`/`jobs`/`checkpoint`/`describe` — e.g. `codec: interface { parse-dsl: func(facet: string, text:
   string) -> result<pack, plugin-error>; print-dsl: func(facet: string, doc: pack) -> result<string,
   plugin-error>; encode: func(facet: string, doc: pack) -> pack; decode: func(facet: string, bytes: pack) ->
   result<pack, plugin-error>; }` (facet distinguishes snapshot/diff/mutations within one artifact; `pack` is
   the existing `list<u8>` wire type already used everywhere else in this WIT package, so payloads round-trip
   through the same `store::pack_rt`/`pack::encode_record_body` primitives the rest of the ABI uses — no new
   JSON-string special case).
2. **Wire each plugin's `📦️glue.rs`** to dispatch `facet` to the real functions that already exist and are
   cited by name in each of the 8 (non-diff-text) facades' new docstrings — e.g. cad's `codec.parse-dsl("cad.cad",
   text)` → `crate::artifacts::cad::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text)`, and
   analogously for mutations' `OpText`/`OpBinary` impls. No new Rust logic is needed — only a dispatch shim.
3. **Build a TS host loader** analogous to the jco-generated bindings already produced for
   `🧫️fixtures/🔌️jcoprobe` (`🌐️harness/out-callback/*.d.ts`) — instantiate the plugin's `.wasm` component
   outside the actor-pool/turn-loop path (this is a stateless call, unlike `poll`) and expose a typed
   `callCodec(plugin, facet, op, payload)` entry point.
4. **Point these 10 facades at that loader** — replace the `throw` bodies with a call through the new loader,
   keeping the exact same exported signatures (already typed against the real `NoteSnapshot`/`CadSnapshot`/
   `NoteDiff`/`CadDiff` mirrors where those exist).
5. **Leave `🔺️diff/📝️text` throwing permanently** for both plugins (or delete the facet's function-facade
   shape entirely and go back to a grammar-registration-only `.rs`/`.ts` pair, mirroring what note's `.rs`
   already documents) — there is no Rust codec to call into, by design, and step 1–4 above cannot change that
   without first deciding to actually implement `ArtifactDsl for {Note,Cad}Diff`, which is a product decision
   out of scope here.

## Files touched

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🟦️component.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🟦️component.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🟦️component.ts`
