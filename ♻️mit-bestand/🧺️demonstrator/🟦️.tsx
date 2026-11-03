// #region 🧲️Header
/** 🎪️ Entwerfen mit Bestand demonstrator landing — general introduction and eight live apps as the pages of one {@link LayeredOverview},
 * with the partner credits in its chrome. */
// #endregion 🧲️Header

import { mountUiRoot, useUiCallback as useCallback, useUiMemo as useMemo, useUiState as useState } from "@semio-tech/ui-react/runtime";
import {
  LayeredOverview,
  Navbar,
  ShellBrandLogo,
  UIIntroduction,
  bootstrapElementsSurfaceChromeDocument,
  capturePosterFromCanvases,
  centeredLastRowCells,
  nearSquareGrid,
  readStoredUiChromeAppearance,
  readStoredUiDriver,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MEDIA_QUERY,
  elementsSurfaceDeviceForMatches,
  useElementsSurfaceChrome,
  useLabel,
  useLabelFormatter,
  useMediaQuery,
  type LayeredCardState,
  type LayeredChromeState,
  type LayeredDirection,
  type LayeredPane,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort, resolvePlaygroundBoot } from "@semio-tech/framework";
import { composeSpecificOsCatalogV1 } from "@semio-tech/s-dev-catalog-ts";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
import { createFrameworkOsHostPortsV1 } from "@semio-tech/framework-os-dev/host-ports";
import { DemonstratorCard } from "./⚛️demonstrator-card.tsx";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "./⚛️footer.tsx";
import { DemonstratorLocalePicker, DemonstratorLocaleSwitcher } from "./⚛️locale-picker.tsx";
import { commitDemonstratorLocale, demonstratorUiLabel, readDemonstratorLocale } from "./🔨️modules/🌐️locale/⚛️.ts";
import { type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";
import { DEMONSTRATOR_PANES, ENTWERFEN_MIT_BESTAND_LOGO_SVG, ENTWERFEN_MIT_BESTAND_NAME, demonstratorGeneralIntroduction, demonstratorPaneBootVariants, demonstratorPaneViews, type DemonstratorPaneView } from "./🪧️brand.ts";
import "./🎨️globals.css";

// 🎪️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
const demonstratorStorage = createBrowserStoragePort();

/** 🪝️ The one set of ports every pane's shell reads: the page's composition root binds them once. */
const FRAMEWORK_OS_HOST_PORTS = createFrameworkOsHostPortsV1();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(demonstratorStorage));
// 🌐️ No language is assumed: the stored choice, else the browser's preferred languages, is committed synchronously before the
// first render so the landing page's own chrome never flashes another language; with neither, the viewer is asked.
const INITIAL_LOCALE = readDemonstratorLocale();
if (INITIAL_LOCALE !== undefined) commitDemonstratorLocale(INITIAL_LOCALE);

/** 📱️ Touch phones swipe through the same grid one app at a time instead of seeing every card at once. */
const DEMONSTRATOR_SWIPE_MEDIA_QUERY = `${UI_MOBILE_MEDIA_QUERY} and (hover: none) and (pointer: coarse)`;

//#region 🎪️DemonstratorPages
/** 📍️ The eight apps fill a gapless 4 × 2 in row-major order. */
const DEMONSTRATOR_CELLS = Object.fromEntries(centeredLastRowCells(DEMONSTRATOR_PANES.length).map((cell, index) => [DEMONSTRATOR_PANES[index]!.id, cell]));

/** 🔢️ The card grid mirrors the strip. */
const DEMONSTRATOR_GRID = nearSquareGrid(DEMONSTRATOR_PANES.length);
const DEMONSTRATOR_OVERLAY_STYLE = { gridTemplateColumns: `repeat(${DEMONSTRATOR_GRID.columns}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${DEMONSTRATOR_GRID.rows}, minmax(min(10rem, 100%), 1fr))` } as const;

/** 🧮️ Every app may stay live; boots start 1.5 s after load, one per 35 s plugin-load budget. A pristine app is released to its poster after
 * 30 s offscreen while another is opened, 5 min idle on the overview or 60 s in a hidden tab — the unattended kiosk case. */
const DEMONSTRATOR_LIFECYCLE = { budget: DEMONSTRATOR_PANES.length, warmStartMs: 1_500, warmIntervalMs: 35_000, suspendIdleMs: 5 * 60_000, suspendOffscreenMs: 30_000, suspendHiddenMs: 60_000 } as const;

