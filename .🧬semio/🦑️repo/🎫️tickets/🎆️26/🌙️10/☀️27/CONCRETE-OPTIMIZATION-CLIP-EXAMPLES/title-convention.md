# Title Convention For Clip Scenes

Read-only investigation, 2026-10-27. No source file was edited and no modifying git command was run.

## Answer

- The removal of `scene_title`, `play_scene_title` and `beat_subtitle` at 19:09 was intentional. It is documented in the ENERGY-CLIP-STYLE ticket (☀️26), and the BIM and optimization `ClipScene.present` persistent headers were explicit targets of that decision.
- Energy clips still show a boxed chip top-left, but it is now a per-beat instructional heading from `section_heading(text)`. The chapter name lives in the presentation window chrome, not in the video body.
- BIM already follows the new convention. Optimization removed the persistent header but has not yet adopted `section_heading` on its beats, and the CONCRETE-OPTIMIZATION ticket docs still describe the old persistent chip.
- Premise correction: at HEAD, `scene_title` was a top-middle, unboxed white title. The boxed persistent top-left chip existed only in the uncommitted working copy before 19:09 (it was called the "persistent primary-outlined video/chapter-name chip" in the audit). That boxed persistent chip is what was removed.

## (1) Ticket evidence

Primary decision record, all in `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ENERGY-CLIP-STYLE/`:

- `inspection-body-title-audit.md` (19:03). Findings: the shared `scene_title` creates a persistent video/chapter-name chip, and `beat_subtitle` anchors to it. It names `BIM ClipScene.present` (adds `_fonts.scene_title(self.TITLE[language])` persistently, "Remove this scene-name header") and `Optimization ClipScene.present` (stores/adds `self._title`, "Remove the field and exclusion"). It proposes one `section_heading(text)` at the shared top-left instructional slot.
- `inspection-body-title-removal.md` (19:19). Decision: "Persistent video-name chips are removed from the authored bodies. The actual chapter name remains in the presentation window chrome. Instructional content now uses a real standalone section_heading(text) chip positioned directly in the first top-left content row." The obsolete `scene_title`, `play_scene_title`, `beat_subtitle` APIs, `_title` state and keep entries are removed. The changed-paths list includes the BIM and optimization visuals and scene files.
- `inspection-body-title-removal-review.md` (19:18). Confirms no authored body calls the removed APIs. The one remaining style-test reference (`energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py`, lines 133–134) is no longer present in the current tree (grep under architecture returns no `scene_title`, `play_scene_title` or `beat_subtitle` callers).
- `inspection-title-free-body-samples.md` (20:03). Visual PASS on 27 frames from 9 sources: "Every sample retains only its instructional first-row heading, with no persistent video-name row above it."
- `title-chip-*.md` concern the native web presentation chrome (the window name chip with the fullscreen icon). They are not about the Manim body and do not contradict the removal.

Ticket notes that are now stale (they predate or ignore the removal):

- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/energy-idioms.md`, rows at lines 18–19 and 32–36, still describe `play_scene_title` in beat 1, `beat_subtitle`, and `_clear_stage(self, (title, subtitle, din))`. Also line 263 and 331 mention the "Title chip top-left" as a sanctioned zone.
- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/audit-consistency.md`, lines 46 and 91 ("Title chip (top left, red outline): consistent in all six"), and line 97. These treat the persistent chip as the convention to keep.
- The CONCRETE-BIM-CLIP-EXAMPLES and CREATE-ARCHITECTURE-OPTIMIZATION-VIDEOS notes contain no decision about the body title. I only grep-checked them, not read them in full.

I searched the ☀️26 and ☀️27 ticket folders for `scene_title`, `section_heading`, `title chip`, `heading`, `top-left` and similar terms. Hit counts were 549 across 65 files in ☀️26 and 76 across 34 files in ☀️27. Most hits are unrelated (for example "heading" as a bearing or compass term, or chip-audit notes about the web UI).

## (2) Where the title is shown now

Git state (`git status` shows the fonts file as `M ` staged, with no working-tree modification):

- Fonts module, `C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py`, mtime 19:09. The staged and working versions are identical. Lines 360–375 define:

  ```python
  TITLE_COLOR: str = _theme.CHROME_FOREGROUND

  def section_heading(text: str):
      """📑 Place a standalone instructional heading in the first content row."""
      ...
      label = centered_body_text(text, font_size=LABEL_FONT_SIZE, color=visuals.P_PRIMARY, max_width=13.0)
      heading = visuals.window_chip(label, color=visuals.P_PRIMARY)
      heading.to_edge(UP, buff=_theme.UI_SPACING * 2)
      heading.to_edge(LEFT, buff=_theme.UI_SPACING * 3)
      return heading
  ```

  `scene_title`, `play_scene_title`, `TITLE_EDGE_BUFF`, `TITLE_RUN_TIME` and `beat_subtitle` no longer exist. HEAD had `scene_title` at line 337, centred at top with `TITLE_COLOR = "#FFFFFF"` and no chip.

