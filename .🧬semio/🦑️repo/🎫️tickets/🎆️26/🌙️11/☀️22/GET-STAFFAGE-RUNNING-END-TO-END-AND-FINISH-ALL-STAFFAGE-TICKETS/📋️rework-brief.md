# 📋️ Rework Brief: Invalid Original Staffage Assets

Read `U/📋️brief.md` first (paths `R`, `S`, `U`, and the rules: they bind you). Then `U/perfect-3d.md` (the finisher's stock-taking of ticket `K` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`), then in `K`: `rework-brief.md` (your authoring contract), `validator.md` ("Rule changes authors must know", the per-scale `volumes-feature` rule of 2026-11-22 01:23), `design.md`, `r3-tools.md`, and for each of your assets its `K/rework/assets/<slug>.md` (if any), its batch file `K/rework/<NN>.md`, its volumes source `K/volumes/<category>/<slug>.json` and its rework scripts `K/rework-*<slug>*`.

## Why

22 of the 152 original catalog assets fail the cheap validation (`r3-validate.ts <category>/<slug> --catalog-only`): 20 through `volumes-feature` (coarse scales still carry the 1:50 parts list: each scale must be its own model for its smallest feature 7.5 / 15 / 30 / 75 mm), one `lateral-ambiguous`, one print finding. While one catalog asset is invalid, the check CLI over the manifest and the site's archive-loading test are red, so staffage does not run end to end. You repair the assets named in your prompt.

## Done, per asset

1. The intent of the rework brief for the coarse scales is met by modelling: each coarse scale handcrafted for its smallest feature (merged, well-proportioned masses, true silhouettes, stable ids and bones), 2D drawings and 3D lists agreeing; never the fine list with entries deleted blindly, never a loosened rule, never `envelope`/`slender` flags used to dodge a finding that is real.
2. `r3-validate.ts <category>/<slug> --catalog-only` prints `VALID`; the full proof `--volumes` prints `VALID` with `checkVolumes 0`; the product check CLI (`S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <slug>` or as `r3-tools.md` says) prints `✓`.
3. Sheets rendered and OPENED at every scale you changed (body, thermal, structure, and the 2D drawing); what you did not open, say so.
4. Volumes source in `K/volumes/…` and the catalog file agree (the ticket's splice/publish route, as the previous authors used it); your authoritative script kept in `K` as `rework-<slug>-….py|ts`; per-asset report `K/rework/assets/<slug>.md` appended with this pass.
5. If an evaluated value of the manifest row changes (`heightM`, `widthM`, `powerW`, poses, clips, representations), do NOT edit `S/🗂️catalog/🔣️.json`: list the asset and the new values in your report for the umbrella coordinator.

## Tools

Pinned bun: `BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`; tools in `K` as `r3-tools.md` describes. NEVER call an `mcp__repo__*` build tool or a blocking `repo run` (it hangs for `@teaching/staffage`). One tool process at a time, in the foreground, through Bash with an explicit timeout (10 minutes; the big trees and walls take minutes). Seven other agents work now: two on the 47 new humans, two on the new plants, one on the flow view code, two on other invalid originals. Touch only your assets and their files in `K`.

## Report

`U/<name from your prompt>.md`: per asset the findings before, what you remodelled per scale, the final proof lines, sheets opened, manifest values changed, files created/updated/removed (repo-relative), what stays imperfect. An asset you cannot bring to `VALID` in this session: say exactly what is left, with the measurement. Final message: report path and at most six lines.
