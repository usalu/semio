const T = (emoji, slug, doc, ranges) => ({ emoji, folder: emoji + "️" + slug, doc, ranges });
export default {
  source: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pqs-tools/ts/orig/home-grid.tsx",
  isTsx: true,
  mode: "module",
  ext: ".tsx",
  blankHeader: 12,
  testsDir: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/❓️quiz/🧪️tests",
  helperFolder: "🛠️home-grid-kit",
  helper: { emoji: "🛠️", doc: "Shared fixtures, stylesheet readers and the home harness of the layered-home suites (the topics sit in the sibling folders `🏠️home-grid`, `🎫️quiz-card-challenges`, `🥇️home-leaderboards` and `📱️home-text-and-viewport`)." },
  topics: [
    T("🏠", "home-grid", "The layered home (design §16–§17): the pages of the overview in reading order and the cell of each (shared\n * vectors), the nine card sections over the glass with the leaderboard fifth and each heading linking its page, every\n * page live behind the cards at rest with its card in its cell, opening a page by its card or the step in front and\n * closing it with Escape (the address and the navbar's way back are the client's, `🚏️navigation`), revealing a page behind its hovered card without running any command, the leaderboard polled and the crowd of\n * every quiz asked for while home shows and again with every new submission, the others inside the pages behind the cards, the actions of every quiz card by\n * run state, and the tracks of the live grid the card layer shares at the design system's breakpoints — read from the\n * stylesheet and the overview's track lists as parsed by `lightningcss` (the CSS engine Tailwind itself runs on) as the\n * third-party oracle.\n *\n * @see ../../🧫️fixtures/🏠️home-grid/🔣️.json\n * @see ../../../../🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx — the layered overview and its hooks\n", [[354, 732]]),
    T("🎫", "quiz-card-challenges", "The challenge on the quiz cards and pages of the layered home.", [[734, 918]]),
    T("🥇", "home-leaderboards", "The leaderboard of the new contract on the layered home and its four leaderboards with their categories.", [[920, 1154]]),
    T("📱", "home-text-and-viewport", "The layered home in the learner's text size, the opened page and the short viewport.", [[1156, 1246]]),
  ],
};
