# Consumer Runtime-Contract Audit R5

Read-only source/data audit on 2026-10-28 of six clip chapters, shared visuals, fixtures, bilingual narration and immediate teaching wrappers. Canonical JSON was independently parsed through Node filesystem API. No source files changed. Python runtime and renders remain unverified while repo launcher restart registry is malformed. Source changing concurrently; line references describe audited snapshot.

## Actionable Findings

1. P1: Default piloti detail crashes. visuals/savoye/detail Python line 21 defaults tag B3; line 26 uses next over pilotis. B3 is removed, so default piloti_detail() raises StopIteration. No current chapter calls this widget. Change default to authored C2 and test omitted tag invocation. Stale B3 examples also remain in sheet, axon and drafting docstrings.

2. P1: Landscape opening is dropped by geometry provider. Canonical OP-SC-FRAME is unfilled and belongs to SOLIDWALL SC-FRAME. Model Python line 365 exports screens with only path/base/top/thickness, without openings; walls contains only STANDARD elements. Catalogue never sees this opening, and axon screen.get(openings, ()) cannot cut it. Supply source opening geometry and identity on the common screen contract.

3. P1: Orthographic screen consumers still paint full solid screens after a provider-only fix. Plan Python lines 316-326 fills complete path. Section Python lines 438-460 cuts/projects complete full-height strips, and elevation lines 559-570 paints full-height faces. No branch reads screen openings. Adapt all three with wall solid-cell decomposition and explicit opening part identity; test the unfilled SC-FRAME landscape void.

4. P2: Nullable-fill test coverage is invalid. Clip main tests Python line 597 asserts every opening window_guid equals oracle_global_id(window_tag). Provider defines those fields null for an unfilled opening. Current only unfilled opening is hidden in screens, allowing false coverage. Test fill_tag/fill_guid/fill_class for windows and doors, and null values for OP-SC-FRAME. Catalogue fallback preserves opening identity if records reach it.

## Other Contracts Checked

- Six chapter executable selected support references use C2 or derive real tags. Only stale executable B3 reference found is widget default above.
- Consistency filters supports by roof tributary membership and uses UP-* identities; reaction areas and values are derived.
- Exchange constructs clipped nearest-support polygons. Mesh resolution 16 is pedagogical tessellation, not element count.
- Representations first-opening/window assumption is scoped to authored W-S ribbon and remains valid. Objects _tags at line 268 returns window_tag for every opening, including doors; filter fill_class==IfcWindow when correcting shared opening contract because this drives IfcWindow highlights.
- No obsolete model dimensions found in chapter consumer code. Source box independently checked as 19 by 21.5 m. Counts are derived or fixture-backed.
- Bilingual fork-load narration 12.92/11.12 remains correct: 5 concrete +0.2 waterproofing +1.32 screed +5.4/3.6 soil +1 imposed. Transcripts agree. No obsolete B3/A1/C3 narration found.
- Immediate teaching wrappers contain no Villa geometry or object identity assumptions. No broad repo scan performed.

## Runtime Follow-Up

Run canonical contract, ortho/axon openings, all six presentation chapters and mesh tests through repo after provider and consumers converge. Include default detail construction and SC-FRAME section/elevation with through void. This source audit does not establish runtime or render success.
