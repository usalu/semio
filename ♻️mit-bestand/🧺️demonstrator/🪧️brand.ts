// #region 🧲️Header
/** 🏷️ Entwerfen mit Bestand demonstrator brands — shared landing introduction plus per-app shell brands. */
// #endregion 🧲️Header

import { DEMONSTRATOR_HOST, DEMONSTRATOR_ASSETS_DIR, demonstratorPaneRuntimeVariant } from "./🔨️modules/🧩️runtime/📇️catalog/🟦️.ts";
export { DEMONSTRATOR_HOST, DEMONSTRATOR_ASSETS_DIR, demonstratorPaneRuntimeVariant };
import {
  panelTabElementId,
  panelTabFirstDraggableElementId,
  windowElementId,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  type IntroductionDefinition,
  type ShellBrand,
  type ShellFooterItem,
  type TutorialDefinition,
} from "@semio-tech/framework";
import type { IconName } from "@semio-tech/ui-react";
import { demonstratorLabel, demonstratorText, type DemonstratorLocale } from "./🔨️modules/🌐️locale/🟦️.ts";

/** 🏛️ Credits shared by the demonstrator's shells, captioned and described in the viewer's `locale`. */
export const demonstratorFooterItems = (locale: DemonstratorLocale): readonly ShellFooterItem[] => [
  { id: "aProjectOfLuhUdk", placement: "leading", caption: { en: demonstratorText("demonstrator.footer.aProjectOf", "en"), de: demonstratorText("demonstrator.footer.aProjectOf", "de") }, separator: { en: demonstratorText("demonstrator.footer.and", "en"), de: demonstratorText("demonstrator.footer.and", "de") }, logos: [
    { href: "https://www.iek.uni-hannover.de/ngs/team", src: `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/🎓️luh/☀️logo.png`, darkSrc: `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/🎓️luh/🌙️logo-dark.png`, alt: demonstratorText("demonstrator.partner.luh", locale) },
    { href: "https://www.udk-berlin.de/studium/architektur/fachgebiete/konstruktives-entwerfen-und-tragwerksplanung/team-2025-2026/", src: `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/🎨️udk/☀️logo.png`, darkSrc: `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/🎨️udk/🌙️logo-dark.png`, alt: demonstratorText("demonstrator.partner.udk", locale) },
  ] },
  { id: "fundedByZukunftBau", placement: "trailing", caption: { en: demonstratorText("demonstrator.footer.fundedBy", "en"), de: demonstratorText("demonstrator.footer.fundedBy", "de") }, logos: [
    { href: "https://www.zukunftbau.de/projekte/forschungsfoerderung/1008187-2506", src: `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/🔮️zukunft-bau/☀️logo.png`, darkSrc: `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/🔮️zukunft-bau/🌙️logo-dark.png`, alt: demonstratorText("demonstrator.partner.zukunftBau", locale) },
  ] },
];

//#region 🏷️DemonstratorShared
/** 🏷️ Shell brand ids that receive Entwerfen-mit-Bestand partner chrome in the react renderer. */
export const ENTWERFEN_MIT_BESTAND_BRAND_IDS = [
  "entwerfen-mit-bestand-aggregator",
  "entwerfen-mit-bestand-aussuchen",
  "entwerfen-mit-bestand-bearbeiten",
  "entwerfen-mit-bestand-energie",
  "entwerfen-mit-bestand-generator",
  "entwerfen-mit-bestand-koordinator",
  "entwerfen-mit-bestand-statik",
  "entwerfen-mit-bestand-verfolgen",
] as const;

export type EntwerfenMitBestandBrandId = (typeof ENTWERFEN_MIT_BESTAND_BRAND_IDS)[number];

/** 🏷️ Whether a shell brand id is one of the demonstrator's Entwerfen-mit-Bestand panes. */
export function isEntwerfenMitBestandBrandId(id: string | undefined): id is EntwerfenMitBestandBrandId {
  return id !== undefined && (ENTWERFEN_MIT_BESTAND_BRAND_IDS as readonly string[]).includes(id);
}

/** 🏷️ The project name, the same proper noun in every locale. */
export const ENTWERFEN_MIT_BESTAND_NAME = "Entwerfen mit Bestand";

/** ✒️ Semio emblem shared across demonstrator brands and the landing page. */
export const ENTWERFEN_MIT_BESTAND_LOGO_SVG = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Entwerfen mit Bestand"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

const demonstratorLogoUrl = (file: string) => `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/${file}`;

