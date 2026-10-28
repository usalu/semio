# Strict Layout Check for the Keyframe Captures

Scope: the twelve teaching clips captured by `capture.py` in `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES` (6 optimization scenes) and `CONCRETE-BIM-CLIP-EXAMPLES` (6 BIM scenes). No scene, clip or shared toolkit file was edited. New and changed files are ticket-local only.

## 1. Root cause: why the reports said "none"

Probe: `probe-layout.py` (census of every mobject per kept frame), run on Pareto `objectives_2` and `walk_2`.

1. **Live labels were invisible to the shared check (the cause).** Every chip, value tag, dimension label and number in the optimization clips is a `LiveText` (room module). It is a `Text` subclass whose own `VMobject` had fill opacity 0; the visible ink lived in a child shape. `_visible_texts` skips texts with `get_fill_opacity() < 0.05`, so in `objectives_2` it saw 14 of 27 texts (13 `LiveText` dropped). All text-vs-text, line-through-text and fill-under-text tests then ran only on plain `Text` (axis titles, ticks, caption) and never on the labels that collide. Another agent has since changed `LiveText` to carry fill opacity 1; after that the shared check finds many collisions (tally in section 4).
2. **Ruled out** (measured): scene mobjects are all reachable through `scene.mobjects` families (`VGroup`, `Group`, `LiveGroup`, `Axes`); `LiveGroup` and updater counts are identical before and after `scene.update_mobjects(0)`, so the capture running without refreshed updaters changes nothing; no text is drawn as `ImageMobject`; content labels carry no `_layout_zone` (only the caption does), so zones do not swallow anything; label roots are per `Text`, not whole panels.
3. **Remaining blind spots of the shared check even with visible labels:** a label's own chip frame is a stroke, not text, so a chip border sitting on an axis or crossed by sun rays is invisible to it; there is no frame-margin, no text-size and no near-touch test; draw order is ignored (a ray hidden under an opaque chip counts the same as a visible one).

## 2. What the strict check flags

`layout_strict.py` (identical copy next to both `capture.py`; `strict_conflicts(scene)`), appended to every keyframe in `…-layout.md` under `### strict` plus a `strict findings: N in M of K frames` line at the top. If the strict check ever raises, the frame section says so and the capture continues.

Labels are read from glyph ink (opacity taken from the glyph children, tight box from outline anchors), shapes from every non-glyph `VMobject` and `ImageMobject`, in draw order (z-index then family order). Boxes are converted to 1280 x 720 pixel spans in every message.

