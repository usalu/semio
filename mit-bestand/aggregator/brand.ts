// #region 🧲Header
/** @emoji 🏷️ "Entwerfen mit Bestand Aggregator" brand — German, reuse-terminology, theme-locked standalone puzzle3d. */
// #endregion 🧲Header

import {
  panelTabElementId,
  panelTabFirstDraggableElementId,
  windowElementId,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  type ShellBrand,
  type TutorialDefinition,
} from "../../framework/js/index.ts";

//#region 🎬EntwerfenMitBestandTutorial
/** 🪟 The Aggregator's one 3D window instance — matches `puzzle/plugin/rs/lib.rs`'s `puzzle3d-main` window
 * kind (unsplit base instance, so the instance id equals the window kind id). Camera keyframes below key
 * off this raw instance id, never the `windowElementId(...)`-transformed element id (that's for anchoring
 * chrome, not for `TutorialCameraKeyframe.windowId`/`ViewWindowInstance.id`). */
const PUZZLE3D_MAIN_WINDOW_ID = "puzzle3d-main";

/** 🎥 A camera-only keyframe helper — every pose in this tutorial keeps the same up vector and field of
 * view, so only `position`/`target` vary shot to shot. */
function heroCameraKeyframe(at: number, position: readonly [number, number, number], target: readonly [number, number, number]): TutorialDefinition["tracks"]["camera"][number] {
  return {
    at,
    windowId: PUZZLE3D_MAIN_WINDOW_ID,
    camera: { kind: "orbit", position, target, up: [0, 0, 1], fov: 45 },
    easing: "easeInOut",
  };
}

/** @emoji 🎬 The Aggregator's first recorded-tutorial demo — a ~4-minute, 12-chapter walkthrough mirroring
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
      // 🎬 Welcome: a slow establishing push toward the seeded "Abbau Aufbau" prototype.
      heroCameraKeyframe(10_000, [34, -32, 22], [7, 0, 3]),
      heroCameraKeyframe(20_000, [30, -30, 20], [7, 0, 3]),
      // 🎥 Viewport chapter: the camera itself performs the zoom/pan/orbit gestures being narrated,
      // timed to match the pointer gestures in `tracks.gestures` below.
      heroCameraKeyframe(50_000, [30, -30, 20], [7, 0, 3]),
      heroCameraKeyframe(56_000, [22, -22, 15], [7, 0, 3]),
      heroCameraKeyframe(62_000, [24, -24, 15], [9, -2, 3.5]),
      heroCameraKeyframe(72_000, [10, -30, 18], [9, -2, 3.5]),
      // 🧩 Settle back onto the seeded object for the catalogue/add-object/transform chapters.
      heroCameraKeyframe(100_000, [18, -20, 12], [7, 0, 3]),
      heroCameraKeyframe(118_000, [14, -16, 10], [7, 0, 3]),
      // 🌐 Pull back a touch for the fill/distribution chapters to frame the growing assembly.
      heroCameraKeyframe(196_000, [26, -28, 18], [7, 0, 3]),
      // 👋 Final pull-back — bookends the opening establishing shot.
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
//#endregion 🎬EntwerfenMitBestandTutorial

//#region 🏷️EntwerfenMitBestandBrand
/** @emoji ✒️ Semio emblem. */
const ENTWERFEN_MIT_BESTAND_LOGO_SVG = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Entwerfen mit Bestand"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

/** 🏷️ The Aggregator ships puzzle3d with locked German locale, locked reuse terminology (window "Aggregator", document "Entwerfen mit Bestand", example "Abbau Aufbau"), locked semio theme, switchable appearance, a brand-owned German introduction, a brand-owned recorded tutorial (`ENTWERFEN_MIT_BESTAND_TUTORIAL`), and Abbau Aufbau (`concrete-forest`) seeded as the default-but-switchable example. Ephemeral: nothing survives a window refresh — dock, panes, chrome prefs, and the introduction all reset to brand defaults. Introduced/shown element ids reference `puzzle/plugin/rs/lib.rs`'s puzzle3d app (`puzzle3d-main`, `transform`, `addObjectKind`, `puzzle3d-play-vortex-show`, `tool.fill`, `puzzle3d-play-distribution`, `puzzle3d-play-kinds.objects`, `setVortexShow`, `acceptSuggestion`) and `framework/core/js`'s `FRAMEWORK_PANEL_TAB_CATALOGUE_ID`. Tour order: viewport → open Katalog panel → expand Baukomponenten → drag-and-drop → transform → Verbindungspunkte → Vorschlag wählen → Füllen → Verteilung. */
export const ENTWERFEN_MIT_BESTAND_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand",
  windowTitle: "Entwerfen mit Bestand · Aggregator",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: "de", terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  tutorials: [ENTWERFEN_MIT_BESTAND_TUTORIAL],
  assetsDir: "mit-bestand/aggregator/asset",
  distDir: "mit-bestand/aggregator/dist",
  cnameHost: "demonstrator.entwerfen.mit-bestand.de",
  introduction: {
    title: "Willkommen beim Aggregator",
    steps: [
      {
        id: "welcome",
        title: "Willkommen bei Entwerfen mit Bestand",
        body: "Der Aggregator ist der Demonstrator des Forschungsprojekts „Entwerfen mit Bestand“ der Leibniz Universität Hannover und der Universität der Künste Berlin.\n\nDas Projekt entwickelt eine offene Plattform, um neue Strukturen aus wiederverwendeten Baukomponenten zu entwerfen — mit vereinfachter Tragwerks- und Lebenszyklusanalyse, KI-Unterstützung entlang funktionaler und struktureller Abhängigkeiten.",
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
          { src: "/mit-bestand/aggregator/asset/logo/bmwsb.png", darkSrc: "/mit-bestand/aggregator/asset/logo/bmwsb-dark.png", alt: "Bundesministerium für Wohnen, Stadtentwicklung und Bauwesen", href: "https://www.bmwsb.bund.de" },
          { src: "/mit-bestand/aggregator/asset/logo/bbsr.png", darkSrc: "/mit-bestand/aggregator/asset/logo/bbsr-dark.png", alt: "Bundesinstitut für Bau-, Stadt- und Raumforschung", href: "https://www.bbsr.bund.de" },
          { src: "/mit-bestand/aggregator/asset/logo/zukunft-bau.png", darkSrc: "/mit-bestand/aggregator/asset/logo/zukunft-bau-dark.png", alt: "Zukunft Bau", href: "https://www.zukunftbau.de/projekte/forschungsfoerderung/1008187-2506" },
        ],
        demonstrations: [],
      },
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
//#endregion 🏷️EntwerfenMitBestandBrand
