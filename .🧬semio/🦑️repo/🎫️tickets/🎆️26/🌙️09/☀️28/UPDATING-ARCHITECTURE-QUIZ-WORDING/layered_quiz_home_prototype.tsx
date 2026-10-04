import type { ReactNode } from "react";
import { mountUiRoot, useUiMemo as useMemo } from "@semio-tech/ui-react/runtime";
import {
  Navbar,
  ShellBrandLogo,
  WindowChrome,
  windowChromeTitleChipClass,
  Icon,
  cn,
  bootstrapElementsSurfaceChromeDocument,
  initUiLocaleSync,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiDriver,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MEDIA_QUERY,
  useElementsSurfaceChrome,
  useMediaQuery,
  navbarFillItem,
  type IconName,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort } from "@semio-tech/framework";
import { LayeredOverview, type LayeredCell, type LayeredPane } from "./layered_overview_prototype.tsx";
// css: the .quiz-home-grid rules of 🗑️generated/ui-reference/rig/layered/quiz-layered.css (see 📓️ui-reference-layered-landing.md §11)

const storage = createBrowserStoragePort();
const forced = new URLSearchParams(window.location.search);
const forcedAppearance = forced.get("appearance");
bootstrapElementsSurfaceChromeDocument(forcedAppearance === "dark" || forcedAppearance === "light" ? forcedAppearance : readStoredUiChromeAppearance(storage));
initUiLocaleSync("en");

const LOGO = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="semio"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

const chip = cn(windowChromeTitleChipClass, "gap-single px-single text-xs font-medium");

//#region 🃏️Cards
function Card(props: { readonly id: string; readonly card: string; readonly icon: ReactNode; readonly title: string; readonly revealed: boolean; readonly mode: "strip" | "list"; readonly open: () => void; readonly left?: ReactNode; readonly right?: ReactNode; readonly children: ReactNode }) {
  return (
    <section
      data-card={props.card}
      data-revealed={props.revealed ? "" : undefined}
      aria-labelledby={props.id}
      onClick={(event) => {
        if (!(event.target as HTMLElement).closest("button,a")) props.open();
      }}
      className={cn("pointer-events-auto min-h-0 min-w-0 cursor-pointer transition-transform duration-200 motion-reduce:transition-none", props.revealed && "-translate-y-0.5 motion-reduce:translate-y-0", props.mode === "list" && "w-full max-w-md")}
    >
      <WindowChrome
        level="dialog"
        active={false}
        stackSlot="quiz-card-stack"
        stackClassName="w-full min-w-0"
        titleChips={
          <div data-slot="quiz-card-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single px-single")}>
            {props.icon}
            <h2 id={props.id} className="min-w-0 truncate text-sm font-medium text-foreground">
              <a href={`#${props.card}`} className="outline-none focus-visible:underline">
                {props.title}
              </a>
            </h2>
          </div>
        }
        body={<div className="flex h-full min-h-0 w-full min-w-0 flex-col gap-double">{props.children}</div>}
        footerLeftChips={props.left}
        footerRightChips={props.right}
        bodyClassName="p-double"
      />
    </section>
  );
}

const Go = ({ children }: { children: ReactNode }) => (
  <button type="button" className={cn(chip, "group")}>
    <span className="text-muted-foreground transition-colors group-hover:text-foreground">{children}</span>
    <Icon icon="chevron-right" size="small" className="transition-transform group-hover:translate-x-0.5" />
  </button>
);
const Quiet = ({ children }: { children: ReactNode }) => (
  <button type="button" className={cn(chip, "text-muted-foreground")}>
    {children}
  </button>
);
const glyph = (emoji: string) => (
  <span aria-hidden="true" className="shrink-0 text-sm">
    {emoji}
  </span>
);
const iconGlyph = (icon: IconName) => <Icon icon={icon} size="small" className="shrink-0 text-muted-foreground" />;
const Facts = ({ items }: { items: readonly string[] }) => (
  <ul className="flex flex-wrap gap-x-double gap-y-half text-xs text-muted-foreground">
    {items.map((item) => (
      <li key={item}>{item}</li>
    ))}
  </ul>
);
//#endregion 🃏️Cards

