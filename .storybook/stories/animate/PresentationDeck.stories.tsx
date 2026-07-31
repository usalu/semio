// #region 🧲️Header
// 💻️ .storybook/stories/animate/PresentationDeck.stories.tsx
// Specs: `PresentationDeck` (`animate/present/renderer/react/index.tsx`) mounts a declarative
// `Presentation` (`@semio-tech/animate-present-core`) — `Presentation → Chapter[] → Sequence[] → Thought[] →
// Slide[]` — through reveal.js. The component's own module already does `import "reveal.js/dist/reveal.css"`
// (see that file's header: "📽️ React + reveal.js renderer for `@semio-tech/animate-present-core` declarative
// decks"), so reveal's structural CSS needs no extra import here. The `--r-*` custom-property theming lives in
// a separate `./globals.css` package export (real production usage:
// `mit-bestand/präsentation/33.projektetage/globals.css` does `@import "…/animate/present/renderer/react/globals.css"`)
// which is NOT pulled in by the component or the root Storybook globals — imported explicitly below via the
// package's declared `exports["./globals.css"]` subpath.
// Summary: A minimal 3-slide deck (one chapter, one sequence, one thought) with plain text embodiments.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react";
import { PresentationDeck } from "@semio-tech/animate-present-renderer-react";
import type { Presentation, Slide } from "@semio-tech/animate-present-core";
import "@semio-tech/animate-present-renderer-react/globals.css";
// #endregion 🔌️Adapters

// #region 🎬️StoryDeckFixture
/** @emoji 🎬️ One title/body text slide, unique embodiment ids per slide so each carries its own copy. */
function textSlide(slideIndex: number, headingLines: readonly string[], bodyLines: readonly string[], headingLevel: "title" | "heading" = "heading"): Slide {
  return {
    arrangement: {
      id: `arrangement-${slideIndex}`,
      embodiments: [
        { kind: "text", id: `heading-${slideIndex}`, lines: headingLines, level: headingLevel },
        { kind: "text", id: `body-${slideIndex}`, lines: bodyLines, level: "body" },
      ],
      dispositions: [
        { participantId: "heading", embodimentId: `heading-${slideIndex}`, emphasis: "active" },
        { participantId: "body", embodimentId: `body-${slideIndex}`, emphasis: "active" },
      ],
    },
    transition: { kind: "fade" },
  };
}

/** @emoji 🎬️ 3-slide deck: title, a feature slide, a closing slide — one chapter/sequence/thought. */
const storyPresentation: Presentation = {
  id: "storybook-deck",
  name: "Storybook Deck",
  participants: [{ id: "heading" }, { id: "body" }],
  chapters: [
    {
      id: "chapter-1",
      sequences: [
        {
          id: "sequence-1",
          thoughts: [
            {
              id: "thought-1",
              slides: [
                textSlide(1, ["Semio Storybook"], ["A composable, data-driven scope registry covering the monorepo."], "title"),
                textSlide(2, ["Reveal.js Decks"], ["PresentationDeck renders a declarative Presentation → Chapter → Sequence → Thought → Slide tree."]),
                textSlide(3, ["Thank You"], ["cad · coda · animate · compose/algorithm"]),
              ],
            },
          ],
        },
      ],
    },
  ],
};
// #endregion 🎬️StoryDeckFixture

const meta = {
  title: "🎬️animate",
  component: PresentationDeck,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof PresentationDeck>;

export default meta;

type Story = StoryObj<typeof meta>;

/** @emoji 🎬️ 3-slide reveal.js deck — arrow keys / space to navigate, no URL hash sync inside Storybook. */
export const ThreeSlideDeck: Story = {
  args: {
    presentation: storyPresentation,
    options: { hash: false, surfaceChrome: false },
  },
};
