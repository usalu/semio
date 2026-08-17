# W4 — Partial Deletion Report: `register_os_media_import_handler_kind`

## Verdict: NOT DELETED — the candidate has live callers, contradicting the "0" briefing

## What I verified before deleting

Repo-wide grep, no directory or extension restriction (`grep -rn "register_os_media_import_handler_kind" --exclude-dir=.git .`), 24 hits. Filtering to code (not the ticket's own `.md` scratch/reports):

- `🧰️framework/🛍️products/💻️os/🦀️component.rs` (ROOT):
  - `:3427` — `pub fn register_os_media_import_handler_kind(...)` — the definition.
  - `:2694` — inside `register_dwg_import_handler`: `register_os_media_import_handler_kind(artifact_kind, "dwg", move |bytes| { ... });`
  - `:2718` — inside `register_mesh_importer`: `register_os_media_import_handler_kind(artifact_kind, format_kind, move |bytes| { ... });`
  - `:2726` — inside `register_mesh_dwg_import_handler`: `register_os_media_import_handler_kind(artifact_kind, "dwg", move |bytes| { ... });`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (HOST) — byte-identical mirror, same three call sites at `:2784`, `:2808`, `:2816`, plus the live definition at `:3525` and a second, `#[cfg(not(feature = "os-host-full"))]`-gated dead-stub definition at `:2601` (unreachable — it sits inside `pub mod workflow`, which is itself gated `#[cfg(feature = "os-host-full")]`, so the `not(...)` branch is never true; not a live second implementation).

`register_dwg_import_handler`, `register_mesh_importer`, and `register_mesh_dwg_import_handler` are three of the ten *other* family members this ticket explicitly says have live callers and must not be touched (5, 7, and 4 external plugin call sites respectively, per the dispatch table). Each of their bodies is a single call into `register_os_media_import_handler_kind` — it is the shared low-level primitive they're built on, not a parallel unused escape hatch.

This matches (and is independently corroborated by) `📓️w0-a-escape-hatch.md` already in this ticket folder, §1 and §2: it separately catalogs `register_os_media_import_handler_kind (live)` at ROOT:3427/HOST:3525 and classifies exactly these three call sites as "internal to ROOT/HOST" (40 internal sites counted repo-wide across the whole family) — distinct from the "0" that was measured for *external, plugin-directory* direct callers of the escape hatch. The "0" in this ticket's dispatch table counts only direct calls from `✏️s/🔌️plugins/**`; it was never a claim of zero callers overall, and the dispatch's own hard rule ("grep including `🧰️framework`... if anything anywhere calls it, do NOT delete it") requires treating these three in-framework call sites as disqualifying.

## What I deleted

Nothing. Deleting `register_os_media_import_handler_kind` (either copy) would leave `register_dwg_import_handler`, `register_mesh_importer`, and `register_mesh_dwg_import_handler` referencing an undefined function in both ROOT and HOST, breaking `semio-framework-os-kernel` and `semio-framework-os` compilation — those three wrappers are the ones with the 5/7/4 live external plugin callers this ticket says must be preserved.

## What I deliberately did not do

- Did not delete `register_os_media_import_handler_kind`'s live definition (ROOT:3427, HOST:3525) — has 3 provable internal callers each side.
- Did not touch the HOST dead stub at `:2601-2610` (`#[cfg(not(feature = "os-host-full"))]` inside an `os-host-full`-only module) — out of scope for this task (the candidate named was the live escape hatch, not this pre-existing dead-code artifact; removing it is a separate, differently-scoped cleanup with its own orphan-checking).
- Did not touch any of the other ten family members, `⚙️engine` directories, or `🧬️mutations/**`.
- Did not run `cargo check` to "verify" this outcome — no source file was edited, so there is nothing new to verify; re-running would only restate the already-measured 00:41 green, which this task's own environment note says is a timestamp, not a property, and irrelevant since untouched by me.

## Attribution

Not an upstream-breakage report — this is a pre-deletion caller check that found real, in-file callers, so the deletion did not proceed. No blame assigned to any other session; the three call sites are original, longstanding code in both ROOT and HOST, not new churn.
