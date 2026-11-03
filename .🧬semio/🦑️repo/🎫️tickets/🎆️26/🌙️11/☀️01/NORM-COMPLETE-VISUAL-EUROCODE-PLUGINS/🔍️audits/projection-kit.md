# Projection kit: shared scene helpers

Ticket: `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`. Not compiled by the author (shared crates belong to the integrator, see `requests.md`); every edit was a mechanical, name-preserving switch to re-exports so call sites did not change.

## New module

`✏️s/🔌️plugins/📕️norm/🎬️structural-scene/🧰️projection-kit` (`scene::projection_kit`), Rust only so far:

| Submodule | Moved items |
|---|---|
| `📐️vectors` | `v3 add mul unit along member_frame world_frame at rotate_yaw polyline ring3 polygon3 polygon2`, `solid::box_from_corner` |
| `🆔️identity` | `loc base_id entity_ref` |
| `📶️utilisation` | `status` (one table: unverified basis is a warning), `worst_where binding_where binding_of_check binding_of_subjects worst_of_subject binding_of_subject check_with clause_of tone_of utilisation_words` |
| `📍️placement` | `Placement` (was copied 10 times), `placed` |
| `🕹️handle-kit` | `grid round field_binding identity_binding HandleSpec handle` (was `Spec`+`handle` in 6 crates) |
| `🏷️legends` | `used_layers layers utilisation_legend role_legend` (the `Out::finish` tail and role-legend loop of all 10 crates) |
| `📈️diagram-parts` | `axis stroke column` |
| `🧪️test-support` | `revision entity assert_scene_laws assert_deterministic assert_diagram_laws` |

Tests: `🧪️tests/🥒️.feature`, `🧪️tests/🔬️unit/🦀️.rs`, oracle `🧪️tests/🔮️oracle/🐍️.py` (stdlib; cross-checks with numpy when installed). Dependency change: `📦️packages/🦀️rust/Cargo.toml` adds `semio-s-plugin-norm-compliance` as a normal dependency (no cycle: compliance does not depend on the scene crate).

## Conventions resolved

- **Copy mismatch**: `SceneRevision` (two `u64`) now `impl Copy` in `🧬️schema/🧩️entities/🦀️.rs`; the `#[derive(Clone, Copy)] SceneInput` of EN 1991/1992/1996/1997/1998 therefore compiles, EN 1990/1993/1994/1995/1999 stay `Clone` (they hold nothing that forces it).
- **Box origin**: convention is the **centre of the base** on the frame origin (`±size/2` across, `0..size.z` up), as the viewport (`viewport3d/🧱️parts`) always drew it. Fixed the tessellator (`🧮️tessellation/🪨️solids`: `cuboid` was corner-based) and documented on `SolidShape`. Callers that thought in corners now use `solid::box_from_corner`: EN 1995 (`🪵️members` bearing and notch, `🔩️connections` timber layers) and EN 1998 (`box_solid`, drift bars). `Prism` keeps frame-local polygon coordinates by design. New test `tessellated_boxes_and_viewport_share_the_base_centre_convention`.

## Per-crate changes (all in `…/✳️any/🎬️scene/`)

| Crate | `🧰️kit` | `📍️layout` | `🕹️handles` | `📈️diagrams` | `🧪️tests` |
|---|---|---|---|---|---|
| EN 1990 | vectors, ids, `binding`, `clause_of`, `tone_of`, `utilisation_words`, layers/legends/role legend | `Placement`, `rotate` | `grid binding Spec handle`, `write:`→`writes:` | `column stroke`, `axis`→shared | `revision entity` |
| EN 1991 | same (keeps its `@`-path `entity_ref`, German `utilisation_words`, `unit`) | `Placement` | untouched (own `Grip`) | `column stroke axis` | `revision entity` |
| EN 1992 | same (+`at ring3`) | `Placement rotate` | untouched (own `Grip`) | `column`, `axis` wrapper | `revision entity` |
| EN 1993 | same | `Placement rotate` | all four helpers | `column stroke axis` | `revision entity` |
| EN 1994 | same | `Placement` | all helpers | `column stroke`, `axis` wrapper | `revision` |
| EN 1995 | same, `binding_of/worst` now wrap shared | `Placement rotate` | helpers, `write` wraps `field_binding` | `column stroke` | `revision entity` + Box fixes |
| EN 1996 | same (keeps tuple `polygon2`) | `Placement` | helpers | `column stroke`, `axis` wrapper | `revision entity` |
| EN 1997 | same | `Placement` | untouched (single `handles` fn) | `column`, `axis` wrapper | untouched |
| EN 1998 | same (+`world_frame`, `box_solid` fix) | `Placement` | untouched (own `Grip`) | `column stroke`, local `loc` removed | untouched |
| EN 1999 | same; `to_scene_status` is now an alias of `status` (fixes the missing `UnverifiedBasis` arm in EN 1995/1999) | `Placement rotate` | helpers (already `writes`) | `column stroke axis` | `revision entity` |

Effect: about 60 duplicated function bodies and 10 copies of `Placement`, `Spec`/`handle` (x6) and the layer/legend/role-legend loop collapsed into one place; the artifacts keep only the layer tables, role label tables, entity-kind registries and dependency maps (Eurocode knowledge).

## Not moved (and why)

- `Spec`/`Out` entity builders: nine variants (ghost, opacity, hatch, channel, nested path, bond budget); a unified builder needs a golden-visible decision on `Ghost`-vs-`Error` precedence (EN 1990 vs EN 1992/1994). Candidate next step: one `EntityDraft` with `style: Option<StyleState>`.
- EN 1992/1998 `Grip` (derived affine limits) and EN 1997 handles: different parameterisation than `HandleSpec`.
- Language-neutral twins of the kit (proto/graphql/ts) are not applicable: the kit has no wire types; only Rust exists.
- German decimal comma in `utilisation_words` (EN 1991 only) should become the shared behaviour once goldens are regenerated.
- `entity_ref` of EN 1991 (`@` prefix), 1992, 1996, 1998 (explicit path) stay local.