- Energy clips: scene bodies call `section_heading("…")` directly and then `self.play(FadeIn(subtitle), FadeIn(din), run_time=BEAT_SUBTITLE_FADE)`. The fundamentals scene is an example. Callers exist across `🔥️heating`, `❄️cooling`, `🌬️ventilation`, `📊️demand` and `⚡️energy/🧲️physics`. Energy clips have no `TITLE = {…}` dict, so they have no chapter name in the body.
- Chapter name: `inspection-body-title-removal.md` states that it "remains in the presentation window chrome", which is the native web app (`⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx`). It is not rendered inside the video.
- BIM: `C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🐍️.py` (mtime 19:09, staged as new) has `ClipScene.present` that only calls `_art.set_vo_language`, `_fonts.apply_scene_style(self)`, `self._caption = None` and `present_<topic>()`. No title. `📊️information/🐍️.py:176` (`_subtitle`) and `🧬️objects/🐍️.py:226–229` (`_act`) use `_fonts.section_heading(self.tr(en, de))` and swap it with FadeIn/FadeOut.
- Optimization: `C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎨️visuals/🐍️.py` (working file modified after staging, mtime 20:32) has `present` (line 123) with no title, and `clear_stage` (line 117) that fades every mobject except `self._caption`. Only `📉️extrema/🐍️.py:124–128` (`_sub`) uses `_fonts.section_heading` for the instructional subtitle. The `TITLE` dicts in the other five optimization scenes (sampling, pareto, algorithms, index, experiments) are not consumed anywhere under architecture. `docs.md` in the CONCRETE-OPTIMIZATION ticket says the transcript H1 is generated from `TITLE`, so keep `TITLE` as metadata and stop drawing it.

## (3) Recommendation for the BIM and optimization bases

The convention to follow: one instructional heading per beat, top-left, from `_fonts.section_heading(text)`. No persistent video-name chip, no `scene_title`, no `beat_subtitle`, no title anchor. The chapter name stays in the window chrome, and `TITLE` stays as metadata for transcripts.

BIM (`🏗️bim/🎬️clip`):

- Base `🎨️visuals/🐍️.py`: no change needed. It already imports `_fonts` and `_art`, and `present` has no title.
- Scenes: keep `_fonts.section_heading(self.tr(en, de))` as in `📊️information` and `🧬️objects`. Fix the stale docstring at `📊️information/🐍️.py:175` ("Subtitle chip under the title") to say "instructional heading chip".
- Scenes without an instructional beat heading (models, exchange, consistency, representations) need nothing. Add one only where the storyboard states a question, as `_fonts.section_heading(self.tr("…", "…"))` with `self.play(_engine.FadeIn(heading))`.

Optimization (`🎯️optimization/🎬️clip`):

- Base `🎨️visuals/🐍️.py`: `_fonts` is already imported. Promote the `_sub` pattern from `📉️extrema/🐍️.py:124–128` into `ClipScene` as one shared method, so that scenes stop duplicating it:

  ```python
  def section(self, en, de):
      """📑 Swap the standalone instructional section heading."""
      new = _fonts.section_heading(self.tr(en, de))
      old, self._section_heading = self._section_heading, new
      return [_engine.FadeOut(old), _engine.FadeIn(new)] if old is not None else [_engine.FadeIn(new)]
  ```

  Then set `self._section_heading = None` in `present` next to `self._caption = None`.
- Call site: `self.step("key", *self.section("Question…", "Frage…"), …)`, as extrema already does with `_sub`. Each new beat needs its own call, because `clear_stage` removes every non-caption mobject between diagrams and the heading would otherwise vanish.
- Scenes that need a heading: `⚖️index`, `🎲️sampling`, `🏆️pareto`, `🧬️algorithms`, `🧪️experiments`. `audit-consistency.md` line 19 asks for a subtitle chip on `Beat2_GridRandomSampling` ("Which nine designs?"). That maps to `self.section("Which nine designs?", "Welche neun Entwürfe?")`.
- Keep `TITLE` in each scene. `docs.md` depends on it for the transcript H1.
- Update the stale ticket docs (`energy-idioms.md`, `audit-consistency.md`) so they describe `section_heading` and the removed persistent chip, rather than the old `title`/`beat_subtitle` pattern.

## Caveats and limits

- The 19:09 change is larger than the scene-title removal. The fonts module was rewritten (bundled Pango faces, glyph fallback, kerning, `_font_digest`). I did not assess that part.
- BIM and optimization are new in the index (not in HEAD), so the change for them is only visible against the earlier working copy described in the audit, not in `git diff HEAD`.
- No render was performed. Visual claims come from the ticket audit notes, not from my own inspection.
- The energy `section_heading` pattern places the chip in the first content row. Note-band and caption positions were validated only in the ENERGY-CLIP-STYLE samples, not in the optimization layout (`layout-strict.md` in the CONCRETE-OPTIMIZATION ticket lists known collisions there).
