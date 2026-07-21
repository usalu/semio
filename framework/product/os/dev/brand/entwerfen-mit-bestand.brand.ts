// #region 🧲Header
/** @emoji 🏷️ "Entwerfen mit Bestand Aggregator" brand — German, reuse-terminology, theme-locked standalone puzzle3d. */
// #endregion 🧲Header

import type { ShellBrand } from "../../../../core/js/index.ts";

//#region 🏷️EntwerfenMitBestandBrand
/** @emoji ✒️ Semio emblem. */
const ENTWERFEN_MIT_BESTAND_LOGO_SVG = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Entwerfen mit Bestand"><path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117"/><path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"/><g fill="#ff344f" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z"/></g><g fill="#34d1bf" stroke="#f7f3e3" stroke-width="2.5" stroke-miterlimit="5"><path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z"/></g></svg>`;

/** 🏷️ The Aggregator ships puzzle3d with locked German locale, locked reuse terminology (window "Aggregator", document "Entwerfen mit Bestand", example "Abbau Aufbau"), locked semio theme, switchable appearance, a brand-owned German introduction, and Abbau Aufbau (`concrete-forest`) seeded as the default-but-switchable example. Ephemeral: nothing survives a window refresh — dock, panes, chrome prefs, and the introduction all reset to brand defaults. Anchors reference `puzzle/plugin/rs/lib.rs`'s puzzle3d app (`puzzle3d-main`, `move`, `addObjectKind`) and `framework/core/js`'s `FRAMEWORK_PANEL_TAB_CATALOGUE_ID`. */
export const ENTWERFEN_MIT_BESTAND_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand",
  windowTitle: "Entwerfen mit Bestand · Aggregator",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: "de", terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  ephemeral: true,
  replayIntroductionOnLoad: true,
  introduction: {
    title: "Willkommen beim Aggregator",
    steps: [
      {
        id: "welcome",
        title: "Willkommen bei Entwerfen mit Bestand",
        body: "Der Aggregator ist der Demonstrator des Forschungsprojekts „Entwerfen mit Bestand“ der Leibniz Universität Hannover und der Universität der Künste Berlin.\n\nDas Projekt entwickelt eine offene Plattform, um neue Strukturen aus wiederverwendeten Baukomponenten zu entwerfen — mit KI-Unterstützung und Leistungsbewertung entlang komplexer funktionaler und struktureller Abhängigkeiten.",
        anchor: { kind: "screen" },
        emphasis: "none",
        placement: "center",
        advance: { kind: "next" },
        logos: [],
      },
      {
        id: "prototype",
        title: "Früher Prototyp",
        body: "Dieser Demonstrator befindet sich in aktiver Entwicklung. Viele Funktionen sind noch unvollständig oder nur als Platzhalter vorhanden — sie zeigen die Richtung des Projekts, nicht seinen finalen Stand.",
        anchor: { kind: "screen" },
        emphasis: "none",
        placement: "center",
        advance: { kind: "next" },
        logos: [],
      },
      {
        id: "funding",
        title: "Förderhinweis",
        body: "Dieses Projekt wurde gefördert vom Bundesinstitut für Bau-, Stadt- und Raumforschung im Auftrag des Bundesministeriums für Wohnen, Stadtentwicklung und Bauwesen aus Mitteln der Zukunft Bau Forschungsförderung.",
        anchor: { kind: "screen" },
        emphasis: "none",
        placement: "center",
        advance: { kind: "next" },
        logos: [
          { src: "/asset/logo/bmwsb.png", darkSrc: "/asset/logo/bmwsb-dark.png", alt: "Bundesministerium für Wohnen, Stadtentwicklung und Bauwesen", href: "https://www.bmwsb.bund.de" },
          { src: "/asset/logo/bbsr.png", darkSrc: "/asset/logo/bbsr-dark.png", alt: "Bundesinstitut für Bau-, Stadt- und Raumforschung", href: "https://www.bbsr.bund.de" },
          { src: "/asset/logo/zukunft-bau.png", darkSrc: "/asset/logo/zukunft-bau-dark.png", alt: "Zukunft Bau", href: "https://www.zukunftbau.de/projekte/forschungsfoerderung/1008187-2506" },
        ],
      },
      {
        id: "viewport",
        title: "Die 3D-Ansicht",
        body: "Hier entsteht Ihr Entwurf aus Bestandskomponenten — orbitieren, verschieben und zoomen Sie, um sich umzusehen.",
        anchor: { kind: "windowKind", id: "puzzle3d-main" },
        emphasis: "cutout",
        placement: "auto",
        advance: { kind: "next" },
        logos: [],
      },
      {
        id: "move-utility",
        title: "Baukomponenten verschieben",
        body: "Aktivieren Sie das Verschieben-Werkzeug, um Baukomponenten neu zu positionieren.",
        anchor: { kind: "utility", id: "move" },
        emphasis: "cutout",
        placement: "auto",
        advance: { kind: "utility", id: "move" },
        logos: [],
      },
      {
        id: "catalogue",
        title: "Der Katalog",
        body: "Durchstöbern Sie hier die verfügbaren Baukomponenten aus dem Bestand.",
        anchor: { kind: "panelTab", id: "framework.panel.catalogue" },
        emphasis: "cutout",
        placement: "auto",
        advance: { kind: "next" },
        logos: [],
      },
      {
        id: "add-object",
        title: "Baukomponente hinzufügen",
        body: "Fügen Sie Ihre erste Baukomponente hinzu — zum Beispiel aus dem Projekt Abbau Aufbau.",
        anchor: { kind: "action", id: "addObjectKind" },
        emphasis: "cutout",
        placement: "auto",
        advance: { kind: "action", id: "addObjectKind" },
        logos: [],
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandBrand
