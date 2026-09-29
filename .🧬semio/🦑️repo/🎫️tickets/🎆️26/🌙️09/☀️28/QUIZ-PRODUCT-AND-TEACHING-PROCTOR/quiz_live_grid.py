"""🥞️ The live grid behind the home cards (design §17): the overview rests as a grid of every page (`rest="grid"` with
the tracks of the card layer), each card sits in the cell of its page, the leaderboard is asked for while home shows
(not by its card or page), and every page with a room shows the others inside itself (`PanePeers` in the frame's
overlay slot). Exact anchors, each found once per file."""

import pathlib
import sys
import time

MODULES = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules")


def edit(relative: str, edits: list[tuple[str, str]]) -> None:
    path = MODULES / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times: {old[:90]!r}")
        text = text.replace(old, new)
    for attempt in range(40):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.5)
    sys.exit(f"{relative}: write failed")


edit("🪟️chrome/🟦️.tsx", [
    ("/** 📄️ A page behind a card of the overview: it fills its pane, scrolls on its own and lays its cards out in a centred\n"
     " * column laid out for the pane, not the window. */\n"
     "export function PageFrame(props: { readonly page: string; readonly wide?: boolean; readonly children: ReactNode }): ReactElement {\n"
     "  return (\n"
     "    <div data-page={props.page} className=\"quiz-page-frame h-full min-h-0 w-full overflow-auto bg-background p-double text-foreground\">\n"
     "      <div className={cn(\"mx-auto flex w-full flex-col gap-double\", props.wide ? \"max-w-6xl\" : \"max-w-4xl\")}>{props.children}</div>\n"
     "    </div>\n"
     "  );\n"
     "}\n",
     "/** 📄️ A page behind a card of the overview: it fills its pane, scrolls on its own and lays its cards out in a centred\n"
     " * column laid out for the pane, not the window; `overlay` lies over that column and scrolls with it (the others on\n"
     " * this page). */\n"
     "export function PageFrame(props: { readonly page: string; readonly wide?: boolean; readonly overlay?: ReactNode; readonly children: ReactNode }): ReactElement {\n"
     "  return (\n"
     "    <div data-page={props.page} className=\"quiz-page-frame h-full min-h-0 w-full overflow-auto bg-background p-double text-foreground\">\n"
     "      <div className={cn(\"relative mx-auto flex w-full flex-col gap-double\", props.wide ? \"max-w-6xl\" : \"max-w-4xl\")}>\n"
     "        {props.children}\n"
     "        {props.overlay}\n"
     "      </div>\n"
     "    </div>\n"
     "  );\n"
     "}\n"),
])
edit("🏆️leaderboard/🟦️.tsx", [
    ("  const { session, state, text, locale, revealed, onOpen } = props;\n  const id = useId();\n  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);\n",
     "  const { session, state, text, locale, revealed, onOpen } = props;\n  const id = useId();\n"),
    ("/** 🏆️ The leaderboard card at the centre of the overview: the top rows, a gap, and the own row; polled while shown. Its\n * heading and \"Full leaderboard\" open the leaderboard page (`#board`). */",
     "/** 🏆️ The leaderboard card at the centre of the overview: the top rows, a gap, and the own row. Its heading and \"Full\n * leaderboard\" open the leaderboard page (`#board`). */"),
    ("/** 🏆️ The leaderboard page behind the centre card: every learner in one sortable table. It asks the proctor again only\n * while it is opened or shown clear; as a blurred backdrop it shows the last snapshot. */",
     "/** 🏆️ The leaderboard page behind the centre card: every learner in one sortable table, kept up to date by home's\n * polling, with the others on this page inside it. */"),
    ("  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS, view.opened || view.revealed);\n", ""),
    ("    <PageFrame page={HOME_PAGES.leaderboard} wide>\n",
     "    <PageFrame page={HOME_PAGES.leaderboard} wide overlay={<PanePeers scope={state.catalog === undefined ? undefined : roomScope(state.catalog.id, { screen: \"leaderboard\" })} opened={view.opened} />}>\n"),
    ('import { OnlineMark, PRESENCE_ANCHORS, usePresenceView } from "../👥️presence/🟦️.tsx";\n',
     'import { OnlineMark, PRESENCE_ANCHORS, PanePeers, usePresenceView } from "../👥️presence/🟦️.tsx";\n'),
    ("import { learnerTag, type CatalogView, type LeaderboardRow } from \"@semio-tech/quiz\";\n",
     "import { learnerTag, roomScope, type CatalogView, type LeaderboardRow } from \"@semio-tech/quiz\";\n"),
])
edit("🏅️badges/🟦️.tsx", [
    ("export function BadgesPage(props: { readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {\n  const { state, text, locale } = props;\n",
     "export function BadgesPage(props: { readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {\n  const { state, text, locale, view } = props;\n"),
    ("    <PageFrame page={HOME_PAGES.badges} wide>\n",
     "    <PageFrame page={HOME_PAGES.badges} wide overlay={<PanePeers scope={state.catalog === undefined ? undefined : roomScope(state.catalog.id, { screen: \"badges\" })} opened={view.opened} />}>\n"),
    ('import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n', 'import { PRESENCE_ANCHORS, PanePeers } from "../👥️presence/🟦️.tsx";\n'),
    ("import type { BadgeAward, CatalogBadgeView, Slug } from \"@semio-tech/quiz\";\n", "import { roomScope, type BadgeAward, type CatalogBadgeView, type Slug } from \"@semio-tech/quiz\";\n"),
])
edit("👋️introduction/🟦️.tsx", [
    ("export function IntroductionPage(props: { readonly catalog: CatalogView; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {\n  const { catalog, locale } = props;\n",
     "export function IntroductionPage(props: { readonly catalog: CatalogView; readonly locale: QuizLocale; readonly view: PaneView }): ReactElement {\n  const { catalog, locale, view } = props;\n"),
    ("    <PageFrame page={HOME_PAGES.introduction}>\n",
     "    <PageFrame page={HOME_PAGES.introduction} overlay={<PanePeers scope={roomScope(catalog.id, { screen: \"introduction\" })} opened={view.opened} />}>\n"),
    ('import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n', 'import { PRESENCE_ANCHORS, PanePeers } from "../👥️presence/🟦️.tsx";\n'),
    ("import type { CatalogView } from \"@semio-tech/quiz\";\n", "import { roomScope, type CatalogView } from \"@semio-tech/quiz\";\n"),
])
edit("🏠️home/🟦️.tsx", [
    ("import { LeaderboardCard, LeaderboardPage } from \"../🏆️leaderboard/🟦️.tsx\";\n",
     "import { LEADERBOARD_POLL_MS, LeaderboardCard, LeaderboardPage, usePolling } from \"../🏆️leaderboard/🟦️.tsx\";\n"),
    ("  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);\n  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);\n",
     "  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);\n  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);\n  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);\n"),
    ("  const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {\n    const shown = { revealed: card.revealed, onOpen: card.open };\n",
     "  const cells = homeCells(pages, layout === \"tablet\" ? \"tablet\" : \"desktop\");\n"
     "  const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {\n"
     "    const cell = cells[pane.id];\n"
     "    const body = cardOf(pane, card);\n"
     "    return card.mode === \"strip\" && cell !== undefined ? (\n"
     "      <div className=\"quiz-home-cell\" style={{ gridColumn: cell.column + 1, gridRow: cell.row + 1 }}>\n"
     "        {body}\n"
     "      </div>\n"
     "    ) : (\n"
     "      body\n"
     "    );\n"
     "  };\n"
     "  const cardOf = (pane: LayeredPane, card: LayeredCardState): ReactNode => {\n"
     "    const shown = { revealed: card.revealed, onOpen: card.open };\n"),
    ("          cells={homeCells(pages, layout === \"tablet\" ? \"tablet\" : \"desktop\")}\n",
     "          cells={cells}\n          rest=\"grid\"\n          gridTracks={layout === \"desktop\" ? HOME_GRID_TRACKS : undefined}\n"),
    ("          lifecycle={{ budget: pages.length, warmStartMs: 600, warmIntervalMs: 250, suspendIdleMs: Number.POSITIVE_INFINITY }}\n",
     "          lifecycle={{ budget: pages.length, suspendIdleMs: Number.POSITIVE_INFINITY, suspendOffscreenMs: Number.POSITIVE_INFINITY, suspendHiddenMs: Number.POSITIVE_INFINITY }}\n"),
    ("interface HomeProps {\n",
     "/** 📏️ The track weights of the desktop grid: the leaderboard's column and row the larger ones, so every page lies\n"
     " * exactly behind its card (the card layer takes the same tracks from the overview). */\n"
     "export const HOME_GRID_TRACKS = { columns: [1, 1.5, 1], rows: [1, 1.4, 1] } as const;\n\n"
     "interface HomeProps {\n"),
])
print("[live grid] done")