| finding | rule |
| --- | --- |
| `text over text` | two different labels whose ink boxes overlap by at least 3 px on both axes (caption lines among themselves exempt) |
| `stroke through` | a stroke sample (every 0.02 units along the bezier) inside the label box shrunk by 1 px; thin filled rectangles (dash segments) count as strokes; also an enclosing non-chip frame that hugs one side within 4 px while the opposite side is looser (row highlights with equal padding are not flagged) |
| `filled shape under` | a shape with fill opacity at least 0.25 covering part, but not all, of the label (point-in-polygon on a 9 x 3 grid); fully covered means backing and is fine |
| `stroke through chip` / `text crosses chip border` / `chips overlap` | the label's chip is the smallest shape that encloses it with at most 0.45 units padding; strokes that enter and leave the chip, other labels that cross its border, and chips overlapping chips. Strokes that end inside the chip (leaders), the chip itself, shapes inside it and badges of at most 2 characters are skipped |
| hidden by backing | a stroke or fill is ignored where an opaque backing (fill at least 0.85, containing the label, drawn before it) is drawn after that stroke or fill |
| `outside frame`, `near frame edge` | label ink box beyond the 16:9 frame, or closer than 12 px |
| `in caption band` | non-caption label below y = -2.62 (the toolkit's `CAPTION_BAND`) |
| `unreadable text` | estimated cap height below 11 px (box height corrected for x-height-only and descender labels), one aggregated line per frame; pure symbols such as `=` are skipped |

## 3. Validation on known-bad frames

All frames below are from the current scene sources, which other agents keep changing, so positions differ from the audit pixel numbers.

| audit item | frame | strict result |
| --- | --- | --- |
| optimization audit: labels on plot lines, `-279` label on orange and dashed line | experiments `interaction`, `saving_70` | `stroke through '-278.9 kWh/a' x Line`, `'-122.3 kWh/a' x Line; DashedLine`, `'if parallel' x Line` |
| tags touching highlight frames (D20) | experiments `response_2`, `main_2` | `stroke through 'sun heat 245 kWh/a' x Rectangle (frame within 4px)` |
| text over text | experiments `interaction` (title chip over axis title), `purpose_3` (`+1` over `full plan: 2^3 ...`, 5 pairs); pareto `knee_3` (`extra cost 427 CHF` over `saved 20.4 CHF/a`, 94 x 10 px) | all reported with pixel spans |
| rays through dimension chips (D22) | pareto `walk`, `walk_2`, `third_3`; experiments `question` | `stroke through chip of 'WWR 30 %' x Arrow`, `'overhang 0.5 m' x VMobject` (ray bundle clips the chip corner) |
| chip on a plot object | pareto `walk_2` | `stroke through chip of '-65 kWh/a' x Circle` (ring marker under the chip) |
| title against chip (D10, de) | pareto de `payback`, `knee_3` | `text over text 'Mehrkosten 192 CHF' x 'Ersparnis 16,2 CHF/a' by 112 x 8 px`, line through both labels |
| edge distance (D24) | pareto `third`, `third_3` | `near frame edge (1px) '2.4 %'`, German BIM/optimization labels at x = -1 px (`outside frame`) |
| BIM `members` | models `members` | the audit's overprint is already fixed in the scene; strict still finds dimension lines through the `1.25` and `4.75` labels |

Clean-looking frames (all inspected as PNG): pareto `decision` (0 to 1 findings, a 10.5 px legend text, depending on the scene revision), experiments `factors_2` (1: the top-right note 10 px from the frame edge), BIM `Beat4 schema`, `Beat3 stored`, `Beat3 elevation`, `copies`, `identity`, `Beat4 whole`, `architecture`, `loads`, `loss` (0 findings). The visible defects left in these frames (for example `3.00 m*` hugging the building in `stored`) are sub-4 px near touches that the thresholds deliberately ignore.

## 4. Tally over the full captures at the time of writing (`tally-strict.py`; scene sources keep changing, so rerun it)

Frames with no strict finding, of all frames: Extrema en 4/29, Sampling en 21/33, Experiments en 0/35, Index en 0/42, Pareto en 20/32, Algorithms en 2/43; BIM Information 13/21, Objects 9/19, Representations 5/10, Models 12/19, Exchange 8/16, Consistency 12/20. Several findings repeat in every frame of a scene because they come from one static widget (the sun-ray bundle clipping the `WWR` chip, the 10.4 px `Total` row in the bar legend, a 12 px right-edge margin); that is the same defect seen again, not noise. Residual risk of false positives: ray bundles partly hidden by an opaque chip that is drawn earlier than the rays, and tightly padded chips with more than 2 characters (reported as `stroke through chip` only when a foreign stroke enters them).

`German` reports of scenes that currently fail in the capture (`No bundled semio font covers U+2504`, `_level abs()` on a tuple, `_away`) were not regenerated; those errors come from scene files other agents are editing, not from the capture or the strict module.

## 5. Files

- `layout_strict.py` (optimization ticket) and identical `layout_strict.py` (BIM ticket)
- `capture.py` in both tickets (imports the module, writes the `### strict` block and summary line, never fails on a strict error)
- Probes and helpers: `probe-layout.py`, `probe-strict.py` (also copied to the BIM ticket), `tally-strict.py`; probe output was deleted after use
