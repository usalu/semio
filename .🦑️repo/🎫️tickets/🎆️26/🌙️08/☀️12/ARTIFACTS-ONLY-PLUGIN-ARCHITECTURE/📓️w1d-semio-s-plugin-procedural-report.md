# W1d — `semio-s-plugin-procedural` — re-verification pass

**Finding: `.setup()` conversion was already complete before this pass started** — done by an earlier
same-ticket pass (`📓️w1b-semio-s-plugin-procedural-report.md`, mtime 22:00). This pass independently
re-verified the current on-disk state (not just trusted the prior report), since concurrent churn had
touched files under this plugin's tree after 22:00 (mutation vocabulary files, `📦️glue.rs` — see
`find -newer` evidence below), and ran one fresh mandated `cargo check`.

## Current `.setup()` state — confirmed, not re-derived

`grep -rn "\.setup(" ✏️s/🔌️plugins/🌀️procedural` → exactly one real call site:
`🦀️component.rs:34: .setup(register_exports)`. `register_exports` (root `🦀️component.rs:22-27`) holds
4 calls, matching w1b's report exactly:

| call | why it stays in `.setup()` |
|---|---|
| `apps::procedural2d::config::schema::register_app_schema()` | app-scope config schema — `register_app_schema_descriptor` is the one §6 registrar `ArtifactDeclaration` deliberately excludes by design (per note's exemplar pattern). |
| `apps::procedural3d::config::schema::register_app_schema()` | same, 3d app. |
| `artifacts::procedural3d::engine::register_dwg_mesh_bridge()` | self-registers procedural3d's OWN kind (`"3d.procedural"`) via `register_mesh_dwg_import_handler` — compliant ownership shape, but that registrar has no `ArtifactDeclaration` field (not one of the 9 §6 artifact-scoped fields). |
| `artifacts::procedural3d::engine::ensure_linked_flow_extensions()` | installs `flow.extension` operator installers via `register_linked_flow_extension_installer` — flow's own extension registry, the other §6 function excluded by design, `Once`-guarded for idempotency. |

`.artifact()` declarations are wired for both artifacts:
`.artifact(crate::artifacts::procedural2d::declaration())` /
`.artifact(crate::artifacts::procedural3d::declaration())`, plus
`.register_document_app::<…Procedural2dPlayApp>(…)` / `…Procedural3dPlayApp…`.

**Silent-rebind check (per the ⛔ hard rule):** `declaration()` in each artifact-root file calls
`.composers(crate::artifacts::procedural2d::standards::v1::engine::io_registry::entries())` —
fully-qualified, not a bare `io_registry::entries()`. Verified this is deliberate: the artifact-root
file itself defines its own `io_registry` module (a `&'static [&'static ComposerEntry]` re-export
shim, incompatible type with what `.composers()` needs), so a bare call there would silently rebind to
the wrong signature. Confirmed both procedural2d and procedural3d qualify the call fully — no rebind.

**Cargo deps:** 7 grandfathered `semio-s-plugin-flow-extension-*` deps untouched (verified present,
unedited, in `Cargo.toml`), per the plugin-specific instruction not to touch them.

**Residue verdict: none beyond what w1b already named.** No item was found that w1b missed; no new
declaration-field gap identified in this pass.

## Churn since the w1b report (evidence this needed re-checking, not just trusting the prior doc)

`find ✏️s/🔌️plugins/🌀️procedural -name '*.rs' -newer 📓️w1b-…-report.md` → 23 files touched after 22:00,
all under `🗿️artifacts/🌀️procedural2d/…/🧬️mutations/**` (generation/replace-widget/replace-synapse
mutation triads) and `📦️glue.rs` — SMO's semantic-mutations-overhaul continuing, exactly the
already-documented out-of-scope gap w1b named. **The plugin-root `🦀️component.rs` (the `.setup()` /
`.artifact()` file) was NOT among the touched files** — the declaration conversion itself is stable.

## Verification

1. **`#[path]` resolution** (`📦️glue.rs`, independently re-checked via script): 195 non-`"."`
   `#[path]` mounts, **0 missing**.
2. **`include_str!`/`include_bytes!` resolution** (independently re-checked across every `.rs` file in
   the plugin tree, resolved relative to each including file's own directory): 121 call sites,
   **0 missing**. Matches w1b's count exactly.
3. **`cargo metadata --no-deps --format-version 1`**: exit 0. Saved at
   `scratch-w1d-procedural-metadata.txt`.
4. **`cargo check -p semio-s-plugin-procedural --all-targets`** (`RUSTC_WRAPPER=""`,
   `CARGO_TARGET_DIR` per the mandated override): run **3 times**, all exit 101. **Not green — but the
   failure is 100% outside this plugin.**

   All 3 attempts fail identically in `semio-s-plugin-stdio` (a path dependency of
   `semio-s-plugin-procedural`), never inside `🌀️procedural`'s own paths:

   ```
   error[E0433]: cannot find `inferences` in `schema`
     --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/🟪️stl/.../⚙️engine/🦀️component.rs:295:119
   error[E0433]: cannot find `inferences` in `schema`
     --> ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/.../🗿️artifacts/📼️avi/.../🚪️io/🦀️component.rs:59:120
   ... (growing across attempts)
   error: could not compile `semio-s-plugin-stdio` (lib) due to N previous errors; 601 warnings emitted
   ```

   - `grep -c "🌀️procedural"` on all 3 output files: **0, 0, 0** — this plugin's own code is never
     reached; `cargo check -p semio-s-plugin-procedural --all-targets` fails while still compiling its
     `semio-s-plugin-stdio` dependency, before it ever gets to compile `procedural` itself.
   - Error count **grew between attempts** at the identical two error sites (2 → 9 → 12 errors,
     same `stl`/`avi` `⚙️engine` files, same `schema::inferences` symbol) — direct proof of a live,
     in-progress edit, not a stable pre-existing defect.
   - `stat -f '%Sm'` on the first erroring file
     (`🗄️stdio/…/🟪️stl/…/⚙️engine/🦀️component.rs`) showed a timestamp **seconds old**, matching the
     attempt window exactly.
   - `🗄️stdio` is explicitly listed in this ticket's HARD RULES as a plugin **held by another
     session** ("Do NOT edit plugins held by other sessions: … 🗄️stdio"). This matches the framework
     agent's own w1d report verbatim: `semio-s-plugin-energy`/`semio-s-plugin-puzzle` hit the exact
     same `E0433: cannot find inferences in schema` in `semio-s-plugin-stdio`, attributed to "UCAS's
     live edit."
   - Raw output saved at `scratch-w1d-procedural-check-{1,2,3}.txt` in this ticket folder.

**Classification: (c) upstream, peer-owned, in-progress.** Caused by a live concurrent edit inside
`semio-s-plugin-stdio` (a dependency this plugin does not own and is instructed not to touch), not by
this plugin's own code, and not by anything changed in this pass or the prior w1b pass. Zero errors
originate in any `🌀️procedural` file across all 3 attempts.

## Honest pass/fail

- `.setup()` on `semio-s-plugin-procedural`: reduced from 5 imperative calls (pre-ticket) to exactly
  4 named, justified survivors — **no bare `.setup()` catch-all remains**, and no item was silently
  dropped or force-converted. This was completed by the w1b pass; this pass re-verified it holds and
  found nothing to change.
- Path/include integrity: 195/195 `#[path]` + 121/121 `include!` targets resolve (independently
  re-checked, not just re-read from w1b).
- `cargo metadata --no-deps`: OK.
- `cargo check -p semio-s-plugin-procedural --all-targets`: **not green**, blocked entirely by a live,
  currently-in-progress edit inside peer-held `semio-s-plugin-stdio` — confirmed by growing error
  count and second-old mtime across 3 attempts. Zero errors trace to `🌀️procedural`. Not caused by,
  and not fixable within, this pass — `stdio` is explicitly off-limits per this ticket's HARD RULES.
