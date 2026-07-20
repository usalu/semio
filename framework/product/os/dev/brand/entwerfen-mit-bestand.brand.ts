// #region 🧲Header
/** @emoji 🏷️ "Entwerfen mit Bestand Aggregator" brand — German, reuse-terminology, theme-locked standalone puzzle3d. */
// #endregion 🧲Header

import type { ShellBrand } from "../../../../core/js/index.ts";

//#region 🏷️EntwerfenMitBestandBrand
/** @emoji ✒️ Typographic "EmB" monogram badge (self-colored round mark, mirrors the semio emblem's badge shape) — swap for a dedicated project mark once one exists. */
const ENTWERFEN_MIT_BESTAND_LOGO_SVG = `<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Entwerfen mit Bestand"><circle cx="175" cy="175" r="175" fill="#1f3d2b"/><text x="175" y="175" text-anchor="middle" dominant-baseline="central" font-family="system-ui, 'Segoe UI', sans-serif" font-size="130" font-weight="600" letter-spacing="-4" fill="#f7f3e3">EmB</text></svg>`;

/** 🏷️ The Aggregator ships puzzle3d with locked German locale, locked reuse terminology (window "Aggregator", document "Entwerfen mit Bestand", example "Abbau Aufbau"), locked semio theme, switchable appearance, a brand-owned German introduction, and Abbau Aufbau (`concrete-forest`) seeded as the default-but-switchable example. Anchors reference `puzzle/plugin/rs/lib.rs`'s puzzle3d app (`puzzle3d-main`, `move`, `addObjectKind`) and `framework/core/js`'s `FRAMEWORK_PANEL_TAB_CATALOGUE_ID`. */
export const ENTWERFEN_MIT_BESTAND_BRAND: ShellBrand = {
  id: "entwerfen-mit-bestand",
  windowTitle: "Entwerfen mit Bestand · Aggregator",
  logoSvg: ENTWERFEN_MIT_BESTAND_LOGO_SVG,
  locks: { locale: "de", terminology: "reuse", themeId: "semio" },
  defaults: { exampleId: "concrete-forest" },
  introduction: {
    title: "Willkommen beim Aggregator",
    steps: [
      {
        id: "welcome",
        title: "Willkommen beim Aggregator",
        body: "Entwerfen mit Bestand: Komponieren Sie neue Strukturen aus wiederverwendeten Baukomponenten. Diese kurze Tour zeigt Ihnen die wichtigsten Werkzeuge.",
        anchor: { kind: "screen" },
        emphasis: "none",
        placement: "center",
        advance: { kind: "next" },
      },
      {
        id: "viewport",
        title: "Die 3D-Ansicht",
        body: "Hier entsteht Ihr Entwurf aus Bestandskomponenten — orbitieren, verschieben und zoomen Sie, um sich umzusehen.",
        anchor: { kind: "windowKind", id: "puzzle3d-main" },
        emphasis: "highlight",
        placement: "auto",
        advance: { kind: "next" },
      },
      {
        id: "move-utility",
        title: "Baukomponenten verschieben",
        body: "Aktivieren Sie das Verschieben-Werkzeug, um Baukomponenten neu zu positionieren.",
        anchor: { kind: "utility", id: "move" },
        emphasis: "highlight",
        placement: "auto",
        advance: { kind: "utility", id: "move" },
      },
      {
        id: "catalogue",
        title: "Der Katalog",
        body: "Durchstöbern Sie hier die verfügbaren Baukomponenten aus dem Bestand.",
        anchor: { kind: "panelTab", id: "framework.panel.catalogue" },
        emphasis: "none",
        placement: "auto",
        advance: { kind: "next" },
      },
      {
        id: "add-object",
        title: "Baukomponente hinzufügen",
        body: "Fügen Sie Ihre erste Baukomponente hinzu — zum Beispiel aus dem Projekt Abbau Aufbau.",
        anchor: { kind: "action", id: "addObjectKind" },
        emphasis: "none",
        placement: "auto",
        advance: { kind: "action", id: "addObjectKind" },
      },
    ],
  },
};
//#endregion 🏷️EntwerfenMitBestandBrand
