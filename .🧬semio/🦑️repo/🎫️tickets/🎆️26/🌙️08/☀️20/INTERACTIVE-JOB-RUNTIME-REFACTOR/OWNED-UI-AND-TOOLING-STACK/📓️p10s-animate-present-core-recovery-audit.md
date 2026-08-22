# Animate Present Core Recovery Audit

## Finding

`@semio-tech/animate-present-core` is not a package-resolution problem. Three active aliases deliberately resolve it to the absent canonical source file:

`✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts`.

The aliases occur in the Animate Vitest config and the 33. Projektetage Vite/Vitest configs. The directory is absent. Its last tracked owned implementation was a single 3,133-line render-independent presentation-model module at `fa51b5c82f`; it was deleted by the taxonomy consolidation commit `4a302f0ba4` (2026-08-05). No current TypeScript source defines `loadPresentationFromSlideGlob`, `SlideFile`, or `presentationPlayAppDefinition`, so no current owned implementation can satisfy either consumer.

The historical module contains every symbol directly consumed below. It is therefore a bounded reconstruction/reintroduction of the canonical module, not an external dependency or a replacement package. Do not point the aliases at `@semio-tech/animate-js`: that package currently only exposes `present_schema` and `present_io`, not the presentation model.

## Required Contract

### Animate React renderer

Types: `AffiliationEntry`, `AffiliationsEmbodiment`, `Arrangement`, `AuthorPerson`, `AuthorsEmbodiment`, `BulletEmbodiment`, `Chapter`, `Disposition`, `DispositionPosition`, `DispositionStyle`, `Embodiment`, `FigureEmbodiment`, `FigureMosaicGrid`, `IframeEmbodiment`, `JsonEmbodiment`, `MarkdownEmbodiment`, `MediaScrollOrigin`, `MediaTeaser`, `MorphFromSlot`, `Participant`, `ParticipantEmphasis`, `PdfEmbodiment`, `Presentation`, `PresentationLanguageKind`, `PresentationSlideBookmark`, `PresentationSlideBookmarkParamKeys`, `PresentationSlideRef`, `RenderSlide`, `ResolvedDisposition`, `Sequence`, `Slide`, `TextEmbodiment`, `TextMorphRoot`, `Thought`, `Transition`, `VideoEmbodiment`.

Values: `PRESENTATION_CHAPTER_QUERY_PARAM`, `PRESENTATION_SEQUENCE_QUERY_PARAM`, `PRESENTATION_SLIDE_QUERY_PARAM`, `PRESENTATION_THOUGHT_QUERY_PARAM`, `abbreviateAuthorFirstName`, `affiliationLineName`, `analogy`, `buildResolutionScope`, `centerResolvedArrangement`, `collectPresentationSlides`, `countArrangements`, `expandThoughtSlides`, `formatPresentationUrlHash`, `intro`, `isIntroArrangementId`, `morphId`, `parsePresentationSlideHash`, `presentationEntityBookmarkName`, `presentationLanguage`, `presentationSequences`, `presentationSlideAt`, `presentationSlideBookmarkParamKeys`, `remapSplitDispositions`, `resolutionScopeForArrangement`, `resolveArrangement`, `resolveEmbodiment`, `resolveMediaScrollOrigin`, `resolveTextMorphRoot`, `split`, `splitFigureGrid`, `tile`, `unionDispositionPositions`, `unionSourceCrops`.

### 33. Projektetage

Types: `Disposition`, `DispositionPosition`, `Embodiment`, `IntroSpec`, `MorphToSlot`, `Participant`, `Presentation`, `PresentationMeta`, `Slide`, `SlideFile`, `SplitArtifacts`, `Thought`.

Values: `MEDIA_SCROLL_ORIGIN_TOP_LEFT`, `PRESENTATION_DEFAULT_SLIDE_ASPECT`, `arrangementRestDispositions`, `buildResolutionScope`, `collectPresentationSlides`, `countArrangements`, `expandThoughtSlides`, `figureFrameForSourceAspect`, `introThoughtFile`, `loadPresentationFromSlideGlob`, `remapSplitDispositions`, `resolveArrangement`, `split`, `splitFigureGrid`, `unionSourceCrops`, plus `presentationPlayAppDefinition` re-exported as `projektetagePlayAppDefinition`.

The historical source text contains every listed symbol. This is the exact direct-import surface; unrelated historical exports can remain internal unless already covered by the module's existing embedded tests.

## Bounded Implementation Plan

1. Recreate only `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts`, using the last owned source as the semantic baseline. Preserve the render-independent model, deck collection/expansion, resolution, split/tile, intro, URL-bookmark, and slide-glob behavior required above. Retain regions and owned public types. Do not add an external runtime dependency or a compatibility adapter.
2. Keep the three aliases unchanged: the recreated file is their intended target. Preserve `presentationPlayAppDefinition` as the concrete current app-definition export because the current Projektetage package exports it; remove the historical deprecation annotation rather than introducing a deprecated API.
3. Do not modify the React renderer, 33. Projektetage source, Rust artifacts, dependency manifests, or the existing PDF-port work. This makes the recovery file-disjoint from all active renderer/dependency packets.
4. Minimal validation after implementation: run the Animate package's existing Vitest route (`bun nx run @semio-tech/animate-js:test`) to exercise collection and renderer imports; then run `bun nx run @semio-tech/mit-bestand-praesentation-projektetage:build` to exercise the consumer's Vite alias and full slide glob. If a focused contract test is needed, add it beside the recovered core module and cover one `loadPresentationFromSlideGlob` hierarchy plus `collectPresentationSlides`/`resolveArrangement`; do not duplicate the Projektetage deck assertions already embedded in its entry module.

## Audit Limits

No Cargo command, dependency-cruiser invocation, build, test, or product-source edit was performed. The evidence is filesystem inspection plus read-only Git history.