/** 🎓️ General demonstrator introduction, logo texts in the viewer's `locale`, shown on the landing page only (not inside app shells). */
export const demonstratorGeneralIntroduction = (locale: DemonstratorLocale): IntroductionDefinition => ({
  title: demonstratorLabel("demonstrator.brand.shared.welcomeToDesigningWith"),
  steps: [
    {
      id: "welcome",
      title: demonstratorLabel("demonstrator.brand.shared.welcomeToDesigningWith"),
      body: demonstratorLabel("demonstrator.brand.general.welcome.body"),
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [],
      demonstrations: [],
    },
    {
      id: "prototype",
      title: demonstratorLabel("demonstrator.brand.shared.earlyPrototype"),
      body: demonstratorLabel("demonstrator.brand.general.prototype.body"),
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [],
      demonstrations: [],
    },
    {
      id: "funding",
      title: demonstratorLabel("demonstrator.brand.shared.fundingNotice"),
      body: demonstratorLabel("demonstrator.brand.shared.thisProjectIsFunded"),
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [
        { src: demonstratorLogoUrl("🏢️bmwsb/☀️logo.png"), darkSrc: demonstratorLogoUrl("🏢️bmwsb/🌙️logo-dark.png"), alt: demonstratorText("demonstrator.partner.bmwsb", locale), href: "https://www.bmwsb.bund.de" },
        { src: demonstratorLogoUrl("🏛️bbsr/☀️logo.png"), darkSrc: demonstratorLogoUrl("🏛️bbsr/🌙️logo-dark.png"), alt: demonstratorText("demonstrator.partner.bbsr", locale), href: "https://www.bbsr.bund.de" },
        { src: demonstratorLogoUrl("🔮️zukunft-bau/☀️logo.png"), darkSrc: demonstratorLogoUrl("🔮️zukunft-bau/🌙️logo-dark.png"), alt: demonstratorText("demonstrator.partner.zukunftBau", locale), href: "https://www.zukunftbau.de/projekte/forschungsfoerderung/1008187-2506" },
      ],
      demonstrations: [],
    },
  ],
});

//#endregion 🏷️DemonstratorShared

//#region 🎬️EntwerfenMitBestandTutorial
/** 🪟️ The Aggregator's one 3D window instance — matches `puzzle/plugin/rs/lib.rs`'s `puzzle3d-main` window
 * kind (unsplit base instance, so the instance id equals the window kind id). Camera keyframes below key
 * off this raw instance id, never the `windowElementId(...)`-transformed element id (that's for anchoring
 * chrome, not for `TutorialCameraKeyframe.windowId`/`ViewWindowInstance.id`). */
const PUZZLE3D_MAIN_WINDOW_ID = "puzzle3d-main";

/** 🎥️ A camera-only keyframe helper — every pose in this tutorial keeps the same up vector and field of
 * view, so only `position`/`target` vary shot to shot. */
function heroCameraKeyframe(at: number, position: readonly [number, number, number], target: readonly [number, number, number]): TutorialDefinition["tracks"]["camera"][number] {
  return {
    at,
    windowId: PUZZLE3D_MAIN_WINDOW_ID,
    camera: { kind: "orbit", position: [...position], target: [...target], up: [0, 0, 1], fov: 45 },
    easing: "easeInOut",
  };
}

/** 🎬️ The Aggregator's first recorded-tutorial demo — a ~4-minute, 12-chapter walkthrough mirroring
 * `ENTWERFEN_MIT_BESTAND_BRAND.introduction`'s tour verbatim (same German narration, same element ids,
 * same demonstrated gestures) but as a TIMED, VOICED, SEEKABLE recording rather than a step-gated
 * walkthrough: the user presses Play once and the whole app follows along, camera included, instead of
 * clicking Next/performing each interaction step by step.
 *
 * Hand-authored skeleton, deliberately sparse on the document track: `tracks.document` is empty. Real
 * document mutation (the `addObjectKind`/`setVortexShow`/`acceptSuggestion`/`setFillCount` edits a live
 * run of this tour would produce) is intentionally NOT hand-invented here — inventing plausible-looking
 * `forwards`/`backwards` op JSON would be indistinguishable from a real recording but silently wrong, and
 * the whole point of `TutorialDocumentEventKind::Edit` is that its ops are copied verbatim from a real
 * `vcs::Edit`. The authoring path (see the ticket) is: ship this skeleton (narration + camera + gestures +
 * UI deltas + annotational events already carry the full experience), then run the tutorial recorder once
 * against a live Aggregator session performing this exact script, and merge the captured `document`/`camera`
 * tracks in. Until that pass lands, playback still narrates, moves the camera, opens panels, and pulses the
 * relevant chrome at each annotational event — it just won't materialize the document edits themselves.
 */
