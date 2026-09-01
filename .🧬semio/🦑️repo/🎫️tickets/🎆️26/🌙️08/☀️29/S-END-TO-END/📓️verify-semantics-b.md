# Semantic Verification — Batch B (stdio artifact mutation migration)

Read-only verification of the `impl protocol::Mutation` → `#[derive(dsl::Mutations)]` migration for
the 18 artifact/subset targets assigned to this batch. No files were edited, no cargo was run, no
mutating git command was run.

## Provenance note (important)

The literal recipe `git show HEAD:"<path>"` does **not** surface the pre-migration code for any of
these 18 files: this is a live, concurrently-committed repo, and by the time verification ran, HEAD
(`67fb4216b2`, commit tag `🚩️579`) already contained the post-migration `agg_diff`/`agg_inverse` code
for every target, with the working tree identical to HEAD (no uncommitted diff).

The migration itself is entirely contained in commit `67fb4216b2` (parent `f7b265d581`, tag `🚩️578`).
That single commit renamed each artifact's aggregate mutation file from `🦀️component.rs` to `🦀️.rs`
in the same `🧬️mutations/` directory (for `docx` the containing subset directory was also renamed
`✳️any` → `✳️base`), while rewriting `impl protocol::Mutation { fn diff / fn inverse }` into free
functions `agg_diff`/`agg_inverse`. The correct pre-migration baseline for every file below is
`git show f7b265d581:".../🦀️component.rs"` (i.e. one commit before HEAD, under the old filename).

## Verdict table

| # | Artifact / subset | Variants checked | Verdict | Notes |
|---|---|---|---|---|
| 1 | pptx (ecma-376/any) | 8 (+NoMutation retired) | **CLEAN** | 16/16 arms (diff+inverse) byte-identical |
| 2 | xlsx (ecma-376/any) | 9 (+NoMutation retired) | **CLEAN** | 18/18 arms byte-identical |
| 3 | docx (ecma-376/base) | 12 (+NoMutation retired) | **CLEAN** | 24/24 arms byte-identical; subset dir renamed any→base in same commit |
| 4 | mp4 (isobmff/any) | 9 (+NoMutation retired) | **CLEAN** | |
| 5 | avi (1.0/any) | 12 (+NoMutation retired) | **CLEAN** | |
| 6 | wav (riff-pcm/any) | 4 (+NoMutation retired) | **CLEAN** | inverse syntactically reshuffled (`vec![match ...]`), RHS unchanged |
| 7 | mp3 (mpeg1-layer3/any) | 4 (+NoMutation retired) | **CLEAN** | same benign reshuffle as wav |
| 8 | ifc 2x3 (2x3/any) | 4 (+NoMutation retired) | **CLEAN** | |
| 9 | ifc 4 (4/any) | 10 (+NoMutation retired) | **CLEAN** | |
| 10 | bcf (2.1/any) | 13 (+NoMutation retired) | **CLEAN** | |
| 11 | step (ap214/any) | 10 (+NoMutation retired) | **CLEAN** | `StepDiff::default()` no-op branches pre-existing, untouched |
| 12 | tsv (iana/any) | 6 (+NoMutation retired) | **CLEAN** | |
| 13 | json i-json (rfc8259/i-json) | 9 (+NoMutation retired) | **CLEAN** | `agg_diff` delegates unchanged to `lower()`; `lower()` arms verified |
| 14 | dwg (ac1024/any) | 2 (+NoMutation retired) | **CLEAN** | ac1018's mutations.rs is a 12-line stub/reexport, correctly not the target |
| 15 | dxf (r12/any) | 18 (+NoMutation retired) | **CLEAN** | largest vocabulary target; 36/36 arms (diff+inverse) byte-identical; binary tags renumbered -1, consistent with dropping variant 0 |
| 16 | obj (3.0/any) | 21 (+NoMutation retired) | **3 REGRESSIONS (keyword-only)** | see below — all 21 `diff`/`inverse` arm bodies PASS byte-for-byte; failures are in `#[dsl(keyword=...)]` wire strings, independent of the diff/inverse logic |
| 17 | svg basic (1.1/basic) | 10 (+NoMutation retired) | **CLEAN** | |
| 18 | svg tiny (1.1/tiny) | 9 (+NoMutation retired) | **CLEAN** | `StripNonTiny` is a genuine non-NoMutation unit-payload variant, correctly re-wrapped |

**17 of 18 artifacts verified clean. 1 artifact (obj) has 3 confirmed regressions, all in wire-format
keyword strings, not in mutation semantics.**

Across every clean artifact, the only systematic differences from the pre-migration code are the two
explicitly sanctioned classes: (1) match-arm pattern heads rewritten from bare-field `E::V{a,b}` to
`E::V(v_mod::V{a,b})`, required by the `#[derive(dsl::Mutations)]` shape, and (2) every
`None => vec![E::NoMutation]` fallback rewritten to `None => Vec::new()` (plus the direct consequence
of that: binary/discriminant tags shifted down by 1 where `NoMutation` used to occupy tag 0). No
index changed, no `.clone()` dropped, no comparison flipped, no field reordered with altered meaning,
no `base`/`self` swap, and no variant silently dropped or invented, in any of the 17 clean artifacts.

