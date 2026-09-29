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
  initUiLocaleSync,
  nearSquareGrid,
  readStoredUiChromeAppearance,
  readStoredUiChromeLayout,
  readStoredUiDriver,
  registerUiTranslationBundles,
  UI_MOBILE_MEDIA_QUERY,
  useElementsSurfaceChrome,
  useLabel,
  useLabelFormatter,
  useMediaQuery,
  type LayeredCardState,
  type LayeredChromeState,
  type LayeredPane,
} from "@semio-tech/ui-react";
import { createBrowserStoragePort, resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "@semio-tech/plugin-registry/catalog";
import { FrameworkOsShell, resolveShellLocks, resolveShellDefaults } from "@semio-tech/framework-renderer-react";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-js";
import { DemonstratorCard } from "./⚛️demonstrator-card.tsx";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "./⚛️footer.tsx";
import { DEMONSTRATOR_LOCALE, DEMONSTRATOR_PANES, ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION, ENTWERFEN_MIT_BESTAND_LOGO_SVG, demonstratorPaneBootVariants, type DemonstratorPaneSpec } from "./🪧️brand.ts";
import "./🎨️globals.css";

// 🎪️ Page-owning (single React root, no `ShellScope` of its own) — plain browser storage is correct;
// each pane's own `FrameworkOsShell` gets its own `ShellScope` (ephemeral brands → in-memory storage).
const demonstratorStorage = createBrowserStoragePort();

bootstrapElementsSurfaceChromeDocument(readStoredUiChromeAppearance(demonstratorStorage));
// 🇩🇪️ The whole demonstrator is German-locked (see 🪧️brand.ts) — resolve synchronously before the
// first render so the landing page's own chrome (Skip/Back/Next/Done) never flashes English.
initUiLocaleSync(DEMONSTRATOR_LOCALE);

/** 📱️ Touch-first viewports use the vertical snap list even when wider than {@link UI_MOBILE_MEDIA_QUERY}. */
const DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY = `${UI_MOBILE_MEDIA_QUERY} and (hover: none) and (pointer: coarse)`;

//#region 🌐️DemonstratorLandingLabels
/** 🌐️ The landing's own chrome strings, for English AND German (no default language) — the German lock picks German. */
export const demonstratorLandingUiLabel = registerUiTranslationBundles({
  en: {
    translation: {
      demonstrator: {
        landing: {
          grid: { label: { normal: "Every demonstrator", beginner: "Every demonstrator" } },
          overview: { label: { normal: "Overview", beginner: "Back to all demonstrators" } },
          paneWaiting: { label: { normal: "{{label}} is being prepared", beginner: "{{label}} is being prepared" } },
          paneFailed: { label: { normal: "{{label}} could not be loaded.", beginner: "{{label}} could not be loaded." } },
        },
      },
    },
  },
  de: {
    translation: {
      demonstrator: {
        landing: {
          grid: { label: { normal: "Alle Demonstratoren", beginner: "Alle Demonstratoren" } },
          overview: { label: { normal: "Übersicht", beginner: "Zurück zu allen Demonstratoren" } },
          paneWaiting: { label: { normal: "{{label}} wird vorbereitet", beginner: "{{label}} wird vorbereitet" } },
          paneFailed: { label: { normal: "{{label}} konnte nicht geladen werden.", beginner: "{{label}} konnte nicht geladen werden." } },
        },
      },
    },
  },
});
//#endregion 🌐️DemonstratorLandingLabels

//#region 🎪️DemonstratorPages
/** 📍️ The eight apps fill a gapless 4 × 2 in row-major order. */
const DEMONSTRATOR_CELLS = Object.fromEntries(centeredLastRowCells(DEMONSTRATOR_PANES.length).map((cell, index) => [DEMONSTRATOR_PANES[index]!.id, cell]));

/** 🔢️ The card grid mirrors the strip. */
const DEMONSTRATOR_GRID = nearSquareGrid(DEMONSTRATOR_PANES.length);
const DEMONSTRATOR_OVERLAY_STYLE = { gridTemplateColumns: `repeat(${DEMONSTRATOR_GRID.columns}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${DEMONSTRATOR_GRID.rows}, minmax(0, 1fr))` } as const;

/** 🧮️ Every app may stay live; boots start 1.5 s after load, one per 35 s plugin-load budget. A pristine app is released to its poster after
 * 30 s offscreen while another is opened, 5 min idle on the overview or 60 s in a hidden tab — the unattended kiosk case. */
const DEMONSTRATOR_LIFECYCLE = { budget: DEMONSTRATOR_PANES.length, warmStartMs: 1_500, warmIntervalMs: 35_000, suspendIdleMs: 5 * 60_000, suspendOffscreenMs: 30_000, suspendHiddenMs: 60_000 } as const;

/** 🎪️ One app's live shell: the standalone module it runs, the branded app id its manifest declares, its tour only while opened. */
function DemonstratorShell({ pane, opened }: { readonly pane: DemonstratorPaneSpec; readonly opened: boolean }) {
  const variants = demonstratorPaneBootVariants(pane.variant);
  const runtimeBoot = useMemo(() => resolvePlaygroundBoot(PLUGIN_CATALOG, variants.runtime), [variants.runtime]);
  const manifestBoot = useMemo(() => resolvePlaygroundBoot(PLUGIN_CATALOG, variants.manifest), [variants.manifest]);
  const locks = useMemo(() => resolveShellLocks(pane.brand.locks), [pane.brand]);
  const defaults = useMemo(() => resolveShellDefaults(pane.brand, undefined), [pane.brand]);
  return (
    <FrameworkOsShell
      pluginFilter={variants.runtime}
      plugins={runtimeBoot.plugins}
      surfaceSessionFactories={PUZZLE_BOARD_SESSION_FACTORIES}
      appId={manifestBoot.defaultAppId}
      locks={locks}
      defaults={defaults}
      brand={pane.brand}
      shellId={pane.id}
      storageNamespace={pane.id}
      suppressAutoIntroduction={!opened}
    />
  );
}

const DEMONSTRATOR_PAGES: readonly LayeredPane[] = DEMONSTRATOR_PANES.map((pane) => ({ id: pane.id, label: pane.label, icon: pane.icon, capturePoster: capturePosterFromCanvases, render: ({ opened }) => <DemonstratorShell pane={pane} opened={opened} /> }));
const DEMONSTRATOR_SPECS = new Map(DEMONSTRATOR_PANES.map((pane) => [pane.id, pane]));

/** 🃏️ An app's card, centred in its cell of the card grid (list sections centre it themselves). */
function renderDemonstratorCard(page: LayeredPane, state: LayeredCardState) {
  const card = <DemonstratorCard pane={DEMONSTRATOR_SPECS.get(page.id)!} lifted={state.revealed} onClick={state.open} />;
  return state.mode === "list" ? card : <div className="flex min-w-0 justify-center px-double">{card}</div>;
}
//#endregion 🎪️DemonstratorPages

//#region 🎪️DemonstratorLanding
function DemonstratorLanding() {
  const viewportMobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const touchList = useMediaQuery(DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY);
  const surfaceChrome = useMemo(
    () => ({
      appearance: readStoredUiChromeAppearance(demonstratorStorage),
      device: (viewportMobile ? "mobile" : readStoredUiChromeLayout(demonstratorStorage) === "tablet" ? "tablet" : "desktop") as "mobile" | "tablet" | "desktop",
      driver: readStoredUiDriver(demonstratorStorage),
    }),
    [viewportMobile],
  );
  useElementsSurfaceChrome(surfaceChrome);

  const [introductionStep, setIntroductionStep] = useState(0);
  const [introductionDismissed, setIntroductionDismissed] = useState(false);
  const gridLabel = useLabel(demonstratorLandingUiLabel("demonstrator.landing.grid"));
  const overviewLabel = useLabel(demonstratorLandingUiLabel("demonstrator.landing.overview"));
  const waitingLabel = useLabelFormatter(demonstratorLandingUiLabel("demonstrator.landing.paneWaiting"));
  const failedLabel = useLabelFormatter(demonstratorLandingUiLabel("demonstrator.landing.paneFailed"));
  const labels = useMemo(
    () => ({ grid: gridLabel, overview: overviewLabel, waiting: (page: LayeredPane) => waitingLabel({ label: page.label }), failed: (page: LayeredPane) => failedLabel({ label: page.label }) }),
    [gridLabel, overviewLabel, waitingLabel, failedLabel],
  );
  const dismissIntroduction = useCallback((id: string | null) => id !== null && setIntroductionDismissed(true), []);

  const renderChrome = ({ revealed, opened }: LayeredChromeState) =>
    opened ? null : (
      <>
        {revealed ? null : (
          <div className="pointer-events-none absolute inset-x-0 bottom-0 z-40 flex flex-wrap items-end justify-between gap-single px-double py-single">
            <div className="pointer-events-auto">{aProjectOfLuhUdkFooterItem("landingProjectOf", DEMONSTRATOR_LOCALE, false).content}</div>
            <div className="pointer-events-auto">{fundedByZukunftBauFooterItem("landingFundedBy", DEMONSTRATOR_LOCALE, false).content}</div>
          </div>
        )}
        {introductionDismissed ? null : (
          <UIIntroduction introduction={ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION} stepIndex={introductionStep} completedInteractionIndices={[]} onStepIndexChange={setIntroductionStep} onDismiss={() => setIntroductionDismissed(true)} />
        )}
        {revealed ? null : (
          <div className="pointer-events-none absolute inset-x-0 top-0 z-40">
            <Navbar
              label="Entwerfen mit Bestand"
              items={[
                {
                  key: "logoAndTitle",
                  centered: true,
                  content: (
                    <div className="flex min-w-0 shrink-0 items-center gap-single">
                      <ShellBrandLogo svg={ENTWERFEN_MIT_BESTAND_LOGO_SVG} className="size-workbench shrink-0" />
                      <span data-slot="app-name" className="px-single text-sm font-semibold text-foreground">
                        Entwerfen mit Bestand
                      </span>
                    </div>
                  ),
                },
              ]}
              showFullscreenToggle={false}
              className="bg-transparent"
            />
          </div>
        )}
      </>
    );

  return (
    <LayeredOverview
      panes={DEMONSTRATOR_PAGES}
      cells={DEMONSTRATOR_CELLS}
      renderCard={renderDemonstratorCard}
      overlayClassName="grid items-center"
      overlayStyle={DEMONSTRATOR_OVERLAY_STYLE}
      renderChrome={renderChrome}
      mode={touchList ? "list" : "strip"}
      lifecycle={DEMONSTRATOR_LIFECYCLE}
      labels={labels}
      onOpenedIdChange={dismissIntroduction}
    />
  );
}
//#endregion 🎪️DemonstratorLanding

mountUiRoot(document.getElementById("root")!, <DemonstratorLanding />);
