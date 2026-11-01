import { describe, it, expect } from "vitest";
import { existsSync } from "node:fs";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Footer, navbarFillItem } from "@semio-tech/ui-react";
import { ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND, ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND, ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND, ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND, ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND, ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND, ENTWERFEN_MIT_BESTAND_STATIK_BRAND, ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND, ENTWERFEN_MIT_BESTAND_BRAND_IDS, demonstratorGeneralIntroduction, demonstratorPaneViews, isEntwerfenMitBestandBrandId } from "../../🪧️brand.ts";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "../../⚛️footer.tsx";

const partnerLogo = (id: string, index: number) => demonstratorFooterItems("de").find((item) => item.id === id)!.logos[index]!;
const LUH_LOGO_URL = partnerLogo("aProjectOfLuhUdk", 0).src;
const UDK_LOGO_URL = partnerLogo("aProjectOfLuhUdk", 1).src;
const LUH_URL = partnerLogo("aProjectOfLuhUdk", 0).href;
const UDK_URL = partnerLogo("aProjectOfLuhUdk", 1).href;
const ZUKUNFT_BAU_PROJECT_URL = partnerLogo("fundedByZukunftBau", 0).href;

type Cells = { readonly native: { readonly de: string }; readonly reuse: { readonly de: string } };
const german = (label: unknown): string => (label as Cells).native.de;
const deLabel = (de: string) => expect.objectContaining({ native: expect.objectContaining({ de }), reuse: expect.objectContaining({ de }) });