export const ENTWERFEN_MIT_BESTAND_TUTORIAL: TutorialDefinition = {
  id: "aggregator-tour",
  title: demonstratorLabel("demonstrator.brand.tutorial.aggregatorTour.title"),
  description: demonstratorLabel("demonstrator.brand.tutorial.aggregatorTour.description"),
  durationMs: 240_000,
  chapters: [
    { id: "welcome", at: 0, title: demonstratorLabel("demonstrator.brand.tutorial.welcome.title") },
    { id: "prototype", at: 20_000, title: demonstratorLabel("demonstrator.brand.shared.earlyPrototype") },
    { id: "funding", at: 33_000, title: demonstratorLabel("demonstrator.brand.shared.fundingNotice") },
    { id: "viewport", at: 48_000, title: demonstratorLabel("demonstrator.brand.shared.the3DView") },
    { id: "panels", at: 72_000, title: demonstratorLabel("demonstrator.brand.shared.panels") },
    { id: "catalogue-objects", at: 88_000, title: demonstratorLabel("demonstrator.brand.shared.expandBuildingComponents") },
    { id: "add-object", at: 100_000, title: demonstratorLabel("demonstrator.brand.shared.addABuildingComponent") },
    { id: "transform-utility", at: 118_000, title: demonstratorLabel("demonstrator.brand.shared.transformBuildingComponents") },
    { id: "verbindungspunkte", at: 134_000, title: demonstratorLabel("demonstrator.brand.shared.connectionPoints") },
    { id: "suggest-objects", at: 150_000, title: demonstratorLabel("demonstrator.brand.shared.suggestBuildingComponents") },
    { id: "fill-tool", at: 172_000, title: demonstratorLabel("demonstrator.brand.shared.fill") },
    { id: "fill-distribution", at: 196_000, title: demonstratorLabel("demonstrator.brand.shared.countAndDistribution") },
  ],
  base: {
    exampleId: "concrete-forest",
    ui: {
      focusedWindowId: PUZZLE3D_MAIN_WINDOW_ID,
      activeUtilityByWindowId: {},
      activePanelTabByGroup: {},
      interactionSelection: {},
      expandedTreeIds: [],
      commandPanelOpen: false,
    },
    cameras: [heroCameraKeyframe(0, [40, -40, 26], [7, 0, 3])],
  },
  tracks: {
    narration: [
      {
        id: "welcome",
        at: 0,
        durationMs: 20_000,
        rate: 1,
        captions: [],
        text:
          demonstratorLabel("demonstrator.brand.tutorial.welcome.text"),
      },
      {
        id: "prototype",
        at: 20_000,
        durationMs: 13_000,
        rate: 1,
        captions: [],
        text:
          demonstratorLabel("demonstrator.brand.tutorial.prototype.text"),
      },
      {
        id: "funding",
        at: 33_000,
        durationMs: 15_000,
        rate: 1,
        captions: [],
        text:
          demonstratorLabel("demonstrator.brand.shared.thisProjectIsFunded"),
      },
      {
        id: "viewport",
        at: 48_000,
        durationMs: 24_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.shared.yourDesignFromExisting"),
      },
      {
        id: "panels",
        at: 72_000,
        durationMs: 16_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.tutorial.panels.text"),
      },
      {
        id: "catalogue-objects",
        at: 88_000,
        durationMs: 12_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.tutorial.catalogueObjects.text"),
      },
      {
        id: "add-object",
        at: 100_000,
        durationMs: 18_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.tutorial.addObject.text"),
      },
      {
        id: "transform-utility",
        at: 118_000,
        durationMs: 16_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.tutorial.transformUtility.text"),
      },
      {
        id: "verbindungspunkte",
        at: 134_000,
        durationMs: 16_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.tutorial.verbindungspunkte.text"),
      },
      {
        id: "suggest-objects",
        at: 150_000,
        durationMs: 22_000,
        rate: 1,
        captions: [],
        text:
          demonstratorLabel("demonstrator.brand.tutorial.suggestObjects.text"),
      },
      {
        id: "fill-tool",
        at: 172_000,
        durationMs: 24_000,
        rate: 1,
        captions: [],
        text: demonstratorLabel("demonstrator.brand.tutorial.fillTool.text"),
      },
      {
        id: "fill-distribution",
        at: 196_000,
        durationMs: 44_000,
        rate: 1,
        captions: [],
        text:
          demonstratorLabel("demonstrator.brand.tutorial.fillDistribution.text"),
      },
    ],
    video: [],
    events: [
      { at: 110_000, kind: { kind: "action", action: "addObjectKind" } },
      { at: 141_000, kind: { kind: "action", action: "setVortexShow", args: { show: "always" } } },
      { at: 165_000, kind: { kind: "action", action: "acceptSuggestion" } },
      { at: 181_000, kind: { kind: "action", action: "setFillCount", args: { count: 40 } } },
    ],
    ui: [
      { at: 76_000, sample: { kind: "delta", changes: [{ kind: "panelTab", group: "top-left", tabId: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }] } },
      { at: 93_000, sample: { kind: "delta", changes: [{ kind: "treeExpansion", id: "puzzle3d-play-kinds.objects", expanded: true }] } },
      { at: 125_000, sample: { kind: "delta", changes: [{ kind: "activeUtility", windowId: PUZZLE3D_MAIN_WINDOW_ID, utilityId: "transform" }] } },
      { at: 180_000, sample: { kind: "delta", changes: [{ kind: "activeTool", id: "fill" }] } },
    ],
    document: [],
    camera: [
      heroCameraKeyframe(10_000, [34, -32, 22], [7, 0, 3]),
      heroCameraKeyframe(20_000, [30, -30, 20], [7, 0, 3]),
      heroCameraKeyframe(50_000, [30, -30, 20], [7, 0, 3]),
      heroCameraKeyframe(56_000, [22, -22, 15], [7, 0, 3]),
      heroCameraKeyframe(62_000, [24, -24, 15], [9, -2, 3.5]),
      heroCameraKeyframe(72_000, [10, -30, 18], [9, -2, 3.5]),
      heroCameraKeyframe(100_000, [18, -20, 12], [7, 0, 3]),
      heroCameraKeyframe(118_000, [14, -16, 10], [7, 0, 3]),
      heroCameraKeyframe(196_000, [26, -28, 18], [7, 0, 3]),
      heroCameraKeyframe(230_000, [40, -40, 26], [7, 0, 3]),
    ],
    gestures: [
      { at: 50_000, durationMs: 1_000, gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
      {
        at: 56_000,
        durationMs: 3_000,
        gesture: {
          kind: "drag",
          from: { kind: "windowNormalized", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 },
          to: { kind: "windowNormalized", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), x: 0.65, y: 0.4 },
          button: "middle",
          modifiers: [],
        },
        cursor: "move",
      },
      {
        at: 63_000,
        durationMs: 7_000,
        gesture: {
          kind: "orbit",
          from: { kind: "windowNormalized", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), x: 0.35, y: 0.5 },
          to: { kind: "windowNormalized", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), x: 0.65, y: 0.5 },
          button: "right",
          modifiers: ["alt"],
        },
      },
      { at: 74_000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } },
      { at: 92_000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-kinds.objects" } } },
      {
        at: 104_000,
        durationMs: 6_000,
        gesture: {
          kind: "drag",
          from: { kind: "element", id: panelTabFirstDraggableElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID) },
          to: { kind: "windowNormalized", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), x: 0.5, y: 0.55 },
          button: "left",
          modifiers: [],
        },
      },
      { at: 124_000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "transform" } } },
      { at: 140_000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-vortex-show" } } },
      { at: 155_000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "entity", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), domain: "vortex", entity: "*" } } },
      { at: 160_000, durationMs: 500, gesture: { kind: "rightClick", at: { kind: "entity", id: windowElementId(PUZZLE3D_MAIN_WINDOW_ID), domain: "vortex", entity: "*" } } },
      { at: 178_000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } },
    ],
  },
};
//#endregion 🎬️EntwerfenMitBestandTutorial

