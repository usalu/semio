"use strict";
// #region 🧲️Header
/** @emoji 🏷️ Entwerfen mit Bestand demonstrator brands — shared landing introduction plus per-app shell brands. */
// #endregion 🧲️Header
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.DEMONSTRATOR_PANES = exports.ENTWERFEN_MIT_BESTAND_STATIK_BRAND = exports.ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND = exports.ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND = exports.ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND = exports.ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND = exports.ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND = exports.ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND = exports.ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND = exports.ENTWERFEN_MIT_BESTAND_TUTORIAL = exports.ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION = exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG = exports.ENTWERFEN_MIT_BESTAND_BRAND_IDS = exports.DEMONSTRATOR_LOCALE = exports.demonstratorPaneRuntimeVariant = exports.DEMONSTRATOR_ASSETS_DIR = exports.DEMONSTRATOR_HOST = void 0;
exports.isEntwerfenMitBestandBrandId = isEntwerfenMitBestandBrandId;
exports.scheduleDemonstratorIdle = scheduleDemonstratorIdle;
exports.demonstratorPaneDescriptionParagraphs = demonstratorPaneDescriptionParagraphs;
exports.demonstratorPaneBootVariants = demonstratorPaneBootVariants;
var ____ts_1 = require("./\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "DEMONSTRATOR_HOST", { enumerable: true, get: function () { return ____ts_1.DEMONSTRATOR_HOST; } });
Object.defineProperty(exports, "DEMONSTRATOR_ASSETS_DIR", { enumerable: true, get: function () { return ____ts_1.DEMONSTRATOR_ASSETS_DIR; } });
Object.defineProperty(exports, "demonstratorPaneRuntimeVariant", { enumerable: true, get: function () { return ____ts_1.demonstratorPaneRuntimeVariant; } });
var framework_1 = require("@semio-tech/framework");
//#region 🏷️DemonstratorShared
/** @emoji 🇩🇪️ The whole demonstrator is German-locked (every brand's `locks.locale` below) — the
 * single source the landing page's boot-time `initUiLocaleSync` call reads, so it can never drift
 * from the per-app brands. */
exports.DEMONSTRATOR_LOCALE = "de";
/** @emoji 🏷️ Shell brand ids that receive Entwerfen-mit-Bestand partner chrome in the react renderer. */
exports.ENTWERFEN_MIT_BESTAND_BRAND_IDS = [
    "entwerfen-mit-bestand-aggregator",
    "entwerfen-mit-bestand-aussuchen",
    "entwerfen-mit-bestand-bearbeiten",
    "entwerfen-mit-bestand-energie",
    "entwerfen-mit-bestand-generator",
    "entwerfen-mit-bestand-koordinator",
    "entwerfen-mit-bestand-statik",
    "entwerfen-mit-bestand-verfolgen",
];
/** @emoji 🏷️ Whether a shell brand id is one of the demonstrator's Entwerfen-mit-Bestand panes. */
function isEntwerfenMitBestandBrandId(id) {
    return id !== undefined && exports.ENTWERFEN_MIT_BESTAND_BRAND_IDS.includes(id);
}
/** @emoji ✒️ Semio emblem shared across demonstrator brands and the landing page. */
exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG = "<svg viewBox=\"0 0 350 350\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-label=\"Entwerfen mit Bestand\"><path d=\"M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076\" fill=\"#001117\"/><path d=\"M76.25 271.933l35-35.808V118.75h-35z\" fill=\"#fa9500\" stroke=\"#f7f3e3\" stroke-width=\"2.5\" stroke-miterlimit=\"5\"/><g fill=\"#ff344f\" stroke=\"#f7f3e3\" stroke-width=\"2.5\" stroke-miterlimit=\"5\"><path d=\"M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z\"/></g><g fill=\"#34d1bf\" stroke=\"#f7f3e3\" stroke-width=\"2.5\" stroke-miterlimit=\"5\"><path d=\"M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z\"/></g></svg>";
var demonstratorLogoUrl = function (file) { return "/".concat(____ts_1.DEMONSTRATOR_ASSETS_DIR, "/\uD83E\uDEA7\uFE0Flogos/").concat(file); };
/** @emoji 🎓️ General demonstrator introduction shown on the landing page only (not inside app shells). */
exports.ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION = {
    title: "Willkommen bei Entwerfen mit Bestand",
    steps: [
        {
            id: "welcome",
            title: "Willkommen bei Entwerfen mit Bestand",
            body: "Der Demonstrator vereint acht Werkzeuge des Forschungsprojekts „Entwerfen mit Bestand“ der Leibniz Universität Hannover und der Universität der Künste Berlin.\n\nDas Projekt entwickelt eine offene Plattform, um neue Strukturen aus wiederverwendeten Baukomponenten zu entwerfen — mit vereinfachter Tragwerks- und Lebenszyklusanalyse, KI-Unterstützung entlang funktionaler und struktureller Abhängigkeiten.",
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
                { src: demonstratorLogoUrl("🏢️bmwsb/☀️logo.png"), darkSrc: demonstratorLogoUrl("🏢️bmwsb/🌙️logo-dark.png"), alt: "Bundesministerium für Wohnen, Stadtentwicklung und Bauwesen", href: "https://www.bmwsb.bund.de" },
                { src: demonstratorLogoUrl("🏛️bbsr/☀️logo.png"), darkSrc: demonstratorLogoUrl("🏛️bbsr/🌙️logo-dark.png"), alt: "Bundesinstitut für Bau-, Stadt- und Raumforschung", href: "https://www.bbsr.bund.de" },
                { src: demonstratorLogoUrl("🔮️zukunft-bau/☀️logo.png"), darkSrc: demonstratorLogoUrl("🔮️zukunft-bau/🌙️logo-dark.png"), alt: "Zukunft Bau", href: "https://www.zukunftbau.de/projekte/forschungsfoerderung/1008187-2506" },
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
var PUZZLE3D_MAIN_WINDOW_ID = "puzzle3d-main";
/** 🎥️ A camera-only keyframe helper — every pose in this tutorial keeps the same up vector and field of
 * view, so only `position`/`target` vary shot to shot. */
function heroCameraKeyframe(at, position, target) {
    return {
        at: at,
        windowId: PUZZLE3D_MAIN_WINDOW_ID,
        camera: { kind: "orbit", position: __spreadArray([], position, true), target: __spreadArray([], target, true), up: [0, 0, 1], fov: 45 },
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
exports.ENTWERFEN_MIT_BESTAND_TUTORIAL = {
    id: "aggregator-tour",
    title: "Aggregator-Tour",
    description: "Eine geführte, gesprochene Tour durch den Aggregator: Ansicht, Katalog, Transformieren, Verbindungspunkte und Füllen.",
    durationMs: 240000,
    chapters: [
        { id: "welcome", at: 0, title: "Willkommen" },
        { id: "prototype", at: 20000, title: "Früher Prototyp" },
        { id: "funding", at: 33000, title: "Förderhinweis" },
        { id: "viewport", at: 48000, title: "Die 3D-Ansicht" },
        { id: "panels", at: 72000, title: "Paneele" },
        { id: "catalogue-objects", at: 88000, title: "Baukomponenten aufklappen" },
        { id: "add-object", at: 100000, title: "Baukomponente hinzufügen" },
        { id: "transform-utility", at: 118000, title: "Baukomponenten transformieren" },
        { id: "verbindungspunkte", at: 134000, title: "Verbindungspunkte" },
        { id: "suggest-objects", at: 150000, title: "Baukomponenten vorschlagen" },
        { id: "fill-tool", at: 172000, title: "Füllen" },
        { id: "fill-distribution", at: 196000, title: "Anzahl und Verteilung" },
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
                durationMs: 20000,
                rate: 1,
                captions: [],
                text: "Der Aggregator ist der Demonstrator des Forschungsprojekts „Entwerfen mit Bestand“ der Leibniz Universität Hannover und der Universität der Künste Berlin. Das Projekt entwickelt eine offene Plattform, um neue Strukturen aus wiederverwendeten Baukomponenten zu entwerfen — mit vereinfachter Tragwerks- und Lebenszyklusanalyse, KI-Unterstützung entlang funktionaler und struktureller Abhängigkeiten.",
            },
            {
                id: "prototype",
                at: 20000,
                durationMs: 13000,
                rate: 1,
                captions: [],
                text: "Dieser Demonstrator befindet sich in aktiver Entwicklung. Viele Funktionen sind noch unvollständig oder nur als Platzhalter vorhanden — sie zeigen die Richtung des Projekts, nicht seinen finalen Stand.",
            },
            {
                id: "funding",
                at: 33000,
                durationMs: 15000,
                rate: 1,
                captions: [],
                text: "Dieses Projekt wird gefördert vom Bundesinstitut für Bau-, Stadt- und Raumforschung im Auftrag des Bundesministeriums für Wohnen, Stadtentwicklung und Bauwesen aus Mitteln der Zukunft Bau Forschungsförderung.",
            },
            {
                id: "viewport",
                at: 48000,
                durationMs: 24000,
                rate: 1,
                captions: [],
                text: "Hier entsteht Ihr Entwurf aus Bestandskomponenten — zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
            },
            {
                id: "panels",
                at: 72000,
                durationMs: 16000,
                rate: 1,
                captions: [],
                text: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Einstellungen. Wir öffnen jetzt den Katalog-Reiter.",
            },
            {
                id: "catalogue-objects",
                at: 88000,
                durationMs: 12000,
                rate: 1,
                captions: [],
                text: "»Baukomponenten« im Katalog klappt die verfügbaren Arten auf.",
            },
            {
                id: "add-object",
                at: 100000,
                durationMs: 18000,
                rate: 1,
                captions: [],
                text: "Per Drag-and-Drop ziehen wir die erste Baukomponente aus dem Katalog in die 3D-Ansicht.",
            },
            {
                id: "transform-utility",
                at: 118000,
                durationMs: 16000,
                rate: 1,
                captions: [],
                text: "Das Transformieren-Hilfsmittel verschiebt und dreht Baukomponenten.",
            },
            {
                id: "verbindungspunkte",
                at: 134000,
                durationMs: 16000,
                rate: 1,
                captions: [],
                text: "»Verbindungspunkte anzeigen« auf »Immer« gestellt macht die Anschlüsse aller Baukomponenten sichtbar.",
            },
            {
                id: "suggest-objects",
                at: 150000,
                durationMs: 22000,
                rate: 1,
                captions: [],
                text: "Ein Verbindungspunkt per Linksklick gewählt und per Rechtsklick das Aktionsmenü geöffnet — »Baukomponenten vorschlagen« zeigt passende Anschlüsse, ein Linksklick auf einen Vorschlag platziert ihn.",
            },
            {
                id: "fill-tool",
                at: 172000,
                durationMs: 24000,
                rate: 1,
                captions: [],
                text: "Das Werkzeug »Füllen« füllt den Entwurf automatisch mit Baukomponenten.",
            },
            {
                id: "fill-distribution",
                at: 196000,
                durationMs: 44000,
                rate: 1,
                captions: [],
                text: "Die Anzahl stellen wir am Schieberegler ein, die Verteilung justieren wir per Ziehen an den Reglern — sie gewichtet, welche Baukomponenten und Verbindungspunkte beim Füllen bevorzugt werden. Damit endet unsere Tour durch den Aggregator.",
            },
        ],
        video: [],
        events: [
            { at: 110000, kind: { kind: "action", action: "addObjectKind" } },
            { at: 141000, kind: { kind: "action", action: "setVortexShow", args: { show: "always" } } },
            { at: 165000, kind: { kind: "action", action: "acceptSuggestion" } },
            { at: 181000, kind: { kind: "action", action: "setFillCount", args: { count: 40 } } },
        ],
        ui: [
            { at: 76000, sample: { kind: "delta", changes: [{ kind: "panelTab", group: "top-left", tabId: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }] } },
            { at: 93000, sample: { kind: "delta", changes: [{ kind: "treeExpansion", id: "puzzle3d-play-kinds.objects", expanded: true }] } },
            { at: 125000, sample: { kind: "delta", changes: [{ kind: "activeUtility", windowId: PUZZLE3D_MAIN_WINDOW_ID, utilityId: "transform" }] } },
            { at: 180000, sample: { kind: "delta", changes: [{ kind: "activeTool", id: "fill" }] } },
        ],
        document: [],
        camera: [
            // 🎬️ Welcome: a slow establishing push toward the seeded "Abbau Aufbau" prototype.
            heroCameraKeyframe(10000, [34, -32, 22], [7, 0, 3]),
            heroCameraKeyframe(20000, [30, -30, 20], [7, 0, 3]),
            // 🎥️ Viewport chapter: the camera itself performs the zoom/pan/orbit gestures being narrated,
            // timed to match the pointer gestures in `tracks.gestures` below.
            heroCameraKeyframe(50000, [30, -30, 20], [7, 0, 3]),
            heroCameraKeyframe(56000, [22, -22, 15], [7, 0, 3]),
            heroCameraKeyframe(62000, [24, -24, 15], [9, -2, 3.5]),
            heroCameraKeyframe(72000, [10, -30, 18], [9, -2, 3.5]),
            // 🧩️ Settle back onto the seeded object for the catalogue/add-object/transform chapters.
            heroCameraKeyframe(100000, [18, -20, 12], [7, 0, 3]),
            heroCameraKeyframe(118000, [14, -16, 10], [7, 0, 3]),
            // 🌐️ Pull back a touch for the fill/distribution chapters to frame the growing assembly.
            heroCameraKeyframe(196000, [26, -28, 18], [7, 0, 3]),
            // 👋️ Final pull-back — bookends the opening establishing shot.
            heroCameraKeyframe(230000, [40, -40, 26], [7, 0, 3]),
        ],
        gestures: [
            { at: 50000, durationMs: 1000, gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
            {
                at: 56000,
                durationMs: 3000,
                gesture: {
                    kind: "drag",
                    from: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 },
                    to: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), x: 0.65, y: 0.4 },
                    button: "middle",
                    modifiers: [],
                },
                cursor: "move",
            },
            {
                at: 63000,
                durationMs: 7000,
                gesture: {
                    kind: "orbit",
                    from: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), x: 0.35, y: 0.5 },
                    to: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), x: 0.65, y: 0.5 },
                    button: "right",
                    modifiers: ["alt"],
                },
            },
            { at: 74000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } },
            { at: 92000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-kinds.objects" } } },
            {
                at: 104000,
                durationMs: 6000,
                gesture: {
                    kind: "drag",
                    from: { kind: "element", id: (0, framework_1.panelTabFirstDraggableElementId)(framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID) },
                    to: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), x: 0.5, y: 0.55 },
                    button: "left",
                    modifiers: [],
                },
            },
            { at: 124000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "transform" } } },
            { at: 140000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "puzzle3d-play-vortex-show" } } },
            { at: 155000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "entity", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), domain: "vortex", entity: "*" } } },
            { at: 160000, durationMs: 500, gesture: { kind: "rightClick", at: { kind: "entity", id: (0, framework_1.windowElementId)(PUZZLE3D_MAIN_WINDOW_ID), domain: "vortex", entity: "*" } } },
            { at: 178000, durationMs: 500, gesture: { kind: "leftClick", at: { kind: "element", id: "tool.fill" } } },
        ],
    },
};
//#endregion 🎬️EntwerfenMitBestandTutorial
//#region 🏷️EntwerfenMitBestandAggregatorBrand
/** 🏷️ Aggregator (puzzle3d): reuse terminology, app-specific introduction, recorded tutorial, Abbau Aufbau default example. */
exports.ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND = {
    id: "entwerfen-mit-bestand-aggregator",
    windowTitle: "Entwerfen mit Bestand · Aggregator",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "concrete-forest" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    tutorials: [exports.ENTWERFEN_MIT_BESTAND_TUTORIAL],
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen beim Aggregator",
        steps: [
            {
                id: "viewport",
                title: "Die 3D-Ansicht",
                body: "Hier entsteht Ihr Entwurf aus Bestandskomponenten — zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
                introduce: (0, framework_1.windowElementId)("puzzle3d-main"),
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
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)("puzzle3d-main"), x: 0.5, y: 0.5 }, deltaY: -100 } },
                    {
                        gesture: {
                            kind: "drag",
                            from: { kind: "windowNormalized", id: (0, framework_1.windowElementId)("puzzle3d-main"), x: 0.5, y: 0.5 },
                            to: { kind: "windowNormalized", id: (0, framework_1.windowElementId)("puzzle3d-main"), x: 0.65, y: 0.4 },
                            button: "middle",
                        },
                        cursor: "move",
                    },
                    { gesture: { kind: "orbit", from: { kind: "windowNormalized", id: (0, framework_1.windowElementId)("puzzle3d-main"), x: 0.35, y: 0.5 }, to: { kind: "windowNormalized", id: (0, framework_1.windowElementId)("puzzle3d-main"), x: 0.65, y: 0.5 } } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Einstellungen. Klicken Sie jetzt mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
            {
                id: "catalogue-objects",
                title: "Baukomponenten aufklappen",
                body: "Klicken Sie mit der linken Maustaste auf »Baukomponenten« im Katalog, um die verfügbaren Arten aufzuklappen.",
                introduce: "puzzle3d-play-kinds.objects",
                show: [(0, framework_1.panelTabElementId)(framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID)],
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
                introduce: (0, framework_1.panelTabFirstDraggableElementId)(framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID),
                show: [(0, framework_1.panelTabElementId)(framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID), (0, framework_1.windowElementId)("puzzle3d-main")],
                placement: "right",
                interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: "Mit linker Maustaste in die Ansicht ziehen" }],
                ordered: false,
                logos: [],
                demonstrations: [
                    {
                        gesture: {
                            kind: "drag",
                            from: { kind: "element", id: (0, framework_1.panelTabFirstDraggableElementId)(framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID) },
                            to: { kind: "windowNormalized", id: (0, framework_1.windowElementId)("puzzle3d-main"), x: 0.5, y: 0.55 },
                        },
                    },
                ],
            },
            {
                id: "transform-utility",
                title: "Baukomponenten transformieren",
                body: "Klicken Sie mit der linken Maustaste auf das Transformieren-Hilfsmittel, um Baukomponenten zu verschieben und zu drehen.",
                introduce: "transform",
                show: [(0, framework_1.windowElementId)("puzzle3d-main")],
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
                show: [(0, framework_1.windowElementId)("puzzle3d-main")],
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
                introduce: (0, framework_1.windowElementId)("puzzle3d-main"),
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "action", id: "acceptSuggestion" }, label: "Vorschlag per Linksklick wählen" }],
                ordered: false,
                logos: [],
                demonstrations: [
                    { gesture: { kind: "leftClick", at: { kind: "entity", id: (0, framework_1.windowElementId)("puzzle3d-main"), domain: "vortex", entity: "*" } } },
                    { gesture: { kind: "rightClick", at: { kind: "entity", id: (0, framework_1.windowElementId)("puzzle3d-main"), domain: "vortex", entity: "*" } } },
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
                show: ["puzzle3d-fill-count", (0, framework_1.panelTabElementId)("tool.fill")],
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
var PROCEDURAL_MAIN_WINDOW_ID = "procedural-main";
var CAD_SHAPE_WINDOW_ID = "cad-play-shape";
//#region 🏷️EntwerfenMitBestandGeneratorBrand
/** 🏷️ Generator (generation3d): parametric flow editor for reuse-oriented component generation. */
exports.ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND = {
    id: "entwerfen-mit-bestand-generator",
    windowTitle: "Entwerfen mit Bestand · Generator",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "hexagonal-mushroom-column" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen beim Generator",
        steps: [
            {
                id: "viewport",
                title: "Der Ablauf-Editor",
                body: "Im Generator entwerfen Sie parametrische Abläufe für Baukomponenten. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
                introduce: (0, framework_1.windowElementId)(PROCEDURAL_MAIN_WINDOW_ID),
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
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PROCEDURAL_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
        ],
    },
};
//#endregion 🏷️EntwerfenMitBestandGeneratorBrand
//#region 🏷️EntwerfenMitBestandKoordinatorBrand
/** 🏷️ Koordinator (cad): multi-model coordination for shape, building, energy, and structure views. */
exports.ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND = {
    id: "entwerfen-mit-bestand-koordinator",
    windowTitle: "Entwerfen mit Bestand · Koordinator",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "hexagonal-cut-concrete-forest-left" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen beim Koordinator",
        steps: [
            {
                id: "viewport",
                title: "Die Modellansichten",
                body: "Der Koordinator verbindet Form-, Gebäude-, Energie- und Tragwerksmodelle. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
                introduce: (0, framework_1.windowElementId)(CAD_SHAPE_WINDOW_ID),
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
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(CAD_SHAPE_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Über die Reiter öffnen Sie Katalog, Dokument und weitere Paneele. Klicken Sie auf den Katalog-Reiter, um verfügbare Bausteine zu durchsuchen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
        ],
    },
};
//#endregion 🏷️EntwerfenMitBestandKoordinatorBrand
var SOURCING_POOL_WINDOW_ID = "sourcing-pool";
var PROCESS_WORKPIECE_WINDOW_ID = "process-workpiece";
var GIS2D_MAIN_WINDOW_ID = "gis2d-main";
//#region 🏷️EntwerfenMitBestandAussuchenBrand
/** 🏷️ Aussuchen (sourcing): curating reclaimed building components from available stock. */
exports.ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND = {
    id: "entwerfen-mit-bestand-aussuchen",
    windowTitle: "Entwerfen mit Bestand · Aussuchen",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "demo-stock" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen bei Aussuchen",
        steps: [
            {
                id: "viewport",
                title: "Der Bestandspool",
                body: "Im Aussuchen sichten Sie verfügbare Bestandskomponenten und stellen daraus eine Kuratierung zusammen. Der Pool listet alle gefundenen Komponenten mit Verfügbarkeit und Typologie.",
                introduce: (0, framework_1.windowElementId)(SOURCING_POOL_WINDOW_ID),
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
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
        ],
    },
};
//#endregion 🏷️EntwerfenMitBestandAussuchenBrand
//#region 🏷️EntwerfenMitBestandBearbeitenBrand
/** 🏷️ Bearbeiten (process3d): machining steps that adapt a reclaimed component to its new use. */
exports.ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND = {
    id: "entwerfen-mit-bestand-bearbeiten",
    windowTitle: "Entwerfen mit Bestand · Bearbeiten",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "timber-beam-joinery" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen bei Bearbeiten",
        steps: [
            {
                id: "viewport",
                title: "Das Werkstück",
                body: "Im Bearbeiten legen Sie die Bearbeitungsschritte fest, mit denen eine Bestandskomponente für ihre neue Aufgabe angepasst wird. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
                introduce: (0, framework_1.windowElementId)(PROCESS_WORKPIECE_WINDOW_ID),
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
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(PROCESS_WORKPIECE_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
        ],
    },
};
//#endregion 🏷️EntwerfenMitBestandBearbeitenBrand
//#region 🏷️EntwerfenMitBestandVerfolgenBrand
/** 🏷️ Verfolgen (gis2d): tracking where reclaimed components come from and where they go. */
exports.ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND = {
    id: "entwerfen-mit-bestand-verfolgen",
    windowTitle: "Entwerfen mit Bestand · Verfolgen",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "demo" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen bei Verfolgen",
        steps: [
            {
                id: "viewport",
                title: "Die Karte",
                body: "Im Verfolgen sehen Sie, woher Bestandskomponenten stammen und wohin sie gehen. Zoomen Sie mit dem Mausrad und verschieben Sie die Karte mit Mittelklick ziehen.",
                introduce: (0, framework_1.windowElementId)(GIS2D_MAIN_WINDOW_ID),
                show: [],
                placement: "auto",
                interactions: [
                    { on: { kind: "zoom", id: GIS2D_MAIN_WINDOW_ID }, label: "Zoomen (Mausrad)" },
                    { on: { kind: "pan", id: GIS2D_MAIN_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
                ],
                ordered: false,
                logos: [],
                demonstrations: [
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(GIS2D_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
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
var ENERGY_MODEL_MAIN_WINDOW_ID = "energy.model.3d";
/** 🪟️ The Energy editor's simulation window — `energy.simulation`, the run transport and settings beside
 * the 3D model (`…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`'s `WINDOW_KIND_ID`). */
var ENERGY_SIMULATION_WINDOW_ID = "energy.simulation";
/** 🪟️ The FEM 3D editor's main window kind — `s.fem.fem3d@1/*#editor`'s `fem3d-model` window (see
 * `✏️s/🔌️plugins/🏗️fem/🔣️.json`), a `world-3d` surface, so the Statik introduction gates on the same
 * zoom/pan/orbit gestures as the other 3D brands. */
var FEM3D_MODEL_WINDOW_ID = "fem3d-model";
//#region 🏷️EntwerfenMitBestandEnergieBrand
/** 🏷️ Energie (energy): thermal simulation of designs made from reused building components. */
exports.ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND = {
    id: "entwerfen-mit-bestand-energie",
    windowTitle: "Entwerfen mit Bestand · Energie",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "bestest-600" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen bei Energie",
        steps: [
            {
                id: "viewport",
                title: "Das Energiemodell",
                body: "Im Energie-Werkzeug simulieren Sie das thermische Verhalten von Entwürfen aus Bestandskomponenten — Zonen, Hüllflächen und Randbedingungen. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
                introduce: (0, framework_1.windowElementId)(ENERGY_MODEL_MAIN_WINDOW_ID),
                show: [],
                placement: "auto",
                interactions: [
                    { on: { kind: "zoom", id: ENERGY_MODEL_MAIN_WINDOW_ID }, label: "Zoomen (Mausrad)" },
                    { on: { kind: "pan", id: ENERGY_MODEL_MAIN_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
                    { on: { kind: "orbit", id: ENERGY_MODEL_MAIN_WINDOW_ID }, label: "Orbitieren (Alt + Rechtsklick ziehen)" },
                ],
                ordered: false,
                logos: [],
                demonstrations: [
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(ENERGY_MODEL_MAIN_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Neben dem Modell stehen Struktur, Zonen und die Energiesimulation. Über die Reiter in der Leiste öffnen und schließen Sie zusätzlich Paneele — zum Beispiel Katalog, Dokument oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [(0, framework_1.windowElementId)(ENERGY_SIMULATION_WINDOW_ID)],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
        ],
    },
};
//#endregion 🏷️EntwerfenMitBestandEnergieBrand
//#region 🏷️EntwerfenMitBestandStatikBrand
/** 🏷️ Statik (fem3d): structural analysis of the Tragwerk assembled from reused building components. */
exports.ENTWERFEN_MIT_BESTAND_STATIK_BRAND = {
    id: "entwerfen-mit-bestand-statik",
    windowTitle: "Entwerfen mit Bestand · Statik",
    logoSvg: exports.ENTWERFEN_MIT_BESTAND_LOGO_SVG,
    locks: { locale: exports.DEMONSTRATOR_LOCALE, terminology: "reuse", themeId: "semio" },
    defaults: { exampleId: "concrete-forest" },
    ephemeral: true,
    replayIntroductionOnLoad: true,
    assetsDir: ____ts_1.DEMONSTRATOR_ASSETS_DIR,
    introduction: {
        title: "Willkommen bei Statik",
        steps: [
            {
                id: "viewport",
                title: "Das Tragwerksmodell",
                body: "In der Statik berechnen Sie das Tragwerk aus wiederverwendeten Baukomponenten — Knoten, Stäbe, Auflager und Lasten. Zoomen Sie mit dem Mausrad, verschieben Sie mit Mittelklick ziehen und orbitieren Sie mit Alt + Rechtsklick ziehen.",
                introduce: (0, framework_1.windowElementId)(FEM3D_MODEL_WINDOW_ID),
                show: [],
                placement: "auto",
                interactions: [
                    { on: { kind: "zoom", id: FEM3D_MODEL_WINDOW_ID }, label: "Zoomen (Mausrad)" },
                    { on: { kind: "pan", id: FEM3D_MODEL_WINDOW_ID }, label: "Verschieben (Mittelklick ziehen)" },
                    { on: { kind: "orbit", id: FEM3D_MODEL_WINDOW_ID }, label: "Orbitieren (Alt + Rechtsklick ziehen)" },
                ],
                ordered: false,
                logos: [],
                demonstrations: [
                    { gesture: { kind: "scroll", at: { kind: "windowNormalized", id: (0, framework_1.windowElementId)(FEM3D_MODEL_WINDOW_ID), x: 0.5, y: 0.5 }, deltaY: -100 } },
                ],
            },
            {
                id: "panels",
                title: "Paneele",
                body: "Über die Reiter in der Leiste öffnen und schließen Sie Paneele — zum Beispiel Katalog, Ergebnisse oder Inspektion. Klicken Sie mit der linken Maustaste auf den Katalog-Reiter, um das Katalog-Paneel zu öffnen.",
                introduce: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                show: [],
                placement: "auto",
                interactions: [{ on: { kind: "panel", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID }, label: "Katalog-Reiter anklicken" }],
                ordered: false,
                logos: [],
                demonstrations: [{ gesture: { kind: "leftClick", at: { kind: "element", id: framework_1.FRAMEWORK_PANEL_TAB_CATALOGUE_ID } } }],
            },
        ],
    },
};
/** @emoji 🐢️ Enforces a minimum delay before yielding the next warm boot to the browser's idle queue. */
function scheduleDemonstratorIdle(callback, delayMs, scheduler) {
    var idleHandle = null;
    var timeoutHandle = scheduler.setTimeout(function () {
        if (scheduler.requestIdleCallback)
            idleHandle = scheduler.requestIdleCallback(callback, { timeout: 1000 });
        else
            callback();
    }, delayMs);
    return function () {
        var _a;
        scheduler.clearTimeout(timeoutHandle);
        if (idleHandle != null)
            (_a = scheduler.cancelIdleCallback) === null || _a === void 0 ? void 0 : _a.call(scheduler, idleHandle);
    };
}
/** @emoji 📖 Splits pane overview copy into introduction-style body paragraphs. */
function demonstratorPaneDescriptionParagraphs(description) {
    return description.split(/\n\n+/).map(function (paragraph) { return paragraph.trim(); }).filter(function (paragraph) { return paragraph.length > 0; });
}
/** @emoji 🧭️ Separates the module-owning runtime variant from the branded pane's manifest row.
 * Generator executes the standalone procedural module, but its branded `generator` row carries the
 * canonical app id that the module manifest actually declares. */
function demonstratorPaneBootVariants(variant) {
    return { runtime: (0, ____ts_1.demonstratorPaneRuntimeVariant)(variant), manifest: variant };
}
exports.DEMONSTRATOR_PANES = [
    {
        id: "generator",
        variant: "generator",
        brand: exports.ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND,
        label: "Generator",
        tagline: "Parametrische Abläufe",
        description: "Im Generator entwerfen Sie parametrische Abläufe für wiederverwendete Baukomponenten — von Regeln und Parametern bis zu fertigen Teilen.\n\nÖffnen Sie den Demonstrator, um den Ablauf-Editor im Vollbild zu erkunden und Beispiele zu laden.",
        icon: "workflow",
    },
    {
        id: "koordinator",
        variant: "koordinator",
        brand: exports.ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND,
        label: "Koordinator",
        tagline: "Modelle koordinieren",
        description: "Der Koordinator verbindet Form-, Gebäude-, Energie- und Tragwerksmodelle in gemeinsamen Ansichten.\n\nWechseln Sie in den Vollbildmodus, um mehrere CAD-Fenster nebeneinander zu vergleichen und zusammenzuführen.",
        icon: "cad-shape",
    },
    {
        id: "aggregator",
        variant: "aggregator",
        brand: exports.ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND,
        label: "Aggregator",
        tagline: "Bestand zusammensetzen",
        description: "Im Aggregator setzen Sie Entwürfe aus Bestandskomponenten im dreidimensionalen Raum zusammen.\n\nÖffnen Sie den Demonstrator, um Bauteile aus dem Katalog zu platzieren und den Aufbau zu prüfen.",
        icon: "puzzle",
    },
    {
        id: "energie",
        variant: "energy",
        brand: exports.ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND,
        label: "Energie",
        tagline: "Energiebilanz simulieren",
        description: "Energie modelliert Zonen, Hüllflächen und Randbedingungen und simuliert das thermische Verhalten Ihres Entwurfs aus Bestand.\n\nIm Vollbildmodus betrachten Sie Modell und Simulationslauf nebeneinander.",
        icon: "sun",
    },
    {
        id: "aussuchen",
        variant: "aussuchen",
        brand: exports.ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND,
        label: "Aussuchen",
        tagline: "Bestand sichten",
        description: "Beim Aussuchen durchsuchen Sie verfügbaren Bestand und stellen daraus eine Kuratierung für Ihr Projekt zusammen.\n\nÖffnen Sie den Pool im Vollbild, filtern Sie nach Modul und Typologie und legen Sie Komponenten in die Auswahl.",
        icon: "library",
    },
    {
        id: "bearbeiten",
        variant: "bearbeiten",
        brand: exports.ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND,
        label: "Bearbeiten",
        tagline: "Bauteile anpassen",
        description: "Bearbeiten plant die Schritte, mit denen eine Bestandskomponente für ihre neue Aufgabe angepasst wird.\n\nWechseln Sie in den Demonstrator, um Bearbeitungsketten am Werkstück zu definieren und zu visualisieren.",
        icon: "hammer",
    },
    {
        id: "verfolgen",
        variant: "verfolgen",
        brand: exports.ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND,
        label: "Verfolgen",
        tagline: "Herkunft verfolgen",
        description: "Verfolgen zeigt räumlich, woher Bestandskomponenten stammen und wohin sie in späteren Nutzungsphasen gehen.\n\nÖffnen Sie die Karte im Vollbild, zoomen Sie auf Standorte und erkunden Sie die Zusammenhänge.",
        icon: "gis2d",
    },
    {
        id: "statik",
        variant: "fem3d",
        brand: exports.ENTWERFEN_MIT_BESTAND_STATIK_BRAND,
        label: "Statik",
        tagline: "Tragwerk berechnen",
        description: "Statik berechnet Stabwerke aus wiederverwendeten Bauteilen — mit Knoten, Auflagern, Lasten und Ergebnisdarstellung.\n\nIm Vollbildmodus bearbeiten Sie das FEM-Modell und prüfen Sie die strukturelle Antwort.",
        icon: "fem-app",
    },
];
//#endregion 🎪️DemonstratorPanes
if (import.meta.vitest) {
    var registerTests1 = (await Promise.resolve().then(function () { return require("./🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts"); })).registerTests1;
    await registerTests1(import.meta.vitest, { demonstratorPaneBootVariants: demonstratorPaneBootVariants, scheduleDemonstratorIdle: scheduleDemonstratorIdle }, { directory: import.meta.dir, url: import.meta.url });
}
