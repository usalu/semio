# POLISH — status

Agent: POLISH. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Scope: consistency polish of the kernel and the charts/matrix/table/hierarchy/flow/geo namespaces,
plus the loader `semio-viz.sty`, the token generator and the schema entries of those families.

Owned files (52 `.sty` + loader): kernel `data transform scale format theme mark shape coordinate
guide annotation label facet composition plot family probe layout hierarchy network flow geo
spatial`; namespaces `charts-* matrix-* table hierarchy-* flow-* geo-*`; loader `semio-viz.sty`.
Not owned (findings recorded in `📓️integration.md` under `## From POLISH`):
`semio-viz-showcase.sty` (FAMILIES-CAPABILITY), `semio-viz-text/domain/diagram-*/scientific-*/
network-*/infographic/interactionstate` (FAMILIES-DOMAIN), tests, docs.

---

## 0. Baseline inventory (measured)

`🔧️polish-inventory.py` over the 52 owned files.

| finding | at start (05:35) | now (session 2) |
|---|---|---|
| definitions without an emoji docstring | 1064 | **1053** |
| legacy `\semio@…` tokens (excl. `\semio@stroke@…`) | 72 in 15 files | **0 in owned files** |
| direct `semio-primary`/… colours | 74 in 16 files | **6, all inside `semio-viz-theme.sty`'s own palette definitions (correct)** |
| `%region 🔖️` blocks | balanced | balanced, every name carries 🔖️ |
| `\fontsize` re-declarations in viz packages | present | **0** (only `semio-viz-theme.sty`'s five `\c_semio_viz_theme_font_*_tl` constants) |

Per-file table: `🗑️generated/POLISH/inventory.json`.

Note: the ticket's `🗑️generated/POLISH/` folder was swept by another agent around 06:15;
it has been recreated and the inventory regenerated.

---

## 1. Landed this session

### Legacy tokens (audit class 2) — owned files now clean
- `semio-viz.sty`: the last five `\semio@viz@font@title` / `\semio@viz@font@label` uses replaced by
  `\c_semio_viz_theme_font_title_tl` / `\c_semio_viz_theme_font_label_tl` (5 sites, plus the prose
  reference in the `\semio_viz_column_note_draw:` docstring).
- The frame/section metric constants added earlier are in place and used:
  `\c_semio_viz_frame_width_tl {80}`, `\c_semio_viz_frame_height_tl {40}`,
  `\c_semio_viz_section_gap_tl {2.5mm}`, `\c_semio_viz_section_content_gap_tl {4.5mm}`,
  `\c_semio_viz_section_note_gap_tl {4mm}` in `semio-viz-guide.sty`, read by `semio-viz.sty`.
- The `\semio@viz@font@*` `\providecommand`s that survive are **not** in owned files — see the
  request to FAMILIES-DOMAIN in `📓️integration.md`.

### INTEGRATION-2's cyclic-ramp note (audit class 8)
- `semio-viz-theme.sty` gained the expandable twin of the palette-width accessor:

  ```
  \cs_new:Npn \semio_viz_theme_color_count:
    { \bool_if:NTF \l_semio_viz_theme_grayscale_bool
        { \clist_count:N \c_semio_viz_theme_gray_clist }
        { \str_if_eq:VnTF \l_semio_viz_theme_palette_tl { brand }
            { \clist_count:N \c_semio_viz_theme_brand_clist }
            { \int_use:N \c_semio_viz_theme_presence_int } } }
  ```

- `\semio_viz_theme_ramp_cyclic:n` now divides the wheel by `\semio_viz_theme_color_count:`
  instead of the hard-coded `\c_semio_viz_theme_wheel_int {7}`, and that constant is deleted
  (no occurrences remain anywhere in `🖋️latex/`). A presence palette therefore turns through its
  own twelve hues; brand and grayscale palettes keep the seven they had, so only the
  presence-palette phase maps change, which is the defect INTEGRATION-2 reported.

### Performance (audit class 8) — pack kernel
Profiled with XeTeX's own `\resettimer` / `\elapsedtime` inside one compile, so the number is TeX
work and not process start-up or the machine's concurrent tectonic load. Benchmark
(`🗑️generated/POLISH/bench.tex`): the seven pack layouts of the committed
`hierarchy-pack.tex` fixture, run five times = 35 pack layouts.

| build | `packtime` (scaled sec) | seconds | per layout |
|---|---|---|---|
| before | `983040`, `983040` | 15.0 s | 429 ms |
| after | `851968`, `851968`, `917504`, `786432` | 13.0 s (mean) | 371 ms |

**−13 %**, and the probe output is byte-identical: `hierarchy-pack.probe.jsonl`
`md5 = f66d1c6f4f738696b982c6a98e8b2847` before and after.

The change is in the innermost operation of every hierarchy layout,
`\semio_viz_hierarchy_get:nnn`: `\cs_if_exist_use:cF` builds the same csname **twice** (once for
the existence test, once for the use). It now builds it once and hands the token to a new
`\semio_viz_hierarchy_read:N`, which reads `\scan_stop:` (what `\cs:w…\cs_end:` leaves for an
undefined name) as the documented `0`. `\semio_viz_hierarchy_index:n` got the same treatment and
its missing docstring.

Whole-document wall clock for the fixture, same machine: package load alone (an empty
`\documentclass{article}\usepackage{semio-viz-hierarchy}`) is **3.1 s**, the whole fixture **8.2 s**
— i.e. the seven pack layouts are ~5.1 s of it and the LaTeX bundle load is the other ~3.1 s.

### `hierarchy-pack` and the 30 s quick budget
Measured cause, for TESTS-2 (numbers in `📓️integration.md`): the case has **four** scenarios and
each one calls `compileVizProbe` on the *same* committed fixture, so tectonic runs four times:
4 × 8.2 s ≈ 33 s against a 30 s per-adapter budget, before any concurrent load. The kernel
optimisation takes that to ≈ 4 × 7.5 s ≈ 30 s — still on the line.

`compileVizProbe` already has a per-process cache (`COMPILED_PROBES`), but
`probeCompilationKey` mixes `options.workDir` into the key and the adapter's `ctx.workDir` is
scenario-specific, so the four scenarios never share the one compile they could. Dropping
`workDir` from that key (or letting the adapter pass a case-level work directory) turns the case
into a single ~8 s compile — five to six times under budget, with no test weakened and no scenario
moved to `long`. That file is TESTS-2's; the request is recorded in `📓️integration.md`.

### Docstrings (audit class 1) — the count was wrong, and the kernel is now clean
`🔧️polish-inventory.py` counted a definition as undocumented unless the **immediately** preceding
line carried an emoji. That over-counted badly: a docstring that runs over several lines carries
its emoji only on the first, a `\tl_new:N` or `\cs_generate_variant:Nn` line often sits between the
note and the definition it documents, and a `\keys_define:nn` nested **inside** another
definition's body was counted too — although a comment there would break the "no comments inside
definitions" rule. The checker now walks the whole comment block upwards, steps over blank lines,
variable declarations and variant lines, stops at another definition (every definition still needs
its own note) and only matches definitions at column 0.

| measure | count |
|---|---|
| naive check, start of session | 1064 |
| naive check, now | 1053 |
| **corrected check, now** | **712** |
| owned files with zero gaps | **23 of 53** |

Fixed this session (every one authored, none templated): `semio-viz-family.sty`,
`semio-viz-probe.sty`, `semio-viz-theme.sty`, `semio-viz-scale.sty`, `semio-viz-format.sty`,
`semio-viz-coordinate.sty`, `semio-viz-facet.sty`, `semio-viz-flow-alluvial.sty`,
`semio-viz-flow-parallelsets.sty`, `semio-viz-hierarchy-tree.sty`,
`semio-viz-hierarchy-partition.sty` — all clean. Two related fixes on the way through:
`semio-viz-format.sty` had a `\tl_if_exist:NF` line wedged between a docstring and its definition
(moved above the note), and `semio_viz_coord_parallel:nNN`'s note opened with `∥`, a maths symbol
and not an emoji.

### Unread keys (audit class 4) — three deleted
A key that is declared, defaulted and never read is worse than no key: it accepts a value and
silently ignores it. Three of those in POLISH-owned kernel files were deleted with their variables,
their resets and their key documentation, none of them named in `🧬️schema/🔣️.json`:

- `semio / viz / theme` → `font` (defaulted to `sans`, read nowhere; the five
  `\c_semio_viz_theme_font_*_tl` constants are `\tl_const:Nn`, so a live switch would be a real
  refactor and not a key binding).
- `semio / viz / grid` → `style` (a grid is drawn by handing fixed keys to
  `\semio_viz_guide_axis_draw:n`, which has no free-form style key to pass it to).
- `semio / viz / legend` → `anchor` (documented as the TikZ anchor of `at`, but the legend places
  its title and every entry from `at` as the top-left corner; honouring another anchor needs the
  block's height measured *before* it is drawn).

The last two are worth having for real. Whoever implements them should add the key back beside the
implementation, not before it.

---

## 2. Decisions

- Docstrings are inserted by script, which **only inserts comment lines** and never rewrites an
  existing line. Every docstring text is authored per definition, not templated.
- TeX is written only with the Write/Edit tools. (Confirmed again this session: on this machine
  Python and the shell cannot *open for writing* a path whose directories contain emoji —
  `OSError: [Errno 22]` / `No such file or directory` — while reading the very same path works.)
- The six direct `semio-…` colour names left in `semio-viz-theme.sty` are the palette's own
  definition (`\c_semio_viz_theme_brand_clist`, the semantic role table); they are the accessor,
  not a bypass of it, and stay.

---

## 3. Verified runs

`bun ./📜️script.ts build viz api` — green after each file (loader rewire, theme twin, hierarchy
kernel):

The closing run, after the last edit of the session, exit code 0:

```
[DEBUG] print built 🧰️framework\🛍️products\📓️print\📦️packages\🟦️typescript\dist\viz-api.pdf
[DEBUG] print built 🧰️framework\🛍️products\📓️print\📦️packages\🟦️typescript\dist\viz-api.pdf
[DEBUG] print built 🧰️framework\🛍️products\📓️print\📦️packages\🟦️typescript\dist\viz-api-dark.pdf
[DEBUG] print built 🧰️framework\🛍️products\📓️print\📦️packages\🟦️typescript\dist\viz-api-dark.pdf

[exited with code 0]
```

One caution for whoever reads a build log next: run `build viz api` **without** piping it through
`tail`. `tail` buffers the whole stream, so a background run looks frozen with an empty output file
for its entire twenty minutes; a concurrent run additionally dies with
`EBUSY: resource busy or locked, rm …dist\source\viz-api` — the collision INTEGRATION-2 already
described, and it clears on a retry.

`bun ./📜️script.ts test fundamental`, run again after the last edit of the session:

```
[test] level=fundamental cases=100 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=98
```

`parity quick --owner 🧰️framework/🛍️products/📓️print`, every case that exercises a package changed
here:

```
--case 🫧️hierarchy-pack        [test] level=quick cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
--case 🌳️hierarchy-aggregates  [test] level=quick cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
--case 🌴️hierarchy-tree-cluster[test] level=quick cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
--case 🎨️scale-color           [test] level=quick cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6
--case 📏️guide-axis-ticks      [test] level=quick cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
```

A local load check of every package edited this session, compiled with the repo tectonic against
`🖋️latex/` (`🗑️generated/POLISH/`): `semio-viz-theme scale format coordinate guide facet probe
family flow-alluvial flow-parallelsets hierarchy-tree hierarchy-partition` in one `article`
preamble — `note: Writing .\bundle.pdf`, no errors.

Pack probe, byte-identical before and after the kernel optimisation:

```
f66d1c6f4f738696b982c6a98e8b2847 *hierarchy-pack.probe.jsonl
```

Pack benchmark (35 layouts, TeX-internal timer, 65536 = 1 s):

```
before: packtime=983040  packtime=983040
after:  packtime=851968  packtime=851968  packtime=917504  packtime=786432
```

Note for the fleet: `⚡️cache/print-fonts` was deleted around 06:15 (same sweep that removed the
`🗑️generated/` folders). Every tectonic compile then failed with
`Package fontspec Error: The font "Anta-Regular" cannot be found.` until the next build rebuilt
the cache — that error is not a LaTeX defect.

---

## 4. Files touched

LaTeX (all through Write/Edit, never a shell heredoc):

- `🖋️latex/semio-viz.sty` — last legacy font tokens → theme constants.
- `🖋️latex/semio-viz-theme.sty` — expandable `\semio_viz_theme_color_count:`; cyclic ramp uses it;
  `\c_semio_viz_theme_wheel_int` deleted; unread `font` key deleted; docstring on
  `\semio_viz_theme_pattern:nN`.
- `🖋️latex/semio-viz-hierarchy.sty` — single-construction `\semio_viz_hierarchy_get:nnn` plus
  `\semio_viz_hierarchy_read:N`; `\semio_viz_hierarchy_index:n` likewise, with its docstring.
- `🖋️latex/semio-viz-guide.sty` — unread `grid/style` and `legend/anchor` keys deleted with their
  variables, resets and key documentation.
- `🖋️latex/semio-viz-scale.sty`, `-format.sty`, `-coordinate.sty`, `-facet.sty`, `-family.sty`,
  `-probe.sty`, `-flow-alluvial.sty`, `-flow-parallelsets.sty`, `-hierarchy-tree.sty`,
  `-hierarchy-partition.sty` — docstrings; each of these files is now gap-free.

Ticket:

- `📓️integration.md` — `## From POLISH`: the FAMILIES-DOMAIN legacy-token table, the measured
  `hierarchy-pack` budget analysis for TESTS-2, and two environment traps for everyone.
- `🔧️polish-inventory.py`, `🔧️polish-sites.py` — the docstring check corrected (see above).
- `🗑️generated/POLISH/` — `inventory.json`, `bench.tex` (the pack benchmark).

## 5. Open — what a successor should pick up, in this order

1. **Docstrings, 712 gaps in 30 files.** The corrected `🔧️polish-inventory.py` /
   `🔧️polish-sites.py` give the exact sites. The mass is in six files —
   `semio-viz-mark.sty` 89, `-charts-bar.sty` 65, `-network.sty` 64, `-geo.sty` 59,
   `-spatial.sty` 50, `-data.sty` 40 — and the other 24 files hold 5 to 33 each. Each note has to
   be authored against the definition it sits above; there is no templating shortcut, and the
   Write/Edit tools are the only way to write these files on this machine.
2. **Unread keys still open in owned files** (implement or delete, and the schema with them):
   `annotation label`; `charts-distribution kernel`; `charts-funnel orient`; `charts-polar` (11:
   `angle party part part node parent label curve ticks label hulls`); `charts-statistical` (5:
   `orient timecol smooth split shared`); `composition name`; `data value`;
   `geo-choropleth dasymetric iterations`; `hierarchy shape`;
   `matrix-correlation labels density levels`; `matrix-heatmap sparse`;
   `network distance-max coordinate`; `plot facet`. Several of these are family options that *are*
   in `🧬️schema/🔣️.json`, so deleting one means deleting its schema entry in the same change —
   which is why they were not swept in bulk here.
3. **`semio-viz-hierarchy.sty` split** into `-hierarchy`, `-hierarchy-tiling`,
   `-hierarchy-packing` (requires inside hierarchy, loader untouched). Not started: the file is
   3 147 lines and currently correct, and a split has no cheap verification beyond a full
   `parity quick` of every hierarchy case. Do it when the fleet is not compiling in parallel.
4. **Hard-coded loop counts → keys**; the `%region 🔖️Keys` block of `semio-viz-composition.sty`;
   the loader/data region layout. Not started.

Nothing in 1–4 is blocking: the bundle loads, `build viz api` is green, `test fundamental` is
green at `parity=7/7`, and the four parity cases that touch the packages changed here all pass.
