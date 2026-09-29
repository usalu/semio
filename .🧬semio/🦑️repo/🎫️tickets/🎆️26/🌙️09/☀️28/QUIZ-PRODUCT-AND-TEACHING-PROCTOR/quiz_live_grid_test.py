"""🏠️ The home-grid suite meets the live grid (design §17): the stylesheet reading moves from column counts to the
tracks the card layer takes (a track list, or the overview's variables) and the spacing (none between tracks, inside
the cells); the overview's own track lists are read as fr weights; the viewport width is emulated through
`matchMedia`; the polling test follows the leaderboard to home; new cases for the live pages with each card in its
cell and for the others inside the pages. Replaces whole regions between exact anchors."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🧪️tests/🏠️home-grid/🟦️.tsx")
text = path.read_text(encoding="utf-8")


def between(start: str, end: str | None, new: str) -> None:
    global text
    if text.count(start) != 1 or (end is not None and text.count(end) != 1):
        sys.exit(f"anchors {text.count(start)} {end and text.count(end)}: {start[:60]!r} {(end or '')[:60]!r}")
    head, rest = text.split(start)
    if end is None:
        text = head + new
        return
    _, tail = rest.split(end)
    text = head + new + end + tail


def once(old: str, new: str) -> None:
    global text
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:60]!r}")
    text = text.replace(old, new)


TRACKS = r'''//#region 🎨️Tracks
/** 🎨️ One declaration of the card layer or its cells in the stylesheet, with the minimum viewport width its media query
 * asks for (0 outside one): a track list as its track count, a variable as `var(--name)`, anything else as its name. */
interface LayerRule {
  readonly minWidth: number;
  readonly selector: string;
  readonly property: string;
  readonly value: number | string;
}

type Declaration = {
  readonly property: string;
  readonly value: { readonly items?: readonly unknown[]; readonly propertyId?: { readonly property: string }; readonly value?: readonly { readonly type: string; readonly value?: { readonly name?: { readonly ident: string } } }[] };
};

const SPACING = ["gap", "row-gap", "column-gap", "padding", "padding-top", "padding-right", "padding-bottom", "padding-left", "padding-inline", "padding-block", "margin"];

function selectorText(selector: readonly { readonly type: string; readonly name?: string; readonly value?: string; readonly operation?: { readonly value: string } }[]): string {
  return selector.map((part) => (part.type === "class" ? `.${part.name}` : part.type === "combinator" ? ">" : part.type === "attribute" ? `[${part.name}=${part.operation?.value}]` : part.type)).join("");
}

function declared(declaration: Declaration): LayerRule["value"] {
  const variable = Array.isArray(declaration.value.value) ? declaration.value.value.find((token) => token.type === "var")?.value?.name?.ident : undefined;
  if (declaration.property === "unparsed" && variable !== undefined) return `var(${variable})`;
  return declaration.value.items?.length ?? declaration.property;
}

/** 🎨️ Every declaration of `.quiz-home-grid` and `.quiz-home-cell` with the minimum viewport width it applies from. */
function layerRules(css: string): readonly LayerRule[] {
  const found: LayerRule[] = [];
  const collect = (rules: readonly { readonly type: string; readonly value: never }[], minWidth: number): void => {
    for (const rule of rules as readonly {
      readonly type: string;
      readonly value: { readonly selectors?: readonly never[][]; readonly declarations?: { readonly declarations: readonly Declaration[] }; readonly query?: never; readonly rules?: never[] };
    }[]) {
      if (rule.type === "media") {
        const query = rule.value.query as unknown as {
          readonly mediaQueries: readonly { readonly condition: { readonly value: { readonly name: string; readonly operator: string; readonly value: { readonly value: { readonly value: { readonly unit: string; readonly value: number } } } } } }[];
        };
        const feature = query.mediaQueries[0]?.condition.value;
        if (feature?.name === "width") {
          if (feature.operator !== "greater-than-equal" || feature.value.value.value.unit !== "px") throw new Error(`unexpected width query ${JSON.stringify(query)}`);
          collect(rule.value.rules ?? [], feature.value.value.value.value);
        } else {
          const before = found.length;
          collect(rule.value.rules ?? [], Number.POSITIVE_INFINITY);
          if (found.length !== before) throw new Error(`card layer rules under a media query that is not about width: ${JSON.stringify(query)}`);
        }
        continue;
      }
      if (rule.type !== "style") continue;
      for (const selector of rule.value.selectors ?? []) {
        const name = selectorText(selector);
        if (!name.startsWith(".quiz-home-grid") && !name.startsWith(".quiz-home-cell")) continue;
        for (const declaration of rule.value.declarations?.declarations ?? []) found.push({ minWidth, selector: name, property: declaration.value.propertyId?.property ?? declaration.property, value: declared(declaration) });
      }
    }
  };
  transform({
    filename: "🎨️.css",
    code: new TextEncoder().encode(css),
    visitor: {
      StyleSheet(sheet) {
        collect(sheet.rules as never, 0);
      },
    },
  });
  return found;
}

/** 🔳️ The tracks the card layer takes at `width`: counted, or the overview's variables. */
function layerAt(rules: readonly LayerRule[], width: number): Readonly<Record<string, number | string>> {
  const tracks: Record<string, number | string> = {};
  for (const rule of rules) {
    if (width < rule.minWidth || rule.selector !== ".quiz-home-grid") continue;
    if (rule.property === "grid-template-columns") tracks.columns = rule.value;
    if (rule.property === "grid-template-rows") tracks.rows = rule.value;
  }
  return tracks;
}

/** ⚖️ The fr weights of a CSS track list, as parsed by lightningcss. */
function weights(template: string): readonly number[] {
  const found: number[] = [];
  transform({
    filename: "tracks.css",
    code: new TextEncoder().encode(`.tracks { grid-template-columns: ${template}; }`),
    visitor: {
      Declaration: {
        "grid-template-columns"(declaration) {
          const value = declaration.value as { readonly items?: readonly { readonly type: string; readonly value: { readonly type: string; readonly value?: number; readonly max?: { readonly type: string; readonly value: number } } }[] };
          for (const item of value.items ?? []) {
            const size = item.value.type === "min-max" ? item.value.max : item.value;
            if (item.type !== "track-size" || size?.type !== "flex") throw new Error(`not a flexible track: ${JSON.stringify(item)}`);
            found.push(size.value as number);
          }
        },
      },
    },
  });
  return found;
}

/** 📱️ Emulates a viewport `width` px wide for the width media queries (`min-width`/`max-width` in px, joined by
 * `and`); every other query is false. */
function atViewport(width: number): void {
  vi.stubGlobal("matchMedia", (query: string) => ({
    matches: query.split(" and ").every((part) => {
      const found = /^\((min|max)-width:\s*(\d+)px\)$/u.exec(part.trim());
      return found !== null && (found[1] === "min" ? width >= Number(found[2]) : width <= Number(found[2]));
    }),
    media: query,
    onchange: null,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
    addListener: () => undefined,
    removeListener: () => undefined,
    dispatchEvent: () => false,
  }));
}
//#endregion 🎨️Tracks

//#region 🥞️Overview
/** 🥞️ Home over a session double whose `open` really moves the step, so the overview's open page follows it. */
function Home(props: { readonly session: ReturnType<typeof stubSession>; readonly initial?: QuizStep }): ReactElement {
  const [step, setStep] = useState<QuizStep>(props.initial ?? { screen: "home" });
  const [session] = useState(() => ({
    ...props.session,
    open: (next: QuizStep) => {
      props.session.open(next);
      setStep(next);
    },
  }));
  return <HomeScreen session={session as unknown as QuizSession} state={{ ...STATE, step }} text={quizText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />;
}

function peer(anchor: string): PeerCursor {
  return { session: `w-${anchor}`, tag: "0badc0de", label: "Mira K.", colour: 3, cursor: { anchor, x: 0.5, y: 0.5 }, focus: undefined, drag: undefined };
}
'''
between("//#region 🎨️Columns\n", "\nfunction cards(): readonly HTMLElement[] {", TRACKS)
once("afterEach(() => {\n  window.history.replaceState(null, \"\", window.location.pathname);\n  vi.useRealTimers();\n});",
     "afterEach(() => {\n  window.history.replaceState(null, \"\", window.location.pathname);\n  vi.useRealTimers();\n  vi.unstubAllGlobals();\n});")

POLLING = r'''  it("polls the leaderboard while home shows, whatever page is opened or revealed, and asks for the crowd of every quiz", async () => {
    vi.useFakeTimers();
    const session = stubSession();
    const { unmount } = render(<Home session={session} />);
    await vi.advanceTimersByTimeAsync(0);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(1);
    expect(session.refreshCrowd.mock.calls.map(([quiz]) => quiz)).toEqual([...QUIZZES]);
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(2);
    fireEvent.pointerOver(screen.getByRole("region", { name: "Quiz heating" }), { pointerType: "mouse" });
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(3);
    fireEvent.click(within(screen.getByRole("region", { name: "Leaderboard" })).getByRole("button", { name: "Full leaderboard" }));
    await vi.advanceTimersByTimeAsync(LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(4);
    expect(session.refreshCrowd).toHaveBeenCalledTimes(QUIZZES.length);
    unmount();
    await vi.advanceTimersByTimeAsync(3 * LEADERBOARD_POLL_MS);
    expect(session.refreshLeaderboard).toHaveBeenCalledTimes(4);
    const alone = stubSession();
    render(<LeaderboardPage session={alone as unknown as QuizSession} state={STATE} text={quizText("en")} locale="en" view={{ opened: true, revealed: false }} />);
    await vi.advanceTimersByTimeAsync(3 * LEADERBOARD_POLL_MS);
    expect(alone.refreshLeaderboard).not.toHaveBeenCalled();
    expect(screen.queryByRole("status")).toBeNull();
  });

  it("keeps every page live behind the cards at rest, each card in the cell of its page on the tracks of the grid", () => {
    render(<Home session={stubSession()} />);
    expect(document.querySelector("[data-layered-overview]")?.getAttribute("data-rest")).toBe("grid");
    for (const page of PAGES) expect(pane(page).querySelector("[data-page]"), page).not.toBeNull();
    const cells = homeCells(PAGES, "desktop");
    for (const page of PAGES) {
      const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
      expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
    }
    const desktop = fixture.layouts.find((layout) => layout.layout === "desktop")!;
    expect(HOME_GRID_TRACKS).toEqual({ columns: desktop.columns, rows: desktop.rows });
    const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
    expect(weights(overlay.style.getPropertyValue("--layered-columns"))).toEqual(desktop.columns);
    expect(weights(overlay.style.getPropertyValue("--layered-rows"))).toEqual(desktop.rows);
  });

  it("shows the others inside the pages behind the cards on each page's own anchors, and none with cursors off", () => {
    const view: PresenceView = {
      ...EMPTY_PRESENCE_VIEW,
      rooms: new Map([
        [roomScope(CATALOG.id, { screen: "leaderboard" })!, [peer("leaderboard")]],
        [roomScope(CATALOG.id, { screen: "quiz", quiz: "heating" })!, [peer("quiz:heating")]],
        [roomScope(CATALOG.id, { screen: "badges" })!, []],
      ]),
    };
    const session = stubSession();
    const tree = (showCursors: boolean) => (
      <PresenceProvider view={view} showCursors={showCursors} setTask={() => undefined}>
        <Home session={session} />
      </PresenceProvider>
    );
    const { rerender } = render(tree(true));
    for (const [page, anchor] of [
      ["board", "leaderboard"],
      ["heating", "quiz:heating"],
    ] as const) {
      const mark = pane(page).querySelector<HTMLElement>('[data-pane-peers] [data-peer="cursor"]')!;
      expect(mark.textContent, page).toBe("Mira K.");
      expect(mark.dataset.anchor).toBe(anchor);
      expect(mark.hidden, page).toBe(false);
      expect(mark.closest('[aria-hidden="true"]')).not.toBeNull();
      expect(pane(page).querySelector(`[data-presence-anchor="${anchor}"]`)).not.toBeNull();
    }
    for (const page of PAGES.filter((candidate) => candidate !== "board" && candidate !== "heating")) expect(pane(page).querySelector("[data-pane-peers]"), page).toBeNull();
    rerender(tree(false));
    expect(document.querySelector("[data-pane-peers]")).toBeNull();
  });

'''
between('  it("asks for the leaderboard only while its page is opened or shown clear", async () => {\n',
        '  it("keeps every leaderboard heading and number on one line, letting only learner names wrap", () => {', POLLING)

LAYOUTS = r'''  it("takes its breakpoints from the design system, shares the grid's tracks with no spacing between them and spaces the cards inside their cells", () => {
    expect(fixture.breakpoints).toEqual({ mobileMaxPx: UI_MOBILE_MAX_WIDTH_PX, tabletMaxPx: UI_TABLET_MAX_WIDTH_PX });
    const rules = layerRules(stylesheet);
    expect([...new Set(rules.filter((rule) => rule.selector === ".quiz-home-grid").map((rule) => rule.minWidth))].sort((a, b) => a - b)).toEqual([0, UI_MOBILE_MAX_WIDTH_PX + 1]);
    expect(rules.filter((rule) => rule.selector.startsWith(".quiz-home-grid") && SPACING.includes(rule.property))).toEqual([]);
    expect(rules.filter((rule) => rule.selector === ".quiz-home-cell").map((rule) => rule.property)).toEqual(expect.arrayContaining(["padding", "align-items", "justify-content", "min-width", "min-height"]));
  });

  for (const expected of fixture.layouts) {
    it(`lays the cards over the ${expected.layout} at ${expected.width} px`, () => {
      const { mobileMaxPx, tabletMaxPx } = fixture.breakpoints;
      expect(expected.width <= mobileMaxPx ? "list" : expected.width <= tabletMaxPx ? "tablet" : "desktop").toBe(expected.layout);
      atViewport(expected.width);
      render(<Home session={stubSession()} />);
      const layer = layerAt(layerRules(stylesheet), expected.width);
      if (expected.rows === null) {
        expect(layer).toEqual({ columns: expected.columns.length });
        expect(document.querySelectorAll(".quiz-home-cell")).toHaveLength(0);
        return;
      }
      expect(layer).toEqual({ columns: "var(--layered-columns)", rows: "var(--layered-rows)" });
      const overlay = document.querySelector<HTMLElement>(".quiz-home-grid")!;
      expect(weights(overlay.style.getPropertyValue("--layered-columns"))).toEqual(expected.columns);
      expect(weights(overlay.style.getPropertyValue("--layered-rows"))).toEqual(expected.rows);
      const cells = homeCells(PAGES, expected.layout as "desktop" | "tablet");
      for (const page of PAGES) {
        const cell = cardHost(page).querySelector<HTMLElement>(".quiz-home-cell")!;
        expect([Number(cell.style.gridColumn) - 1, Number(cell.style.gridRow) - 1], page).toEqual([cells[page]!.column, cells[page]!.row]);
      }
    });
  }
});
'''
between('  it("takes its breakpoints from the design system and centres compact cards in their cells", () => {\n', None, LAYOUTS)
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[live grid test] done")