## REGRESSION detail — obj (3.0/any), wire keyword mismatch

All 21 `agg_diff`/`agg_inverse` arm bodies for `obj` are byte-identical to the pre-migration
`fn diff`/`fn inverse` bodies (verified against `git show f7b265d581:".../🧊️obj/.../🧬️mutations/🦀️component.rs"`).
The regression is separate: it's in the per-leaf `#[dsl(keyword = "...")]` wire-format attribute,
which the task flagged as needing an extra cross-check against the committed grammar because `obj`
kept `dsl::DslOps` (rather than moving to hand-rolled serde_json like `deflate`/`binary`) and gained
`dsl::DslRecord` per leaf.

During the migration, the Rust variant identifiers `InsertTexCoord` / `RemoveTexCoord` / `SetTexCoord`
were renamed to `InsertTexcoord` / `RemoveTexcoord` / `SetTexcoord` (capital `C` → lowercase `c` in
`TexCoord`/`Texcoord`). The new leaf files' explicit keyword attributes were set to match the **new**
spelling's naive kebab-case, but the committed, pinned wire grammar — itself edited by this same
migration commit to update an unrelated doc-comment example, but **not** to update the actual
keyword productions — still specifies the **old** spelling with a hyphen inside "tex-coord":

- `.../🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio:50-52`
  (unchanged since before the migration, confirmed via `git diff f7b265d581 67fb4216b2 -- <grammar file>`):
  ```
  insert-tex-coord = "insert-tex-coord" "index" "=" INT texcoord-block
  remove-tex-coord = "remove-tex-coord" "index" "=" INT
  set-tex-coord = "set-tex-coord" "index" "=" INT texcoord-block
  ```
  and `op = ... | insert-tex-coord | remove-tex-coord | set-tex-coord | ...` (line 20).

- New Rust leaf attributes (verified directly on disk):
  - `.../🧬️mutations/🧷insert-tex-coord/🦀️.rs:16` → `#[dsl(keyword = "insert-texcoord")]`
  - `.../🧬️mutations/🚮remove-tex-coord/🦀️.rs:16` → `#[dsl(keyword = "remove-texcoord")]`
  - `.../🧬️mutations/🧭set-tex-coord/🦀️.rs:16` → `#[dsl(keyword = "set-texcoord")]`

So the code now emits/expects `"insert-texcoord ..."` / `"remove-texcoord ..."` / `"set-texcoord ..."`
(no hyphen between `tex` and `coord`), while the committed `.grammar.semio` (and, by the conformance
laws that pin it, the DSL text-protocol wire format) still expects `"insert-tex-coord ..."` etc. with
the hyphen. This is a real behavioral regression: it compiles (nothing type-checks a `#[dsl(keyword)]`
string against the committed grammar file), but round-tripping through the DSL text codec for these
three ops now diverges from the pinned format — a previously-valid `"insert-tex-coord ..."` line no
longer parses, and freshly-printed ops no longer match the committed grammar/conformance fixtures.

**Affected variants**: `InsertTexcoord`, `RemoveTexcoord`, `SetTexcoord` (all under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`).

All other 18 obj variants' keywords (`set-snapshot`, `insert-vertex`, `remove-vertex`, `set-vertex`,
`insert-normal`, `remove-normal`, `set-normal`, `insert-face`, `remove-face`, `set-face`, `set-group`,
`remove-group`, `set-object`, `remove-object`, `set-mtllib`, `set-usemtl`, `set-smoothing-groups`,
`set-unknown-statements`) match the grammar exactly — MATCH, no regression.

## Method used

Per artifact: fetched the pre-migration file via `git show f7b265d581:".../🦀️component.rs"`, located
the old `impl protocol::Mutation { fn diff / fn inverse }` match arms, located the new
`agg_diff`/`agg_inverse` match arms in the current `.../🦀️.rs`, and diffed each variant's arm body
(everything after `=>`) character-for-character, ignoring only whitespace/indentation. Pattern-head
rewriting (`E::V{a,b}` → `E::V(v_mod::V{a,b})`) was the only allowed head change. `vec![E::NoMutation]`
→ `Vec::new()` was the only allowed benign body change, along with the binary-tag renumbering that
directly follows from dropping variant 0. For `obj`, additionally cross-checked every leaf's
`#[dsl(keyword = "...")]` against the committed `component.grammar.semio` production for that op.

This was executed via seven parallel read-only verification subagents (one each for: dxf; obj;
pptx+xlsx+docx; mp4+avi+wav+mp3+bcf+step; ifc-2x3+ifc-4+tsv+json-i-json+dwg+svg-basic+svg-tiny), with
dxf and obj each re-run once after the initial `git show HEAD:<path>` recipe returned an
already-migrated file (the `🦀️component.rs`→`🦀️.rs` rename inside the migration commit was not
apparent until the media-format group's agent discovered it independently). The obj keyword
regression was independently re-verified directly against the source files (grammar file and leaf
`.rs` attributes) before being recorded here.
