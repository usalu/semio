# Window 3 Landing Plan (session 14c)

Opens when W4 reports `final-publish.rc` = 0 and the coordinator writes "WINDOW 3 OPEN" in `📓️fleet-14-agents.md`.
Closes when the coordinator launches the next chain (rebuild-all → publish → 7800 onto the new catalog).

## Method: landing trains, not one-set-per-hold

~25 prepared sets each needing native + wasm32 proof would take far more than one night through 1-slot lanes. Sets land in
**trains**: the train owner applies each set in the listed order (`--dry-run` first, then `--write`; stop at the first
dry-run problem and skip that set), then ONE combined proof per train:

1. native: `cargo check --keep-going --lib --tests -p <every crate any set in the train touched>` (native lane, build-fleet-b);
2. wasm32: `cargo check --lib --target wasm32-wasip2 -p <touched guest crates>` (+ `wasm32-unknown-unknown` for the renderer) via
   the wasm lane (free between chains);
3. tsc of every touched TS package + one `serve s react dev` boot to Home when host TS changed (rule 20).

On red: map each error to its set by file path, revert THAT set with its own revert/backup (every set script must support
`--revert` or keep byte backups under `wp-<slice>/w3-backup/`), re-run the failed proof, continue. The set's owner fixes it for
the next train. Laws run after the train is green (each owner runs its own laws in the native lane). The next chain is the
full proof of everything.

Every set owner, before its train: re-run its dry run on the live tree, keep a revert path, and list its touched crates in its
report under "Window 3 set: <name> — crates: …".

## Trains (order matters)

**T0 — taxonomy + launch (R10, serialized, taxonomy-load probe + one serve boot after each):** R10's registration pass for every
new directory relayed this session (io-matrix, hub-collaboration, agent-ceiling, transient-apply-refusal fixture, docker-image,
forwarding-proxy, docker-image-v1, set-named-layout, admit/retire-local-document + plugin-host case + local-catalog dirs,
program-matrix reducers, interaction-latency reducers, Panel component tests, boot-budget, 🐳️containers lifecycle), V1's
directory registration, R9's launch-inputs discovery patch, generated launch rows + nx targets for every relayed spec.

**T1 — SDK core (owner LB2 + T14 + P9 + S20; LB2 drives):**
LB2 `lb2-w3-stdio-sets.sh` (xml demo regen → p6 publish_declared_catalogs → p1 → p7) · T14 F9 content ids + id-map + carrier
regeneration → G12 `g12-authoring-seed.py` → T14 SDK class fix (+ raster/note genesis hooks) → LC P8 orphan → T14 H9-L kind
labels → T14 5b `dsl_value!` + codemod → T14 fallback-wrapper removal + 2 SDK lib reds · P9 `p9-land.sh write` (agent-lane +
jack nodeIds) · S20 initializer (`s20-patch-initializer.py`) → document verbs → chunk staging → loadRequest retirement.
Watch after T1: every `.artifact(…)` plugin's describe for `plugin-assembly.declaration-schema|inference` (p6).

**T2 — stdio (ST2 then R10):** ST2 `st2-apply.py --part code` → R10 `--part r10` → CX1 `cx1-apply.py` (NEW bootstrap generation via
`h14-bootstrap-generation.ts 19`) → ST2 1b remainder (drop `--skip-nx-cache`) → LB2 p5 docx/xlsx OPC reds → LB2 p3 + WG11 table-row
painter (`lb2-p3-wg11-joint-land.sh`).

**T3 — apps & shells:** SH2 space-home set (39 files) → SH2 route B (44) · S18 named-layout Rust twin · WG11 reseed, board-pointer,
a11y text-name, shell-turn stack fix, harness landing · C12 writer concurrent typing · C13 P1 cross-undo fold · AV2
`av2-land.sh` (video export) · EN2 energy epJSON, draw descriptions, gltf/obj/bcf/docx · S19 procedural import traps, generation3d
seated example, contributions framework, flow extensions, norm sets (+ i18n 1b after H9-L) · S20 cad solids, process formats,
silent exports.

**T4 — host runtime & hub-adjacent:** H13 transport refill (kernel directory client + services + os-mcp 🔗️remote) · H14 codec
origin (plugin host `OwnedRuntime::codec_call`) · G12 live catalog (`g12-live-catalog.py`) · Z4 fresh-clone B1–B5 + devcontainer
lifecycle · R10 dependency sets (png, zip, image, three manifests step 11b).

**T5 — text codemods, LAST:** R10 `[DEBUG]`→`[TRACE]` codemod → guest comment-hoist (61 emoji picks) → `@emoji` codemod.

## After the trains

Coordinator launches the next chain (full rebuild-all → publish → 7800). Live verification wave on 7800: G12 MCP battery
(all packages), C12/C13/WG11 collaboration harnesses, S18 matrix en/de, S20 io-matrix, H13 live proofs, H14 idle-release,
F3 hub-open latency, R10 goal gate (per-outcome verdict table).