//#region 📄️Pages
const QUIZZES = [
  { id: "physics", emoji: "🧲", title: "Physical Understanding", text: "Tell power from energy and develop a feeling for orders of magnitude — from a tea light to the Sun.", tasks: ["Classify: power or energy?", "Sort by power", "Sort by energy"], best: "82 %", state: "again" },
  { id: "heating", emoji: "🔥", title: "Heating", text: "U-values from single glazing to the passive-house roof, heating loads and demands from the 1960s to today.", tasks: ["Match U-values to components", "Match heating loads to buildings"], best: undefined, state: "resume" },
  { id: "cooling", emoji: "❄️", title: "Cooling", text: "Air change rates from the warehouse to the cleanroom; cooling loads from the passive house to the data centre.", tasks: ["Match air change rates", "Match cooling loads"], best: "100 %", state: "again" },
  { id: "demand", emoji: "📊", title: "Energy Demand", text: "Recognise energy standards by their profiles and estimate the final energy buildings have to buy.", tasks: ["Classify by profile", "Match final energy demand"], best: undefined, state: "start" },
] as const;

const LEADERS = [
  ["1", "Mira", "1 240", "95 %", "88 %", "100 %", "57 %", "6"],
  ["2", "solar_fox", "1 180", "82 %", "91 %", "70 %", "61 %", "5"],
  ["3", "Anonymous 7F3A", "1 105", "70 %", "80 %", "75 %", "48 %", "5"],
  ["4", "Jonas B.", "990", "66 %", "72 %", "68 %", "40 %", "4"],
  ["5", "kwh_kid", "940", "61 %", "70 %", "66 %", "39 %", "4"],
  ["6", "Lena", "870", "58 %", "64 %", "60 %", "35 %", "3"],
  ["7", "u_value", "800", "55 %", "60 %", "58 %", "30 %", "3"],
  ["8", "Ann K.", "620", "82 %", "–", "100 %", "–", "3"],
  ["9", "Anonymous 02BB", "590", "50 %", "45 %", "52 %", "–", "2"],
  ["10", "passivhaus", "480", "44 %", "40 %", "–", "–", "1"],
] as const;

function Page(props: { readonly emoji?: string; readonly icon?: IconName; readonly title: string; readonly children: ReactNode }) {
  return (
    <div className="flex h-full w-full flex-col bg-background text-foreground">
      <div className="flex items-center gap-double border-b border-normal p-double">
        {props.emoji ? (
          <span aria-hidden className="text-2xl">
            {props.emoji}
          </span>
        ) : props.icon ? (
          <Icon icon={props.icon} size="large" />
        ) : null}
        <h1 className="text-2xl font-semibold">{props.title}</h1>
      </div>
      <div className="min-h-0 flex-1 overflow-auto p-double">{props.children}</div>
    </div>
  );
}

function QuizPage({ quiz }: { quiz: (typeof QUIZZES)[number] }) {
  return (
    <Page emoji={quiz.emoji} title={quiz.title}>
      <div className="mx-auto flex max-w-3xl flex-col gap-double">
        <p className="text-lg leading-normal">{quiz.text}</p>
        <ol className="flex flex-col gap-single">
          {quiz.tasks.map((task, index) => (
            <li key={task} className="flex items-center gap-double border border-normal p-double text-lg">
              <span className="w-8 text-2xl font-semibold tabular-nums text-muted-foreground">{index + 1}</span>
              {task}
            </li>
          ))}
        </ol>
        <div className="flex flex-col gap-single border border-normal p-double text-lg">
          {["Tea light", "Phone charger", "Toaster", "Wind turbine", "Power plant"].map((item, index) => (
            <div key={item} className="flex items-center gap-double border-b border-normal py-single last:border-b-0">
              <span aria-hidden className="text-xl text-muted-foreground">
                ⠿
              </span>
              <span className="w-8 tabular-nums text-muted-foreground">{index + 1}</span>
              {item}
            </div>
          ))}
        </div>
        <div className="flex gap-double text-lg">
          <span className="border border-normal bg-active-base px-double py-single font-semibold text-active-foreground">Start</span>
          <span className="border border-normal px-double py-single">Best {quiz.best ?? "–"}</span>
        </div>
      </div>
    </Page>
  );
}

