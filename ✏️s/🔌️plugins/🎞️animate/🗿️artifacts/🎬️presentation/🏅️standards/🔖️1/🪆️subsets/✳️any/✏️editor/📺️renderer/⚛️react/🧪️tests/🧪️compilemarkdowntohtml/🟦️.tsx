type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { compileMarkdownToHtml } = dependencies;

  const { describe, expect, it } = vitest;

  describe("compileMarkdownToHtml", () => {
    it("renders GFM tables as HTML", async () => {
      const html = await compileMarkdownToHtml("| A | B |\n| - | - |\n| `x` | y |");
      expect(html).toContain("<table");
      expect(html).toContain("<code>x</code>");
    });
  });

}

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { FIGURE_WHEEL_ZOOM_MAX, PRESENTATION_MANY_TO_ONE_MORPH_CLASS, Reveal, SLIDE_INTERACTIVE_ENLARGE_FRAME, act, buildInteractiveSlideLayout, buildResolutionScope, clearRevealAutoAnimateInlineLayout, clientToSectionFraction, collectPresentationSlides, dispositionHasEphemeralLayout, dispositionPlacementContainer, elementIsInteractiveFigureDisposition, elementIsSourceGhostAnchor, elementIsTargetGhostAnchor, expandThoughtSlides, figureBackgroundSizeScrollAxis, figureBackgroundSizeZoomed, figureCoverOverflowAxis, figureCoverScrollContentSize, figureCoverScrollElementStyle, figureCropBackgroundPosition, figureCropBackgroundSize, figureCropBackgroundVars, figureCropBackgroundVarsTargetGhost, figureCropScrollBackgroundSize, figureEmbodimentScrollEnabled, figureScrollOffsetForBackgroundPosition, figureScrollOverlayThumbMetrics, figureWheelZoomStep, finalizeRevealAutoAnimateRestState, flowDispositionManipulationRect, flowDispositionOffsetStyle, flowPixelOffsetToSectionRect, flowPointerDeltaToLocal, groupBoundingRect, interactiveDispositionChromeStyle, interactiveDispositionContentScale, interactiveDispositionContentScaleStyle, intro, isFlowPixelOffsetTransform, isManyToOneMorphTransition, isNormalizedSlideFrame, isRevealAutoAnimatePairSource, isRevealSlideAutoAnimating, isSectionAutoAnimating, isUsableMeasuredRect, marqueeSelectionRule, marqueeSelects, measureDispositionBoundsInContainer, measureDispositionBoundsInSection, mediaTeaserActive, morphFrameCssVars, morphId, mosaicWindowedCoverVars, mountPresentation, normalizeMarquee, patchAutoAnimateUniformScale, patchPresentationAutoAnimateRunStyleSheet, patchPresentationAutoAnimateStyleSheet, pdfAdjacentPage, pdfCanGoToNextPage, pdfCanGoToPreviousPage, pdfCoverScale, pdfEmbodimentInitialPage, pdfPageNavEnabled, pdfScrollCoverScale, pointerNearSlideResetHotspot, prepareArrangementBeforeAutoAnimate, presentationAutoAnimateMatcher, readPresentationSlideIndicesFromUrl, rectContains, rectsIntersect, relaxHiddenPreflight, remapSplitDispositions, resetPdfCanvasPort, resizeDispositionRect, resolveArrangement, resolvePresentationAssetUrl, resolvePresentationAutoAnimateRunSlides, resolveRevealArrangement, revealDeckInOverview, revealInkMeasureForAutoAnimate, revealTextAutoAnimatePairOptions, scaleRectWithinGroup, setPdfCanvasPort, slideCoordinateRoot, slideHasEphemeralLayout, slidesShareAutoAnimateId, split, splitFigureGrid, suppressRevealOverviewSlideNavigation, syncArrangementSettledState, syncManyToOneGhostMorphFramesFromDom, syncPresentationSlideUrl, tightElementBoundsRect, toggleEnlargeRect, transformFrameStyle, translateDispositionRect, unmountPresentation } = dependencies;
  type AutoAnimateMatcherHost = any;
  type DispositionPosition = any;
  type PdfCanvasDocument = any;
  type PdfCanvasPort = any;
  type PdfEmbodiment = any;
  type Presentation = any;
  type Slide = any;

  const { describe, expect, it, beforeEach, afterEach } = vitest;
  const { readFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const { fileURLToPath } = await import("node:url");
  const globalsCssSource = readFileSync(join(dirname(fileURLToPath(source.url)), "🎨️.css"), "utf8");

  function stockRevealAutoAnimateMatcherHost(): AutoAnimateMatcherHost {
    return {
      findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
        const reserved = new Set<HTMLElement>();
        for (const fromElement of fromScope.querySelectorAll<HTMLElement>(selector)) {
          const fromKey = serializer(fromElement);
          for (const toElement of toScope.querySelectorAll<HTMLElement>(selector)) {
            if (reserved.has(toElement) || serializer(toElement) !== fromKey) {
              continue;
            }
            pairs.push({ from: fromElement, to: toElement });
            reserved.add(toElement);
            break;
          }
        }
      },
    };
  }

  describe("resolveRevealArrangement", () => {
    it("appends target companions for morphFrom on the labels slide", () => {
      const scope = buildResolutionScope([
        {
          participants: [{ id: "col1" }, { id: "tile" }],
          embodiments: [
            { kind: "text", id: "label", lines: ["A"], level: "heading" },
            { kind: "figure", id: "tile-figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
          ],
        },
      ]);
      const previousSlide: Slide = {
        arrangement: {
          id: "focus",
          dispositions: [
            {
              participantId: "tile",
              embodimentId: "tile-figure",
              emphasis: "active",
              position: { x: 0.5, y: 0.5, width: 0.2, height: 0.2 },
            },
          ],
        },
      };
      const resolved = resolveRevealArrangement(
        scope,
        {
          id: "labels",
          dispositions: [
            {
              participantId: "col1",
              embodimentId: "label",
              emphasis: "active",
              morphFrom: [
                {
                  participantId: "tile",
                  embodimentId: "tile-figure",
                  position: { x: 0.2, y: 0.3, width: 0.2, height: 0.1 },
                },
              ],
            },
          ],
        },
        { previousSlide },
      );
      expect(resolved).toHaveLength(2);
      const companion = resolved.find((entry) => entry.revealMorphCompanion === "target");
      expect(companion?.morphId).toBe("tile");
      expect(companion?.position).toEqual({ x: 0.2, y: 0.3, width: 0.2, height: 0.1 });
      expect(companion?.revealMorphFromFrame).toEqual({ x: 0.5, y: 0.5, width: 0.2, height: 0.2 });
    });

    it("sets revealMorphToFrame on focus tiles when the next slide morphFrom references them", () => {
      const scope = buildResolutionScope([
        {
          participants: [{ id: "tile" }, { id: "col1" }],
          embodiments: [
            { kind: "figure", id: "tile-figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
            { kind: "text", id: "label", lines: ["A"], level: "heading" },
          ],
        },
      ]);
      const focusPosition = { x: 0.5, y: 0.5, width: 0.2, height: 0.2 };
      const labelPosition = { x: 0.2, y: 0.3, width: 0.2, height: 0.1 };
      const resolved = resolveRevealArrangement(
        scope,
        {
          id: "focus",
          dispositions: [
            {
              participantId: "tile",
              embodimentId: "tile-figure",
              emphasis: "active",
              position: focusPosition,
            },
          ],
        },
        {
          nextSlide: {
            arrangement: {
              id: "labels",
              dispositions: [
                {
                  participantId: "col1",
                  embodimentId: "label",
                  emphasis: "active",
                  morphFrom: [
                    {
                      participantId: "tile",
                      embodimentId: "tile-figure",
                      position: labelPosition,
                    },
                  ],
                },
              ],
            },
          },
        },
      );
      expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphToFrame).toEqual(labelPosition);
    });

    it("morphFrameCssVars maps normalized frames to presentation custom properties", () => {
      expect(morphFrameCssVars({ x: 0.1, y: 0.2, width: 0.3, height: 0.4 }, { x: 0.5, y: 0.6, width: 0.2, height: 0.1 })).toEqual({
        "--presentation-morph-frame-left": "10%",
        "--presentation-morph-frame-top": "20%",
        "--presentation-morph-frame-width": "30%",
        "--presentation-morph-frame-height": "40%",
        "--presentation-frame-left": "50%",
        "--presentation-frame-top": "60%",
        "--presentation-frame-width": "20%",
        "--presentation-frame-height": "10%",
      });
    });

    it("keeps target-ghost rest crop on the source tile frame and morph crop on the label slot", () => {
      const crop = { x: 0.8, y: 0.7, width: 0.15, height: 0.2 };
      const embodiment = { kind: "figure" as const, src: "/catalogue.png", crop };
      const sourceFrame = { x: 0.77, y: 0.11, width: 0.2, height: 0.78 };
      const labelFrame = { x: 0.653333, y: 0.44, width: 0.246667, height: 0.12 };
      const vars = figureCropBackgroundVarsTargetGhost(embodiment, crop, sourceFrame, labelFrame);
      const sourceOnly = figureCropBackgroundVars(embodiment, crop, sourceFrame);
      const labelOnly = figureCropBackgroundVars(embodiment, crop, labelFrame);
      expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe(sourceOnly["--presentation-figure-bg-size" as keyof typeof sourceOnly]);
      expect(vars["--presentation-figure-bg-size-morph" as keyof typeof vars]).toBe(labelOnly["--presentation-figure-bg-size" as keyof typeof labelOnly]);
    });

    it("appends source companions for morphTo on the whole slide", () => {
      const scope = buildResolutionScope([
        {
          participants: [{ id: "whole" }, { id: "tile-a" }],
          embodiments: [
            { kind: "figure", id: "whole--figure", src: "/a.png" },
            { kind: "figure", id: "tile-a--figure", src: "/a.png", crop: { x: 0, y: 0, width: 0.5, height: 1 } },
          ],
        },
      ]);
      const nextSlide: Slide = {
        arrangement: {
          id: "tiles",
          dispositions: [{ participantId: "tile-a", embodimentId: "tile-a--figure", emphasis: "active" }],
        },
      };
      const resolved = resolveRevealArrangement(
        scope,
        {
          id: "whole",
          dispositions: [
            {
              participantId: "whole",
              embodimentId: "whole--figure",
              emphasis: "active",
              morphTo: [{ participantId: "tile-a", position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 } }],
            },
          ],
        },
        { nextSlide },
      );
      expect(resolved).toHaveLength(2);
      const companion = resolved.find((entry) => entry.revealMorphCompanion === "source");
      expect(companion?.morphId).toBe("tile-a");
    });
  });

  describe("PresentationDeck", () => {
    let container: HTMLDivElement;

    const testAffiliationSteps = {
      steps: [
        [{ mark: "a", name: "Faculty" }],
        [
          { mark: "a", name: "Faculty" },
          { mark: "1", name: "Uni" },
        ],
        [
          { mark: "a", name: "Faculty" },
          { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
        ],
      ],
    } as const;

    beforeEach(() => {
      container = document.createElement("div");
      document.body.appendChild(container);
    });

    afterEach(() => {
      unmountPresentation();
      container.remove();
    });

    it("renders expanded intro slides", () => {
      const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false });
      });
      const sections = container.querySelectorAll(".slides > section > section");
      expect(sections.length).toBe(7);
      expect(sections[0]?.hasAttribute("data-auto-animate")).toBe(true);
      const revealEl = container.querySelector(".reveal");
      expect(revealEl?.getAttribute("style")).toContain("100vw");
      expect(container.querySelector('[data-id^="title"]')).toBeTruthy();
      expect(container.querySelector('[data-id^="description"]')).toBeTruthy();
      expect(container.querySelector('[data-id^="goal"]')).toBeTruthy();
      expect(container.querySelector('[data-id^="authors--"]')).toBeTruthy();
      expect(container.querySelector('[data-id^="institutions--"]')).toBeTruthy();
    });

    it("keeps intro flow slides centered while enabling interactive dispositions", () => {
      const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
        expect(slide.classList.contains("presentation-arrangement--interactive")).toBe(true);
        expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
        expect(slide.classList.contains("presentation-arrangement--positioned")).toBe(false);
        expect(slide.querySelector(".presentation-arrangement-canvas")).toBeNull();
        expect(slide.querySelectorAll("[data-disposition-id]").length).toBeGreaterThan(0);
      }
      expect(globalsCssSource).toMatch(/\.presentation-arrangement--interactive\s*\{[^}]*overflow\s*:\s*visible/s);
      expect(globalsCssSource).toMatch(/\.presentation-arrangement-canvas\s*>\s*\.presentation-interactive-disposition\[data-id\][\s\S]*:not\(\s*\.presentation-interactive-disposition--gesturing\s*\)[\s\S]*overflow\s*:\s*hidden/s);
      expect(globalsCssSource).not.toMatch(/\.presentation-arrangement--interactive\s*\{[^}]*position\s*:\s*relative/s);
    });

    it("applies muted opacity on layered description slide", () => {
      const deck = intro({ language: "de", title: { full: ["A"], short: "Short" }, description: { full: ["D"], short: "D short" }, goal: ["G"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const descriptionSlide = container.querySelector('.slides > section > section[title="description"]');
      expect(descriptionSlide?.querySelector(".opacity-20")).toBeTruthy();
    });

    it("matches eg-ice-25 intro morph DOM per arrangement", () => {
      const deck = intro({
        language: "de",
        title: { full: ["A", "B", "C"], short: "Short" },
        description: { full: ["D1", "D2"], short: "D short" },
        goal: ["G1"],
        authors: {
          lines: [
            [
              { name: "Alice Example", marks: ["a", "1", "x"] },
              { name: "Bob Beta", marks: ["a", "1", "x"] },
            ],
            [{ name: "Carol Creator" }],
          ],
        },
        affiliations: testAffiliationSteps,
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
      expect(slide("title")?.querySelectorAll('h2[data-id^="title"]').length).toBe(3);
      expect(slide("description")?.querySelector('h2[data-id="title"]')).toBeTruthy();
      expect(slide("description")?.querySelector('div[data-id="title"].presentation-disposition-frame')).toBeNull();
      expect(slide("description")?.querySelectorAll('h2[data-id^="description"]').length).toBe(2);
      expect(slide("goal")?.querySelector('h2[data-id="description"]')?.textContent).toBe("D short");
      expect(slide("goal")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
      expect(slide("description")?.querySelector('.presentation-interactive-disposition[data-id="description"]')).toBeNull();
      expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
      const authorLines = slide("authors")?.querySelectorAll('h4[data-id^="authors--"]');
      expect(authorLines?.length).toBe(3);
      expect(slide("authors")?.getAttribute("data-auto-animate-id")).toMatch(/^einleitung--/);
      expect(slide("authors")?.querySelector(".presentation-intro-line")?.className).toContain("gap-x-");
      expect(slide("affiliations-1")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(1);
      expect(slide("affiliations-2")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
      expect(slide("affiliations-3")?.querySelectorAll('h4[data-id^="institutions--"]').length).toBe(2);
      expect(slide("affiliations-3")?.querySelectorAll('[data-id^="institutions--"]').length).toBe(3);
      expect(slide("affiliations-2")?.querySelector('h5[data-id="institutions"]')).toBeNull();
      expect(slide("affiliations-2")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("Uni");
      expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
      expect(slide("affiliations-3")?.textContent).toContain("Chair X");
      expect(slide("affiliations-3")?.querySelector(".presentation-affiliation-morph-source")).toBeNull();
      expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"]')?.classList.contains("presentation-affiliation-row")).toBe(true);
      expect(slide("affiliations-3")?.querySelector('[data-id="institutions--x"]')?.textContent).toContain("Chair X");
      expect(slide("affiliations-3")?.querySelector('h4[data-id="institutions--1"] [data-id="institutions--x"]')).toBeTruthy();
      expect(slide("affiliations-1")?.querySelector('h4[data-id="authors--Alice Example"] sup')?.textContent).toBe("a");
      const marked2 = slide("affiliations-2")?.querySelector('h4[data-id="authors--Alice Example"] sup');
      expect(marked2?.textContent).toBe("a,1");
      expect(marked2?.querySelector("span:not(.opacity-20)")?.textContent).toBe("1");
      const marked3 = slide("affiliations-3")?.querySelector('h4[data-id="authors--Alice Example"] sup');
      expect(marked3?.textContent).toBe("a,1,x");
      expect(marked3?.querySelector("span:not(.opacity-20)")?.textContent).toBe("x");
      expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("A. Example");
      expect(slide("affiliations-1")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeTruthy();
      expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"] .opacity-20')).toBeNull();
      expect(slide("authors")?.querySelector('[data-id="authors--Alice Example"]')?.textContent).toContain("Alice Example");
      const aff2 = slide("affiliations-2");
      expect(aff2?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
      expect(aff2?.querySelector('h4[data-id="institutions--1"] .opacity-20')).toBeNull();
      const aff3 = slide("affiliations-3");
      expect(aff3?.querySelector('h4[data-id="institutions--a"] .opacity-20')).toBeTruthy();
      expect(aff3?.querySelector('h4[data-id="institutions--1"]')?.textContent).toContain("LUH");
      expect(aff3?.querySelector('[data-id="institutions--x"] .opacity-20')).toBeNull();
    });

    it("applies title and secondary morph text sizes on intro slides", () => {
      const deck = intro({
        language: "de",
        title: { full: ["A", "B", "C"], short: "Short" },
        description: { full: ["D1", "D2"], short: "D short" },
        goal: ["G1"],
        authors: {
          lines: [[{ name: "Alice", marks: ["1"] }, { name: "Bob" }], [{ name: "Carol" }]],
        },
        affiliations: testAffiliationSteps,
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const expectMorphClass = (selector: string, sizeClass: string) => {
        for (const node of container.querySelectorAll(selector)) {
          expect(node.classList.contains("presentation-morph-text")).toBe(true);
          expect(node.classList.contains(sizeClass)).toBe(true);
        }
      };
      expectMorphClass('h1[data-id^="title"], h2[data-id^="title"]', "presentation-morph-text--title");
      expectMorphClass('h2[data-id^="description"], p[data-id^="description"], h2[data-id^="goal"], p[data-id^="goal"]', "presentation-morph-text--secondary");
      expect(container.querySelector('h2[data-id^="title"].presentation-morph-text--secondary')).toBeNull();
      expect(container.querySelector('h2[data-id^="goal"].presentation-morph-text--title')).toBeNull();
    });

    it("does not use reveal fit-text on intro headings", () => {
      const deck = intro({ language: "de", title: { full: ["A", "B"], short: "Short" }, description: { full: ["D1"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      expect(container.querySelector(".r-fit-text")).toBeNull();
    });

    it("enables reveal auto-animate on intro morph slides with unmatched layering", () => {
      const deck = intro({
        language: "de",
        title: { full: ["A"], short: "Short" },
        description: { full: ["D"], short: "D short" },
        goal: ["G1", "G2"],
        authors: {
          lines: [
            [
              { name: "Alice", marks: ["a"] },
              { name: "Bob", marks: ["a"] },
            ],
            [{ name: "Carol", marks: ["a"] }],
          ],
        },
        affiliations: testAffiliationSteps,
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      for (const slide of container.querySelectorAll('.slides > section > section[data-auto-animate-id="einleitung--m0"]')) {
        expect(slide.classList.contains("presentation-arrangement--intro")).toBe(true);
        expect(slide.hasAttribute("data-auto-animate")).toBe(true);
        expect(slide.querySelector(".presentation-arrangement-surface")).toBeNull();
      }
      expect(globalsCssSource).toMatch(/\.presentation-arrangement--intro:not\(\.presentation-arrangement--positioned\)[\s\S]*:is\(h1,\s*h2,\s*h3,\s*h4,\s*p\)[\s\S]*margin:\s*0/s);
      expect(globalsCssSource).not.toMatch(/data-auto-animate-id\^="intro--"/);
    });

    it("enables reveal auto-animate and tags every morph arrangement with data-auto-animate", () => {
      const deck = intro({ language: "de", title: { full: ["A", "B", "C"], short: "Short" }, description: { full: ["D1", "D2"], short: "D short" }, goal: ["G1"], authors: { lines: [[{ name: "Alice" }]] }, affiliations: testAffiliationSteps });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const morphSections = container.querySelectorAll('.slides > section > section[data-auto-animate][data-auto-animate-id="einleitung--m0"]');
      expect(morphSections.length).toBe(7);
      const slide = (id: string) => container.querySelector(`.slides > section > section[title="${id}"]`);
      expect(slide("title")?.querySelector('h2[data-id^="title"]')).toBeTruthy();
      expect(slide("title")?.querySelector('[data-id^="description"]')).toBeNull();
      expect(slide("description")?.querySelector('h2[data-id^="description"]')).toBeTruthy();
      expect(slide("goal")?.querySelector('h2[data-id^="goal"]')).toBeTruthy();
    });

    it("renders video and pdf embodiments with data-id for auto-animate", () => {
      const deck: Presentation = {
        id: "media-test",
        name: "Media",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "media",
                thoughts: [
                  {
                    id: "media",
                    participants: [{ id: "clip" }, { id: "doc" }],
                    embodiments: [
                      { kind: "video", id: "clip--video", src: "/demo.mp4" },
                      { kind: "pdf", id: "doc--pdf", src: "/paper.pdf", page: 1 },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "slide",
                          dispositions: [
                            { participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
                            { participantId: "doc", embodimentId: "doc--pdf", emphasis: "active" },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      expect(container.querySelector('[data-id="clip"] video[src="/demo.mp4"]')).toBeTruthy();
      expect(container.querySelector('[data-id="doc"] .presentation-media-pdf-document')).toBeTruthy();
    });

    it("renders glassy teaser veils on media embodiments with optional labels", () => {
      const deck: Presentation = {
        id: "teaser-test",
        name: "Teaser",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "teaser",
                thoughts: [
                  {
                    id: "teaser",
                    participants: [{ id: "fig" }, { id: "clip" }, { id: "embed" }],
                    embodiments: [
                      { kind: "figure", id: "fig--figure", src: "/a.png", teaser: {} },
                      {
                        kind: "video",
                        id: "clip--video",
                        src: "/demo.mp4",
                        teaser: { label: "Coming soon" },
                      },
                      {
                        kind: "iframe",
                        id: "embed--iframe",
                        src: "/demo.html",
                        teaser: { label: "Interactive demo" },
                      },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "slide",
                          dispositions: [
                            { participantId: "fig", embodimentId: "fig--figure", emphasis: "active" },
                            { participantId: "clip", embodimentId: "clip--video", emphasis: "active" },
                            { participantId: "embed", embodimentId: "embed--iframe", emphasis: "active" },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const figureVeil = container.querySelector('[data-id="fig"] .presentation-media-teaser__veil');
      const videoLabel = container.querySelector('[data-id="clip"] .presentation-media-teaser__label');
      const iframeVeil = container.querySelector('[data-id="embed"] .presentation-media-teaser__veil');
      expect(figureVeil).toBeTruthy();
      expect(videoLabel?.textContent).toBe("Coming soon");
      expect(iframeVeil?.querySelector(".presentation-media-teaser__label")?.textContent).toBe("Interactive demo");
      expect(container.querySelector('[data-id="embed"] iframe.presentation-media-iframe')).toBeTruthy();
    });

    it("applies absolute positioning for dispositions with position", () => {
      const deck: Presentation = {
        id: "position-test",
        name: "Position",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "pos",
                thoughts: [
                  {
                    id: "pos",
                    participants: [{ id: "box" }],
                    embodiments: [{ kind: "text", id: "box--main", lines: ["A"], level: "body" }],
                    slides: [
                      {
                        arrangement: {
                          id: "placed",
                          dispositions: [
                            {
                              participantId: "box",
                              embodimentId: "box--main",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.2, width: 0.5, height: 0.3 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const frame = container.querySelector(".presentation-disposition-frame") as HTMLElement | null;
      expect(frame?.style.position).toBe("absolute");
      expect(frame?.style.left).toBe("25%");
      expect(frame?.style.width).toBe("50%");
    });

    it("renders split figure tiles with per-participant data-id and background crops", () => {
      const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
      const artifacts = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
      const deck: Presentation = {
        id: "split-figure",
        name: "Split",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "split",
                    participants: artifacts.participants,
                    embodiments: artifacts.embodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "tiles",
                          dispositions: artifacts.dispositions,
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const wrappers = container.querySelectorAll("[data-disposition-id]");
      expect(wrappers.length).toBe(4);
      const first = wrappers[0] as HTMLElement;
      expect(first.classList.contains("presentation-interactive-disposition")).toBe(true);
      expect(first.style.position).toBe("absolute");
      const tileFrame = first.querySelector('.presentation-morph-slot--figure[data-id^="tile-r"]') as HTMLElement;
      expect(tileFrame).toBeTruthy();
      expect(tileFrame.style.backgroundImage).toContain("/catalogue.png");
      expect(tileFrame.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
      const tiles = [...container.querySelectorAll(".presentation-morph-slot--figure")] as HTMLElement[];
      for (const node of tiles) {
        expect(node.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
      }
      const positions = tiles.map((node) => node.style.getPropertyValue("--presentation-figure-bg-position"));
      expect(new Set(positions).size).toBe(4);
      expect(first.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
    });

    it("pairs catalogue tile morph anchors on interactive wrappers across slides", () => {
      const frameA = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
      const gridA = split({ source: "/catalogue.png", rows: 2, columns: 2, frame: frameA });
      const frameB = { x: 0.1, y: 0.15, width: 0.35, height: 0.3 };
      const positionsB = Object.fromEntries(
        gridA.dispositions.map((disposition, index) => [
          disposition.participantId,
          {
            x: frameB.x + (index % 2) * (frameB.width / 2),
            y: frameB.y + Math.floor(index / 2) * (frameB.height / 2),
            width: frameB.width / 2,
            height: frameB.height / 2,
          },
        ]),
      );
      const gridB = {
        ...gridA,
        dispositions: remapSplitDispositions(gridA.dispositions, positionsB),
      };
      const deck: Presentation = {
        id: "tile-morph",
        name: "Tile morph",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "media",
                thoughts: [
                  {
                    id: "media",
                    participants: gridA.participants,
                    embodiments: gridA.embodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "catalogue",
                          dispositions: gridA.dispositions,
                        },
                        transition: { kind: "morph" },
                      },
                      {
                        arrangement: {
                          id: "catalogue-focus",
                          dispositions: gridB.dispositions,
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const sections = [...container.querySelectorAll("section[data-auto-animate-id]")] as HTMLElement[];
      expect(sections.length).toBeGreaterThanOrEqual(2);
      const fromSlide = sections.find((section) => section.title === "catalogue");
      const toSlide = sections.find((section) => section.title === "catalogue-focus");
      expect(fromSlide).toBeDefined();
      expect(toSlide).toBeDefined();
      const pairs = presentationAutoAnimateMatcher.call({ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost, fromSlide!, toSlide!);
      expect(pairs).toHaveLength(4);
      expect(pairs.every((pair) => pair.from.getAttribute("data-id") === pair.to.getAttribute("data-id"))).toBe(true);
      expect(pairs[0]?.from.classList.contains("presentation-interactive-disposition")).toBe(true);
      expect(pairs[0]?.to.classList.contains("presentation-interactive-disposition")).toBe(true);
    });

    it("renders only the dispositions listed on a slide", () => {
      const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
      const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
      const deck: Presentation = {
        id: "partial-split",
        name: "Partial",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "partial",
                    participants: grid.participants,
                    embodiments: grid.embodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "focus",
                          dispositions: grid.dispositions.slice(0, 2),
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      expect(container.querySelectorAll('[data-id^="tile-r"]').length).toBe(2);
    });

    it("renders cropped figure dispositions with matching morph slot DOM", () => {
      const deck: Presentation = {
        id: "crop-figure",
        name: "Crop",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "crop",
                    participants: [{ id: "catalogue-col1" }],
                    embodiments: [
                      {
                        kind: "figure",
                        id: "catalogue-col1--crop",
                        src: "/catalogue.png",
                        crop: { x: 0, y: 0, width: 0.5, height: 1 },
                      },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "focus",
                          dispositions: [
                            {
                              participantId: "catalogue-col1",
                              embodimentId: "catalogue-col1--crop",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const slot = container.querySelector('[data-id="catalogue-col1"]') as HTMLElement | null;
      const cropSlot = slot?.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
      expect(cropSlot?.style.backgroundImage).toContain("/catalogue.png");
      expect(cropSlot?.querySelector("h2")).toBeNull();
      expect(slot?.style.left).toBe("35%");
      expect(cropSlot?.style.getPropertyValue("--presentation-figure-bg-size")).toMatch(/% auto$/);
      expect(cropSlot?.style.getPropertyValue("--presentation-figure-bg-position")).toBe("0% 50%");
      expect(slot?.querySelector(".presentation-figure-scroll-viewport--axis-x")).not.toBeNull();
    });

    it("rewrites non-uniform auto-animate scale() to a uniform zoom", () => {
      expect(patchAutoAnimateUniformScale("transform: translate(1px, 2px) scale(1.5, 2) !important;")).toBe("transform: translate(1px, 2px) scale(2) !important;");
    });

    it("appends morph ghost opacity 1→0 and morph-into 0→1 rules to the auto-animate sheet", () => {
      const sheet = { innerHTML: "transform: scale(1, 2);" };
      patchPresentationAutoAnimateStyleSheet(sheet, 0.8);
      expect(sheet.innerHTML).toContain("scale(2)");
      expect(sheet.innerHTML).toContain('[data-auto-animate="running"] .presentation-target-ghost[data-auto-animate-target]');
      expect(sheet.innerHTML).toContain("opacity: 1 !important");
      expect(sheet.innerHTML).toContain("presentation-target-ghost-fade-out 0.8s ease forwards !important");
      expect(sheet.innerHTML).not.toContain("presentation-morph-target-fade-in");
    });

    it("preserves anisotropic scale() in the auto-animate sheet for intro flow morph", () => {
      const sheet = { innerHTML: "transform: scale(1, 2) !important;" };
      patchPresentationAutoAnimateStyleSheet(sheet, 0.8, { introFlowMorph: true });
      expect(sheet.innerHTML).toContain("scale(1, 2)");
      expect(sheet.innerHTML).not.toContain("scale(2)");
    });

    it("leaves intro flow auto-animate sheets identical to reveal.js output", () => {
      const fromSlide = document.createElement("section");
      fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const toSlide = document.createElement("section");
      toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const sheet = { innerHTML: "transform: translate(10px, 20px) scale(0.4, 1.2) !important;" };
      patchPresentationAutoAnimateRunStyleSheet(sheet, 0.8, fromSlide, toSlide);
      expect(sheet.innerHTML).toBe("transform: translate(10px, 20px) scale(0.4, 1.2) !important;");
    });

    it("uses pending slide pair when reveal autoanimate omits event slides", () => {
      const fromSlide = document.createElement("section");
      fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const toSlide = document.createElement("section");
      toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const resolved = resolvePresentationAutoAnimateRunSlides({}, { fromSlide, toSlide });
      const sheet = { innerHTML: "transform: translate(10px, 20px) scale(0.4, 1.2) !important;" };
      patchPresentationAutoAnimateRunStyleSheet(sheet, 0.8, resolved.fromSlide, resolved.toSlide);
      expect(resolved).toEqual({ fromSlide, toSlide });
      expect(sheet.innerHTML).toBe("transform: translate(10px, 20px) scale(0.4, 1.2) !important;");
    });

    it("preserves anisotropic scale() in the auto-animate sheet for many-to-one morph", () => {
      const sheet = { innerHTML: "transform: scale(0.4, 0.9) !important;" };
      patchPresentationAutoAnimateStyleSheet(sheet, 0.8, { manyToOneMorph: true });
      expect(sheet.innerHTML).toContain("scale(0.4, 0.9)");
      expect(sheet.innerHTML).not.toContain("scale(0.9)");
      expect(sheet.innerHTML).toContain("presentation-target-ghost-frame 0.8s ease forwards");
    });

    it("animates figure crop and target-ghost frames only during many-to-one auto-animate", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, resolve } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "🎨️.css"), "utf8");
      expect(css).toContain("presentation-figure-crop-morph-from-rest");
      expect(css).toContain("presentation-figure-crop-morph-grid-to-focus");
      expect(css.match(/presentation-morph-crop-from[\s\S]*?presentation-figure-crop-morph-from-rest/)).toBeTruthy();
      expect(css).toContain("@keyframes presentation-target-ghost-frame");
      expect(css).toContain('section.presentation-arrangement--many-to-one-morph[data-auto-animate="running"]');
      expect(css).toContain(".presentation-morph-crop-from");
      expect(css).toContain(".presentation-morph-crop-to");
    });

    it("rests morph-source ghosts with opacity only so reveal can measure FLIP targets", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, resolve } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const cssPath = resolve(dirname(fileURLToPath(source.url)), "🎨️.css");
      const css = readFileSync(cssPath, "utf8");
      const restRule = css.match(/\.reveal \.presentation-target-ghost \{[\s\S]*?\}/)?.[0] ?? "";
      expect(restRule).toContain("opacity: 0");
      expect(restRule).not.toContain("opacity: 0 !important");
      expect(restRule).not.toContain("visibility: hidden");
      expect(css).toMatch(/section\[data-auto-animate="running"\][\s\S]*?presentation-target-ghost-fade-out/);
      expect(restRule).toContain("pointer-events: none !important");
      expect(restRule).toContain("z-index: 0");
      expect(css).toContain("> .presentation-interactive-disposition:not(.presentation-target-ghost):not(.presentation-source-ghost)");
      expect(css).toMatch(/section\[data-auto-animate="pending"\][\s\S]*?\.presentation-target-ghost/);
    });

    it("resolvePresentationAssetUrl maps deck-relative paths through the Vite base", () => {
      expect(resolvePresentationAssetUrl("/🖼️bauteilbörse.png")).toBe("/🖼️bauteilbörse.png");
      expect(resolvePresentationAssetUrl("./🖼️bauteilbörse.png")).toBe("/🖼️bauteilbörse.png");
    });

    it("keeps figure selection chrome as a translucent overlay, not a solid fill", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, resolve } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "🎨️.css"), "utf8");
      expect(css).toContain(".presentation-interactive-disposition--kind-figure.presentation-interactive-disposition--selected");
      expect(css).toContain(".presentation-morph-slot--figure::after");
      expect(css).not.toMatch(/\.presentation-interactive-disposition--kind-figure\.presentation-interactive-disposition--selected[\s\S]*?\.presentation-morph-slot--figure\s*\{[^}]*background-color:\s*var\(--color-primary\)/);
    });

    it("computes pdf cover scale from container and page viewport", () => {
      expect(pdfCoverScale(768, 280, 595, 842)).toBeCloseTo(768 / 595);
      expect(pdfCoverScale(200, 400, 595, 842)).toBeCloseTo(400 / 842);
      expect(pdfCoverScale(0, 400, 595, 842)).toBeNull();
      expect(pdfScrollCoverScale(768, 280, 595, 842)).toBeCloseTo(768 / 595);
      expect(pdfScrollCoverScale(200, 400, 595, 842)).toBeCloseTo(400 / 842);
    });

    it("figureCoverScrollElementStyle fits one axis for wide video frames", () => {
      const style = figureCoverScrollElementStyle(565, 350, 1200 / 1080);
      expect(style.width).toBe("100%");
      expect(style.height).toBe("auto");
      expect(style.aspectRatio).toBe(String(1200 / 1080));
    });

    it("navigates only within a declared pdf page subset", () => {
      const thesis: PdfEmbodiment = {
        kind: "pdf",
        id: "thesis--doc",
        src: "/thesis.pdf",
        page: 25,
        pages: [1, 12, 25, 35, 42, 43, 51],
      };
      expect(pdfEmbodimentInitialPage(thesis)).toBe(25);
      expect(pdfEmbodimentInitialPage({ ...thesis, page: 99 })).toBe(1);
      expect(pdfPageNavEnabled(thesis, null)).toBe(true);
      expect(pdfPageNavEnabled({ ...thesis, pages: [1] }, null)).toBe(false);
      expect(pdfCanGoToPreviousPage(25, thesis, null)).toBe(true);
      expect(pdfCanGoToNextPage(51, thesis, null)).toBe(false);
      expect(pdfAdjacentPage(25, "next", thesis, null)).toBe(35);
      expect(pdfAdjacentPage(51, "next", thesis, null)).toBe(51);
      expect(pdfAdjacentPage(51, "prev", thesis, null)).toBe(43);
      expect(pdfAdjacentPage(1, "prev", thesis, null)).toBe(1);
    });

    it("figureCoverOverflowAxis picks the overflowing axis from frame and source aspect", () => {
      expect(figureCoverOverflowAxis(200, 100, 1)).toBe("y");
      expect(figureCoverOverflowAxis(100, 200, 1)).toBe("x");
      expect(figureCoverOverflowAxis(100, 100, 1)).toBeNull();
    });

    it("figureScrollOffsetForBackgroundPosition maps origin percents to scroll offsets", () => {
      expect(figureScrollOffsetForBackgroundPosition("y", 50, 1000, 400)).toBe(300);
      expect(figureScrollOffsetForBackgroundPosition("y", 0, 1000, 400)).toBe(0);
      expect(figureScrollOffsetForBackgroundPosition("x", 0, 800, 200)).toBe(0);
    });

    it("figureBackgroundSizeScrollAxis maps crop background-size to one scroll axis", () => {
      expect(figureBackgroundSizeScrollAxis("400% auto")).toBe("x");
      expect(figureBackgroundSizeScrollAxis("auto 1600%")).toBe("y");
      expect(figureBackgroundSizeScrollAxis("cover")).toBeNull();
      expect(figureCropScrollBackgroundSize("cover", "x")).toBe("100% auto");
      expect(figureCropScrollBackgroundSize("cover", "y")).toBe("auto 100%");
    });

    it("figureWheelZoomStep clamps ctrl+wheel zoom between cover baseline and max", () => {
      expect(figureWheelZoomStep(-100, 1)).toBeCloseTo(1.1);
      expect(figureWheelZoomStep(100, 1)).toBe(1);
      expect(figureWheelZoomStep(-100, FIGURE_WHEEL_ZOOM_MAX)).toBe(FIGURE_WHEEL_ZOOM_MAX);
      expect(figureWheelZoomStep(100, 2)).toBeCloseTo(2 / 1.1);
    });

    it("figureBackgroundSizeZoomed scales crop background-size strings", () => {
      expect(figureBackgroundSizeZoomed("400% auto", 2)).toBe("800% auto");
      expect(figureBackgroundSizeZoomed("auto 1600%", 1.5)).toBe("auto 2400%");
      expect(figureBackgroundSizeZoomed("cover", 2)).toBe("cover");
    });

    it("figureCoverScrollElementStyle grows with ctrl+wheel zoom", () => {
      const base = figureCoverScrollElementStyle(565, 350, 1200 / 1080);
      const zoomed = figureCoverScrollElementStyle(565, 350, 1200 / 1080, 2);
      expect(base.width).toBe("100%");
      expect(zoomed.width).toBe("200%");
    });

    it("figureCoverScrollContentSize enables both axes when zoomed past cover", () => {
      const square = figureCoverScrollContentSize(100, 100, 1, 2);
      expect(square.axis).toBe("both");
      const tall = figureCoverScrollContentSize(200, 100, 1, 2);
      expect(tall.axis).toBe("both");
    });

    it("mediaTeaserActive is true only when teaser is set", () => {
      expect(mediaTeaserActive(undefined)).toBe(false);
      expect(mediaTeaserActive({})).toBe(true);
      expect(mediaTeaserActive({ label: "Preview" })).toBe(true);
    });

    it("figureEmbodimentScrollEnabled defaults on and respects mosaic and scroll:false", () => {
      expect(figureEmbodimentScrollEnabled({ kind: "figure", id: "a", src: "/a.png" })).toBe(true);
      expect(
        figureEmbodimentScrollEnabled({
          kind: "figure",
          id: "a",
          src: "/a.png",
          scroll: false,
        }),
      ).toBe(false);
      expect(
        figureEmbodimentScrollEnabled({
          kind: "figure",
          id: "a",
          src: "/a.png",
          mosaic: { rows: 2, columns: 2 },
        }),
      ).toBe(false);
    });

    it("renders clipped crop figures when scroll is disabled", () => {
      const container = document.createElement("div");
      const deck: Presentation = {
        id: "scroll-off",
        chapters: [
          {
            id: "c",
            sequences: [
              {
                id: "s",
                thoughts: [
                  {
                    id: "t",
                    participants: [{ id: "figure" }],
                    embodiments: [
                      {
                        kind: "figure",
                        id: "figure--img",
                        src: "/catalogue.png",
                        crop: { x: 0, y: 0, width: 0.5, height: 1 },
                        scroll: false,
                      },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "a",
                          dispositions: [
                            {
                              participantId: "figure",
                              embodimentId: "figure--img",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.2, width: 0.3, height: 0.6 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const slot = container.querySelector('[data-id="figure"].presentation-morph-slot--figure') as HTMLElement | null;
      expect(slot).not.toBeNull();
      expect(container.querySelector(".presentation-figure-scroll-viewport")).toBeNull();
    });

    it("figureScrollOverlayThumbMetrics sizes overlay thumbs from scroll metrics", () => {
      expect(figureScrollOverlayThumbMetrics(280, 1086, 0).visible).toBe(true);
      expect(figureScrollOverlayThumbMetrics(280, 1086, 0).thumbSize).toBeGreaterThan(0);
      expect(figureScrollOverlayThumbMetrics(280, 280, 0).visible).toBe(false);
    });

    it("styles figure scroll viewports for one-axis overflow", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, resolve } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const css = readFileSync(resolve(dirname(fileURLToPath(source.url)), "🎨️.css"), "utf8");
      expect(css).toContain(".presentation-figure-scroll-viewport--axis-x");
      expect(css).toContain(".presentation-figure-scroll-viewport--axis-y");
      expect(css).toContain(".presentation-figure-scroll-scroller--axis-x");
      expect(css).toContain(".presentation-figure-scroll-scroller--axis-y");
      expect(css).toContain(".presentation-figure-scroll-scroller--axis-both");
      expect(css).toContain(".presentation-figure-scroll-viewport--overlay");
      expect(css).toContain(".presentation-figure-scroll-bar-thumb");
      expect(css).toContain(".presentation-figure-scroll-media");
      expect(css).toContain(".presentation-figure-scroll-content");
      expect(css).toContain("--scrollbar-thumb-active");
      expect(css).toContain(".presentation-figure-scroll-bar:hover .presentation-figure-scroll-bar-thumb");
      expect(css).toContain(".presentation-figure-scroll-bar--dragging .presentation-figure-scroll-bar-thumb");
      expect(css).toContain(".presentation-figure-scroll-bar-thumb:hover");
      expect(css).toContain(".presentation-figure-scroll-bar-thumb:active");
      expect(css).not.toContain("scrollbar-gutter: stable");
      expect(css).toMatch(/\.presentation-morph-slot--figure:not\(\.presentation-figure-scroll-content\)[\s\S]*width:\s*100%\s*!important/);
    });

    it("uses cover for full image and uniform crop zoom for partial crops", () => {
      const crop = { x: 0, y: 0, width: 0.5, height: 1 };
      const square = figureCropBackgroundVars({ kind: "figure", src: "/catalogue.png", crop }, crop, { x: 0, y: 0, width: 0.5, height: 0.5 });
      const wide = figureCropBackgroundVars({ kind: "figure", src: "/catalogue.png", crop }, crop, { x: 0, y: 0, width: 1, height: 0.25 });
      expect(square["--presentation-figure-bg-size" as keyof typeof square]).toBe("400% auto");
      expect(wide["--presentation-figure-bg-size" as keyof typeof wide]).toBe("1600% auto");
      expect(wide["--presentation-figure-bg-position" as keyof typeof wide]).toBe("0% 50%");
      expect(square["--presentation-figure-bg-position" as keyof typeof square]).toBe("0% 50%");
    });

    it("uses CSS cover for full catalogue in the catalogue frame", () => {
      const sourceAspect = 1222 / 896;
      const crop = { x: 0, y: 0, width: 1, height: 1 };
      const frame = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
      const vars = figureCropBackgroundVars({ kind: "figure", src: "/🖼️bauteilbörse.png", crop, sourceAspect }, crop, frame);
      expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe("cover");
      expect(vars["--presentation-figure-bg-position" as keyof typeof vars]).toBe("50% 50%");
    });

    it("uses windowed mosaic for rest and grid and centered crop for morph on catalogue-to-focus", () => {
      const sourceAspect = 1222 / 896;
      const catalogueFrame = { x: 0.127, y: 0.2, width: 0.746, height: 0.75 };
      const mosaic = { rows: 3, columns: 5, frame: catalogueFrame };
      const crop = {
        x: catalogueFrame.x + catalogueFrame.width / 5,
        y: catalogueFrame.y + catalogueFrame.height / 3,
        width: catalogueFrame.width / 5,
        height: catalogueFrame.height / 3,
      };
      const gridFrame = { x: 0.127, y: 0.2, width: 0.1492, height: 0.1823 };
      const focusFrame = { x: 0.1, y: 0.2, width: 0.15, height: 0.18 };
      const labelFrame = { x: 0.6, y: 0.44, width: 0.24, height: 0.12 };
      const windowed = mosaicWindowedCoverVars({ column: 1, row: 1 }, mosaic, catalogueFrame, sourceAspect);
      const morphPos = figureCropBackgroundPosition(crop);
      const morphSize = figureCropBackgroundSize(crop, labelFrame, sourceAspect);
      const vars = figureCropBackgroundVars(
        {
          kind: "figure",
          id: "tile-figure",
          src: "/🖼️bauteilbörse.png",
          crop,
          sourceAspect,
          mosaic,
        },
        crop,
        focusFrame,
        labelFrame,
        gridFrame,
      );
      expect(vars["--presentation-figure-bg-size" as keyof typeof vars]).toBe(windowed.size);
      expect(vars["--presentation-figure-bg-position" as keyof typeof vars]).toBe(`${windowed.posX}% ${windowed.posY}%`);
      expect(vars["--presentation-figure-bg-grid-size" as keyof typeof vars]).toBe(windowed.size);
      expect(vars["--presentation-figure-bg-grid-position" as keyof typeof vars]).toBe(`${windowed.posX}% ${windowed.posY}%`);
      expect(vars["--presentation-figure-bg-size-morph" as keyof typeof vars]).toBe(morphSize);
      expect(vars["--presentation-figure-bg-position-morph" as keyof typeof vars]).toBe(`${morphPos.posX}% ${morphPos.posY}%`);
    });

    it("mosaic windowed cover uses columns×100% width for catalogue 3×5 on a wide slide", () => {
      const sourceAspect = 1222 / 896;
      const frame = { x: 0.127, y: 0.1, width: 0.746, height: 0.75 };
      const slideAspect = 960 / 700;
      const vars = mosaicWindowedCoverVars({ column: 0, row: 1 }, { rows: 3, columns: 5 }, frame, sourceAspect, slideAspect);
      expect(vars.size).toBe("500% auto");
      expect(vars.posX).toBe(0);
    });

    it("sets revealMorphFromMorphToFrame from the previous slide morphTo slots", () => {
      const scope = buildResolutionScope([
        {
          participants: [{ id: "tile" }],
          embodiments: [
            {
              kind: "figure",
              id: "tile-figure",
              src: "/a.png",
              crop: { x: 0, y: 0, width: 0.5, height: 1 },
            },
          ],
        },
      ]);
      const gridFrame = { x: 0.1, y: 0.1, width: 0.2, height: 0.3 };
      const focusFrame = { x: 0.4, y: 0.2, width: 0.15, height: 0.18 };
      const resolved = resolveRevealArrangement(
        scope,
        {
          id: "focus",
          dispositions: [
            {
              participantId: "tile",
              embodimentId: "tile-figure",
              emphasis: "active",
              position: focusFrame,
            },
          ],
        },
        {
          previousSlide: {
            arrangement: {
              id: "catalogue",
              dispositions: [
                {
                  participantId: "catalogue",
                  embodimentId: "catalogue--figure",
                  morphTo: [{ participantId: "tile", embodimentId: "tile-figure", position: gridFrame }],
                },
              ],
            },
          },
        },
      );
      expect(resolved.find((entry) => entry.participant.id === "tile")?.revealMorphFromMorphToFrame).toEqual(gridFrame);
    });

    it("assigns windowed mosaic cover per split tile", () => {
      const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
      const mosaic = { rows: 2, columns: 2, frame };
      const tiles = splitFigureGrid({ rows: 2, columns: 2, frame });
      const embodiment = {
        kind: "figure" as const,
        src: "/catalogue.png",
        mosaic,
      };
      const expected = tiles.map((tile, index) => {
        const column = index % 2;
        const row = Math.floor(index / 2);
        return mosaicWindowedCoverVars({ column, row }, mosaic, frame, 1);
      });
      const varsList = tiles.map((tile) => figureCropBackgroundVars(embodiment, tile.crop, tile.position));
      expect(varsList.map((vars) => vars["--presentation-figure-bg-size" as keyof typeof vars])).toEqual(expected.map((entry) => entry.size));
      expect(varsList.map((vars) => vars["--presentation-figure-bg-position" as keyof typeof vars])).toEqual(expected.map((entry) => `${entry.posX}% ${entry.posY}%`));
    });

    it("clearRevealAutoAnimateInlineLayout removes only FLIP transform from auto-animate targets", () => {
      const deckEl = document.createElement("div");
      const target = document.createElement("div");
      target.dataset.autoAnimateTarget = "0";
      target.style.left = "65%";
      target.style.transform = "translate(10px, 20px) scale(2)";
      deckEl.appendChild(target);
      clearRevealAutoAnimateInlineLayout(deckEl);
      expect(target.style.left).toBe("65%");
      expect(target.style.transform).toBe("");
    });

    it("finalizeRevealAutoAnimateRestState clears running so morph sources rest hidden", () => {
      const deckEl = document.createElement("div");
      deckEl.className = "reveal";
      const slide = document.createElement("section");
      slide.classList.add("present");
      slide.setAttribute("data-auto-animate", "running");
      slide.classList.add(PRESENTATION_MANY_TO_ONE_MORPH_CLASS);
      const morphSource = document.createElement("div");
      morphSource.className = "presentation-target-ghost";
      morphSource.dataset.autoAnimateTarget = "0";
      slide.appendChild(morphSource);
      deckEl.appendChild(slide);
      document.body.appendChild(deckEl);
      finalizeRevealAutoAnimateRestState(deckEl);
      expect(slide.getAttribute("data-auto-animate")).toBe("");
      expect(slide.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
      expect(morphSource.hasAttribute("data-auto-animate-target")).toBe(false);
      deckEl.remove();
    });

    it("matches auto-animate targets only by data-id", () => {
      const fromSlide = document.createElement("section");
      fromSlide.innerHTML = '<div data-id="catalogue-col1" class="presentation-morph-slot--figure"><h2>Rippenplatte</h2></div>';
      const toSlide = document.createElement("section");
      toSlide.innerHTML = '<div data-id="catalogue-col1" class="presentation-morph-slot--label"><h2>Rippenplatte</h2></div>';
      const host: AutoAnimateMatcherHost = {
        findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
          for (const element of fromScope.querySelectorAll<HTMLElement>(selector)) {
            const key = serializer(element);
            const toElement = toScope.querySelector<HTMLElement>(`${selector}[data-id="${element.getAttribute("data-id")}"]`);
            if (toElement) {
              pairs.push({ from: element, to: toElement });
            }
          }
        },
      };
      const pairs = presentationAutoAnimateMatcher.call(host, fromSlide, toSlide);
      expect(pairs).toHaveLength(1);
      expect(pairs[0]?.from.getAttribute("data-id")).toBe("catalogue-col1");
      expect(pairs[0]?.from.classList.contains("presentation-morph-slot--figure")).toBe(true);
    });

    it("detects reveal slide auto-animate pending or running on section or deck", () => {
      const section = document.createElement("section");
      const deck = document.createElement("div");
      deck.className = "reveal";
      deck.appendChild(section);
      expect(isRevealSlideAutoAnimating(section)).toBe(false);
      expect(isSectionAutoAnimating(section)).toBe(false);
      section.setAttribute("data-auto-animate", "pending");
      expect(isSectionAutoAnimating(section)).toBe(true);
      expect(isRevealSlideAutoAnimating(section)).toBe(true);
      section.setAttribute("data-auto-animate", "");
      const other = document.createElement("section");
      other.setAttribute("data-auto-animate", "running");
      deck.appendChild(other);
      expect(isSectionAutoAnimating(section)).toBe(false);
      expect(isRevealSlideAutoAnimating(section)).toBe(true);
    });

    it("detects consecutive slides in the same auto-animate run", () => {
      const focus = document.createElement("section");
      focus.setAttribute("data-auto-animate-id", "medien--m0");
      const labels = document.createElement("section");
      labels.setAttribute("data-auto-animate-id", "medien--m0");
      const overview = document.createElement("section");
      overview.setAttribute("data-auto-animate-id", "medien--m1");
      expect(slidesShareAutoAnimateId(focus, labels)).toBe(true);
      expect(slidesShareAutoAnimateId(focus, overview)).toBe(false);
    });

    it("clears settled state when arriving on a slide and prepares it before morph to listed targets", () => {
      const deckEl = document.createElement("div");
      deckEl.className = "reveal";
      const focus = document.createElement("section");
      focus.setAttribute("title", "catalogue-focus");
      focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
      focus.classList.add("presentation-arrangement--positioned");
      deckEl.appendChild(focus);
      focus.classList.add("presentation-arrangement--settled");
      syncArrangementSettledState(deckEl, focus, null);
      expect(focus.classList.contains("presentation-arrangement--settled")).toBe(false);
      const labels = document.createElement("section");
      labels.setAttribute("title", "catalogue-labels");
      deckEl.appendChild(labels);
      prepareArrangementBeforeAutoAnimate(focus, labels);
      expect(focus.classList.contains("presentation-arrangement--settled")).toBe(true);
      expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
      expect(labels.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(true);
    });

    it("syncManyToOneGhostMorphFramesFromDom uses live source tile geometry on target ghosts", () => {
      const deckEl = document.createElement("div");
      deckEl.className = "reveal";
      const focus = document.createElement("section");
      focus.setAttribute("title", "catalogue-focus");
      focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
      focus.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
      const labels = document.createElement("section");
      labels.setAttribute("title", "catalogue-labels");
      labels.style.cssText = "position:relative;width:var(--layout-deck-width);height:33.75rem;";
      const source = document.createElement("div");
      source.className = "presentation-interactive-disposition presentation-interactive-disposition--canvas-framed";
      source.setAttribute("data-id", "Stütze");
      Object.assign(source.style, {
        position: "absolute",
        left: "20%",
        top: "10%",
        width: "15%",
        height: "70%",
      });
      const ghost = document.createElement("div");
      ghost.className = "presentation-interactive-disposition presentation-target-ghost presentation-interactive-disposition--canvas-framed";
      ghost.setAttribute("data-id", "Stütze");
      Object.assign(ghost.style, {
        position: "absolute",
        left: "60%",
        top: "44%",
        width: "24%",
        height: "12%",
      });
      const slot = document.createElement("div");
      slot.className = "presentation-morph-slot presentation-morph-slot--figure";
      slot.dataset.presentationMorphCrop = JSON.stringify({ x: 0, y: 0, width: 0.5, height: 1 });
      ghost.append(slot);
      focus.append(source);
      labels.append(ghost);
      deckEl.append(focus, labels);
      document.body.append(deckEl);
      syncManyToOneGhostMorphFramesFromDom(focus, labels);
      expect(ghost.style.getPropertyValue("--presentation-morph-frame-left")).toBe("20%");
      expect(ghost.style.getPropertyValue("--presentation-morph-frame-top")).toBe("10%");
      expect(ghost.style.getPropertyValue("--presentation-morph-frame-width")).toBe("15%");
      expect(ghost.style.getPropertyValue("--presentation-morph-frame-height")).toBe("70%");
      expect(ghost.style.getPropertyValue("--presentation-frame-left")).toBe("60%");
      deckEl.remove();
    });

    it("does not mark many-to-one morph for catalogue to focus auto-animate", () => {
      const deckEl = document.createElement("div");
      deckEl.className = "reveal";
      const catalogue = document.createElement("section");
      catalogue.setAttribute("title", "catalogue");
      const focus = document.createElement("section");
      focus.setAttribute("title", "catalogue-focus");
      focus.setAttribute("data-settle-before-morph-to", "catalogue-labels");
      deckEl.append(catalogue, focus);
      expect(isManyToOneMorphTransition(catalogue, focus)).toBe(false);
      prepareArrangementBeforeAutoAnimate(catalogue, focus);
      expect(focus.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
      expect(catalogue.classList.contains(PRESENTATION_MANY_TO_ONE_MORPH_CLASS)).toBe(false);
    });

    it("renders expanded source ghosts for one-to-many morphTo", () => {
      const deck: Presentation = {
        id: "source-ghost",
        name: "Source ghost",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "split",
                    participants: [{ id: "whole" }, { id: "tile-a" }],
                    embodiments: [
                      { kind: "figure", id: "whole--figure", src: "/catalogue.png" },
                      {
                        kind: "figure",
                        id: "tile-a--figure",
                        src: "/catalogue.png",
                        crop: { x: 0, y: 0, width: 0.5, height: 1 },
                      },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "whole",
                          dispositions: [
                            {
                              participantId: "whole",
                              embodimentId: "whole--figure",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.1, width: 0.8, height: 0.8 },
                              morphTo: [
                                {
                                  participantId: "tile-a",
                                  position: { x: 0.1, y: 0.1, width: 0.35, height: 0.8 },
                                },
                              ],
                            },
                          ],
                        },
                        transition: { kind: "morph" },
                      },
                      {
                        arrangement: {
                          id: "tiles",
                          dispositions: [
                            {
                              participantId: "tile-a",
                              embodimentId: "tile-a--figure",
                              emphasis: "active",
                              position: { x: 0.05, y: 0.2, width: 0.4, height: 0.6 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const wholeSlide = container.querySelector('section[title="whole"]') as HTMLElement;
      const sourceGhost = wholeSlide.querySelector('.presentation-interactive-disposition.presentation-source-ghost[data-id="tile-a"]');
      expect(sourceGhost).toBeTruthy();
      expect(wholeSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
    });

    it("renders positioned labels with data-id on the morph slot frame", () => {
      const deck: Presentation = {
        id: "label-slot",
        name: "Labels",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "labels",
                    participants: [{ id: "catalogue-col1" }],
                    embodiments: [
                      {
                        kind: "text",
                        id: "catalogue-col1--label",
                        lines: ["Rippenplatte"],
                        level: "heading",
                        morphRoot: "heading-line",
                      },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "labels",
                          dispositions: [
                            {
                              participantId: "catalogue-col1",
                              embodimentId: "catalogue-col1--label",
                              emphasis: "active",
                              position: { x: 0.38, y: 0.12, width: 0.24, height: 0.24 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const slot = container.querySelector('[data-id="catalogue-col1"].presentation-morph-slot--label');
      expect(slot?.querySelector("h2")?.textContent).toBe("Rippenplatte");
      expect(slot?.querySelector("h2[data-id]")).toBeNull();
    });

    it("relaxes Tailwind preflight [hidden] so reveal's inline display drives slide visibility", () => {
      const style = document.createElement("style");
      style.textContent = '[hidden]:where(:not([hidden="until-found"])) { display: none; color: red; }';
      document.head.appendChild(style);
      const hiddenRule = (style.sheet as CSSStyleSheet).cssRules[0] as CSSStyleRule;
      expect(hiddenRule.style.getPropertyValue("display")).toBe("none");
      relaxHiddenPreflight();
      expect(hiddenRule.style.getPropertyValue("display")).toBe("");
      expect(hiddenRule.style.getPropertyValue("color")).toBe("red");
      style.remove();
    });

    it("does not navigate from bookmark query params", () => {
      expect(readPresentationSlideIndicesFromUrl("#/?sequence=main&thought=intro&slide=goal")).toEqual({
        h: 0,
        v: 0,
      });
      expect(readPresentationSlideIndicesFromUrl("#/0/2?sequence=main&thought=intro&slide=goal")).toEqual({
        h: 0,
        v: 2,
      });
      history.replaceState(null, "", "/presentation");
    });
  });

  describe("syncPresentationSlideUrl", () => {
    const sampleDeck = intro({
      language: "de",
      title: { full: ["A"], short: "Short" },
      description: { full: ["D"], short: "D short" },
      goal: ["G"],
      authors: { lines: [[{ name: "Alice" }]] },
      affiliations: {
        steps: [
          [{ mark: "a", name: "Faculty" }],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
          [
            { mark: "a", name: "Faculty" },
            { mark: "1", name: "Uni" },
          ],
        ],
      },
    });

    it("uses German bookmark labels for de intro decks", () => {
      const deck = intro({
        language: "de",
        title: { full: ["A"], short: "Short" },
        description: { full: ["D"], short: "D short" },
        goal: ["G"],
        authors: { lines: [[{ name: "Alice" }]] },
        affiliations: {
          steps: [
            [{ mark: "a", name: "Faculty" }],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
          ],
        },
      });
      history.replaceState(null, "", "/deck");
      for (const slide of collectPresentationSlides(deck)) {
        if (slide.slide === "Ziel") {
          syncPresentationSlideUrl(deck, { h: slide.h, v: slide.v });
          break;
        }
      }
      const params = new URLSearchParams(new URL(window.location.href).hash.split("?")[1] ?? "");
      expect(params.get("kapitel")).toBe("Hauptteil");
      expect(params.get("sequenz")).toBe("Einführung");
      expect(params.get("gedanke")).toBe("Einleitung");
      expect(params.get("folie")).toBe("Ziel");
      history.replaceState(null, "", "/deck");
    });

    it("writes chapter, sequence, thought, and slide bookmark params after the hash path", () => {
      history.replaceState(null, "", "/deck");
      const goalSlide = collectPresentationSlides(sampleDeck).find((slide) => slide.slide === "Ziel");
      syncPresentationSlideUrl(sampleDeck, { h: goalSlide!.h, v: goalSlide!.v });
      const url = new URL(window.location.href);
      expect(url.search).toBe("");
      expect(url.hash).toContain("folie=Ziel");
      history.replaceState(null, "", "/deck");
    });

    it("readPresentationSlideIndicesFromUrl ignores bookmark query params", () => {
      expect(readPresentationSlideIndicesFromUrl("#/1/3")).toEqual({ h: 1, v: 3 });
      expect(readPresentationSlideIndicesFromUrl("")).toEqual({ h: 0, v: 0 });
    });
  });

  describe("reveal text morph", () => {
    it("pairs intro leaf text nodes, not disposition wrappers", () => {
      const fromSlide = document.createElement("section");
      fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const toSlide = document.createElement("section");
      toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const fromWrap = document.createElement("div");
      fromWrap.className = "presentation-interactive-disposition";
      const fromHeading = document.createElement("h2");
      fromHeading.setAttribute("data-id", "description");
      fromHeading.textContent = "Long description line";
      fromWrap.append(fromHeading);
      fromSlide.append(fromWrap);
      const toWrap = document.createElement("div");
      toWrap.className = "presentation-interactive-disposition";
      const toHeading = document.createElement("h2");
      toHeading.setAttribute("data-id", "description");
      toHeading.textContent = "Short";
      toWrap.append(toHeading);
      toSlide.append(toWrap);
      expect(isRevealAutoAnimatePairSource(fromWrap)).toBe(false);
      expect(isRevealAutoAnimatePairSource(fromHeading)).toBe(true);
      const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), fromSlide, toSlide);
      expect(pairs).toEqual([
        {
          from: fromHeading,
          to: toHeading,
        },
      ]);
    });

    it("pairs short and full heading-block lines via a shared --0 suffix", () => {
      const fromSlide = document.createElement("section");
      fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const toSlide = document.createElement("section");
      toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const fromHeading = document.createElement("h2");
      fromHeading.setAttribute("data-id", "description--0");
      fromHeading.textContent = "Short";
      fromSlide.append(fromHeading);
      const toLine0 = document.createElement("h2");
      toLine0.setAttribute("data-id", "description--0");
      toLine0.textContent = "Long line one";
      const toLine1 = document.createElement("h2");
      toLine1.setAttribute("data-id", "description--1");
      toLine1.textContent = "Long line two";
      toSlide.append(toLine0, toLine1);
      const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), fromSlide, toSlide);
      expect(pairs).toEqual([
        {
          from: fromHeading,
          to: toLine0,
        },
      ]);
    });

    it("uses stock reveal auto-animate options on intro text pairs", () => {
      const fromSlide = document.createElement("section");
      fromSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const toSlide = document.createElement("section");
      toSlide.className = "presentation-arrangement--interactive presentation-arrangement--intro";
      const fromHeading = document.createElement("h2");
      fromHeading.setAttribute("data-id", "description--0");
      fromSlide.append(fromHeading);
      const toHeading = document.createElement("h2");
      toHeading.setAttribute("data-id", "description--0");
      toSlide.append(toHeading);
      const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), fromSlide, toSlide);
      expect(pairs[0]?.options).toBeUndefined();
    });

    it("uses slide-local measure options on non-intro text pairs", () => {
      const fromSlide = document.createElement("section");
      fromSlide.className = "presentation-arrangement--interactive";
      const toSlide = document.createElement("section");
      toSlide.className = "presentation-arrangement--interactive";
      const heading = document.createElement("h2");
      heading.setAttribute("data-id", "label");
      expect(revealTextAutoAnimatePairOptions(heading, fromSlide, toSlide)).toEqual({
        scale: false,
        measure: revealInkMeasureForAutoAnimate,
      });
    });

    it("measures morph text relative to the slide box, not the viewport", () => {
      const section = document.createElement("section");
      section.className = "presentation-arrangement--interactive";
      const heading = document.createElement("h2");
      heading.textContent = "Plattform";
      section.append(heading);
      document.body.append(section);
      section.getBoundingClientRect = () => new DOMRect(500, 0, 960, 700);
      heading.getBoundingClientRect = () => new DOMRect(780, 300, 400, 48);
      const viewport = heading.getBoundingClientRect();
      const measured = revealInkMeasureForAutoAnimate(heading);
      document.body.removeChild(section);
      expect(measured.x).not.toBeCloseTo(viewport.left);
      expect(measured.x).toBeGreaterThanOrEqual(0);
      expect(measured.x + measured.width).toBeLessThanOrEqual(960);
    });
  });

  describe("presentation interaction geometry", () => {
    it("detects disposition ephemeral layout from anchor and transform", () => {
      const declared = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
      expect(dispositionHasEphemeralLayout(undefined, declared, undefined, false)).toBe(false);
      expect(dispositionHasEphemeralLayout(declared, declared, undefined, false)).toBe(false);
      expect(dispositionHasEphemeralLayout({ x: 0.3, y: 0.3, width: 0.4, height: 0.2 }, declared, undefined, false)).toBe(true);
    });

    it("detects slide ephemeral layout and reset hotspot proximity", () => {
      const declared = new Map<string, DispositionPosition | undefined>([["a", { x: 0.2, y: 0.3, width: 0.4, height: 0.2 }]]);
      expect(slideHasEphemeralLayout(new Map(), new Set(), declared)).toBe(false);
      expect(slideHasEphemeralLayout(new Map([["a", { x: 0.2, y: 0.3, width: 0.4, height: 0.2 }]]), new Set(), declared)).toBe(false);
      expect(slideHasEphemeralLayout(new Map([["a", { x: 0.3, y: 0.3, width: 0.4, height: 0.2 }]]), new Set(), declared)).toBe(true);
      expect(slideHasEphemeralLayout(new Map(), new Set(["a"]), declared)).toBe(true);
      const section = document.createElement("section");
      document.body.appendChild(section);
      section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
      expect(pointerNearSlideResetHotspot(section, 920, 20)).toBe(true);
      expect(pointerNearSlideResetHotspot(section, 100, 20)).toBe(false);
      document.body.removeChild(section);
    });

    it("detects intersection and containment", () => {
      const a = { x: 0.1, y: 0.1, width: 0.3, height: 0.3 };
      const b = { x: 0.25, y: 0.25, width: 0.3, height: 0.3 };
      const outer = { x: 0, y: 0, width: 1, height: 1 };
      expect(rectsIntersect(a, b)).toBe(true);
      expect(rectContains(outer, a)).toBe(true);
      expect(rectContains(a, b)).toBe(false);
    });

    it("applies crossing vs window marquee rules", () => {
      const inside = { x: 0.2, y: 0.2, width: 0.1, height: 0.1 };
      const partial = { x: 0.55, y: 0.55, width: 0.3, height: 0.3 };
      const crossingMarquee = normalizeMarquee({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 });
      const windowMarquee = normalizeMarquee({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 });
      expect(marqueeSelectionRule({ x: 0.7, y: 0.7 }, { x: 0.1, y: 0.1 })).toBe("crossing");
      expect(marqueeSelectionRule({ x: 0.1, y: 0.1 }, { x: 0.5, y: 0.5 })).toBe("window");
      expect(marqueeSelects(crossingMarquee, inside, "crossing")).toBe(true);
      expect(marqueeSelects(windowMarquee, inside, "window")).toBe(true);
      expect(marqueeSelects(windowMarquee, partial, "window")).toBe(false);
      expect(marqueeSelects(crossingMarquee, partial, "crossing")).toBe(true);
    });

    it("detects reveal overview from an element inside the deck", () => {
      const reveal = document.createElement("div");
      reveal.className = "reveal";
      const section = document.createElement("section");
      reveal.appendChild(section);
      expect(revealDeckInOverview(section)).toBe(false);
      reveal.classList.add("overview");
      expect(revealDeckInOverview(section)).toBe(true);
    });

    it("suppresses the reveal overview slide click after pointer gestures", () => {
      const reveal = document.createElement("div");
      reveal.className = "reveal";
      const slide = document.createElement("section");
      const inner = document.createElement("div");
      reveal.appendChild(slide);
      slide.appendChild(inner);
      let overviewClickCalls = 0;
      slide.addEventListener(
        "click",
        () => {
          overviewClickCalls += 1;
        },
        true,
      );
      suppressRevealOverviewSlideNavigation({ target: inner } as Event);
      const click = new MouseEvent("click", { bubbles: true, cancelable: true });
      inner.dispatchEvent(click);
      expect(overviewClickCalls).toBe(0);
      expect(click.defaultPrevented).toBe(true);
    });

    it("translates and resizes with minimum size", () => {
      const rect = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
      const moved = translateDispositionRect(rect, 0.1, -0.05);
      expect(moved.x).toBeCloseTo(0.3);
      expect(moved.y).toBeCloseTo(0.25);
      const narrow = { x: 0.35, y: 0.4, width: 0.3, height: 0.1 };
      const movedX = translateDispositionRect(narrow, 0.12, 0);
      expect(movedX.x).toBeCloseTo(0.47);
      expect(movedX.y).toBeCloseTo(0.4);
      const unbounded = translateDispositionRect({ x: 0.8, y: 0.5, width: 0.2, height: 0.1 }, 0.5, -0.3);
      expect(unbounded.x).toBeCloseTo(1.3);
      expect(unbounded.y).toBeCloseTo(0.2);
      const resized = resizeDispositionRect(rect, "se", 0.2, 0.1);
      expect(resized.width).toBeCloseTo(0.6);
      expect(resized.height).toBeCloseTo(0.3);
    });

    it("starts flow manipulation at zero offset with measured size", () => {
      const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
      expect(flowDispositionManipulationRect(measured, undefined)).toEqual({
        x: 0,
        y: 0,
        width: 0.3,
        height: 0.08,
      });
    });

    it("detects flow pixel-offset transforms and maps them to section frames", () => {
      const measured = { x: 0.2, y: 0.3, width: 0.25, height: 0.1 };
      const offset = { x: 96, y: 40, width: 0.25, height: 0.1 };
      expect(isFlowPixelOffsetTransform(offset, measured)).toBe(true);
      expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.25, height: 0.1 }, measured)).toBe(false);
      expect(isFlowPixelOffsetTransform({ x: 12, y: -4, width: 0.25, height: 0.1 }, measured)).toBe(true);
      expect(isFlowPixelOffsetTransform(offset, undefined)).toBe(true);
      expect(isFlowPixelOffsetTransform({ ...offset, width: 0.3 }, measured)).toBe(false);
      const section = document.createElement("section");
      section.style.width = "var(--layout-deck-width)";
      section.style.height = "var(--layout-deck-height)";
      document.body.appendChild(section);
      section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
      const sectionRect = flowPixelOffsetToSectionRect(measured, offset, section);
      document.body.removeChild(section);
      expect(sectionRect.x).toBeCloseTo(0.3);
      expect(sectionRect.y).toBeCloseTo(0.357, 2);
    });

    it("maps flow offsets to local translate pixels", () => {
      expect(flowDispositionOffsetStyle({ x: 96, y: 140, width: 0.3, height: 0.08 }).transform).toBe("translate3d(96px, 140px, 0)");
    });

    it("resolves arrangement canvas as placement container for positioned slides", () => {
      const section = document.createElement("section");
      const canvas = document.createElement("div");
      canvas.className = "presentation-arrangement-canvas";
      section.append(canvas);
      Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
      expect(dispositionPlacementContainer(section, false)).toBe(section);
      expect(dispositionPlacementContainer(section, true)).toBe(canvas);
    });

    it("maps pointer fractions via parent stack when nested reveal section has zero height", () => {
      const outer = document.createElement("section");
      const inner = document.createElement("section");
      outer.append(inner);
      Object.defineProperty(outer, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(outer, "offsetHeight", { value: 700, configurable: true });
      Object.defineProperty(inner, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(inner, "offsetHeight", { value: 0, configurable: true });
      outer.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
      inner.getBoundingClientRect = () => new DOMRect(0, 0, 960, 0);
      expect(dispositionPlacementContainer(inner, false)).toBe(outer);
      const fraction = clientToSectionFraction(inner, 480, 350, { clamp: false });
      expect(fraction.x).toBeCloseTo(0.5);
      expect(fraction.y).toBeCloseTo(0.5);
    });

    it("uses explicit chrome frame only for flow slides, not canvas-framed slides", () => {
      const rect = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
      expect(
        interactiveDispositionChromeStyle({
          selected: true,
          effectiveRect: rect,
          canvasFramed: false,
          enlarged: false,
        }),
      ).toEqual(transformFrameStyle(rect));
      expect(
        interactiveDispositionChromeStyle({
          selected: true,
          effectiveRect: rect,
          canvasFramed: true,
          enlarged: false,
        }),
      ).toBeUndefined();
    });

    it("detects flow pixel-offset transforms without measured rect", () => {
      const measured = { x: 0.2, y: 0.3, width: 0.25, height: 0.1 };
      expect(isFlowPixelOffsetTransform({ x: 12, y: -4, width: 0.25, height: 0.1 }, undefined)).toBe(true);
      expect(isFlowPixelOffsetTransform({ x: 0, y: 0, width: 0.25, height: 0.1 }, undefined)).toBe(true);
      expect(isFlowPixelOffsetTransform({ x: 0.2, y: 0.3, width: 0.25, height: 0.1 }, undefined)).toBe(false);
      expect(isFlowPixelOffsetTransform({ x: 0, y: 0, width: 0.25, height: 0.1 }, measured)).toBe(true);
    });

    it("maps flow pointer delta through drag target visual scale", () => {
      const section = document.createElement("section");
      section.className = "presentation-arrangement--interactive";
      document.body.appendChild(section);
      section.getBoundingClientRect = () => new DOMRect(0, 0, 480, 350);
      Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
      const delta = flowPointerDeltaToLocal(section, 100, 200, 150, 260);
      document.body.removeChild(section);
      expect(delta.dx).toBeCloseTo(100);
      expect(delta.dy).toBeCloseTo(120);
    });

    it("intro morph title placements have no declared slide frame", () => {
      const deck = intro({
        language: "de",
        title: { full: ["Title"], short: "T" },
        description: { full: ["D"], short: "d" },
        goal: ["G"],
        authors: { lines: [[{ name: "A" }]] },
        affiliations: {
          steps: [
            [{ mark: "a", name: "Faculty" }],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni", shortName: "U", suffix: { mark: "x", name: "Chair" } },
            ],
          ],
        },
      });
      const thought = deck.chapters[0]!.sequences[0]!.thoughts[0]!;
      const renderSlide = expandThoughtSlides(thought)[0]!;
      const scope = buildResolutionScope([thought]);
      const resolved = resolveRevealArrangement(scope, renderSlide.arrangement, {});
      const layout = buildInteractiveSlideLayout(renderSlide.id, resolved, true);
      expect(layout.placements.every((entry) => entry.sectionRect === undefined)).toBe(true);
    });

    it("maps flow drag 1:1 when reveal stack layout is tall but drag target is slide-sized", () => {
      const reveal = document.createElement("div");
      reveal.className = "reveal";
      reveal.style.setProperty("--presentation-slide-width", "960");
      reveal.style.setProperty("--presentation-slide-height", "700");
      const stack = document.createElement("section");
      const inner = document.createElement("section");
      inner.className = "presentation-arrangement--interactive";
      const content = document.createElement("div");
      content.className = "presentation-interactive-disposition__content";
      inner.append(content);
      stack.append(inner);
      reveal.append(stack);
      document.body.append(reveal);
      Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(stack, "offsetHeight", { value: 4900, configurable: true });
      Object.defineProperty(inner, "offsetWidth", { value: 0, configurable: true });
      Object.defineProperty(inner, "offsetHeight", { value: 0, configurable: true });
      Object.defineProperty(content, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(content, "offsetHeight", { value: 120, configurable: true });
      stack.getBoundingClientRect = () => new DOMRect(0, 0, 480, 350);
      content.getBoundingClientRect = () => new DOMRect(0, 280, 480, 60);
      expect(slideCoordinateRoot(inner)).toBe(inner);
      const delta = flowPointerDeltaToLocal(content, 100, 200, 120, 220);
      document.body.removeChild(reveal);
      expect(delta.dx).toBeCloseTo(40);
      expect(delta.dy).toBeCloseTo(40);
    });

    it("does not treat sub-unit flow drag offsets as normalized slide frames", () => {
      const measured = { x: 0.35, y: 0.4, width: 0.3, height: 0.08 };
      expect(isFlowPixelOffsetTransform({ x: 12, y: 4, width: 0.3, height: 0.08 }, measured)).toBe(true);
      expect(isNormalizedSlideFrame({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 })).toBe(true);
      expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, measured)).toBe(false);
      expect(isFlowPixelOffsetTransform({ x: 0.5, y: 0.2, width: 0.3, height: 0.08 }, undefined)).toBe(false);
    });

    it("rejects unusable measured fractions", () => {
      expect(isUsableMeasuredRect({ x: 0.2, y: 0.3, width: 0.1, height: 0.1 })).toBe(true);
      expect(isUsableMeasuredRect({ x: 0.2, y: 0.3, width: 0.005, height: 0.1 })).toBe(false);
    });

    it("measures flow disposition bounds from morph nodes not full-width wrapper", () => {
      const section = document.createElement("section");
      const root = document.createElement("div");
      const heading = document.createElement("h2");
      heading.textContent = "Entwerfen mit Bestand";
      root.appendChild(heading);
      section.appendChild(root);
      document.body.appendChild(section);
      section.getBoundingClientRect = () => new DOMRect(0, 0, 960, 700);
      root.getBoundingClientRect = () => new DOMRect(0, 280, 960, 120);
      heading.getBoundingClientRect = () => new DOMRect(0, 300, 960, 80);
      const measured = measureDispositionBoundsInSection(root, section);
      document.body.removeChild(section);
      expect(measured?.width).toBeLessThan(0.5);
      expect(measured?.x).toBeGreaterThan(0.1);
    });

    it("measures ink bounds relative to the disposition wrapper for selection chrome", () => {
      const root = document.createElement("div");
      const heading = document.createElement("h2");
      heading.textContent = "Title";
      root.appendChild(heading);
      document.body.appendChild(root);
      root.getBoundingClientRect = () => new DOMRect(0, 280, 960, 120);
      heading.getBoundingClientRect = () => new DOMRect(330, 300, 300, 80);
      const inWrapper = measureDispositionBoundsInContainer(root, root);
      document.body.removeChild(root);
      expect(inWrapper?.x).toBeCloseTo(330 / 960);
      expect(inWrapper?.width).toBeCloseTo(300 / 960);
      expect(inWrapper?.y).toBeCloseTo(20 / 120);
    });

    it("uses tight ink bounds for block headings", () => {
      const heading = document.createElement("h2");
      heading.textContent = "Goal line";
      document.body.appendChild(heading);
      heading.getBoundingClientRect = () => new DOMRect(0, 200, 960, 48);
      const tight = tightElementBoundsRect(heading);
      document.body.removeChild(heading);
      expect(tight).not.toBeNull();
      expect(tight!.width).toBeLessThan(960);
    });

    it("scales group members and toggles enlarge", () => {
      const a = { x: 0.1, y: 0.2, width: 0.2, height: 0.2 };
      const b = { x: 0.5, y: 0.2, width: 0.2, height: 0.2 };
      const group = groupBoundingRect([a, b]);
      expect(group?.width).toBeCloseTo(0.6);
      const grown = { x: 0, y: 0.1, width: 0.8, height: 0.3 };
      const scaledA = scaleRectWithinGroup(a, group!, grown);
      expect(scaledA.x).toBeCloseTo(0);
      const full = toggleEnlargeRect(a, undefined);
      expect(full.rect).toEqual(SLIDE_INTERACTIVE_ENLARGE_FRAME);
      expect(full.stash).toEqual(a);
      const restored = toggleEnlargeRect(full.rect, full.stash);
      expect(restored.rect).toEqual(a);
    });

    it("uses uniform min-axis scale for interactive resize content", () => {
      const baseline = { x: 0.2, y: 0.3, width: 0.4, height: 0.2 };
      const grown = { x: 0.2, y: 0.3, width: 0.6, height: 0.2 };
      expect(interactiveDispositionContentScale(grown, baseline)).toBeCloseTo(1.5);
      const stretched = { x: 0.2, y: 0.3, width: 0.6, height: 0.35 };
      expect(interactiveDispositionContentScale(stretched, baseline)).toBeCloseTo(1.5);
      expect(interactiveDispositionContentScale(baseline, baseline)).toBeNull();
      expect(interactiveDispositionContentScaleStyle(1.25).transform).toBe("scale(1.25)");
    });

    it("builds one interactive placement per tile disposition", () => {
      const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
      const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
      const scope = buildResolutionScope([{ participants: grid.participants, embodiments: grid.embodiments }]);
      const resolved = resolveArrangement(scope, {
        id: "tiles",
        dispositions: grid.dispositions,
      });
      const layout = buildInteractiveSlideLayout("slide-1", resolved, true);
      expect(layout.placements).toHaveLength(4);
      expect(layout.rowBands).toHaveLength(0);
      expect(layout.placements.every((entry) => entry.sectionRect !== undefined)).toBe(true);
      expect(layout.placements.every((entry) => entry.revealMorphId !== undefined)).toBe(true);
    });

    it("keeps reveal morph data-id on canvas placements only", () => {
      const scope = buildResolutionScope([
        {
          participants: [{ id: "title" }, { id: "description" }],
          embodiments: [
            { kind: "text", id: "title--full", lines: ["A"], level: "heading", morphRoot: "heading-line" },
            {
              kind: "text",
              id: "description--full",
              lines: ["Long description"],
              level: "heading",
              morphRoot: "heading-block",
            },
          ],
        },
      ]);
      const flow = buildInteractiveSlideLayout(
        "goal",
        resolveArrangement(scope, {
          id: "goal",
          dispositions: [
            { participantId: "title", embodimentId: "title--full", emphasis: "muted" },
            { participantId: "description", embodimentId: "description--full", emphasis: "muted" },
          ],
        }),
        true,
      );
      expect(flow.placements.every((entry) => entry.revealMorphId === undefined)).toBe(true);
      const canvas = buildInteractiveSlideLayout(
        "tiles",
        resolveArrangement(scope, {
          id: "tiles",
          dispositions: [
            {
              participantId: "description",
              embodimentId: "description--full",
              emphasis: "active",
              position: { x: 0.1, y: 0.2, width: 0.8, height: 0.3 },
            },
          ],
        }),
        true,
      );
      expect(canvas.placements[0]?.revealMorphId).toBe("description");
    });

    it("auto-animates catalogue focus into inline column labels", async () => {
      const {
        CATALOGUE_COL1,
        CATALOGUE_COL2,
        CATALOGUE_COL3,
        CATALOGUE_EMBODIMENT_COL1_LABEL,
        CATALOGUE_EMBODIMENT_COL2_LABEL,
        CATALOGUE_EMBODIMENT_COL3_LABEL,
        CATALOGUE_COLUMN_TILE_KEYS,
        CATALOGUE_SPLIT,
        catalogueFocusDispositions,
        columnLabelMorphFrom,
        inlineColumnLabelPosition,
        mediaEmbodiments,
        mediaParticipants,
      } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const deck: Presentation = {
        id: "projektetage-morph",
        name: "Morph",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "medien",
                    participants: mediaParticipants,
                    embodiments: mediaEmbodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "catalogue",
                          dispositions: CATALOGUE_SPLIT.dispositions,
                        },
                        transition: { kind: "morph" },
                      },
                      {
                        arrangement: {
                          id: "catalogue-focus",
                          settleBeforeMorphTo: ["catalogue-labels"],
                          dispositions: catalogueFocusDispositions(),
                        },
                        transition: { kind: "morph" },
                      },
                      {
                        arrangement: {
                          id: "catalogue-labels",
                          dispositions: [
                            {
                              participantId: CATALOGUE_COL1,
                              embodimentId: CATALOGUE_EMBODIMENT_COL1_LABEL,
                              emphasis: "active",
                              position: inlineColumnLabelPosition(0),
                              morphFrom: columnLabelMorphFrom("col1", inlineColumnLabelPosition(0)),
                            },
                            {
                              participantId: CATALOGUE_COL2,
                              embodimentId: CATALOGUE_EMBODIMENT_COL2_LABEL,
                              emphasis: "active",
                              position: inlineColumnLabelPosition(1),
                              morphFrom: columnLabelMorphFrom("col2", inlineColumnLabelPosition(1)),
                            },
                            {
                              participantId: CATALOGUE_COL3,
                              embodimentId: CATALOGUE_EMBODIMENT_COL3_LABEL,
                              emphasis: "active",
                              position: inlineColumnLabelPosition(2),
                              morphFrom: columnLabelMorphFrom("col3", inlineColumnLabelPosition(2)),
                            },
                          ],
                        },
                        transition: { kind: "morph" },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const revealEl = mountRoot.querySelector(".reveal") as HTMLElement;
      const focusSlide = revealEl.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = revealEl.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      expect(focusSlide).toBeTruthy();
      expect(labelSlide).toBeTruthy();
      expect(focusSlide.getAttribute("data-auto-animate-id")).toBe(labelSlide.getAttribute("data-auto-animate-id"));
      prepareArrangementBeforeAutoAnimate(focusSlide, labelSlide);
      expect(focusSlide.classList.contains("presentation-arrangement--settled")).toBe(true);
      const pairs = presentationAutoAnimateMatcher.call({ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost, focusSlide, labelSlide);
      expect(pairs.every((pair) => !elementIsSourceGhostAnchor(pair.from))).toBe(true);
      expect(pairs.every((pair) => !elementIsSourceGhostAnchor(pair.to))).toBe(true);
      expect(pairs.every((pair) => elementIsTargetGhostAnchor(pair.to))).toBe(true);
      expect(pairs.every((pair) => elementIsInteractiveFigureDisposition(pair.from))).toBe(true);
      const columnIds = [CATALOGUE_COL1, CATALOGUE_COL2, CATALOGUE_COL3];
      const tileIds = [...CATALOGUE_COLUMN_TILE_KEYS.col1, ...CATALOGUE_COLUMN_TILE_KEYS.col2, ...CATALOGUE_COLUMN_TILE_KEYS.col3];
      expect(pairs.length).toBeGreaterThanOrEqual(8);
      expect(tileIds.filter((id) => pairs.some((pair) => pair.from.getAttribute("data-id") === id && elementIsTargetGhostAnchor(pair.to))).length).toBeGreaterThanOrEqual(8);
      expect(labelSlide.querySelectorAll(".presentation-interactive-disposition.presentation-morph-target").length).toBe(3);
      expect(labelSlide.querySelectorAll(".presentation-interactive-disposition.presentation-morph-target[data-id]").length).toBe(0);
      expect(tileIds.every((id) => labelSlide.querySelector(`[data-id="${id}"]`)?.closest(".presentation-target-ghost") !== null)).toBe(true);
      expect(labelSlide.querySelectorAll(".presentation-target-ghost").length).toBeGreaterThanOrEqual(8);
      const labelSlot = inlineColumnLabelPosition(2);
      const focusStuetze = focusSlide.querySelector('[data-disposition-id^="catalogue-focus--Stütze"]') as HTMLElement | null;
      const labelGhost = labelSlide.querySelector('[data-disposition-id^="catalogue-labels--Stütze"].presentation-target-ghost') as HTMLElement | null;
      expect(focusStuetze?.style.left).not.toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.left).toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.top).toBe(`${labelSlot.y * 100}%`);
      expect(labelGhost?.style.width).toBe(`${labelSlot.width * 100}%`);
      expect(labelGhost?.style.height).toBe(`${labelSlot.height * 100}%`);
      expect(labelGhost?.classList.contains("presentation-target-ghost")).toBe(true);
      expect(labelGhost?.getAttribute("data-disposition-id")).toContain("catalogue-labels");
      mountRoot.remove();
    });

    it("styles source ghosts hidden at rest and visible during auto-animate", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, resolve } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const cssPath = resolve(dirname(fileURLToPath(source.url)), "🎨️.css");
      const css = readFileSync(cssPath, "utf8");
      expect(css).toContain(".reveal .presentation-source-ghost");
      expect(css).toContain('section[data-auto-animate="pending"] .presentation-source-ghost');
      expect(css).not.toContain('section[title="catalogue-focus"]');
    });

    it("shows catalogue full figure at rest with source ghosts and focus tiles visible", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/animate-presentation-core");
      const { deck } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
      expect(catalogueSlide.classList.contains("presentation-arrangement--settled")).toBe(false);
      expect(catalogueSlide.hasAttribute("data-settle-before-morph-to")).toBe(false);
      const catalogueFigure = catalogueSlide.querySelector('.presentation-morph-one .presentation-morph-slot--figure[role="img"]') as HTMLElement | null;
      expect(catalogueFigure?.style.backgroundImage).toContain("bauteilb");
      expect(catalogueSlide.querySelectorAll(".presentation-morph-one").length).toBe(1);
      const sourceGhosts = catalogueSlide.querySelectorAll(".presentation-interactive-disposition.presentation-source-ghost");
      expect(sourceGhosts.length).toBe(10);
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const focusSlots = focusSlide.querySelectorAll(".presentation-morph-slot--figure");
      expect(focusSlots.length).toBe(10);
      for (const slot of focusSlots) {
        expect(slot.classList.contains("presentation-target-ghost")).toBe(false);
        expect(slot.classList.contains("presentation-source-ghost")).toBe(false);
        const backgroundImage = (slot as HTMLElement).style.backgroundImage;
        expect(backgroundImage.length).toBeGreaterThan(0);
        expect(backgroundImage).toContain("bauteilb");
      }
      mountRoot.remove();
    });

    it("renders target ghosts but no source ghosts on focus and labels slides", async () => {
      const { deck } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = mountRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      expect(focusSlide.querySelectorAll(".presentation-source-ghost").length).toBe(0);
      expect(labelSlide.querySelectorAll(".presentation-source-ghost").length).toBe(0);
      expect(labelSlide.querySelectorAll(".presentation-target-ghost").length).toBeGreaterThanOrEqual(8);
      for (const ghost of labelSlide.querySelectorAll<HTMLElement>(".presentation-interactive-disposition.presentation-target-ghost")) {
        expect(ghost.style.pointerEvents).toBe("none");
        expect(ghost.getAttribute("aria-hidden")).toBe("true");
      }
      mountRoot.remove();
    });

    it("puts reveal data-id on catalogue tile wrappers for catalogue-to-focus morph", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/animate-presentation-core");
      const { deck, CATALOGUE_FOCUS_TILES } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      expect(catalogueRef).toBeDefined();
      expect(focusRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const tileId = CATALOGUE_FOCUS_TILES[0]!.participantId;
      const catalogueTile = catalogueSlide.querySelector(`.presentation-interactive-disposition[data-id="${tileId}"]`) as HTMLElement;
      const focusTile = focusSlide.querySelector(`.presentation-interactive-disposition[data-id="${tileId}"]`) as HTMLElement;
      expect(catalogueTile).toBeTruthy();
      expect(focusTile).toBeTruthy();
      expect(catalogueTile.querySelector(`[data-id="${tileId}"]`)).toBeNull();
      expect(catalogueSlide.getAttribute("data-auto-animate-id")).toBe(focusSlide.getAttribute("data-auto-animate-id"));
      mountRoot.remove();
    });

    it("auto-animates catalogue tiles into focus layout", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/animate-presentation-core");
      const { deck, CATALOGUE_FOCUS_TILES } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      expect(catalogueRef).toBeDefined();
      expect(focusRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const catalogueSlide = mountRoot.querySelector('section[title="catalogue"]') as HTMLElement;
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      expect(catalogueSlide.getAttribute("data-auto-animate-id")).toBe(focusSlide.getAttribute("data-auto-animate-id"));
      expect(catalogueSlide.querySelectorAll(".presentation-interactive-disposition.presentation-source-ghost").length).toBe(10);
      catalogueSlide.setAttribute("data-auto-animate", "pending");
      const pairs = presentationAutoAnimateMatcher.call({ findAutoAnimateMatches: () => {} } as AutoAnimateMatcherHost, catalogueSlide, focusSlide);
      const componentTileIds = CATALOGUE_FOCUS_TILES.map((tile) => tile.participantId);
      expect(componentTileIds.every((id) => pairs.some((pair) => pair.from.getAttribute("data-id") === id))).toBe(true);
      mountRoot.remove();
    });

    it("places catalogue-labels target ghosts at inline label frames", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/animate-presentation-core");
      const { deck, inlineColumnLabelPosition } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      const labelRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilbeschriftungen");
      expect(focusRef).toBeDefined();
      expect(labelRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      act(() => {
        mountPresentation(mountRoot, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      await new Promise((resolve) => setTimeout(resolve, 100));
      const focusSlide = mountRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = mountRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      const labelSlot = inlineColumnLabelPosition(2);
      const focusStuetze = focusSlide.querySelector('[data-disposition-id^="catalogue-focus--Stütze"]') as HTMLElement | null;
      const labelGhost = labelSlide.querySelector('[data-disposition-id^="catalogue-labels--Stütze"].presentation-target-ghost') as HTMLElement | null;
      expect(focusStuetze?.style.left).not.toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.left).toBe(`${labelSlot.x * 100}%`);
      expect(labelGhost?.style.top).toBe(`${labelSlot.y * 100}%`);
      expect(labelGhost?.style.width).toBe(`${labelSlot.width * 100}%`);
      expect(labelGhost?.style.height).toBe(`${labelSlot.height * 100}%`);
      mountRoot.remove();
    });

    it("fires reveal auto-animate when advancing projektetage focus to labels", async () => {
      const { collectPresentationSlides } = await import("@semio-tech/animate-presentation-core");
      const { deck } = await import("@semio-tech/mit-bestand-praesentation-projektetage-spec");
      const catalogueRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilkatalog");
      const focusRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilarten");
      const labelRef = collectPresentationSlides(deck).find((slide) => slide.slide === "Bauteilbeschriftungen");
      expect(catalogueRef).toBeDefined();
      expect(focusRef).toBeDefined();
      expect(labelRef).toBeDefined();
      const mountRoot = document.createElement("div");
      document.body.appendChild(mountRoot);
      let revealApi: Reveal.Api | undefined;
      let autoAnimateCount = 0;
      act(() => {
        mountPresentation(mountRoot, deck, {
          hash: false,
          slideNumber: false,
          surfaceChrome: false,
          onRevealReady: (api) => {
            revealApi = api;
          },
        });
      });
      const revealRoot = mountRoot.querySelector(".reveal") as HTMLElement;
      revealRoot.addEventListener("autoanimate", () => {
        autoAnimateCount += 1;
      });
      await new Promise<void>((resolve) => {
        const start = Date.now();
        const wait = (): void => {
          if (revealApi) {
            resolve();
            return;
          }
          if (Date.now() - start > 5000) {
            throw new Error("reveal.js did not become ready.");
          }
          setTimeout(wait, 50);
        };
        wait();
      });
      await revealApi!.slide(catalogueRef!.h, catalogueRef!.v);
      await new Promise((resolve) => setTimeout(resolve, 50));
      await revealApi!.slide(focusRef!.h, focusRef!.v);
      await new Promise((resolve) => setTimeout(resolve, 50));
      const afterCatalogueToFocus = autoAnimateCount;
      await revealApi!.slide(labelRef!.h, labelRef!.v);
      const autoAnimateDurationMs = (typeof revealApi!.getConfig().autoAnimateDuration === "number" ? revealApi!.getConfig().autoAnimateDuration : 1) * 1000 + 120;
      await new Promise((resolve) => setTimeout(resolve, autoAnimateDurationMs));
      expect(afterCatalogueToFocus).toBeGreaterThan(0);
      expect(autoAnimateCount).toBeGreaterThan(afterCatalogueToFocus);
      const focusSlide = revealRoot.querySelector('section[title="catalogue-focus"]') as HTMLElement;
      const labelSlide = revealRoot.querySelector('section[title="catalogue-labels"]') as HTMLElement;
      expect(focusSlide.getAttribute("data-auto-animate-id")).toBe(labelSlide.getAttribute("data-auto-animate-id"));
      expect(labelSlide.getAttribute("data-auto-animate")).not.toBe("running");
      expect(labelSlide.querySelector("[data-auto-animate-target]")).toBeNull();
      expect(labelSlide.querySelectorAll(".presentation-target-ghost[data-auto-animate-target]").length).toBe(0);
      mountRoot.remove();
    });
  });

  describe("presentation interaction dom", () => {
    let container: HTMLDivElement;

    const testPdfCanvasPort: PdfCanvasPort = {
      load() {
        const document: PdfCanvasDocument = {
          numPages: 3,
          getPage: async () => ({
            getViewport: ({ scale }) => ({ width: 595 * scale, height: 842 * scale }),
            render: () => ({ promise: Promise.resolve(), cancel: () => undefined }),
            cleanup: () => undefined,
          }),
          destroy: () => undefined,
        };
        return { promise: Promise.resolve(document), destroy: () => undefined };
      },
    };

    const positionedDeck: Presentation = {
      id: "interactive-dom",
      name: "Interactive DOM",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "placed",
                  participants: [{ id: "box" }],
                  embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
                  slides: [
                    {
                      arrangement: {
                        id: "placed",
                        dispositions: [
                          {
                            participantId: "box",
                            embodimentId: "box--main",
                            emphasis: "active",
                            position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
                          },
                        ],
                      },
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    };

    const twoSlideDeck: Presentation = {
      id: "interactive-two-slide",
      name: "Interactive Two Slide",
      chapters: [
        {
          id: "main",
          sequences: [
            {
              id: "main",
              thoughts: [
                {
                  id: "pair",
                  participants: [{ id: "box" }],
                  embodiments: [{ kind: "text", id: "box--main", lines: ["Hello"], level: "body" }],
                  slides: [
                    {
                      arrangement: {
                        id: "alpha",
                        dispositions: [
                          {
                            participantId: "box",
                            embodimentId: "box--main",
                            emphasis: "active",
                            position: { x: 0.2, y: 0.3, width: 0.4, height: 0.2 },
                          },
                        ],
                      },
                    },
                    {
                      arrangement: {
                        id: "beta",
                        dispositions: [
                          {
                            participantId: "box",
                            embodimentId: "box--main",
                            emphasis: "active",
                            position: { x: 0.55, y: 0.55, width: 0.3, height: 0.15 },
                          },
                        ],
                      },
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    };

    const pointerClick = (target: Element, clientX = 20, clientY = 20): void => {
      target.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, cancelable: true, button: 0, clientX, clientY }));
      target.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, cancelable: true, button: 0, clientX, clientY }));
    };

    const mockClientRect = (element: Element, left: number, top: number, width: number, height: number): void => {
      const rect = {
        left,
        top,
        width,
        height,
        right: left + width,
        bottom: top + height,
        x: left,
        y: top,
        toJSON: () => ({}),
      };
      element.getBoundingClientRect = () => rect as DOMRect;
    };

    const waitForPdfCanvas = async (root: Element, predicate: (canvas: HTMLCanvasElement) => boolean): Promise<HTMLCanvasElement> => {
      for (let attempt = 0; attempt < 40; attempt += 1) {
        const canvas = root.querySelector<HTMLCanvasElement>(".presentation-media-pdf canvas");
        if (canvas && predicate(canvas)) {
          return canvas;
        }
        await act(async () => {
          await new Promise((resolve) => setTimeout(resolve, 0));
        });
      }
      throw new Error("PDF canvas did not reach the expected state.");
    };

    const pointerDrag = (target: Element, fromX: number, fromY: number, toX: number, toY: number, pointerId = 1): void => {
      target.dispatchEvent(
        new PointerEvent("pointerdown", {
          bubbles: true,
          cancelable: true,
          button: 0,
          clientX: fromX,
          clientY: fromY,
          pointerId,
        }),
      );
      window.dispatchEvent(
        new PointerEvent("pointermove", {
          bubbles: true,
          cancelable: true,
          clientX: toX,
          clientY: toY,
          pointerId,
        }),
      );
      window.dispatchEvent(
        new PointerEvent("pointerup", {
          bubbles: true,
          cancelable: true,
          clientX: toX,
          clientY: toY,
          pointerId,
        }),
      );
    };

    beforeEach(() => {
      container = document.createElement("div");
      document.body.appendChild(container);
      setPdfCanvasPort(testPdfCanvasPort);
    });

    afterEach(() => {
      unmountPresentation();
      resetPdfCanvasPort();
      container.remove();
    });

    it("renders data-disposition-id on every disposition", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      expect(container.querySelectorAll("[data-disposition-id]").length).toBe(1);
    });

    it("renders one interactive wrapper per tile disposition", () => {
      const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
      const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
      const deck: Presentation = {
        id: "split-interactive",
        name: "Split Interactive",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "split",
                    participants: grid.participants,
                    embodiments: grid.embodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "tiles",
                          dispositions: grid.dispositions,
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      expect(container.querySelectorAll("[data-disposition-id]").length).toBe(4);
      expect(container.querySelectorAll(".presentation-interactive-row-band").length).toBe(0);
    });

    it("selects on click and deselects on empty slide click", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
      act(() => {
        pointerClick(layer);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      act(() => {
        pointerClick(canvas, 8, 8);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      const reveal = container.querySelector(".reveal") as HTMLElement;
      const background = document.createElement("div");
      background.className = "slide-background presentation";
      reveal.appendChild(background);
      act(() => {
        pointerClick(background, 8, 8);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    });

    it("keeps selection when empty slide click started in reveal overview", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const reveal = container.querySelector(".reveal") as HTMLElement;
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const layer = section.querySelector(".presentation-interaction-layer") as HTMLElement;
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      reveal.classList.add("overview");
      act(() => {
        pointerClick(layer);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      reveal.classList.remove("overview");
      act(() => {
        pointerClick(layer);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    });

    it("allows intro flow disposition drag with pixel offsets", () => {
      const deck = intro({
        language: "de",
        title: { full: ["A", "B", "C"], short: "Short" },
        description: { full: ["D1"], short: "D short" },
        goal: ["G1"],
        authors: { lines: [[{ name: "Alice" }]] },
        affiliations: {
          steps: [
            [{ mark: "a", name: "Faculty" }],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
            ],
          ],
        },
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const section = container.querySelector('.slides > section > section[title="title"]') as HTMLElement;
      section.classList.add("present");
      const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
      const titleLine = section.querySelector('h2[data-id^="title"]') as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(disposition, 330, 120, 300, 80);
      mockClientRect(titleLine, 330, 120, 300, 80);
      act(() => {
        pointerClick(disposition);
        pointerDrag(disposition, 480, 320, 560, 360);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--offset")).toBe(true);
      expect(disposition.style.transform).toContain("translate3d(");
    });

    it("pairs intro description morph on leaf text for reveal auto-animate", () => {
      unmountPresentation();
      const deck = intro({
        language: "de",
        title: { full: ["Entwerfen mit Bestand"], short: "Entwerfen mit Bestand" },
        description: {
          full: ["Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten"],
          short: "Plattform zum Entwerfen mit wiederverwendete Bauteilen",
        },
        goal: ["Mehr Zeit zum manuellen Entwerfen", "dank Automatisierung!"],
        authors: { lines: [[{ name: "Alice" }]] },
        affiliations: {
          steps: [
            [{ mark: "a", name: "Faculty" }],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
            ],
          ],
        },
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const descriptionSlide = container.querySelector('section[title="description"]') as HTMLElement;
      const goalSlide = container.querySelector('section[title="goal"]') as HTMLElement;
      expect(descriptionSlide.hasAttribute("data-auto-animate")).toBe(true);
      expect(descriptionSlide.getAttribute("data-auto-animate-id")).toBe(goalSlide.getAttribute("data-auto-animate-id"));
      const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), descriptionSlide, goalSlide);
      expect(pairs.some((pair) => pair.from.getAttribute("data-id") === "description")).toBe(true);
      expect(pairs.every((pair) => pair.from.matches("h1, h2, h3, h4, h5, h6, p"))).toBe(true);
      const descriptionPair = pairs.find((pair) => pair.from.getAttribute("data-id") === "description");
      expect(descriptionPair?.to.getAttribute("data-id")).toBe("description");
      expect(descriptionPair?.to.textContent).toContain("Plattform zum Entwerfen");
      expect(descriptionPair?.options).toBeUndefined();
      expect(goalSlide.querySelector('h2[data-id="description"]')).toBeTruthy();
      const authorsSlide = container.querySelector('section[title="authors"]') as HTMLElement;
      const goalToAuthors = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), goalSlide, authorsSlide);
      expect(goalToAuthors.some((pair) => pair.from.getAttribute("data-id") === "description" && pair.to.getAttribute("data-id") === "description")).toBe(true);
      const authorsPair = goalToAuthors.find((pair) => pair.from.getAttribute("data-id")?.startsWith("authors--"));
      expect(authorsPair).toBeTruthy();
      expect(authorsPair?.from.matches("h4")).toBe(true);
      expect(authorsPair?.options).toBeUndefined();
      expect(authorsSlide.querySelector('h2[data-id="description"].opacity-20')).toBeTruthy();
    });

    it("pairs authors across affiliation steps with non-zero morph measure delta", () => {
      unmountPresentation();
      const deck = intro({
        language: "de",
        title: { full: ["A"], short: "A" },
        description: { full: ["D"], short: "D" },
        goal: ["G"],
        authors: {
          lines: [[{ name: "Alice Example" }, { name: "Bob Beta" }], [{ name: "Carol Creator" }]],
        },
        affiliations: {
          steps: [
            [{ mark: "a", name: "Faculty" }],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair" } },
            ],
          ],
        },
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const authorsSlide = container.querySelector('section[title="authors"]') as HTMLElement;
      const aff1Slide = container.querySelector('section[title="affiliations-1"]') as HTMLElement;
      const pairs = presentationAutoAnimateMatcher.call(stockRevealAutoAnimateMatcherHost(), authorsSlide, aff1Slide);
      const authorsPair = pairs.find((pair) => pair.from.getAttribute("data-id")?.startsWith("authors--"));
      expect(authorsPair).toBeTruthy();
      expect(authorsPair?.options).toBeUndefined();
      expect(authorsPair?.from.textContent).toContain("Alice Example");
      expect(authorsPair?.to.textContent).toContain("A.");
    });

    it("resizes flow disposition from se handle when nested reveal section has zero height", () => {
      const deck: Presentation = {
        id: "flow-resize",
        name: "Flow Resize",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "flow",
                    participants: [{ id: "label" }],
                    embodiments: [{ kind: "text", id: "label--body", lines: ["Flow label"], level: "body" }],
                    slides: [
                      {
                        arrangement: {
                          id: "flow",
                          dispositions: [
                            {
                              participantId: "label",
                              embodimentId: "label--body",
                              emphasis: "active",
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const section = container.querySelector(".slides > section > section.presentation-arrangement--interactive:not(.presentation-arrangement--intro)") as HTMLElement;
      const stack = section.parentElement as HTMLElement;
      section.classList.add("present");
      stack.classList.add("present");
      Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(stack, "offsetHeight", { value: 700, configurable: true });
      Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(section, "offsetHeight", { value: 0, configurable: true });
      const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
      const label = disposition.querySelector("p") as HTMLElement;
      mockClientRect(stack, 0, 0, 960, 700);
      mockClientRect(section, 0, 0, 960, 0);
      mockClientRect(disposition, 330, 300, 300, 80);
      mockClientRect(label, 330, 300, 300, 80);
      act(() => {
        pointerClick(disposition);
      });
      const handle = disposition.querySelector(".presentation-interaction-handle--se") as HTMLElement;
      act(() => {
        pointerDrag(handle, 620, 370, 720, 420);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
      expect(parseFloat(disposition.style.width)).toBeGreaterThan(15);
      expect(parseFloat(disposition.style.height)).toBeGreaterThan(5);
      expect(disposition.querySelector(".presentation-interactive-disposition__content")?.style.transform).toContain("scale(");
    });

    it("toggles enlarge on an intro flow disposition", () => {
      const deck = intro({
        language: "de",
        title: { full: ["A", "B", "C"], short: "Short" },
        description: { full: ["D1"], short: "D short" },
        goal: ["G1"],
        authors: { lines: [[{ name: "Alice" }]] },
        affiliations: {
          steps: [
            [{ mark: "a", name: "Faculty" }],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni" },
            ],
            [
              { mark: "a", name: "Faculty" },
              { mark: "1", name: "Uni", shortName: "LUH", suffix: { mark: "x", name: "Chair X" } },
            ],
          ],
        },
      });
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const section = container.querySelector('.slides > section > section[title="title"]') as HTMLElement;
      const stack = section.parentElement as HTMLElement;
      section.classList.add("present");
      stack.classList.add("present");
      Object.defineProperty(stack, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(stack, "offsetHeight", { value: 700, configurable: true });
      Object.defineProperty(section, "offsetWidth", { value: 960, configurable: true });
      Object.defineProperty(section, "offsetHeight", { value: 700, configurable: true });
      mockClientRect(stack, 0, 0, 960, 700);
      mockClientRect(section, 0, 0, 960, 700);
      const disposition = section.querySelector("[data-disposition-id]") as HTMLElement;
      const heading = disposition.querySelector("h2") as HTMLElement;
      mockClientRect(disposition, 330, 300, 300, 80);
      mockClientRect(heading, 330, 300, 300, 80);
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      expect(enlargeButton).toBeTruthy();
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      expect(parseFloat(disposition.style.height)).toBeCloseTo(SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100);
      const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
      expect(content.style.transform).toMatch(/^scale\(/);
      expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
      const resetButton = disposition.querySelector(".presentation-interaction-reset") as HTMLButtonElement;
      expect(resetButton).toBeTruthy();
      act(() => {
        resetButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
      expect(content.style.transform).toBe("");
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
      expect(globalsCssSource).toMatch(/\.presentation-arrangement-surface\s*>\s*\.presentation-interactive-disposition--enlarged/s);
    });

    it("drags positioned disposition", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 192, 210, 384, 140);
      act(() => {
        pointerDrag(disposition, 300, 280, 380, 320);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
      // 🔀️ The wrapper owns the reveal `data-id` morph anchor, so the ephemeral drag must move the
      // wrapper frame itself (not just translate inner content). Auto-animate then morphs from the
      // dragged frame, including the ephemeral modification. The declared frame is centered first
      // (single box shifts +0.1/+0.1), then the 80px/40px drag adds +8.333%/+5.714%.
      expect(parseFloat(disposition.style.left)).toBeCloseTo(38.333, 1);
      expect(parseFloat(disposition.style.top)).toBeCloseTo(45.714, 1);
      expect(content.style.transform).toBe("");
    });

    it("keeps other canvas dispositions on their declared frames while one is dragged", () => {
      const twoBoxDeck: Presentation = {
        id: "interactive-dom-two",
        name: "Interactive DOM Two",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "placed",
                    participants: [{ id: "left" }, { id: "right" }],
                    embodiments: [
                      { kind: "text", id: "left--main", lines: ["Left"], level: "body" },
                      { kind: "text", id: "right--main", lines: ["Right"], level: "body" },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "placed",
                          dispositions: [
                            {
                              participantId: "left",
                              embodimentId: "left--main",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
                            },
                            {
                              participantId: "right",
                              embodimentId: "right--main",
                              emphasis: "active",
                              position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
      expect(dispositions).toHaveLength(2);
      const section = dispositions[0]!.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(dispositions[0]!, 96, 210, 288, 140);
      mockClientRect(dispositions[1]!, 576, 210, 288, 140);
      const peerBefore = dispositions[1]!.style.left;
      act(() => {
        pointerDrag(dispositions[0]!, 240, 280, 400, 320);
      });
      // 🔀️ Dragged disposition's wrapper (its morph anchor) follows the ephemeral frame...
      expect(parseFloat(dispositions[0]!.style.left)).toBeCloseTo(26.667, 1);
      // ...while peers keep their declared frames.
      expect(dispositions[1]!.style.left).toBe(peerBefore);
      expect(dispositions[0]!.querySelector(".presentation-interactive-disposition__content")?.style.transform).toBe("");
    });

    it("click-drags only the newly targeted disposition when another stays selected", () => {
      const twoBoxDeck: Presentation = {
        id: "interactive-dom-click-drag",
        name: "Interactive DOM Click Drag",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "placed",
                    participants: [{ id: "left" }, { id: "right" }],
                    embodiments: [
                      { kind: "text", id: "left--main", lines: ["Left"], level: "body" },
                      { kind: "text", id: "right--main", lines: ["Right"], level: "body" },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "placed",
                          dispositions: [
                            {
                              participantId: "left",
                              embodimentId: "left--main",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.3, width: 0.3, height: 0.2 },
                            },
                            {
                              participantId: "right",
                              embodimentId: "right--main",
                              emphasis: "active",
                              position: { x: 0.6, y: 0.3, width: 0.3, height: 0.2 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, twoBoxDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const dispositions = [...container.querySelectorAll("[data-disposition-id]")] as HTMLElement[];
      const section = dispositions[0]!.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(dispositions[0]!, 96, 210, 288, 140);
      mockClientRect(dispositions[1]!, 576, 210, 288, 140);
      act(() => {
        pointerClick(dispositions[0]!, 240, 280);
      });
      const leftBefore = dispositions[0]!.style.left;
      const rightBefore = dispositions[1]!.style.left;
      act(() => {
        pointerDrag(dispositions[1]!, 720, 280, 880, 320);
      });
      expect(dispositions[0]!.style.left).toBe(leftBefore);
      expect(dispositions[1]!.style.left).not.toBe(rightBefore);
    });

    it("shows tile disposition drag preview outside the declared frame without clipping", () => {
      const frame = { x: 0.1, y: 0.1, width: 0.8, height: 0.6 };
      const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame });
      const splitDeck: Presentation = {
        id: "interactive-dom-split-tiles",
        name: "Interactive DOM Split Tiles",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "tiles",
                    participants: grid.participants,
                    embodiments: grid.embodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "tiles",
                          dispositions: grid.dispositions,
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, splitDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const tile = container.querySelector(".presentation-arrangement-canvas > .presentation-interactive-disposition[data-disposition-id]") as HTMLElement;
      expect(tile).toBeTruthy();
      const section = tile.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(tile, 96, 70, 384, 210);
      act(() => {
        pointerDrag(tile, 200, 140, 360, 260);
      });
      // 🔀️ The tile wrapper (morph anchor) moves to the ephemeral frame so the morph starts there.
      expect(parseFloat(tile.style.left)).toBeCloseTo(26.667, 1);
      expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
      expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
      // Pinned keeps the dragged tile unclipped while it sits outside the declared frame.
      expect(getComputedStyle(tile).overflow).not.toBe("hidden");
      expect(tile.querySelector(".presentation-interactive-disposition__content")?.style.transform).toBe("");
    });

    it("resizes positioned disposition from se handle", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 192, 210, 384, 140);
      act(() => {
        pointerClick(disposition);
      });
      const handle = disposition.querySelector(".presentation-interaction-handle--se") as HTMLElement;
      act(() => {
        pointerDrag(handle, 560, 340, 640, 400);
      });
      const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
      expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
      // 🔀️ Resize grows the wrapper frame (the morph anchor) itself; the content fills it at 100%
      // rather than carrying a scaled inline size, so auto-animate morphs from the resized frame.
      expect(parseFloat(disposition.style.width)).toBeCloseTo(48.333, 1);
      expect(parseFloat(disposition.style.height)).toBeCloseTo(28.571, 1);
      expect(content.style.width).toBe("");
      expect(content.style.height).toBe("");
      const chrome = disposition.querySelector(".presentation-interactive-disposition__chrome") as HTMLElement;
      expect(chrome.isConnected).toBe(true);
      expect(chrome.querySelectorAll(".presentation-interaction-handle").length).toBe(8);
    });

    it("aligns canvas-framed chrome with the disposition wrapper on the arrangement canvas", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
      mockClientRect(section, 0, 0, 1200, 900);
      mockClientRect(canvas, 120, 100, 960, 700);
      const frameBox = { left: 408, top: 380, width: 384, height: 140 };
      mockClientRect(disposition, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
      mockClientRect(frame, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
      const chrome = disposition.querySelector(".presentation-interactive-disposition__chrome") as HTMLElement;
      mockClientRect(chrome, frameBox.left, frameBox.top, frameBox.width, frameBox.height);
      const chromeRect = chrome.getBoundingClientRect();
      const wrapperRect = disposition.getBoundingClientRect();
      expect(chromeRect.left).toBeCloseTo(wrapperRect.left, 0);
      expect(chromeRect.top).toBeCloseTo(wrapperRect.top, 0);
      expect(chromeRect.width).toBeCloseTo(wrapperRect.width, 0);
      expect(chromeRect.height).toBeCloseTo(wrapperRect.height, 0);
    });

    it("toggles enlarge on a canvas-framed disposition", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      act(() => {
        pointerClick(disposition);
      });
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
      expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
      const enlargeOn = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      expect(enlargeOn.getAttribute("aria-pressed")).toBe("true");
      act(() => {
        enlargeOn.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
      const enlargeOff = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      expect(enlargeOff.getAttribute("aria-pressed")).toBe("false");
    });

    it("restores pre-enlarge frame after exit enlarge following drag", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 192, 210, 384, 140);
      act(() => {
        pointerClick(disposition);
      });
      const declaredLeft = parseFloat(disposition.style.left);
      act(() => {
        pointerDrag(disposition, 300, 280, 400, 320);
      });
      const draggedLeft = parseFloat(disposition.style.left);
      expect(draggedLeft).not.toBeCloseTo(declaredLeft, 1);
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
      expect(parseFloat(disposition.style.left)).toBeCloseTo(draggedLeft, 1);
      expect(parseFloat(disposition.style.left)).not.toBeCloseTo(declaredLeft, 1);
    });

    it("exits enlarge through the corner control while slide reset host is active", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      act(() => {
        pointerClick(disposition);
      });
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      act(() => {
        enlargeButton.click();
      });
      expect(section.querySelector(".presentation-interaction-slide-reset-host")).toBeTruthy();
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
      expect(enlargeButton.getAttribute("aria-pressed")).toBe("false");
    });

    it("enlarges when pointerdown lands on the svg icon inside the enlarge button", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      const icon = enlargeButton.querySelector("svg");
      expect(icon).toBeTruthy();
      act(() => {
        pointerClick(icon!);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
    });

    it("keeps enlarge when empty slide click clears selection", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      act(() => {
        pointerClick(disposition);
      });
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      act(() => {
        pointerClick(canvas, 8, 8);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      const enlargeWhileDeselected = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      expect(enlargeWhileDeselected).toBeTruthy();
      act(() => {
        enlargeWhileDeselected.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
    });

    it("clears drag and selection after navigating away and back", async () => {
      const alphaRef = collectPresentationSlides(twoSlideDeck)[0];
      const betaRef = collectPresentationSlides(twoSlideDeck)[1];
      expect(alphaRef?.slide).toBe("alpha");
      expect(betaRef?.slide).toBe("beta");
      let revealApi: Reveal.Api | undefined;
      act(() => {
        mountPresentation(container, twoSlideDeck, {
          hash: false,
          slideNumber: false,
          surfaceChrome: false,
          onRevealReady: (api) => {
            revealApi = api;
          },
        });
      });
      await new Promise<void>((resolve) => {
        const start = Date.now();
        const wait = (): void => {
          if (revealApi) {
            resolve();
            return;
          }
          if (Date.now() - start > 5000) {
            throw new Error("reveal.js did not become ready.");
          }
          setTimeout(wait, 50);
        };
        wait();
      });
      const alphaSlide = container.querySelector('section[title="alpha"]') as HTMLElement;
      const disposition = alphaSlide.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 192, 210, 384, 140);
      const originLeft = disposition.style.left;
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--selected")).toBe(true);
      act(() => {
        pointerDrag(disposition, 300, 280, 380, 320);
      });
      const modifiedLeft = disposition.style.left;
      expect(modifiedLeft).not.toBe(originLeft);
      await revealApi!.slide(betaRef!.h, betaRef!.v);
      await new Promise((resolve) => setTimeout(resolve, 50));
      await revealApi!.slide(alphaRef!.h, alphaRef!.v);
      await new Promise((resolve) => setTimeout(resolve, 50));
      const alphaSlideAgain = container.querySelector('section[title="alpha"]') as HTMLElement;
      const dispositionAgain = alphaSlideAgain.querySelector("[data-disposition-id]") as HTMLElement;
      expect(dispositionAgain.style.left).toBe(originLeft);
      expect(dispositionAgain.classList.contains("presentation-interactive-disposition--selected")).toBe(false);
    });

    it("resets the whole slide from the proximity control in the top-right corner", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      expect(globalsCssSource).toMatch(/\.presentation-interaction-slide-reset-host\s*\{[^}]*pointer-events:\s*none/s);
      expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition\s*>\s*\.presentation-interaction-actions\s*\{[^}]*z-index:\s*90/s);
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 192, 210, 384, 140);
      const originLeft = disposition.style.left;
      expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
      act(() => {
        pointerDrag(disposition, 300, 280, 380, 320);
      });
      expect(disposition.style.left).not.toBe(originLeft);
      const slideResetHost = section.querySelector(".presentation-interaction-slide-reset-host") as HTMLElement;
      const slideReset = slideResetHost.querySelector(".presentation-interaction-slide-reset") as HTMLButtonElement;
      expect(slideResetHost).toBeTruthy();
      expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(false);
      act(() => {
        window.dispatchEvent(new PointerEvent("pointermove", { bubbles: true, clientX: 920, clientY: 20, pointerId: 2 }));
      });
      expect(slideResetHost.classList.contains("presentation-interaction-slide-reset-host--near")).toBe(true);
      act(() => {
        slideReset.click();
      });
      expect(disposition.style.left).toBe(originLeft);
      expect(section.querySelector(".presentation-interaction-slide-reset")).toBeNull();
    });

    it("resets a dragged canvas-framed disposition to its declared position", () => {
      act(() => {
        mountPresentation(container, positionedDeck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector("[data-disposition-id]") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 192, 210, 384, 140);
      const originLeft = disposition.style.left;
      const originTop = disposition.style.top;
      act(() => {
        pointerDrag(disposition, 300, 280, 380, 320);
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--pinned")).toBe(true);
      expect(disposition.style.left).not.toBe(originLeft);
      expect(disposition.querySelector(".presentation-interaction-enlarge")).toBeTruthy();
      const reset = disposition.querySelector(".presentation-interaction-reset") as HTMLButtonElement;
      expect(reset).toBeTruthy();
      const actions = disposition.querySelector(".presentation-interaction-actions")!;
      const buttons = [...actions.querySelectorAll("button")];
      expect(buttons[0]?.classList.contains("presentation-interaction-reset")).toBe(true);
      expect(buttons[1]?.classList.contains("presentation-interaction-enlarge")).toBe(true);
      act(() => {
        reset.click();
      });
      expect(disposition.style.left).toBe(originLeft);
      expect(disposition.style.top).toBe(originTop);
      expect(disposition.querySelector(".presentation-interaction-reset")).toBeNull();
    });

    it("scales pdf pages to cover the disposition frame", async () => {
      const deck: Presentation = {
        id: "pdf-cover",
        name: "Pdf Cover",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "media",
                    participants: [{ id: "thesis" }],
                    embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                    slides: [
                      {
                        arrangement: {
                          id: "media",
                          dispositions: [
                            {
                              participantId: "thesis",
                              embodimentId: "thesis--doc",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const frame = disposition.querySelector(".presentation-disposition-frame") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(frame, 96, 385, 768, 280);
      const expected = pdfScrollCoverScale(768, 280, 595, 842);
      expect(expected).not.toBeNull();
      const page = await waitForPdfCanvas(disposition, (canvas) => Math.abs(Number(canvas.dataset.scale) - (expected ?? 0)) < 0.01);
      expect(Number(page.dataset.scale)).toBeCloseTo(expected ?? 0);
    });

    it("drops nested pdf frame and uses enlarged slide sizing when toggling enlarge", async () => {
      const deck: Presentation = {
        id: "pdf-enlarge",
        name: "Pdf Enlarge",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "media",
                    participants: [{ id: "thesis" }],
                    embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                    slides: [
                      {
                        arrangement: {
                          id: "media",
                          dispositions: [
                            {
                              participantId: "thesis",
                              embodimentId: "thesis--doc",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 96, 385, 768, 280);
      act(() => {
        pointerClick(disposition);
      });
      expect(disposition.querySelector(".presentation-disposition-frame")).toBeTruthy();
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
      mockClientRect(content, 48, 52, 864, 595);
      act(() => {
        enlargeButton.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      expect(disposition.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
      expect(disposition.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
      expect(disposition.querySelector(".presentation-disposition-frame")).toBeNull();
      const pageCanvas = await waitForPdfCanvas(disposition, (canvas) => canvas.height > 400);
      expect(pageCanvas.height).toBeGreaterThan(400);
      expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition--kind-pdf\.presentation-interactive-disposition--enlarged[\s\S]*\.presentation-media-pdf-document[\s\S]*height\s*:\s*100%/s);
    });

    it("shows center-bottom pdf page nav when enlarged and switches pages", async () => {
      const deck: Presentation = {
        id: "pdf-page-nav",
        name: "Pdf Page Nav",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "media",
                    participants: [{ id: "thesis" }],
                    embodiments: [{ kind: "pdf", id: "thesis--doc", src: "/thesis.pdf", page: 1 }],
                    slides: [
                      {
                        arrangement: {
                          id: "media",
                          dispositions: [
                            {
                              participantId: "thesis",
                              embodimentId: "thesis--doc",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      const canvas = section.querySelector(".presentation-arrangement-canvas") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(canvas, 0, 0, 960, 700);
      mockClientRect(disposition, 96, 385, 768, 280);
      act(() => {
        pointerClick(disposition);
      });
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      expect(disposition.querySelector(".presentation-pdf-page-nav")).toBeTruthy();
      const content = disposition.querySelector(".presentation-interactive-disposition__content") as HTMLElement;
      mockClientRect(content, 48, 52, 864, 595);
      const enlargeButton = disposition.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      act(() => {
        enlargeButton.click();
      });
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      const nav = disposition.querySelector(".presentation-pdf-page-nav");
      expect(nav).toBeTruthy();
      const page = disposition.querySelector(".presentation-media-pdf canvas") as HTMLElement;
      expect(page.dataset.page).toBe("1");
      const nextButton = disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
      const prevButton = disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
      expect(prevButton.disabled).toBe(true);
      expect(nextButton.disabled).toBe(false);
      act(() => {
        nextButton.click();
      });
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      expect(disposition.querySelector(".presentation-media-pdf canvas")?.getAttribute("data-page")).toBe("2");
      expect((disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement).disabled).toBe(false);
    });

    it("navigates only within pdf pages declared on the embodiment", async () => {
      const deck: Presentation = {
        id: "pdf-page-subset",
        name: "Pdf Page Subset",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "media",
                    participants: [{ id: "thesis" }],
                    embodiments: [
                      {
                        kind: "pdf",
                        id: "thesis--doc",
                        src: "/thesis.pdf",
                        page: 1,
                        pages: [1, 12, 25],
                      },
                    ],
                    slides: [
                      {
                        arrangement: {
                          id: "media",
                          dispositions: [
                            {
                              participantId: "thesis",
                              embodimentId: "thesis--doc",
                              emphasis: "active",
                              position: { x: 0.1, y: 0.55, width: 0.8, height: 0.4 },
                            },
                          ],
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const disposition = container.querySelector(".presentation-interactive-disposition--kind-pdf") as HTMLElement;
      const section = disposition.closest("section.presentation-arrangement--interactive") as HTMLElement;
      mockClientRect(section, 0, 0, 960, 700);
      mockClientRect(disposition, 96, 385, 768, 280);
      act(() => {
        pointerClick(disposition);
      });
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      const nextButton = () => disposition.querySelector(".presentation-pdf-page-nav__button--next") as HTMLButtonElement;
      const prevButton = () => disposition.querySelector(".presentation-pdf-page-nav__button--prev") as HTMLButtonElement;
      const pageNumber = () => disposition.querySelector(".presentation-media-pdf canvas")?.getAttribute("data-page");
      expect(pageNumber()).toBe("1");
      act(() => {
        const next = nextButton();
        pointerClick(next);
        next.click();
      });
      expect(disposition.classList.contains("presentation-interactive-disposition--gesturing")).toBe(false);
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      expect(pageNumber()).toBe("12");
      act(() => {
        const next = nextButton();
        pointerClick(next);
        next.click();
      });
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      expect(pageNumber()).toBe("25");
      expect(nextButton().disabled).toBe(true);
      act(() => {
        prevButton().click();
      });
      await act(async () => {
        await new Promise((resolve) => setTimeout(resolve, 0));
      });
      expect(pageNumber()).toBe("12");
    });

    it("toggles enlarge on a cropped figure tile disposition", () => {
      const frame = { x: 0.05, y: 0.1, width: 0.9, height: 0.75 };
      const grid = split({ source: "/catalogue.png", rows: 2, columns: 2, frame, alt: "Catalogue" });
      const deck: Presentation = {
        id: "split-enlarge",
        name: "Split Enlarge",
        chapters: [
          {
            id: "main",
            sequences: [
              {
                id: "main",
                thoughts: [
                  {
                    id: "split",
                    participants: grid.participants,
                    embodiments: grid.embodiments,
                    slides: [
                      {
                        arrangement: {
                          id: "tiles",
                          dispositions: grid.dispositions,
                        },
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      };
      act(() => {
        mountPresentation(container, deck, { hash: false, slideNumber: false, surfaceChrome: false });
      });
      const tile = container.querySelector("[data-disposition-id]") as HTMLElement;
      const canvas = tile.closest(".presentation-arrangement-canvas") as HTMLElement;
      act(() => {
        pointerClick(tile);
      });
      const enlargeButton = tile.querySelector(".presentation-interaction-enlarge") as HTMLButtonElement;
      act(() => {
        enlargeButton.click();
      });
      expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(true);
      expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(false);
      expect(tile.classList.contains("presentation-interactive-disposition--pinned")).toBe(false);
      expect(tile.style.position).toBe("absolute");
      expect(tile.style.width).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.width * 100}%`);
      expect(tile.style.height).toBe(`${SLIDE_INTERACTIVE_ENLARGE_FRAME.height * 100}%`);
      expect(canvas.contains(tile)).toBe(true);
      const cropSlot = tile.querySelector(".presentation-morph-slot--figure") as HTMLElement | null;
      expect(cropSlot).toBeTruthy();
      expect(tile.querySelector(".presentation-media-figure")).toBeNull();
      expect(cropSlot?.style.backgroundImage).toContain("catalogue.png");
      const bgSize = cropSlot?.style.getPropertyValue("--presentation-figure-bg-size");
      expect(bgSize).toBeTruthy();
      expect(bgSize).not.toBe("100% 100%");
      expect(globalsCssSource).toMatch(/\.presentation-interactive-disposition--enlarged:not\(\.presentation-interactive-disposition--offset\)[\s\S]*\.presentation-figure-crop-fill[\s\S]*width\s*:\s*100%\s*!important/s);
      act(() => {
        enlargeButton.click();
      });
      expect(tile.classList.contains("presentation-interactive-disposition--enlarged")).toBe(false);
      expect(tile.classList.contains("presentation-interactive-disposition--canvas-framed")).toBe(true);
    });
  });

}

export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { jsonPreview, renderJsonTree } = dependencies;

  const { describe, expect, it } = vitest;

  describe("renderJsonTree", () => {
    it("renders nested object keys in preview metadata", () => {
      expect(jsonPreview({ item: { item_id: "x" }, tags: ["a", "b"] })).toBe("Object(2)");
      expect(jsonPreview(["alpha", "beta"])).toBe("Array(2)");
      expect(jsonPreview(null)).toBe("null");
    });

    it("accepts nested null and undefined property values", () => {
      expect(() =>
        renderJsonTree({
          price_amount: null,
          currency: undefined,
          nested: { value: null },
        }),
      ).not.toThrow();
    });
  });

}
