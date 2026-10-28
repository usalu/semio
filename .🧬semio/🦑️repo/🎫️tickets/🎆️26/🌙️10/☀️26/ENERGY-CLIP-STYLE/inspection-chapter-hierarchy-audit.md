# Chapter Hierarchy Coverage Audit

Read-only audit against the supplied active-goal criteria: course → topic → clip → chapter, a virtual window for every chapter, zoom out/into each next chapter, no current title, and individual chapter exports with exact adjacent endpoints. No code edit or duplicate render was performed.

## Concrete Gaps

1. The presentation catalog contains topics and videos only. Topic fields are id/title/icon/cell/videos; video fields are id/title/icon/symbol/cell. There is no chapter collection or localized chapter metadata. The manifest separately declares38 logical chapters across17 videos, but the browser catalog does not project them.
2. Browser TSX lines64–65 render a root topic Overview, a topic video Overview, then VideoPane. Hierarchy state at hierarchy/render line30 contains opened_topic/opened_video and revealed_topic/revealed_video only. No opened chapter state, chapter card/window, or chapter target exists. This implements topic → video nesting, not the required chapter level.
3. Presentation composition lines187–188 adds one focus and one restore around the entire concatenated content. Runtime lines814/819 concatenate all logical chapters before that one envelope. No intermediate chapter zoom-out/zoom-in is inserted. The five accepted energy final navigation/coverage sheets agree: each has only an opening and closing UI trajectory, while chapter boundaries inside the body are direct authored-source concatenations.
4. Individual chapter export support exists via chapter_scenes at runtime789–798, but each selected chapter is published using its parent video_id and independently receives the same lesson-level focus/restore envelope. It does not identify a chapter virtual window or chain its final frame to the next chapter opening. Existing exact overview endpoint verification proves a shared lesson-level root, not exact adjacent chapter handoffs. No individual chapter deliverables are retained in the current inspection media set.
5. VideoPane TSX41 explicitly supplies a title chip. Existing body samples also contain authored chapter/topic title labels. Content-only playback removes browser chrome as already verified, but a literal requirement for no current title at all is not satisfied by the visible authored headings. The intended title scope should be resolved from the active goal before claiming compliance; no speculative removal was made.

## Existing Tests And Acceptance Limits

The runtime tests cover selected-chapter publication, source reuse and preserved chapter order (tests lines140,239,433–482). They do not establish chapter virtual-window navigation, interchapter camera trajectories or adjacent chapter export endpoint identity. The current independent final verifier checks body/source order and the two whole-lesson overview endpoints. Those checks remain valuable but do not prove the new chapter-level requirements.

## Actual Artifact Checkpoint

Current durable publish.json contains completed physical_fundamentals, ventilation, heating, cooling and energy_balance; bim_information is running attempt51 and bim_objects attempt48. No publish-complete marker was found in this read. generated/iv contains final coverage sheets only for those five energy lessons (plus verifier selftest). All five were visually inspected and passed their implemented whole-lesson UI contract; this is not a pass of the missing chapter hierarchy. Earlier retained physical/energy inspection movies remain intended preview deliverables. The final17gallery and chapter exports are not complete.

Source evidence: authoring/clip/presentation catalog JSON/schema, browser TSX lines38–65, hierarchy/render Python lines30/59–60/121–125, presentation Python161–219, runtime Python789–819. Paths are under 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip.
