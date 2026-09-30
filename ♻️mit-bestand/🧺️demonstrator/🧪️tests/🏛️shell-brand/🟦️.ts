import { describe, it, expect } from "vitest";
import { existsSync } from "node:fs";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Footer, navbarFillItem } from "@semio-tech/ui-react";
import { ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND, ENTWERFEN_MIT_BESTAND_AUSSUCHEN_BRAND, ENTWERFEN_MIT_BESTAND_BEARBEITEN_BRAND, ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND, ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND, ENTWERFEN_MIT_BESTAND_KOORDINATOR_BRAND, ENTWERFEN_MIT_BESTAND_STATIK_BRAND, ENTWERFEN_MIT_BESTAND_VERFOLGEN_BRAND, ENTWERFEN_MIT_BESTAND_BRAND_IDS, ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION, isEntwerfenMitBestandBrandId } from "../../🪧️brand.ts";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem, LUH_LOGO_URL, UDK_LOGO_URL, LUH_URL, UDK_URL, ZUKUNFT_BAU_PROJECT_URL } from "../../⚛️footer.tsx";

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
    expect(ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND.windowTitle).toBe("Entwerfen mit Bestand · Energie");
    expect(ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION.steps.map((step) => step.id)).toEqual(["welcome", "prototype", "funding"]);
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
    expect(viewport.interactions.map((interaction) => interaction.label)).toEqual(["Zoomen (Mausrad)", "Verschieben (Mittelklick ziehen)", "Orbitieren (Alt + Rechtsklick ziehen)"]);
    expect(viewport.body).toMatch(/Mausrad|Mittelklick|Alt \+ Rechtsklick/i);
    expect(steps.find((step) => step.id === "panels")).toMatchObject({
      introduce: "framework.panel.catalogue",
      interactions: [{ on: { kind: "panel", id: "framework.panel.catalogue" }, label: "Katalog-Reiter anklicken" }],
    });
    expect(steps.find((step) => step.id === "panels")?.body).toMatch(/linken Maustaste|Katalog-Reiter/i);
    expect(steps.find((step) => step.id === "catalogue-objects")).toMatchObject({
      introduce: "puzzle3d-play-kinds.objects",
      placement: "right",
      interactions: [{ on: { kind: "expand", id: "puzzle3d-play-kinds.objects" }, label: "»Baukomponenten« anklicken" }],
      show: ["framework.panelTab.framework.panel.catalogue"],
    });
    expect(steps.find((step) => step.id === "catalogue-objects")?.body).toMatch(/linken Maustaste|Baukomponenten/i);
    expect(steps.find((step) => step.id === "add-object")).toMatchObject({
      introduce: "framework.panelTab.framework.panel.catalogue.firstDraggable",
      placement: "right",
      interactions: [{ on: { kind: "action", id: "addObjectKind" }, label: "Mit linker Maustaste in die Ansicht ziehen" }],
      show: ["framework.panelTab.framework.panel.catalogue", "framework.window.puzzle3dMain"],
    });
    expect(steps.find((step) => step.id === "add-object")?.body).toMatch(/linken Maustaste|Drag-and-Drop/i);
    expect(steps.find((step) => step.id === "transform-utility")).toMatchObject({
      introduce: "transform",
      interactions: [{ on: { kind: "utility", id: "transform" }, label: "Transformieren anklicken" }],
      show: ["framework.window.puzzle3dMain"],
    });
    expect(steps.find((step) => step.id === "transform-utility")?.body).toMatch(/linken Maustaste|Transformieren/i);
    expect(steps.find((step) => step.id === "verbindungspunkte")).toMatchObject({
      introduce: "puzzle3d-play-vortex-show",
      interactions: [{ on: { kind: "action", id: "setVortexShow" }, label: "»Verbindungspunkte anzeigen« auf »Immer« stellen" }],
      show: ["framework.window.puzzle3dMain"],
    });
    expect(steps.find((step) => step.id === "verbindungspunkte")?.body).toMatch(/Linksklick|Verbindungspunkte/i);
    expect(steps.find((step) => step.id === "suggest-objects")).toMatchObject({
      introduce: "framework.window.puzzle3dMain",
      interactions: [{ on: { kind: "action", id: "acceptSuggestion" }, label: "Vorschlag per Linksklick wählen" }],
    });
    expect(steps.find((step) => step.id === "suggest-objects")?.body).toMatch(/Linksklick|Rechtsklick|Aktionsmenü/i);
    expect(steps.find((step) => step.id === "fill-tool")).toMatchObject({
      introduce: "tool.fill",
      interactions: [{ on: { kind: "tool", id: "fill" }, label: "»Füllen« anklicken" }],
      show: [],
      placement: "top",
    });
    expect(steps.find((step) => step.id === "fill-tool")?.body).toMatch(/linken Maustaste|Füllen/i);
    expect(steps.find((step) => step.id === "fill-distribution")).toMatchObject({
      introduce: "puzzle3d-play-distribution",
      interactions: [],
      show: ["puzzle3d-fill-count", "framework.panelTab.tool.fill"],
      placement: "top",
    });
    expect(steps.find((step) => step.id === "fill-distribution")?.body).toMatch(/Schieberegler|Verteilung/i);

    const funding = ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION.steps.find((step) => step.id === "funding")!;
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
    const fundedByMarkup = renderToStaticMarkup(createElement(Footer, { items: [navbarFillItem("fillLeft"), fundedByZukunftBauFooterItem("fundedByEn", "en"), navbarFillItem("fillRight")] }));
    expect(fundedByMarkup).toContain("<button");
    expect(fundedByMarkup).toContain("Funded by");
    expect(fundedByMarkup).toContain("z-40");
    expect(ZUKUNFT_BAU_PROJECT_URL).toMatch(/^https:\/\/www\.zukunftbau\.de\//);
    const fundedByDeMarkup = renderToStaticMarkup(createElement(Footer, { items: [fundedByZukunftBauFooterItem("fundedByDe", "de")] }));
    expect(fundedByDeMarkup).toContain("Gefördert durch");
    expect(fundedByDeMarkup).toContain("hover:text-foreground");
    const projectOfMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem()] }));
    expect(projectOfMarkup).toContain("Ein Projekt von");
    expect(projectOfMarkup).toContain("hover:text-foreground");
    expect(projectOfMarkup).toContain("und");
    expect(projectOfMarkup).toContain(LUH_LOGO_URL);
    expect(projectOfMarkup).toContain(UDK_LOGO_URL);
    expect(projectOfMarkup).toContain(LUH_URL);
    expect(projectOfMarkup).toContain(UDK_URL);
    expect(projectOfMarkup).toContain("z-40");
    const projectOfEnMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfEn", "en")] }));
    expect(projectOfEnMarkup).toContain("A project of");
    expect(projectOfEnMarkup).toContain("and");
    // 📱️ iconOnly (mobile) drops the surrounding text but keeps both logos and their links.
    const fundedByIconOnlyMarkup = renderToStaticMarkup(createElement(Footer, { items: [fundedByZukunftBauFooterItem("fundedByIconOnly", "en", true)] }));
    expect(fundedByIconOnlyMarkup).not.toContain("Funded by");
    const projectOfIconOnlyMarkup = renderToStaticMarkup(createElement(Footer, { items: [aProjectOfLuhUdkFooterItem("projectOfIconOnly", "de", true)] }));
    expect(projectOfIconOnlyMarkup).not.toContain("Ein Projekt von");
    expect(projectOfIconOnlyMarkup).not.toContain(">und<");
    expect(projectOfIconOnlyMarkup).toContain(LUH_LOGO_URL);
    expect(projectOfIconOnlyMarkup).toContain(UDK_LOGO_URL);
    expect(LUH_LOGO_URL).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logos\//);
    expect(UDK_LOGO_URL).toMatch(/♻️mit-bestand\/🧺️demonstrator\/🖼️asset\/🪧️logos\//);
  });

});