describe("demonstrator shell brands", () => {
  it("registers all eight Entwerfen mit Bestand demonstrator shell brands", () => {
    expect(ENTWERFEN_MIT_BESTAND_BRAND_IDS).toEqual([
      "entwerfen-mit-bestand-aggregator",
      "entwerfen-mit-bestand-aussuchen",
      "entwerfen-mit-bestand-bearbeiten",
      "entwerfen-mit-bestand-energie",
      "entwerfen-mit-bestand-generator",
      "entwerfen-mit-bestand-koordinator",
      "entwerfen-mit-bestand-statik",
      "entwerfen-mit-bestand-verfolgen",
    ]);
    expect(ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND.id).toBe("entwerfen-mit-bestand-aggregator");
    expect(ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND.id).toBe("entwerfen-mit-bestand-aussuchen");
    expect(ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND.id).toBe("entwerfen-mit-bestand-bearbeiten");
    expect(ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND.id).toBe("entwerfen-mit-bestand-energie");
    expect(ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND.id).toBe("entwerfen-mit-bestand-generator");
    expect(ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND.id).toBe("entwerfen-mit-bestand-koordinator");
    expect(ENTWERFEN_MIT_BESTAND_STATIK_BRAND.id).toBe("entwerfen-mit-bestand-statik");
    expect(ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND.id).toBe("entwerfen-mit-bestand-verfolgen");
    expect(isEntwerfenMitBestandBrandId(ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND.id)).toBe(true);
    expect(isEntwerfenMitBestandBrandId("semio-os")).toBe(false);
    expect(ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND.windowTitle).toBe("Entwerfen mit Bestand");
    expect(ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND.locks?.locale).toBeUndefined();
    const energy = (locale: "en" | "de") => demonstratorPaneViews(locale).find((pane) => pane.id === "energie")!;
    expect(energy("en").brand.windowTitle).toBe("Entwerfen mit Bestand · Energy");
    expect(energy("de").brand.windowTitle).toBe("Entwerfen mit Bestand · Energie");
    expect(energy("en").brand.shortWindowTitle).toBe("EmB · Energy");
    expect(energy("de").brand.shortWindowTitle).toBe("EmB · Energie");
    expect(demonstratorGeneralIntroduction("de").steps.map((step) => step.id)).toEqual(["welcome", "prototype", "funding"]);
  });

  it("ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND introduction is app-specific only after the general landing tour was split out", () => {
    const steps = ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND.introduction!.steps;
    expect(steps.map((step) => step.id)).toEqual(["viewport", "panels", "catalogue-objects", "add-object", "transform-utility", "verbindungspunkte", "suggest-objects", "fill-tool", "fill-distribution"]);
    const viewport = steps.find((step) => step.id === "viewport")!;
    expect(viewport.ordered).toBe(false);
    expect(viewport.interactions.map((interaction) => interaction.on)).toEqual([
      { kind: "zoom", id: "puzzle3d-main" },
      { kind: "pan", id: "puzzle3d-main" },
      { kind: "orbit", id: "puzzle3d-main" },
    ]);
    expect(viewport.interactions.map((interaction) => german(interaction.label))).toEqual(["Zoomen (Mausrad)", "Verschieben (Mittelklick ziehen)", "Orbitieren (Alt + Rechtsklick ziehen)"]);
    expect(german(viewport.body)).toMatch(/Mausrad|Mittelklick|Alt \+ Rechtsklick/i);
    expect(steps.find((step) => step.id === "panels")).toMatchObject({
      introduce: "framework.panel.catalogue",
      interactions: [{ on: { kind: "panel", id: "framework.panel.catalogue" }, label: deLabel("Katalog-Reiter anklicken") }],
    });
    expect(german(steps.find((step) => step.id === "panels")?.body)).toMatch(/linken Maustaste|Katalog-Reiter/i);
    expect(steps.find((step) => step.id === "catalogue-objects")).toMatchObject({
      introduce: "puzzle3d-play-kinds.objects",
      placement: "right",
      interactions: [{ on: { kind: "expand", id: "puzzle3d-play-kinds.objects" }, label: deLabel("»Baukomponenten« anklicken") }],
      show: ["framework.panelTab.framework.panel.catalogue"],
    });
    expect(german(steps.find((step) => step.id === "catalogue-objects")?.body)).toMatch(/linken Maustaste|Baukomponenten/i);
    expect(steps.find((step) => step.id === "add-object")).toMatchObject({
      introduce: "framework.panelTab.framework.panel.catalogue.firstDraggable",
      placement: "right",
      interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: deLabel("Mit linker Maustaste in die Ansicht ziehen") }],
      show: ["framework.panelTab.framework.panel.catalogue", "framework.window.puzzle3dMain"],
    });
    expect(german(steps.find((step) => step.id === "add-object")?.body)).toMatch(/linken Maustaste|Drag-and-Drop/i);
    expect(steps.find((step) => step.id === "transform-utility")).toMatchObject({
      introduce: "transform",
      interactions: [{ on: { kind: "utility", id: "transform" }, label: deLabel("Transformieren anklicken") }],
      show: ["framework.window.puzzle3dMain"],
    });
    expect(german(steps.find((step) => step.id === "transform-utility")?.body)).toMatch(/linken Maustaste|Transformieren/i);
    expect(steps.find((step) => step.id === "verbindungspunkte")).toMatchObject({
      introduce: "puzzle3d-play-vortex-show",
      interactions: [{ on: { kind: "action", id: "setVortexShow" }, label: deLabel("»Verbindungspunkte anzeigen« auf »Immer« stellen") }],
      show: ["framework.window.puzzle3dMain"],
    });
    expect(german(steps.find((step) => step.id === "verbindungspunkte")?.body)).toMatch(/Linksklick|Verbindungspunkte/i);
    expect(steps.find((step) => step.id === "suggest-objects")).toMatchObject({
      introduce: "framework.window.puzzle3dMain",
      interactions: [{ on: { kind: "action", id: "acceptSuggestion" }, label: deLabel("Vorschlag per Linksklick wählen") }],
    });
    expect(german(steps.find((step) => step.id === "suggest-objects")?.body)).toMatch(/Linksklick|Rechtsklick|Aktionsmenü/i);
    expect(steps.find((step) => step.id === "fill-tool")).toMatchObject({
      introduce: "tool.fill",
      interactions: [{ on: { kind: "tool", id: "fill" }, label: deLabel("»Füllen« anklicken") }],
      show: [],
      placement: "top",
    });
    expect(german(steps.find((step) => step.id === "fill-tool")?.body)).toMatch(/linken Maustaste|Füllen/i);
    expect(steps.find((step) => step.id === "fill-distribution")).toMatchObject({
      introduce: "puzzle3d-play-distribution",
      interactions: [],
      show: ["puzzle3d-fill-count", "framework.panelTab.tool.fill"],
      placement: "top",
    });
    expect(german(steps.find((step) => step.id === "fill-distribution")?.body)).toMatch(/Schieberegler|Verteilung/i);

    const funding = demonstratorGeneralIntroduction("de").steps.find((step) => step.id === "funding")!;
    expect(funding.logos).toHaveLength(3);
    for (const logo of funding.logos!) {
      expect(logo.src).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logos\//);
      expect(logo.darkSrc).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logos\//);
      expect(logo.alt).toBeTruthy();
      let root = import.meta.dirname;
      for (const url of [logo.src, logo.darkSrc!]) {
        const relative = url.replace(/^\//, "");
        for (let hop = 0; hop < 12 && !existsSync(`${root}/${relative}`); hop += 1) root = `${root}/..`;
        expect(existsSync(`${root}/${relative}`)).toBe(true);
      }
    }
    const zukunftBauLogo = funding.logos!.find((logo) => logo.href === ZUKUNFT_BAU_PROJECT_URL);
    expect(zukunftBauLogo).toBeDefined();
  });

  it("mit-bestand/demonstrator footer credits render the funding/partner logos, links, and locale text", () => {
    const fundedByMarkup = renderToStaticMarkup(createElement(Footer, { items: [navbarFillItem("fillLeft"), fundedByZukunftBauFooterItem("fundedByEn", "en", false), navbarFillItem("fillRight")] }));
    expect(fundedByMarkup).toContain("<button");
    expect(fundedByMarkup).toContain("Funded by");
    expect(fundedByMarkup).toContain("z-40");
    expect(ZUKUNFT_BAU_PROJECT_URL).toMatch(/^https:\/\/www\.zukunftbau\.de\//);
    const fundedByDeMarkup = renderToStaticMarkup(createElement(Footer, { items: [fundedByZukunftBauFooterItem("fundedByDe", "de", false)] }));
    expect(fundedByDeMarkup).toContain("Gefördert durch");
    expect(fundedByDeMarkup).toContain("hover:text-foreground");
    const projectOfMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfDe", "de", false)] }));
    expect(projectOfMarkup).toContain("Ein Projekt von");
    expect(projectOfMarkup).toContain("hover:text-foreground");
    expect(projectOfMarkup).toContain("und");
    expect(projectOfMarkup).toContain(LUH_LOGO_URL);
    expect(projectOfMarkup).toContain(UDK_LOGO_URL);
    expect(projectOfMarkup).toContain(LUH_URL);
    expect(projectOfMarkup).toContain(UDK_URL);
    expect(projectOfMarkup).toContain("z-40");
    const projectOfEnMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfEn", "en", false)] }));
    expect(projectOfEnMarkup).toContain("A project of");
    expect(projectOfEnMarkup).toContain("and");
    const fundedByIconOnlyMarkup = renderToStaticMarkup(createElement(Footer, { items: [fundedByZukunftBauFooterItem("fundedByIconOnly", "en", true)] }));
    expect(fundedByIconOnlyMarkup).not.toContain(">Funded by<");
    expect(fundedByIconOnlyMarkup).toContain('aria-label="Funded by Zukunft Bau (opens in a new tab)"');
    const projectOfIconOnlyMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfIconOnly", "de", true)] }));
    expect(projectOfIconOnlyMarkup).not.toContain(">Ein Projekt von<");
    expect(projectOfIconOnlyMarkup).toContain('aria-label="Leibniz Universität Hannover (öffnet in einem neuen Tab)"');
    expect(projectOfIconOnlyMarkup).not.toContain(">und<");
    expect(projectOfIconOnlyMarkup).toContain(LUH_LOGO_URL);
    expect(projectOfIconOnlyMarkup).toContain(UDK_LOGO_URL);
    expect(LUH_LOGO_URL).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logos\//);
    expect(UDK_LOGO_URL).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logos\//);
  });

});