/** 📏️ When swiping, the cards and the neighbour hints stay between the navbar over the top of the overview and the partner credits over its
 * bottom (two rows of logos on a phone). */
const DEMONSTRATOR_INSETS = { top: "calc(var(--size-workbench) * 1.5)", bottom: "5.5rem" } as const;

/** 🎪️ One app's live shell: the standalone module it runs, the branded app id its manifest declares, its tour only while opened. */
function DemonstratorShell({ pane, locale, opened }: { readonly pane: DemonstratorPaneView; readonly locale: DemonstratorLocale; readonly opened: boolean }) {
  const variants = demonstratorPaneBootVariants(pane.variant);
  const catalog = useMemo(() => composeSpecificOsCatalogV1(window.location.href, { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }).catalog, []);
  const runtimeBoot = useMemo(() => resolvePlaygroundBoot(catalog, variants.runtime), [catalog, variants.runtime]);
  const manifestBoot = useMemo(() => resolvePlaygroundBoot(catalog, variants.manifest), [catalog, variants.manifest]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand.locks]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);
  return (
    <FrameworkOsShell catalog={catalog}
      pluginFilter={variants.runtime}
      plugins={runtimeBoot.plugins}
      surfaceSessionFactories={PUZZLE_BOARD_SESSION_FACTORIES}
      hostPorts={FRAMEWORK_OS_HOST_PORTS}
      appId={manifestBoot.defaultAppId}
      locks={locks}
      defaults={defaults}
      brand={pane.brand}
      shellId={pane.id}
      storageNamespace={pane.id}
      initialLocale={locale}
      suppressAutoIntroduction={!opened}
    />
  );
}

const demonstratorPaneShortLabel = (pane: DemonstratorPaneView): string => pane.shortWindowTitle.split(" · ").at(-1) ?? pane.label;

/** 📄️ The eight apps as pages of the overview, worded and branded for `locale`. */
const demonstratorPages = (views: readonly DemonstratorPaneView[], locale: DemonstratorLocale): readonly LayeredPane[] =>
  views.map((pane) => ({ id: pane.id, label: pane.label, shortLabel: demonstratorPaneShortLabel(pane), icon: pane.icon, capturePoster: capturePosterFromCanvases, render: ({ opened }) => <DemonstratorShell pane={pane} locale={locale} opened={opened} /> }));

/** 🃏️ An app's card, centred in its cell of the card grid (when swiping, the overview centres it on its page itself). */
const demonstratorCardRenderer = (views: readonly DemonstratorPaneView[]) => (page: LayeredPane, state: LayeredCardState) => {
  const card = <DemonstratorCard pane={views.find((view) => view.id === page.id)!} lifted={state.revealed} onClick={state.open} />;
  return state.mode === "swipe" ? card : <div className="flex min-h-0 min-w-0 justify-center overflow-x-hidden overflow-y-auto px-double">{card}</div>;
};
//#endregion 🎪️DemonstratorPages

//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
  const [locale, setLocale] = useState<DemonstratorLocale | undefined>(INITIAL_LOCALE);
  const chooseLocale = useCallback((chosen: DemonstratorLocale) => {
    commitDemonstratorLocale(chosen);
    setLocale(chosen);
  }, []);
  return locale === undefined ? <DemonstratorLocalePicker onChoose={chooseLocale} /> : <DemonstratorLocaleLanding locale={locale} onChooseLocale={chooseLocale} />;
}