import Ajv from "ajv";
import { DEMONSTRATOR_SHELL_BRANDS, DEMONSTRATOR_FOOTER_ITEMS } from "../../🪧️brand.ts";
import { shellFooterNavbarItem } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellFooter/🟦️.tsx";
import schema from "../../../../🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json";
import fixture from "../../../../🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🏛️shell-footer/🔣️.json";
import type { ShellFooterItem, ShellLocale } from "@semio-tech/framework";

it("renders schema-owned neutral credits and validates every concrete owner contribution", () => {
  const validate = new Ajv({ strict: true }).addKeyword("x-semio-formats").compile(schema.$defs.ShellFooterItemV1);
  expect(validate(fixture.item)).toBe(true);
  for (const item of DEMONSTRATOR_FOOTER_ITEMS) {
    expect(validate(item)).toBe(true);
    for (const logo of item.logos) for (const path of [logo.src, logo.darkSrc].filter((path): path is string => path !== undefined)) {
      expect(existsSync(new URL("../../../../" + path.slice(1), import.meta.url))).toBe(true);
    }
  }
  expect(DEMONSTRATOR_SHELL_BRANDS).toHaveLength(8);
  for (const brand of DEMONSTRATOR_SHELL_BRANDS) expect(brand.footerItems).toBe(DEMONSTRATOR_FOOTER_ITEMS);
  for (const row of fixture.cases) {
    const item = shellFooterNavbarItem(fixture.item as ShellFooterItem, row.locale as ShellLocale, row.compact);
    const markup = renderToStaticMarkup(createElement(Footer, { items: [item] }));
    for (const logo of fixture.item.logos) { expect(markup).toContain(logo.href); expect(markup).toContain(logo.alt); }
    if (row.caption) expect(markup).toContain(row.caption); else { expect(markup).not.toContain("Created by"); expect(markup).not.toContain("Erstellt von"); }
    if (row.separator) expect(markup).toContain(`>${row.separator}<`);
    expect(markup).toContain('rel="noopener noreferrer"');
  }
  console.log(`Shell footer neutral cases=${fixture.cases.length}, owner brands=${DEMONSTRATOR_SHELL_BRANDS.length}, credits=${DEMONSTRATOR_FOOTER_ITEMS.length}`);
});