import Ajv from "ajv";
import { DEMONSTRATOR_SHELL_BRANDS, demonstratorFooterItems } from "../../🪧️brand.ts";
import { shellFooterNavbarItem } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellFooter";
import { manifestSchema as schema } from "@semio-tech/framework-manifest/schema";
import fixture from "@semio-tech/framework-manifest/testing/fixtures/shell-footer.json" with { type: "json" };
import type { ShellFooterItem, ShellLocale } from "@semio-tech/framework";

it("renders schema-owned neutral credits and validates every concrete owner contribution", () => {
  const validate = new Ajv({ strict: true }).addKeyword("x-semio-formats").compile(schema.$defs.ShellFooterItemV1);
  expect(validate(fixture.item)).toBe(true);
  for (const locale of ["en", "de"] as const) for (const item of demonstratorFooterItems(locale)) {
    expect(validate(item)).toBe(true);
    for (const logo of item.logos) for (const path of [logo.src, logo.darkSrc].filter((path): path is string => path !== undefined)) {
      expect(existsSync(new URL("../../../../" + path.slice(1), import.meta.url))).toBe(true);
    }
  }
  expect(demonstratorFooterItems("en")[0]!.logos[0]!.alt).not.toBe(demonstratorFooterItems("de")[0]!.logos[0]!.alt);
  expect(DEMONSTRATOR_SHELL_BRANDS).toHaveLength(8);
  for (const locale of ["en", "de"] as const) for (const pane of demonstratorPaneViews(locale)) expect(pane.brand.footerItems).toEqual(demonstratorFooterItems(locale));
  for (const row of fixture.cases) {
    const item = shellFooterNavbarItem(fixture.item as ShellFooterItem, row.locale as ShellLocale, row.compact);
    const markup = renderToStaticMarkup(createElement(Footer, { items: [item] }));
    for (const logo of fixture.item.logos) { expect(markup).toContain(logo.href); expect(markup).toContain(logo.alt); }
    if (row.caption) expect(markup).toContain(row.caption); else { expect(markup).not.toContain("Created by"); expect(markup).not.toContain("Erstellt von"); }
    if (row.separator) expect(markup).toContain(`>${row.separator}<`);
    expect(markup).toContain('rel="noopener noreferrer"');
  }
  console.log(`Shell footer neutral cases=${fixture.cases.length}, owner brands=${DEMONSTRATOR_SHELL_BRANDS.length}, credits=${demonstratorFooterItems("en").length}`);
});