//#region 🏷️EntwerfenMitBestandAggregatorBrand
/** 🏷️ Aggregator (puzzle3d): reuse terminology, app-specific introduction, recorded tutorial, Abbau Aufbau default example. */
export const ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-aggregator",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  tutorials: [ENTWERFEN_MIT_BESTAND_TUTORIAL],
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.aggregator.entwerfenMitBestandAggregator.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.shared.the3DView"),
        body: demonstratorLabel("demonstrator.brand.shared.yourDesignFromExisting"),
        introduce: windowElementId("puzzle3d-main"),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: "puzzle3d-main" }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: "puzzle3d-main" }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
          { on: { kind: "orbit", id: "puzzle3d-main" }, label: demonstratorLabel("demonstrator.brand.shared.orbitDragWithAlt") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId("puzzle3d-main"), x: 0.5, y: 0.5 }, deltaY: -100 } },
          {
            gesture: {
              kind: "drag",
              from: { kind: "windowNormalized", id: windowElementId("puzzle3d-main"), x: 0.5, y: 0.5 },
              to: { kind: "windowNormalized", id: windowElementId("puzzle3d-main"), x: 0.65, y: 0.4 },
              button: "middle",
            },
            cursor: "move",
          },
          { gesture: { kind: "orbit", from: { kind: "windowNormalized", id: windowElementId("puzzle3d-main"), x: 0.35, y: 0.5 }, to: { kind: "windowNormalized", id: windowElementId("puzzle3d-main"), x: 0.65, y: 0.5 } } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.aggregator.panels.body"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
      {
        id: "catalogue-objects",
        title: demonstratorLabel("demonstrator.brand.shared.expandBuildingComponents"),
        body: demonstratorLabel("demonstrator.brand.aggregator.catalogueObjects.body"),
        introduce: "puzzle3d-play-kinds.objects",
        show: [panelTabElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID)],
        placement: "right",
        interactions: [{ on: { kind: "expand", id: "puzzle3d-play-kinds.objects" }, label: demonstratorLabel("demonstrator.brand.aggregator.puzzle3dPlayKinds.objects.label") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-kinds.objects" } } }],
      },
      {
        id: "add-object",
        title: demonstratorLabel("demonstrator.brand.shared.addABuildingComponent"),
        body: demonstratorLabel("demonstrator.brand.aggregator.addObject.body"),
        introduce: panelTabFirstDraggableElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID),
        show: [panelTabElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID), windowElementId("puzzle3d-main")],
        placement: "right",
        interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: demonstratorLabel("demonstrator.brand.aggregator.addObjectKind.label") }],
        ordered: false,
        logos: [],
        demonstrations: [
          {
            gesture: {
              kind: "drag",
              from: { kind: "element", id: panelTabFirstDraggableElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID) },
              to: { kind: "windowNormalized", id: windowElementId("puzzle3d-main"), x: 0.5, y: 0.55 },
            },
          },
        ],
      },
      {
        id: "transform-utility",
        title: demonstratorLabel("demonstrator.brand.shared.transformBuildingComponents"),
        body: demonstratorLabel("demonstrator.brand.aggregator.transformUtility.body"),
        introduce: "transform",
        show: [windowElementId("puzzle3d-main")],
        placement: "auto",
        interactions: [{ on: { kind: "utility", id: "transform" }, label: demonstratorLabel("demonstrator.brand.aggregator.transform.label") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "transform" } } }],
      },
      {
        id: "verbindungspunkte",
        title: demonstratorLabel("demonstrator.brand.shared.connectionPoints"),
        body: demonstratorLabel("demonstrator.brand.aggregator.verbindungspunkte.body"),
        introduce: "puzzle3d-play-vortex-show",
        show: [windowElementId("puzzle3d-main")],
        placement: "auto",
        interactions: [{ on: { kind: "action", id: "setVortexShow" }, label: demonstratorLabel("demonstrator.brand.aggregator.setVortexShow.label") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-vortex-show" } } }],
      },
      {
        id: "suggest-objects",
        title: demonstratorLabel("demonstrator.brand.shared.suggestBuildingComponents"),
        body: demonstratorLabel("demonstrator.brand.aggregator.suggestObjects.body"),
        introduce: windowElementId("puzzle3d-main"),
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "action", id: "acceptSuggestion" }, label: demonstratorLabel("demonstrator.brand.aggregator.acceptSuggestion.label") }],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "leftClick", at: { kind: "entity", id: windowElementId("puzzle3d-main"), domain: "vortex", entity: "*" } } },
          { gesture: { kind: "rightClick", at: { kind: "entity", id: windowElementId("puzzle3d-main"), domain: "vortex", entity: "*" } } },
        ],
      },
      {
        id: "fill-tool",
        title: demonstratorLabel("demonstrator.brand.shared.fill"),
        body: demonstratorLabel("demonstrator.brand.aggregator.fillTool.body"),
        introduce: "tool.fill",
        show: [],
        placement: "top",
        interactions: [{ on: { kind: "tool", id: "fill" }, label: demonstratorLabel("demonstrator.brand.aggregator.fill.label") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } }],
      },
      {
        id: "fill-distribution",
        title: demonstratorLabel("demonstrator.brand.shared.countAndDistribution"),
        body: demonstratorLabel("demonstrator.brand.aggregator.fillDistribution.body"),
        introduce: "puzzle3d-play-distribution",
        show: ["puzzle3d-fill-count", panelTabElementId("tool.fill")],
        placement: "top",
        interactions: [],
        ordered: false,
        logos: [],
        demonstrations: [],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandAggregatorBrand

const PROCEDURAL_MAIN_WINDOW_ID = "procedural-main";
const CAD_SHAPE_WINDOW_ID = "cad-play-shape";

//#region 🏷️EntwerfenMitBestandGeneratorBrand
/** 🏷️ Generator (generation3d): parametric flow editor for reuse-oriented component generation. */
export const ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-generator",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "hexagonal-mushroom-column" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.generator.entwerfenMitBestandGenerator.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.generator.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.generator.viewport.body"),
        introduce: windowElementId(PROCEDURAL_MAIN_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: PROCEDURAL_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: PROCEDURAL_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
          { on: { kind: "orbit", id: PROCEDURAL_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.orbitDragWithAlt") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(PROCEDURAL_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.shared.useTheTabsIn"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandGeneratorBrand

//#region 🏷️EntwerfenMitBestandKoordinatorBrand
/** 🏷️ Koordinator (cad): multi-model coordination for shape, building, energy, and structure views. */
export const ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-koordinator",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "demo" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.koordinator.entwerfenMitBestandKoordinator.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.koordinator.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.koordinator.viewport.body"),
        introduce: windowElementId(CAD_SHAPE_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: CAD_SHAPE_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: CAD_SHAPE_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
          { on: { kind: "orbit", id: CAD_SHAPE_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.orbitDragWithAlt") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(CAD_SHAPE_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.koordinator.panels.body"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandKoordinatorBrand

const SOURCING_POOL_WINDOW_ID = "sourcing-pool";
const PROCESS_WORKPIECE_WINDOW_ID = "process-workpiece";
const GIS2D_MAIN_WINDOW_ID = "gis2d-main";

//#region 🏷️EntwerfenMitBestandAussuchenBrand
/** 🏷️ Aussuchen (sourcing): curating reclaimed building components from available stock. */
export const ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-aussuchen",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "demo" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.aussuchen.entwerfenMitBestandAussuchen.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.aussuchen.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.aussuchen.viewport.body"),
        introduce: windowElementId(SOURCING_POOL_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [],
        ordered: false,
        logos: [],
        demonstrations: [],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.shared.useTheTabsIn"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandAussuchenBrand

//#region 🏷️EntwerfenMitBestandBearbeitenBrand
/** 🏷️ Bearbeiten (process3d): machining steps that adapt a reclaimed component to its new use. */
export const ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-bearbeiten",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.bearbeiten.entwerfenMitBestandBearbeiten.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.bearbeiten.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.bearbeiten.viewport.body"),
        introduce: windowElementId(PROCESS_WORKPIECE_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: PROCESS_WORKPIECE_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: PROCESS_WORKPIECE_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
          { on: { kind: "orbit", id: PROCESS_WORKPIECE_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.orbitDragWithAlt") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(PROCESS_WORKPIECE_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.shared.useTheTabsIn"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandBearbeitenBrand

//#region 🏷️EntwerfenMitBestandVerfolgenBrand
/** 🏷️ Verfolgen (gis2d): tracking where reclaimed components come from and where they go. */
export const ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-verfolgen",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "demo" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.verfolgen.entwerfenMitBestandVerfolgen.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.verfolgen.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.verfolgen.viewport.body"),
        introduce: windowElementId(GIS2D_MAIN_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: GIS2D_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: GIS2D_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(GIS2D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.shared.useTheTabsIn"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandVerfolgenBrand

/** 🪟️ The Energy editor's main viewport — `s.energy.model@1/*#editor`'s `energy.model.3d` window, the
 * `world-3d` surface its edit-mode layout leads with at `MODEL_VIEWPORT_SHARE` (0.55); source of truth is
 * `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs`'s
 * `WINDOW_KIND_ID`, NOT the committed `✏️s/🔌️plugins/🔋️energy/🔣️.json`, whose `windowKinds` list is stale:
 * it omits this window entirely and still spells `framework.window.tree` as `block-list`. The three data
 * windows beside it are `framework.window.tree` (Struktur), `framework.window.table` (Zonen) and
 * `energy.simulation` (Energiesimulation). */
const ENERGY_MODEL_MAIN_WINDOW_ID = "energy.model.3d";

/** 🪟️ The Energy editor's simulation window — `energy.simulation`, the run transport and settings beside
 * the 3D model (`…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`'s `WINDOW_KIND_ID`). */
const ENERGY_SIMULATION_WINDOW_ID = "energy.simulation";

/** 🪟️ The FEM 3D editor's main window kind — `s.fem.fem3d@1/*#editor`'s `fem3d-model` window (see
 * `✏️s/🔌️plugins/🏗️fem/🔣️.json`), a `world-3d` surface, so the Statik introduction gates on the same
 * zoom/pan/orbit gestures as the other 3D brands. */
const FEM3D_MODEL_WINDOW_ID = "fem3d-model";

//#region 🏷️EntwerfenMitBestandEnergieBrand
/** 🏷️ Energie (energy): thermal simulation of designs made from reused building components. */
export const ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-energie",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "bestest-600" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.energie.entwerfenMitBestandEnergie.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.energie.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.energie.viewport.body"),
        introduce: windowElementId(ENERGY_MODEL_MAIN_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: ENERGY_MODEL_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: ENERGY_MODEL_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
          { on: { kind: "orbit", id: ENERGY_MODEL_MAIN_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.orbitDragWithAlt") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(ENERGY_MODEL_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.energie.panels.body"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [windowElementId(ENERGY_SIMULATION_WINDOW_ID)],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandEnergieBrand

//#region 🏷️EntwerfenMitBestandStatikBrand
/** 🏷️ Statik (fem3d): structural analysis of the Tragwerk assembled from reused building components. */
export const ENTWERFEN_MIT_BESTAND_STATIK_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-statik",
  windowTitle: ENTWERFEN_MIT_BESTAND_NAME,
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: demonstratorLabel("demonstrator.brand.statik.entwerfenMitBestandStatik.title"),
    steps: [
      {
        id: "viewport",
        title: demonstratorLabel("demonstrator.brand.statik.viewport.title"),
        body: demonstratorLabel("demonstrator.brand.statik.viewport.body"),
        introduce: windowElementId(FEM3D_MODEL_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: FEM3D_MODEL_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.zoomMouseWheel") },
          { on: { kind: "pan", id: FEM3D_MODEL_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.panDragWithMiddle") },
          { on: { kind: "orbit", id: FEM3D_MODEL_WINDOW_ID }, label: demonstratorLabel("demonstrator.brand.shared.orbitDragWithAlt") },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(FEM3D_MODEL_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: demonstratorLabel("demonstrator.brand.shared.panels"),
        body: demonstratorLabel("demonstrator.brand.statik.panels.body"),
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: demonstratorLabel("demonstrator.brand.shared.clickTheCatalogueTab") }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandStatikBrand

//#region 🎪️DemonstratorPanes
/** 🎪️ One live pane in the demonstrator's 4×2 grid — order here IS grid order (row-major: index
 * 0-3 top row, 4-7 bottom row). */
export type DemonstratorPaneSpec = {
  readonly id: string;
  readonly variant: string;
  readonly brand: ShellBrand;
  readonly icon: IconName;
  readonly text: (locale: DemonstratorLocale) => DemonstratorPaneText;
};

/** 🏷️ A pane's wording in one locale. */
export type DemonstratorPaneText = {
  readonly label: string;
  readonly tagline: string;
  /** 📖 Overview copy — blank-line separated paragraphs (same convention as introduction steps). */
  readonly description: string;
  readonly windowTitle: string;
  readonly shortWindowTitle: string;
};

/** 🌐️ A pane as one viewer sees it: its wording plus its brand carrying the localized window titles and partner credits. */
export type DemonstratorPaneView = Omit<DemonstratorPaneSpec, "text" | "brand"> & DemonstratorPaneText & { readonly brand: ShellBrand };

/** 🗣️ Resolves every pane in `locale`. */
export const demonstratorPaneViews = (locale: DemonstratorLocale): readonly DemonstratorPaneView[] =>
  DEMONSTRATOR_PANES.map(({ text, brand, ...pane }) => {
    const wording = text(locale);
    return { ...pane, ...wording, brand: { ...brand, windowTitle: wording.windowTitle, shortWindowTitle: wording.shortWindowTitle, footerItems: demonstratorFooterItems(locale) } };
  });

/** 📖 Splits pane overview copy into introduction-style body paragraphs. */
export function demonstratorPaneDescriptionParagraphs(description: string): readonly string[] {
  return description.split(/\n\n+/).map((paragraph) => paragraph.trim()).filter((paragraph) => paragraph.length > 0);
}

/** 🧭️ Separates the module-owning runtime variant from the branded pane's manifest row.
 * Generator executes the standalone procedural module, but its branded `generator` row carries the
 * canonical app id that the module manifest actually declares. */
export function demonstratorPaneBootVariants(variant: string): { readonly runtime: string; readonly manifest: string } {
  return { runtime: demonstratorPaneRuntimeVariant(variant), manifest: variant };
}

export const DEMONSTRATOR_PANES: readonly DemonstratorPaneSpec[] = [
  {
    id: "generator",
    variant: "generator",
    brand: ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.generator.label", locale), tagline: demonstratorText("demonstrator.brand.pane.generator.tagline", locale), description: demonstratorText("demonstrator.brand.pane.generator.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.generator.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.generator.shortWindowTitle", locale) }),
    icon: "workflow",
  },
  {
    id: "koordinator",
    variant: "koordinator",
    brand: ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.koordinator.label", locale), tagline: demonstratorText("demonstrator.brand.pane.koordinator.tagline", locale), description: demonstratorText("demonstrator.brand.pane.koordinator.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.koordinator.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.koordinator.shortWindowTitle", locale) }),
    icon: "cad-shape",
  },
  {
    id: "aggregator",
    variant: "aggregator",
    brand: ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.aggregator.label", locale), tagline: demonstratorText("demonstrator.brand.pane.aggregator.tagline", locale), description: demonstratorText("demonstrator.brand.pane.aggregator.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.aggregator.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.aggregator.shortWindowTitle", locale) }),
    icon: "puzzle",
  },
  {
    id: "energie",
    variant: "energy",
    brand: ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.energie.label", locale), tagline: demonstratorText("demonstrator.brand.pane.energie.tagline", locale), description: demonstratorText("demonstrator.brand.pane.energie.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.energie.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.energie.shortWindowTitle", locale) }),
    icon: "sun",
  },
  {
    id: "aussuchen",
    variant: "aussuchen",
    brand: ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.aussuchen.label", locale), tagline: demonstratorText("demonstrator.brand.pane.aussuchen.tagline", locale), description: demonstratorText("demonstrator.brand.pane.aussuchen.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.aussuchen.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.aussuchen.shortWindowTitle", locale) }),
    icon: "library",
  },
  {
    id: "bearbeiten",
    variant: "bearbeiten",
    brand: ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.bearbeiten.label", locale), tagline: demonstratorText("demonstrator.brand.pane.bearbeiten.tagline", locale), description: demonstratorText("demonstrator.brand.pane.bearbeiten.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.bearbeiten.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.bearbeiten.shortWindowTitle", locale) }),
    icon: "hammer",
  },
  {
    id: "verfolgen",
    variant: "verfolgen",
    brand: ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.verfolgen.label", locale), tagline: demonstratorText("demonstrator.brand.pane.verfolgen.tagline", locale), description: demonstratorText("demonstrator.brand.pane.verfolgen.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.verfolgen.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.verfolgen.shortWindowTitle", locale) }),
    icon: "gis2d",
  },
  {
    id: "statik",
    variant: "fem3d",
    brand: ENTWERFEN_MIT_BESTAND_STATIK_BRAND,
    text: (locale) => ({ label: demonstratorText("demonstrator.brand.pane.statik.label", locale), tagline: demonstratorText("demonstrator.brand.pane.statik.tagline", locale), description: demonstratorText("demonstrator.brand.pane.statik.description", locale), windowTitle: demonstratorText("demonstrator.brand.pane.statik.windowTitle", locale), shortWindowTitle: demonstratorText("demonstrator.brand.pane.statik.shortWindowTitle", locale) }),
    icon: "fem-app",
  },
];
//#endregion 🎪️DemonstratorPanes

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️demonstratorpanebootvariants/🟦️.ts");
  await registerTests1(import.meta.vitest, { demonstratorPaneBootVariants });
  const { registerTests1: registerExampleTests } = await import("./🧪️tests/🧪️demonstratorexamples/🟦️.ts");
  await registerExampleTests(import.meta.vitest, { panes: DEMONSTRATOR_PANES });
}

/** 🏷️ Complete owner catalog of the eight standalone shell identities. */
export const DEMONSTRATOR_SHELL_BRANDS: readonly ShellBrand[] = DEMONSTRATOR_PANES.map((pane) => pane.brand);
