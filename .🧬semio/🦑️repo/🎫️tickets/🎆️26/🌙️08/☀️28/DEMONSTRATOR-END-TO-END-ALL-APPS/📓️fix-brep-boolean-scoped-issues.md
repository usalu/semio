# Fix: missing `issues_scoped_to_new_solids` in brep boolean

## Problem

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs:500`
called `issues_scoped_to_new_solids(body, &pre_existing_solids, validate_body(body))`, a function
that did not exist anywhere in the repo — blocking every WASM plugin build
(`cargo check -p semio-s-plugin-stdio --lib --target wasm32-wasip2`) with E0425.

## Fix

Added two private helpers in the same file, right after `exact_imprint_boolean` (which owns the
call site) and before the `#region 🔖️Clip` marker:

- **`issues_scoped_to_new_solids(body, pre_existing_solids, issues) -> Vec<ValidationIssue>`** —
  drops any issue whose `entity` string resolves to something owned by a solid that existed
  *before* the boolean ran, so a pre-existing operand's already-broken topology is never blamed on
  the boolean. An issue whose entity cannot be resolved to any pre-existing solid (a new solid, an
  orphan, or an unrecognized format) is kept — conservative by design, since a boolean that leaves
  an unattributable issue is a real failure.

- **`pre_existing_entity_strings(body, solids) -> HashSet<String>`** — builds a forward map: for
  every solid in `solids`, walks it down through shells → faces → loops → coedges → edges →
  vertices via `Body`'s existing traversal accessors (`solid_faces`, `solid_shells`, `shell_faces`,
  `face_loops`, `face_coedges`, `loop_coedges`) and formats the *exact same* strings
  `validate_body`'s checks would emit for each. A forward map was chosen over inverting
  `raw_index()` back into an id (there is no such inverse in the repo, and hand-rolling one risks
  silently desyncing from the id representation) per the task's own guidance.

### Entity prefixes handled (every `format!` in `validate_body`'s checks, in
`✅validation-report/🧪️body/🦀️.rs`)

| prefix | source check(s) |
|---|---|
| `loop-{id}` | `check_loop_rings` (empty-loop, broken-ring, loop-not-closed, next-prev-mismatch) |
| `edge-{id}` | `check_edge_valence` (non-manifold-edge), `check_degenerate_geometry` (degenerate-edge), and as the "finer"/"coarser" side of `check_tolerance_containment` |
| `vertex-{id}` | `check_tolerance_containment` (tolerance-containment-violated, vertex side) |
| `coedge-{id}` | `check_missing_pcurves` (missing-pcurve), `check_same_parameter` (same-parameter-violated) |
| `shell-{shell_id}-edge-{edge_id}` | `check_shell_closure_and_orientation` (shell-not-closed, orientation-inconsistent) |
| `solid-{id}` | `check_solid_orientation` (shell-orientation-inward) |
| `solid-{id}-void-shell-{shell_id}` | `check_solid_orientation` (void-shell-not-inverted) |
| `face-{id}` | `check_degenerate_geometry` (sliver-face) |
| `face-{id}-face-{id}` | `check_self_intersection_probe` (warning-possible-self-intersection) — pairs reconstructed with the same `solid_faces(solid_id)` ordering and `i < j` nesting the real check uses |

Also updated the file's import list to bring in `ValidationIssue` from
`snapshot::error` (needed by the new helpers' signatures).

## Verification

```
cd /Users/ueli/Documents/semio && RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --lib --target wasm32-wasip2 --keep-going
```

Run in the foreground; log captured to scratchpad, `grep -cE '^error'` → `0`, exit code `0`.

Final log lines (verbatim):

```
warning: `semio-s-plugin-stdio` (lib) generated 1464 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 1431 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3m 33s
```

`EXIT_CODE=0` (captured immediately after the `cargo check` invocation).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs`
  — added `ValidationIssue` to the existing `snapshot::error` import, added
  `issues_scoped_to_new_solids` and `pre_existing_entity_strings`.