function DemonstratorLocaleLanding({ locale, onChooseLocale }: { readonly locale: DemonstratorLocale; readonly onChooseLocale: (locale: DemonstratorLocale) => void }) {
  const views = useMemo(() => demonstratorPaneViews(locale), [locale]);
  const pages = useMemo(() => demonstratorPages(views, locale), [views, locale]);
  const renderCard = useMemo(() => demonstratorCardRenderer(views), [views]);
  const introduction = useMemo(() => demonstratorGeneralIntroduction(locale), [locale]);
  const viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const viewportTablet = useMediaQuery(UI_TABLET_MEDIA_QUERY);
  const swipe = useMediaQuery(DEMONSTRATOR_SWIPE_MEDIA_QUERY);
  const surfaceChrome = useMemo(
    () => ({
      appearance: readStoredUiChromeAppearance(demonstratorStorage),
      device: elementsSurfaceDeviceForMatches({ mobile: viewportMobile, tablet: viewportTablet }),
      driver: readStoredUiDriver(demonstratorStorage),
    }),
    [viewportMobile, viewportTablet],
  );
  useElementsSurfaceChrome(surfaceChrome);

  const [introductionStep, setIntroductionStep] = useState(0);
  const [introductionDismissed, setIntroductionDismissed] = useState(false);
  const gridLabel = useLabel(demonstratorUiLabel("demonstrator.landing.grid"));
  const overviewLabel = useLabel(demonstratorUiLabel("demonstrator.landing.overview"));
  const waitingLabel = useLabelFormatter(demonstratorUiLabel("demonstrator.landing.paneWaiting"));
  const failedLabel = useLabelFormatter(demonstratorUiLabel("demonstrator.landing.paneFailed"));
  const up = useLabelFormatter(demonstratorUiLabel("demonstrator.landing.neighbourUp"));
  const left = useLabelFormatter(demonstratorUiLabel("demonstrator.landing.neighbourLeft"));
  const right = useLabelFormatter(demonstratorUiLabel("demonstrator.landing.neighbourRight"));
  const down = useLabelFormatter(demonstratorUiLabel("demonstrator.landing.neighbourDown"));
  const labels = useMemo(
    () => ({
      grid: gridLabel,
      overview: overviewLabel,
      waiting: (page: LayeredPane) => waitingLabel({ label: page.label }),
      failed: (page: LayeredPane) => failedLabel({ label: page.label }),
      neighbour: (page: LayeredPane, direction: LayeredDirection) => ({ up, left, right, down })[direction]({ label: page.label }),
    }),
    [gridLabel, overviewLabel, waitingLabel, failedLabel, up, left, right, down],
  );
  const dismissIntroduction = useCallback((id: string | null) => id !== null && setIntroductionDismissed(true), []);

  const renderChrome = ({ revealed, opened }: LayeredChromeState) =>
    opened ? null : (
      <>
        {revealed ? null : (
          <div className="pointer-events-none absolute inset-x-0 bottom-0 z-40 flex flex-wrap items-end justify-between gap-single px-double py-single">
            <div className="pointer-events-auto">{aProjectOfLuhUdkFooterItem("landingProjectOf", locale, viewportMobile).content}</div>
            <div className="pointer-events-auto">{fundedByZukunftBauFooterItem("landingFundedBy", locale, viewportMobile).content}</div>
          </div>
        )}
        {introductionDismissed ? null : (
          <UIIntroduction introduction={introduction} stepIndex={introductionStep} completedInteractionIndices={[]} onStepIndexChange={setIntroductionStep} onDismiss={() => setIntroductionDismissed(true)} />
        )}
        {revealed ? null : (
          <div className="pointer-events-none absolute inset-x-0 top-0 z-40">
            <Navbar
              label={ENTWERFEN_MIT_BESTAND_NAME}
              items={[
                {
                  key: "logoAndTitle",
                  centered: true,
                  content: (
                    <div className="flex min-w-0 shrink-0 items-center gap-single">
                      <ShellBrandLogo svg={ENTWERFEN_MIT_BESTAND_LOGO_SVG} className="size-workbench shrink-0" />
                      <h1 data-slot="app-name" className="m-0 px-single text-sm font-semibold text-foreground">
                        {ENTWERFEN_MIT_BESTAND_NAME}
                      </h1>
                    </div>
                  ),
                },
                { key: "language", content: <DemonstratorLocaleSwitcher locale={locale} onChoose={onChooseLocale} /> },
              ]}
              showFullscreenToggle={false}
              className="bg-transparent"
            />
          </div>
        )}
      </>
    );

  return (
    <main className="h-full w-full">
      <LayeredOverview
        panes={pages}
        cells={DEMONSTRATOR_CELLS}
        renderCard={renderCard}
        overlayClassName="grid items-center"
        overlayStyle={DEMONSTRATOR_OVERLAY_STYLE}
        renderChrome={renderChrome}
        mode={swipe ? "swipe" : "strip"}
        insets={DEMONSTRATOR_INSETS}
        lifecycle={DEMONSTRATOR_LIFECYCLE}
        labels={labels}
        onOpenedIdChange={dismissIntroduction}
      />
    </main>
  );
}
//#endregion 🎪️DemonstratorLanding

mountUiRoot(document.getElementById("root")!, <DemonstratorLanding />);
