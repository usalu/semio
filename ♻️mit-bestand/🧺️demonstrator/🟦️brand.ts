// #region 🧲️Header
/** @emoji 🏷️ Entwerfen mit Bestand demonstrator brands — shared landing introduction plus per-app shell brands. */
// #endregion 🧲️Header

import {
  panelTabElementId,
  panelTabFirstDraggableElementId,
  windowElementId,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  type IntroductionDefinition,
  type ShellBrand,
  type ShellLocale,
  type TutorialDefinition,
} from "../../🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts";
import type { IconName } from "@semio-tech/ui-react";

//#region 🏷️DemonstratorShared
/** @emoji 🇩🇪️ The whole demonstrator is German-locked (every brand's `locks.locale` below) — the
 * single source the landing page's boot-time `initUiLocaleSync` call reads, so it can never drift
 * from the per-app brands. */
export const DEMONSTRATOR_LOCALE: ShellLocale = "de";

/** @emoji 🌐️ Production host for the merged demonstrator static site. */
export const DEMONSTRATOR_HOST = "demonstrator.entwerfen.mit-bestand.de";

/** @emoji 🗂️ Repo-root-relative static assets for all demonstrator brands and the landing page. */
export const DEMONSTRATOR_ASSETS_DIR = "♻️mit-bestand/🧺️demonstrator/🖼️asset";

/** @emoji 🏷️ Shell brand ids that receive Entwerfen-mit-Bestand partner chrome in the react renderer. */
export const ENTWERFEN_MIT_BESTAND_BRAND_IDS = [
  "entwerfen-mit-bestand-aggregator",
  "entwerfen-mit-bestand-aussuchen",
  "entwerfen-mit-bestand-bearbeiten",
  "entwerfen-mit-bestand-generator",
  "entwerfen-mit-bestand-koordinator",
  "entwerfen-mit-bestand-verfolgen",
] as const;

export type EntwerfenMitBestandBrandId = (typeof ENTWERFEN_MIT_BESTAND_BRAND_IDS)[number];

/** @emoji ✒️ Semio emblem shared across demonstrator brands and the landing page. */
export const ENTWERFEN_MIT_BESTAND_LOGO_SVG = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Entwerfen mit Bestand"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

const demonstratorLogoUrl = (file: string) => `/${DEMONSTRATOR_ASSETS_DIR}/🪧️logos/${file}`;

/** @emoji 🎓️ General demonstrator introduction shown on the landing page only (not inside app shells). */
export const ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION: IntroductionDefinition = {
  title: "Willkommen bei Entwerfen mit Bestand",
  steps: [
    {
      id: "welcome",
      title: "Willkommen bei Entwerfen mit Bestand",
      body: "Der Demonstrator vereint sechs Werkzeuge des Forschungsprojekts „Entwerfen mit Bestand“ der Leibniz Universität Hannover und der Universität der Künste Berlin.\n\nDas Projekt entwickelt eine offene Plattform, um neue Strukturen aus wiederverwendeten Baukomponenten zu entwerfen — mit vereinfachter Tragwerks- und Lebenszyklusanalyse, KI-Unterstützung entlang funktionaler und struktureller Abhängigkeiten.",
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
      title: "Früher Prototyp",
      body: "Dieser Demonstrator befindet sich in aktiver Entwicklung. Viele Funktionen sind noch unvollständig oder nur als Platzhalter vorhanden — sie zeigen die Richtung des Projekts, nicht seinen finalen Stand. Wenn die Seite nicht mehr richtig funktioniert, können Sie sie neu laden.",
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
      title: "Förderhinweis",
      body: "Dieses Projekt wird gefördert vom Bundesinstitut für Bau-, Stadt- und Raumforschung im Auftrag des Bundesministeriums für Wohnen, Stadtentwicklung und Bauwesen aus Mitteln der Zukunft Bau Forschungsförderung.",
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [
        { src: demonstratorLogoUrl("🖼️bmwsb.png"), darkSrc: demonstratorLogoUrl("🖼️bmwsb-dark.png"), alt: "Bundesministerium für Wohnen, Stadtentwicklung und Bauwesen", href: "https://www.bmwsb.bund.de" },
        { src: demonstratorLogoUrl("🖼️bbsr.png"), darkSrc: demonstratorLogoUrl("🖼️bbsr-dark.png"), alt: "Bundesinstitut für Bau-, Stadt- und Raumforschung", href: "https://www.bbsr.bund.de" },
        { src: demonstratorLogoUrl("🖼️zukunft-bau.png"), darkSrc: demonstratorLogoUrl("🖼️zukunft-bau-dark.png"), alt: "Zukunft Bau", href: "https://www.zukunftbau.de/projekte/forschungsfoerderung/1008187-2506" },
      ],
      demonstrations: [],
    },
  ],
};

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
    camera: { kind: "orbit", position, target, up: [0, 0, 1], fov: 45 },
    easing: "easeInOut",
  };
}

