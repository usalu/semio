# Body Title Removal Review

Read-only review of the current heading-paths inventory (34 entries) and all architecture Python files. No production edits or duplicate render.

## Findings

- No authored body retains calls to scene_title, play_scene_title, beat_subtitle, or self._title.
- One existing style test still calls removed fonts.scene_title and fonts.beat_subtitle: energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py lines 133–134. It must use the real section_heading API.
- Current heating solar-gains source fails AST parsing at line 1256 with unmatched closing parenthesis, text n_time=0.3). Executor notified.
- Other 33 inventoried paths parse. A function-scoped AST check found no remaining unbound title, header, or title_anchor load candidates in them. This is a targeted header-reference check, not a general Python type checker.
- Calculation retains standalone section_heading(text) and the instructional main equation heading (DIN V 18599), rather than deleting mathematical instruction with the persistent video header.
- Shared visuals line 1333 retains an obsolete beat_subtitle comment; no executable dependency.

Native lifecycle and math cache tests are owned by verifier/executor and were still running at this checkpoint. New final body visual acceptance remains pending the legitimate post-font-change renders.

Confirmed all five calculation instructional headings remain: losses overview (line 334), annual energy (434), gains overview (664), utilization factor (830), system losses/end energy (1330), plus main equation (1080). Their helper now returns the standalone section_heading.

Follow-up parse checkpoint: 34 of 34 inventoried files parse; remaining errors: [].

## Standalone Native Visual Check

Inspected generated/ih/en-heading.png, de-heading.png and actual-source-header.png. English and German real instructional section headings occupy the first content row, with complete text, visible margins, clean box bounds and no persistent video-title row. The actual 480×270 source capture shows the German instructional heading at upper left and its DIN reference at upper right without collision. These are heading-stage snapshots, so they validate placement and removal only; settled full-body and final chapter navigation acceptance remain pending. Existing style fixture migration is reported complete by executor. No visual defect found in these three snapshots.