function LeaderboardPage() {
  return (
    <Page icon="list-ordered" title="Leaderboard">
      <table className="w-full border-collapse text-lg">
        <thead>
          <tr className="text-left text-muted-foreground">
            {["#", "Learner", "Points", "Physics", "Heating", "Cooling", "Demand", "Badges"].map((head) => (
              <th key={head} className="border-b border-normal px-double py-single font-medium">
                {head}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {LEADERS.map((row) => (
            <tr key={row[0]} className="border-b border-normal" style={row[1] === "Ann K." ? { background: "color-mix(in srgb, var(--active-base) 14%, transparent)", fontWeight: 600 } : undefined}>
              {row.map((cell, index) => (
                <td key={index} className="px-double py-single tabular-nums">
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </Page>
  );
}

const INTRO = [
  "These quizzes train your feeling for the numbers behind energy-efficient architecture: power and energy, U-values, heating and cooling loads, air change rates and energy demands. Each quiz consists of a few tasks in which you classify, sort or match.",
  "Every run draws and shuffles the items anew, so no two runs look the same. A run only counts as a whole: answer every task, then submit. Your answers are saved as you go, and you may repeat a quiz as often as you like; your best run counts.",
  "Every task earns between 0 and 100 %, and partially correct answers earn partial points. Swapping two neighbours costs little, mistaking a tea light for a power plant costs a lot.",
  "You decide how you appear: anonymously, under a pseudonym or with your name. There is no password: whoever later enters the same pseudonym or name continues the same record.",
];

const BADGES = [
  ["🧲", "Physics Expert", "Scored 100 % in a run of the Physical Understanding quiz.", true],
  ["🔥", "Heating Expert", "Scored 100 % in a run of the Heating quiz.", false],
  ["❄️", "Cooling Expert", "Scored 100 % in a run of the Cooling quiz.", true],
  ["📊", "Energy Demand Expert", "Scored 100 % in a run of the Energy Demand quiz.", false],
  ["🧮", "Numerical Brain", "Sorted every sorting task without a single mistake.", true],
  ["🔍", "Pattern Seer", "Solved every classification task without a single mistake.", false],
  ["🏁", "Completionist", "Submitted at least one run of every quiz.", false],
] as const;

function Segments({ labels, on }: { readonly labels: readonly string[]; readonly on: number }) {
  return (
    <div role="group" className="flex w-fit border border-normal">
      {labels.map((label, index) => (
        <button key={label} type="button" aria-pressed={index === on} className={cn(chip, index === on && "bg-active-base text-active-foreground")}>
          {label}
        </button>
      ))}
    </div>
  );
}
//#endregion 📄️Pages

const IDS = ["learner", "physics", "intro", "heating", "board", "cooling", "badges", "demand", "prefs"] as const;

const DESKTOP_CELLS: Record<string, LayeredCell> = {
  learner: { column: 0, row: 0 },
  physics: { column: 1, row: 0 },
  intro: { column: 2, row: 0 },
  heating: { column: 0, row: 1 },
  board: { column: 1, row: 1 },
  cooling: { column: 2, row: 1 },
  badges: { column: 0, row: 2 },
  demand: { column: 1, row: 2 },
  prefs: { column: 2, row: 2 },
};
const TABLET_CELLS: Record<string, LayeredCell> = {
  learner: { column: 0, row: 0 },
  physics: { column: 1, row: 0 },
  intro: { column: 0, row: 1 },
  heating: { column: 1, row: 1 },
  board: { column: 0, row: 2 },
  cooling: { column: 0, row: 3 },
  badges: { column: 1, row: 3 },
  demand: { column: 0, row: 4 },
  prefs: { column: 1, row: 4 },
};

const PANES: readonly LayeredPane[] = [
  { id: "learner", label: "Ann K.", icon: "user", render: () => (<Page icon="user" title="Ann K."><div className="flex max-w-3xl flex-col gap-double text-lg"><p className="text-3xl font-semibold tabular-nums">620 points</p><p>Rank 8 of 41 · 2 of 4 quizzes played · 3 of 7 badges</p><table className="w-full border-collapse"><tbody>{["Physics · 82 % · 12 May", "Cooling · 100 % · 14 May", "Heating · open run"].map((r) => (<tr key={r} className="border-b border-normal"><td className="py-single">{r}</td></tr>))}</tbody></table></div></Page>) },
  { id: "physics", label: "Physical Understanding", render: () => <QuizPage quiz={QUIZZES[0]} /> },
  { id: "intro", label: "How it works", icon: "info", render: () => (<Page icon="info" title="How the quizzes work"><div className="mx-auto flex max-w-3xl flex-col gap-double text-lg leading-normal">{INTRO.map((p) => (<p key={p}>{p}</p>))}</div></Page>) },
  { id: "heating", label: "Heating", render: () => <QuizPage quiz={QUIZZES[1]} /> },
  { id: "board", label: "Leaderboard", icon: "list-ordered", render: () => <LeaderboardPage /> },
  { id: "cooling", label: "Cooling", render: () => <QuizPage quiz={QUIZZES[2]} /> },
  { id: "badges", label: "Badges", icon: "award", render: () => (<Page icon="award" title="Badges"><ul className="mx-auto flex max-w-3xl flex-col gap-double">{BADGES.map(([emoji, name, text, earned]) => (<li key={name} className={cn("flex items-center gap-double border border-normal p-double", !earned && "opacity-50")}><span aria-hidden className="text-3xl">{emoji}</span><div><p className="text-xl font-semibold">{name}</p><p className="text-lg text-muted-foreground">{text}</p></div></li>))}</ul></Page>) },
  { id: "demand", label: "Energy Demand", render: () => <QuizPage quiz={QUIZZES[3]} /> },
  { id: "prefs", label: "Preferences", icon: "settings", render: () => (<Page icon="settings" title="Preferences"><div className="flex max-w-3xl flex-col gap-double text-lg">{[["Language", ["EN", "DE"]], ["Theme", ["System", "Light", "Dark"]], ["Text size", ["A", "A+", "A++"]]].map(([label, opts]) => (<div key={label as string} className="flex items-center gap-double"><span className="w-32 text-muted-foreground">{label as string}</span><Segments labels={opts as string[]} on={0} /></div>))}</div></Page>) },
];

function renderCard(pane: LayeredPane, state: { readonly mode: "strip" | "list"; readonly revealed: boolean; readonly open: () => void }) {
  const base = { id: `c-${pane.id}`, card: pane.id, revealed: state.revealed, mode: state.mode, open: state.open };
  const quiz = QUIZZES.find((q) => q.id === pane.id);
  if (quiz) {
    const action = quiz.state === "start" ? "Start" : quiz.state === "resume" ? "Resume" : "Play again";
    return (
      <Card {...base} icon={glyph(quiz.emoji)} title={quiz.title} left={quiz.state === "resume" || quiz.best ? <Quiet>Last result</Quiet> : undefined} right={<Go>{action}</Go>}>
        <p className="text-xs leading-normal text-foreground">{quiz.text}</p>
        <Facts items={[`${quiz.tasks.length} tasks`, quiz.best ? `Best ${quiz.best}` : "Not played yet", ...(quiz.state === "resume" ? ["Run open"] : [])]} />
      </Card>
    );
  }
  switch (pane.id) {
    case "board":
      return (
        <Card {...base} icon={iconGlyph("list-ordered")} title="Leaderboard" left={<Quiet>Updated 14:02</Quiet>} right={<Go>Full leaderboard</Go>}>
          <div className="min-h-0 flex-1 overflow-auto">
            <table className="w-full border-collapse text-xs">
              <caption className="sr-only">Top five learners and you</caption>
              <tbody>
                {LEADERS.slice(0, 5).map((row) => (
                  <tr key={row[0]} className="border-t border-normal">
                    <td className="px-single py-half tabular-nums">{row[0]}</td>
                    <th scope="row" className="px-single py-half text-left font-normal">{row[1]}</th>
                    <td className="px-single py-half text-right tabular-nums">{row[2]}</td>
                  </tr>
                ))}
                <tr aria-current="true" className="border-t border-normal font-semibold" style={{ boxShadow: "inset 3px 0 0 var(--active-base)", background: "color-mix(in srgb, var(--active-base) 14%, transparent)" }}>
                  <td className="px-single py-half tabular-nums">8</td>
                  <th scope="row" className="px-single py-half text-left font-semibold">Ann K.</th>
                  <td className="px-single py-half text-right tabular-nums">620</td>
                </tr>
              </tbody>
            </table>
          </div>
        </Card>
      );
    case "learner":
      return (
        <Card {...base} icon={iconGlyph("user")} title="Ann K." right={<Go>Switch learner</Go>}>
          <p className="text-lg font-semibold tabular-nums">620 points</p>
          <Facts items={["2 of 4 quizzes played", "3 of 7 badges", "Rank 8 of 41"]} />
        </Card>
      );
    case "intro":
      return (
        <Card {...base} icon={iconGlyph("info")} title="How it works" right={<Go>Read more</Go>}>
          <p className="text-xs leading-normal text-muted-foreground">Classify, sort and match. Every run draws and shuffles anew, counts only as a whole, and your best run counts.</p>
        </Card>
      );
    case "badges":
      return (
        <Card {...base} icon={iconGlyph("award")} title="Badges" right={<Go>All badges</Go>}>
          <ul className="flex flex-wrap gap-single">
            {BADGES.map(([emoji, name, , earned]) => (
              <li key={name} className={cn("flex size-workbench items-center justify-center border border-normal text-lg", earned ? "text-foreground" : "text-muted-foreground opacity-50")}>
                <span aria-hidden>{emoji}</span>
                <span className="sr-only">{`${name}: ${earned ? "earned" : "locked"}`}</span>
              </li>
            ))}
          </ul>
          <p className="text-xs text-muted-foreground">3 of 7 earned</p>
        </Card>
      );
    default:
      return (
        <Card {...base} icon={iconGlyph("settings")} title="Preferences" right={<Go>Open</Go>}>
          <p className="text-xs leading-normal text-muted-foreground">Language, theme and text size.</p>
        </Card>
      );
  }
}

function Home() {
  const viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const tablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  const surfaceChrome = useMemo(() => {
    const device: "mobile" | "tablet" | "desktop" = viewportMobile ? "mobile" : readStoredUiChromeLayout(storage) === "tablet" ? "tablet" : "desktop";
    return { appearance: readStoredUiChromeAppearance(storage), device, driver: readStoredUiDriver(storage) };
  }, [viewportMobile]);
  useElementsSurfaceChrome(surfaceChrome);
  const listMode = forced.get("mode") === "list" ? true : forced.get("mode") === "strip" ? false : viewportMobile;
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden bg-background text-foreground">
      <a href="#main" className="sr-only focus:not-sr-only focus:absolute focus:left-double focus:top-double focus:z-50 focus:bg-background focus:p-double">
        Skip to content
      </a>
      <header>
        <Navbar
          items={[
            { key: "brand", content: (<div className="flex min-w-0 shrink-0 items-center gap-single"><ShellBrandLogo svg={LOGO} className="size-workbench shrink-0" /><span className="px-single text-sm font-semibold text-foreground">Architecture and Technology Quizzes</span></div>) },
            navbarFillItem("fill"),
            { key: "conn", content: <span className="px-single text-xs text-muted-foreground">Online · answers saved</span> },
          ]}
          showFullscreenToggle={false}
        />
      </header>
      <main id="main" tabIndex={-1} className="relative min-h-0 flex-1">
        <LayeredOverview
          panes={PANES}
          cells={tablet ? TABLET_CELLS : DESKTOP_CELLS}
          renderCard={renderCard}
          mode={listMode ? "list" : "strip"}
          overlayClassName={cn("quiz-home-grid", tablet && "pointer-events-auto overflow-auto")}
          overviewLabel="Overview"
          gridLabel="Quizzes"
          waitingLabel={(pane) => `${pane.label} is waiting to start`}
          failedLabel={(pane) => `${pane.label} could not be loaded.`}
          lifecycle={{ warmStartMs: 600, warmIntervalMs: 250 }}
        />
      </main>
    </div>
  );
}

mountUiRoot(document.getElementById("root")!, <Home />);