/** @emoji 🎬️ The Aggregator's first recorded-tutorial demo — a ~4-minute, 12-chapter walkthrough mirroring
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
  title: "Aggregator-Tour",
  description: "Eine geführte, gesprochene Tour durch den Aggregator: Ansicht, Katalog, Transformieren, Verbindungspunkte und Füllen.",
  durationMs: 240_000,
  chapters: [
    { id: "welcome", at: 0, title: "Willkommen" },
    { id: "prototype", at: 20_000, title: "Früher Prototyp" },
    { id: "funding", at: 33_000, title: "Förderhinweis" },
    { id: "viewport", at: 48_000, title: "Die 3D-Ansicht" },
    { id: "panels", at: 72_000, title: "Paneele" },
    { id: "catalogue-objects", at: 88_000, title: "Baukomponenten aufklappen" },
    { id: "add-object", at: 100_000, title: "Baukomponente hinzufügen" },
    { id: "transform-utility", at: 118_000, title: "Baukomponenten transformieren" },
    { id: "verbindungspunkte", at: 134_000, title: "Verbindungspunkte" },
    { id: "suggest-objects", at: 150_000, title: "Baukomponenten vorschlagen" },
    { id: "fill-tool", at: 172_000, title: "Füllen" },
    { id: "fill-distribution", at: 196_000, title: "Anzahl und Verteilung" },
  ],
  base: {
    exampleId: "concrete-forest",
    ui: {
      focusedWindowId: PUZZLE3D_MAIN_WINDOW_ID,
      activeUtilityByWindowId: {},
      activePanelTabByGroup: {},
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
          "Der Aggregator ist der Demonstrator des Forschungsprojekts „Entwerfen mit Bestand“ der Leibniz Universität Hannover und der Universität der Künste Berlin. Das Projekt entwickelt eine offene Plattform, um neue Strukturen aus wiederverwendeten Baukomponenten zu entwerfen — mit vereinfachter Tragwerks- und Lebenszyklusanalyse, KI-Unterstützung entlang funktionaler und struktureller Abhängigkeiten.",
      },
      {
        id: "prototype",
        at: 20_000,
        durationMs: 13_000,
        rate: 1,
        captions: [],
        text:
          "Dieser Demonstrator befindet sich in aktiver Entwicklung. Viele Funktionen sind noch unvollständig oder nur als Platzhalter vorhanden — sie zeigen die Richtung des Projekts, nicht seinen finalen Stand.",
      },
      {
        id: "funding",
        at: 33_000,
        durationMs: 15_000,
        rate: 1,
        captions: [],
        text:
          "Dieses Projekt wird gefördert vom Bundesinstitut für Bau-, Stadt- und Raumforschung im Auftrag des Bundesministeriums für Wohnen, Stadtentwicklung und Bauwesen aus Mitteln der Zukunft Bau Forschungsförderung.",
      },
      {
        id: "viewport",
        at: 48_000,
        durationMs: 24_000,
        rate: 1,
        captions: [],
        text: "Hier entsteht Ihr Entwurf aus Bestandskomponenten — zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
      },
      {
        id: "panels",
        at: 72_000,
        durationMs: 16_000,
        rate: 1,
        captions: [],
        text: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Einstellungen. Wir öffnen jetzt den Katalog-Reiter.",
      },
      {
        id: "catalogue-objects",
        at: 88_000,
        durationMs: 12_000,
        rate: 1,
        captions: [],
        text: "»Baukomponenten« im Katalog klappt die verfügbaren Arten auf.",
      },
      {
        id: "add-object",
        at: 100_000,
        durationMs: 18_000,
        rate: 1,
        captions: [],
        text: "Per Drag-and-Drop ziehen wir die erste Baukomponente aus dem Katalog in die 3D-Ansicht.",
      },
      {
        id: "transform-utility",
        at: 118_000,
        durationMs: 16_000,
        rate: 1,
        captions: [],
        text: "Das Transformieren-Hilfsmittel verschiebt und dreht Baukomponenten.",
      },
      {
        id: "verbindungspunkte",
        at: 134_000,
        durationMs: 16_000,
        rate: 1,
        captions: [],
        text: "»Verbindungspunkte anzeigen« auf »Immer« gestellt macht die Anschlüsse aller Baukomponenten sichtbar.",
      },
      {
        id: "suggest-objects",
        at: 150_000,
        durationMs: 22_000,
        rate: 1,
        captions: [],
        text:
          "Ein Verbindungspunkt per Linksklick gewählt und per Rechtsklick das Aktionsmenü geöffnet — »Baukomponenten vorschlagen« zeigt passende Anschlüsse, ein Linksklick auf einen Vorschlag platziert ihn.",
      },
      {
        id: "fill-tool",
        at: 172_000,
        durationMs: 24_000,
        rate: 1,
        captions: [],
        text: "Das Werkzeug »Füllen« füllt den Entwurf automatisch mit Baukomponenten.",
      },
      {
        id: "fill-distribution",
        at: 196_000,
        durationMs: 44_000,
        rate: 1,
        captions: [],
        text:
          "Die Anzahl stellen wir am Schieberegler ein, die Verteilung justieren wir per Ziehen an den Reglern — sie gewichtet, welche Baukomponenten und Verbindungspunkte beim Füllen bevorzugt werden. Damit endet unsere Tour durch den Aggregator.",
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
      // 🎬️ Welcome: a slow establishing push toward the seeded "Abbau Aufbau" prototype.
      heroCameraKeyframe(10_000, [34, -32, 22], [7, 0, 3]),
      heroCameraKeyframe(20_000, [30, -30, 20], [7, 0, 3]),
      // 🎥️ Viewport chapter: the camera itself performs the zoom/pan/orbit gestures being narrated,
      // timed to match the pointer gestures in `tracks.gestures` below.
      heroCameraKeyframe(50_000, [30, -30, 20], [7, 0, 3]),
      heroCameraKeyframe(56_000, [22, -22, 15], [7, 0, 3]),
      heroCameraKeyframe(62_000, [24, -24, 15], [9, -2, 3.5]),
      heroCameraKeyframe(72_000, [10, -30, 18], [9, -2, 3.5]),
      // 🧩️ Settle back onto the seeded object for the catalogue/add-object/transform chapters.
      heroCameraKeyframe(100_000, [18, -20, 12], [7, 0, 3]),
      heroCameraKeyframe(118_000, [14, -16, 10], [7, 0, 3]),
      // 🌐️ Pull back a touch for the fill/distribution chapters to frame the growing assembly.
      heroCameraKeyframe(196_000, [26, -28, 18], [7, 0, 3]),
      // 👋️ Final pull-back — bookends the opening establishing shot.
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
  windowTitle: "Entwerfen mit Bestand · Aggregator",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  tutorials: [ENTWERFEN_MIT_BESTAND_TUTORIAL],
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: "Willkommen beim Aggregator",
    steps: [
      {
        id: "viewport",
        title: "Die 3D-Ansicht",
        body: "Hier entsteht Ihr Entwurf aus Bestandskomponenten — zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
        introduce: windowElementId("puzzle3d-main"),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoomen (Mausrad)" },
          { on: { kind: "pan", id: "puzzle3d-main" }, label: "Verschieben (Mittelklick ziehen)" },
          { on: { kind: "orbit", id: "puzzle3d-main" }, label: "Orbitieren (Alt + Rechtsklick ziehen)" },
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
        title: "Paneele",
        body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Einstellungen. Klicken Sie jetzt mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
      {
        id: "catalogue-objects",
        title: "Baukomponenten aufklappen",
        body: "Klicken Sie mit der linken Maustaste auf »Baukomponenten« im Katalog, um die verfügbaren Arten aufzuklappen.",
        introduce: "puzzle3d-play-kinds.objects",
        show: [panelTabElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID)],
        placement: "right",
        interactions: [{ on: { kind: "expand", id: "puzzle3d-play-kinds.objects" }, label: "»Baukomponenten« anklicken" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-kinds.objects" } } }],
      },
      {
        id: "add-object",
        title: "Baukomponente hinzufügen",
        body: "Ziehen Sie die erste Baukomponente mit der linken Maustaste per Drag-and-Drop aus dem Katalog in die 3D-Ansicht.",
        introduce: panelTabFirstDraggableElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID),
        show: [panelTabElementId(FRAMEWORK_PANEL_TAB_CATALOGUE_ID), windowElementId("puzzle3d-main")],
        placement: "right",
        interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: "Mit linker Maustaste in die Ansicht ziehen" }],
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
        title: "Baukomponenten transformieren",
        body: "Klicken Sie mit der linken Maustaste auf das Transformieren-Hilfsmittel, um Baukomponenten zu verschieben und zu drehen.",
        introduce: "transform",
        show: [windowElementId("puzzle3d-main")],
        placement: "auto",
        interactions: [{ on: { kind: "utility", id: "transform" }, label: "Transformieren anklicken" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "transform" } } }],
      },
      {
        id: "verbindungspunkte",
        title: "Verbindungspunkte",
        body: "Öffnen Sie die Fensteroptionen und stellen Sie per Linksklick »Verbindungspunkte anzeigen« auf »Immer«, damit die Anschlüsse aller Baukomponenten sichtbar werden.",
        introduce: "puzzle3d-play-vortex-show",
        show: [windowElementId("puzzle3d-main")],
        placement: "auto",
        interactions: [{ on: { kind: "action", id: "setVortexShow" }, label: "»Verbindungspunkte anzeigen« auf »Immer« stellen" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-vortex-show" } } }],
      },
      {
        id: "suggest-objects",
        title: "Baukomponenten vorschlagen",
        body: "Wählen Sie einen Verbindungspunkt per Linksklick und öffnen Sie per Rechtsklick das Aktionsmenü. Mit »Baukomponenten vorschlagen« erscheint eine Liste passender Anschlüsse — fahren Sie mit der Maus über einen Eintrag zur Vorschau und wählen Sie ihn per Linksklick aus, um ihn zu platzieren.",
        introduce: windowElementId("puzzle3d-main"),
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "action", id: "acceptSuggestion" }, label: "Vorschlag per Linksklick wählen" }],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "leftClick", at: { kind: "entity", id: windowElementId("puzzle3d-main"), domain: "vortex", entity: "*" } } },
          { gesture: { kind: "rightClick", at: { kind: "entity", id: windowElementId("puzzle3d-main"), domain: "vortex", entity: "*" } } },
        ],
      },
      {
        id: "fill-tool",
        title: "Füllen",
        body: "Klicken Sie mit der linken Maustaste auf das Werkzeug »Füllen« in der Werkzeugleiste, um den Entwurf automatisch mit Baukomponenten zu füllen.",
        introduce: "tool.fill",
        show: [],
        placement: "top",
        interactions: [{ on: { kind: "tool", id: "fill" }, label: "»Füllen« anklicken" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } }],
      },
      {
        id: "fill-distribution",
        title: "Anzahl und Verteilung",
        body: "Stellen Sie die Anzahl am Schieberegler ein und justieren Sie die Verteilung per Ziehen an den Reglern — sie gewichtet, welche Baukomponenten und Verbindungspunkte beim Füllen bevorzugt werden.",
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
/** 🏷️ Generator (procedural3d): parametric flow editor for reuse-oriented component generation. */
export const ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand-generator",
  windowTitle: "Entwerfen mit Bestand · Generator",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "hexagonal-mushroom-column" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: "Willkommen beim Generator",
    steps: [
      {
        id: "viewport",
        title: "Der Ablauf-Editor",
        body: "Im Generator entwerfen Sie parametrische Abläufe für Baukomponenten. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
        introduce: windowElementId(PROCEDURAL_MAIN_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: PROCEDURAL_MAIN_WINDOW_ID }, label: "Zoomen (Mausrad)" },
          { on: { kind: "pan", id: PROCEDURAL_MAIN_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
          { on: { kind: "orbit", id: PROCEDURAL_MAIN_WINDOW_ID }, label: "Orbitieren (Alt + Rechtsklick ziehen)" },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(PROCEDURAL_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: "Paneele",
        body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
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
  windowTitle: "Entwerfen mit Bestand · Koordinator",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "hexagonal-cut-concrete-forest-left" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: "Willkommen beim Koordinator",
    steps: [
      {
        id: "viewport",
        title: "Die Modellansichten",
        body: "Der Koordinator verbindet Form-, Gebäude-, Energie- und Tragwerksmodelle. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
        introduce: windowElementId(CAD_SHAPE_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: CAD_SHAPE_WINDOW_ID }, label: "Zoomen (Mausrad)" },
          { on: { kind: "pan", id: CAD_SHAPE_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
          { on: { kind: "orbit", id: CAD_SHAPE_WINDOW_ID }, label: "Orbitieren (Alt + Rechtsklick ziehen)" },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(CAD_SHAPE_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: "Paneele",
        body: "Über die Reiter öffnen Sie Katalog, Dokument und weitere Paneele. Klicken Sie auf den Katalog-Reiter, um verfügbare Bausteine zu durchsuchen.",
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
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
  windowTitle: "Entwerfen mit Bestand · Aussuchen",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "demo-stock" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: "Willkommen bei Aussuchen",
    steps: [
      {
        id: "viewport",
        title: "Der Bestandspool",
        body: "Im Aussuchen sichten Sie verfügbare Bestandskomponenten und stellen daraus eine Kuratierung zusammen. Der Pool listet alle gefundenen Komponenten mit Verfügbarkeit und Typologie.",
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
        title: "Paneele",
        body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
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
  windowTitle: "Entwerfen mit Bestand · Bearbeiten",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "timber-beam-joinery" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: "Willkommen bei Bearbeiten",
    steps: [
      {
        id: "viewport",
        title: "Das Werkstück",
        body: "Im Bearbeiten legen Sie die Bearbeitungsschritte fest, mit denen eine Bestandskomponente für ihre neue Aufgabe angepasst wird. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
        introduce: windowElementId(PROCESS_WORKPIECE_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: PROCESS_WORKPIECE_WINDOW_ID }, label: "Zoomen (Mausrad)" },
          { on: { kind: "pan", id: PROCESS_WORKPIECE_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
          { on: { kind: "orbit", id: PROCESS_WORKPIECE_WINDOW_ID }, label: "Orbitieren (Alt + Rechtsklick ziehen)" },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(PROCESS_WORKPIECE_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: "Paneele",
        body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
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
  windowTitle: "Entwerfen mit Bestand · Verfolgen",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "reuse-map" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  assetsDir: DEMONSTRATOR_ASSETS_DIR,
  introduction: {
    title: "Willkommen bei Verfolgen",
    steps: [
      {
        id: "viewport",
        title: "Die Karte",
        body: "Im Verfolgen sehen Sie, woher Bestandskomponenten stammen und wohin sie gehen. Zoomen Sie mit dem Mausrad und verschieben Sie die Karte mit Mittelklick ziehen.",
        introduce: windowElementId(GIS2D_MAIN_WINDOW_ID),
        show: [],
        placement: "auto",
        interactions: [
          { on: { kind: "zoom", id: GIS2D_MAIN_WINDOW_ID }, label: "Zoomen (Mausrad)" },
          { on: { kind: "pan", id: GIS2D_MAIN_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
        ],
        ordered: false,
        logos: [],
        demonstrations: [
          { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: windowElementId(GIS2D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
        ],
      },
      {
        id: "panels",
        title: "Paneele",
        body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
        introduce: FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        show: [],
        placement: "auto",
        interactions: [{ on: { kind: "panel", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
        ordered: false,
        logos: [],
        demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandVerfolgenBrand

//#region 🎪️DemonstratorPanes
/** @emoji 🎪️ One live pane in the demonstrator's 3×2 grid — order here IS grid order (row-major: index
 * 0-2 top row, 3-5 bottom row). `variant` is the same playground alias `bun ./📜️script.ts dev <variant>`
 * already resolves (see `resolveFrameworkOsPlaygroundPlugin`), so `resolvePlaygroundBoot(variant)` finds
 * the right plugin/app without a separate mapping table. */
export type DemonstratorPaneSpec = {
  readonly id: string;
  readonly variant: string;
  readonly brand: ShellBrand;
  readonly label: string;
  readonly tagline: string;
  readonly icon: IconName;
};

export const DEMONSTRATOR_PANES: readonly DemonstratorPaneSpec[] = [
  { id: "generator", variant: "generator", brand: ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND, label: "Generator", tagline: "Parametrische Abläufe", icon: "workflow" },
  { id: "koordinator", variant: "koordinator", brand: ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND, label: "Koordinator", tagline: "Modelle koordinieren", icon: "cad-shape" },
  { id: "aggregator", variant: "aggregator", brand: ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND, label: "Aggregator", tagline: "Bestand zusammensetzen", icon: "puzzle" },
  { id: "aussuchen", variant: "aussuchen", brand: ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND, label: "Aussuchen", tagline: "Bestand sichten", icon: "library" },
  { id: "bearbeiten", variant: "bearbeiten", brand: ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND, label: "Bearbeiten", tagline: "Bauteile anpassen", icon: "hammer" },
  { id: "verfolgen", variant: "verfolgen", brand: ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND, label: "Verfolgen", tagline: "Herkunft verfolgen", icon: "gis2d" },
];
//#endregion 🎪️DemonstratorPanes
