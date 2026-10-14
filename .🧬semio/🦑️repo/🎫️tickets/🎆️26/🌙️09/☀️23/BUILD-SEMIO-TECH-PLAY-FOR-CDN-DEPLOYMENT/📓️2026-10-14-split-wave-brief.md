# ✂️ Split Wave Brief (2026-10-14)

Brief that every split lane (P1–P8) reads first. The coordinator owns it; lanes do not edit it.

## Law

The canonical compilation-unit ceiling is the repo policy `sourceBoundary` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`:

| Limit | Value |
|---|---|
| Active cfg-selected lines | 4,000 |
| Full source bytes, embedded inputs included | 262,144 |
| Files | 32 |

The limits apply per unit, which means per crate and cfg. Lib and test units each count separately.

- It supersedes the ≤15k lines law L3 of `📓️2026-10-13-scalable-build-architecture.md`. One policy, no second law.
- The release driver and component pass refuse any unit above it, so every crate in the play closure must fit.
- Measure with the canonical Source helper `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`. If it can't run, fall back to an upper-bound walk of the `#[path]` closure, and say which one you used.

## Shape of a cut (family pattern, law L6)

For an artifact `<a>` of plugin `<p>`, package names follow `semio-s-artifact-<p>-<a>-<layer>` (law L4: ≤ 48 chars).

| Layer | Contains | May depend on |
|---|---|---|
| `-model` (split further: `-model-<facet>`) | schema records, snapshot, document types | framework only |
| `-mutations` (split by mutation group: `-mutations-<group>`) | mutation vocabulary, apply, inverse | model |
| `-inferences` (split by group) | inference owners | model |
| `-read` / `-write` (or `-io`) | codecs, import, export | model |
| `-editor` | editor, panels, actions | the layers it names |
| aggregator `semio-s-artifact-<p>-<a>` | registration and composition glue only | layers |

- **Cut along the domain, not by file count.** The taxonomy subtree usually already names the facets, so use its folders as crate boundaries. A group that still exceeds the ceiling gets split along its next taxonomy level.
- **One owner per type.** No re-export facades for compatibility, no duplicated code, no `pub use` shims at old paths. Consumers name the owning crate directly, as in the E1b rule.
- **Tests go with their owner.** Round-trip and full-stack tests go to the aggregator's integration tests; support helpers go to a test crate or `[dev-dependencies]`. Never a `testing` feature in a consumer, because features are banned by L2. See the pdf lesson in `📓️2026-10-13-wave-e3b-pdf-cycle-and-cuts.md` §5.
- **No cycles.** Fix upward edges by inversion, the way the pdf subset validator by dialect did. Fix it in the domain, not with a callback shim.
- **Every new crate needs:**
  - real manifest at `<owner>/📦️packages/🦀️rust/Cargo.toml` with `path = "../../🦀️.rs"`;
  - workspace membership via the membership generator;
  - workspace dependency rows;
  - consumers declaring exactly the crates they name;
  - taxonomy admission, so the taxonomy gate stays green.
- **Composition contracts (stdio):** `catalogPackage` plus `symbolPackages`, as in Lane C.
- **Tools:** reuse E3b's `e3b-format-cut/✂️crate-cut.py` (`plan`/`apply`), `🧾️consumer-manifests.py` and its JSON configs as the model. **Copy** them into your lane folder before changing them; never edit another lane's scripts. Read the incidents in E3b §8 first: a truncated manifest, census-strip offsets, and attributes on flattened uses.

## Rules (all lanes)

- AGENTS.md applies.
- No git-modifying command, no worktree, no sub-agent, no ticket MCP operation.
- `🗑️generated` only in your own sub-folder.
- Don't use the Grep tool; use Bash. `cd` explicitly in every Bash call.
- Cargo:
  - foreground only, through `bash "$T/🚦️cargo-slot.sh" <lane> -- …`;
  - private `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` under `.🧬semio/🦑️repo/⚡️cache/<lane>/`;
  - delete that dir at the end.
- Tasks: use `bun repo run <project>:<target>`, which is Nx underneath.
- **Peer fleet A** (`🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`) owns framework kernel, UI, store, history, directory, job and pack (see `📓️2026-10-14-peer-scope-map.md` §1). Never edit those packages.
  - If they are red, wait for a green window: one foreground loop that re-checks every ~10 min. Or verify your crates with their consumers excluded.
  - Do not stop.
- **Other lanes of this ticket** own:
  - E2b: plugin SDK `🔌️plugin`.
  - E3b: stdio pdf, dwg, gltf, step.
  - E4: `🏙️bim`, `🏛️architect`, framework 3d.
  - L2: first-party features.
  - R1: release driver.
  - Stay out of their scope.
- **Concurrent edits:** re-read a file right before editing it, keep edits compile-atomic, and never revert edits that are not yours.
- **Code style:** no compat layers, shims, legacy or deprecations. Docstrings start with a unique emoji; no comments inside definitions. Prefix temporary logs with `[DEBUG] `.
- **Verification:** for each artifact, run the native `--lib` check of every new crate, `--tests` for the test units, the hub composition's wasm32-wasip2 check, and the artifact's existing test suite (`bun repo run <project>:test` or cargo test). Report honestly what ran and what didn't.
- **Report:** write `📓️2026-10-14-split-<lane>.md` in the ticket folder with:
  - a before/after table per crate: files, active lines, bytes;
  - the dependency graph;
  - verification results;
  - incidents;
  - remaining over-ceiling units.
  Append one entry to `📓️2026-10-10-migration-corrections.md` with the next free number.

## Lanes

| Lane | Scope (`✏️s/🔌️plugins/…/🗿️artifacts/…`) | Approx. Rust lines incl. tests |
|---|---|---|
| P1 | `🔋️energy/🔋️model` | 115k |
| P2 | `📸️remodel/📸️remodeling` | 70k |
| P3 | `🌀️procedural/🧊️generation3d`, `🌀️procedural/🌀️generation2d` | 83k |
| P4 | `🧩️puzzle/🧊️3d`, `◻️2d`, `🖐️5d` | 154k |
| P5 | `🏗️fem/◻️2d`, `🏗️fem/🧊️3d` | 91k |
| P6 | `🖍️draw/🖍️drawing`, `📏️layout/📏️layout`, `🗒️note/🗒️note`, `🖨️raster/🖨️raster` | 130k |
| P7 | `📐️cad`, `🏭️process/🧊️process3d`, `💠️lowpoly`, `🌍️gis/🗺️gismap`, `🎞️animate/🎬️presentation`, `🎥️shooting`, `📋️forms` | 145k |
| P8 | `🗄️stdio` except pdf, dwg, gltf and step: `🧿️semio`, re-checked against the 4k ceiling after Lane C, then `ifc`, `docx`, `xlsx`, `gif`, `xml`, `svg`, `pptx` and every other stdio format | 330k |
| later | `📕️norm/*`, `➗️mathematical`, `🔱️trinity`, `🀄️wfc`, `🌊️flow`, `🧱️block`, `🪐️space`, `✒️writer` and the rest | — |

Order inside a lane: the biggest crate first. Prefer finishing whole artifacts, compile-clean, over touching many artifacts halfway.
