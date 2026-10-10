/**
 * 🏷️ WP-18 `w2-wp18-psets`: the library data of the `house` and `office` examples, merged into the snapshot by `r4-x-examples-gen.ts`: the seed classification systems (Uniclass 2015 Pr/EF, DIN 276 cost groups,
 * OmniClass Table 23), the property set templates, the classifications as `element -> {system id: code}` (the old single `{system, code, title}` rows of the generator are converted) and type-level properties that
 * the instances inherit. `psetsFor(name, model)` refuses a code the table lacks and an own value that breaks its definition, so an example carries no property or classification finding.
 *
 * Uniclass codes and titles: <https://uniclass.thenbs.com>; DIN 276-1:2018-12 cost groups; OmniClass Table 23 (CSI/CSC 2012), curated subsets.
 */
type Json = Record<string, any>;

const item = (code: string, title: string, parent?: string) => ({ code, title, ...(parent ? { parent } : {}) });
const text = (value: string) => ({ Text: { value } });
const real = (value: number) => ({ Real: { value } });
const flag = (value: boolean) => ({ Boolean: { value } });
const def = (name: string, kind: string, over: Json = {}) => ({ name, kind, required: false, allowed: [] as unknown[], ...over });
const template = (name: string, applies_to: string[], properties: unknown[]) => ({ name, applies_to, properties });
const choices = (...values: string[]) => values.map(text);

export const SYSTEMS: Record<string, any> = {
  "cs-uniclass": {
    name: "Uniclass 2015",
    edition: "EF v1.16, Pr v1.34",
    source: "https://uniclass.thenbs.com",
    entries: [
      item("EF_25", "Wall and barrier elements"),
      item("EF_25_10", "Walls", "EF_25"),
      item("EF_25_10_25", "External walls", "EF_25_10"),
      item("EF_25_10_30", "Free-standing walls", "EF_25_10"),
      item("EF_25_10_40", "Internal walls", "EF_25_10"),
      item("EF_25_10_60", "Parapet walls", "EF_25_10"),
      item("EF_25_30", "Openings", "EF_25"),
      item("Pr", "Products"),
      item("Pr_20", "Structure and general products", "Pr"),
      item("Pr_20_93", "Unit structure and general products", "Pr_20"),
      item("Pr_20_93_52", "Masonry walling units", "Pr_20_93"),
      item("Pr_25", "Skin products", "Pr"),
      item("Pr_25_71", "Rigid board, panel, sheet and sectional products", "Pr_25"),
      item("Pr_25_93", "Unit skin products", "Pr_25"),
    ],
  },
  "cs-din-276": {
    name: "DIN 276",
    edition: "2018-12",
    source: "https://www.din.de",
    entries: [
      item("300", "Bauwerk - Baukonstruktionen"),
      item("330", "Außenwände/Vertikale Baukonstruktionen, außen", "300"),
      item("331", "Tragende Außenwände", "330"),
      item("332", "Nichttragende Außenwände", "330"),
      item("333", "Außenstützen", "330"),
      item("334", "Außentüren und -fenster", "330"),
      item("337", "Elementierte Außenwände", "330"),
      item("340", "Innenwände/Vertikale Baukonstruktionen, innen", "300"),
      item("341", "Tragende Innenwände", "340"),
      item("342", "Nichttragende Innenwände", "340"),
      item("343", "Innenstützen", "340"),
      item("344", "Innentüren und -fenster", "340"),
      item("350", "Decken/Horizontale Baukonstruktionen", "300"),
      item("351", "Deckenkonstruktionen", "350"),
      item("360", "Dächer", "300"),
      item("361", "Dachkonstruktionen", "360"),
      item("362", "Dachfenster, Dachöffnungen", "360"),
      item("363", "Dachbeläge", "360"),
    ],
  },
  "cs-omniclass-23": {
    name: "OmniClass Table 23 Products",
    edition: "2012",
    source: "https://www.csiresources.org/standards/omniclass",
    entries: [
      item("23-13 00 00", "Structural and Exterior Enclosure Products"),
      item("23-13 35 00", "Framing Products", "23-13 00 00"),
      item("23-13 35 11", "Structural Frames", "23-13 35 00"),
      item("23-13 35 11 13", "Column Slab Frames", "23-13 35 11"),
      item("23-17 00 00", "Openings, Passages, and Protection Products"),
      item("23-17 23 00", "Circulation and Escape Products", "23-17 00 00"),
      item("23-17 23 17", "Stairs", "23-17 23 00"),
    ],
  },
};

const transmittance = (maximum = 5) => def("ThermalTransmittance", "Real", { unit: "W/(m2.K)", minimum: 0, maximum });
const external = () => def("IsExternal", "Boolean", { default_value: flag(false) });
const loadBearing = () => def("LoadBearing", "Boolean", { default_value: flag(false) });

export const TEMPLATES: Record<string, any> = {
  "pt-wall": template("Pset_WallCommon", ["Wall", "WallType"], [external(), loadBearing(), def("FireRating", "Text", { allowed: choices("REI 30", "REI 60", "REI 90", "REI 120") }), transmittance()]),
  "pt-column": template("Pset_ColumnCommon", ["Column", "ColumnType"], [loadBearing(), def("FireRating", "Text", { allowed: choices("R 30", "R 60", "R 90", "R 120") })]),
  "pt-curtain": template("Pset_CurtainWallCommon", ["CurtainWall"], [external(), transmittance()]),
  "pt-door": template("Pset_DoorCommon", ["Door", "DoorType"], [external(), def("SecurityRating", "Text", { allowed: choices("RC1", "RC2", "RC3") })]),
  "pt-window": template("Pset_WindowCommon", ["Window", "WindowType"], [transmittance(), def("GlazingAreaFraction", "Real", { minimum: 0, maximum: 1 })]),
  "pt-roof": template("Pset_RoofCommon", ["Roof", "RoofType"], [transmittance(), def("FireRating", "Text", { allowed: choices("RF1", "RF2", "RF3") })]),
  "pt-space": template("Pset_SpaceCommon", ["Space"], [def("OccupancyType", "Text"), def("Reference", "Text")]),
  "pt-energy": template("Semio_Energy", ["Space"], [def("HeatedZone", "Boolean", { default_value: flag(false) }), def("TargetTemperature", "Real", { unit: "°C", minimum: 5, maximum: 30 })]),
};

const SYSTEM_OF: Record<string, string> = { "Uniclass 2015": "cs-uniclass", "DIN 276": "cs-din-276" };

const violates = (definition: any, value: any): string | null => {
  const [kind, body] = Object.entries<any>(value)[0];
  if (kind !== definition.kind) return "kind";
  if (typeof body.value === "number") {
    if (definition.minimum !== undefined && body.value < definition.minimum) return "minimum";
    if (definition.maximum !== undefined && body.value > definition.maximum) return "maximum";
  }
  if (definition.allowed.length && !definition.allowed.some((allowed: any) => JSON.stringify(allowed) === JSON.stringify(value))) return "allowed";
  return null;
};

const classify = (classifications: Json, id: string, system: string, code: string) => {
  if (!SYSTEMS[system].entries.some((entry: any) => entry.code === code)) throw new Error(`${system} has no code ${code}`);
  classifications[id] = { ...(classifications[id] ?? {}), [system]: code };
};

/** 🏷️ The pset collections of one example: `classification_systems`, `property_templates`, `properties` and `classifications`. */
export function psetsFor(name: string, model: any): Json {
  const classifications: Json = {};
  for (const [id, old] of Object.entries<any>(model.classifications ?? {})) {
    const system = SYSTEM_OF[old.system];
    if (!system) throw new Error(`unknown classification system ${old.system} of ${id}`);
    classify(classifications, id, system, old.code);
  }
  const properties: Json = JSON.parse(JSON.stringify(model.properties ?? {}));
  const set = (id: string, pset: string, props: Json) => {
    properties[id] = { ...(properties[id] ?? {}), [pset]: { ...(properties[id]?.[pset] ?? {}), ...props } };
  };
  if (name === "house") {
    const wallType = model.walls["w-g-south"].wall_type;
    set(wallType, "Pset_WallCommon", { IsExternal: flag(true), ThermalTransmittance: real(0.24), FireRating: text("REI 60") });
    classify(classifications, wallType, "cs-uniclass", "EF_25_10_25");
    classify(classifications, "w-g-south", "cs-din-276", "331");
    classify(classifications, "sr-main", "cs-omniclass-23", "23-17 23 17");
  } else {
    const columnType = model.columns["c-0-A1"].column_type;
    set(columnType, "Pset_ColumnCommon", { LoadBearing: flag(true), FireRating: text("R 90") });
    classify(classifications, columnType, "cs-omniclass-23", "23-13 35 11 13");
    classify(classifications, "cu-0-south", "cs-uniclass", "EF_25_10_25");
  }
  const kinds: Record<string, string> = {};
  const target = (id: string): string | undefined => {
    const lookups: [string, string][] = [["walls", "Wall"], ["curtain_walls", "CurtainWall"], ["columns", "Column"], ["roofs", "Roof"], ["spaces", "Space"], ["wall_types", "WallType"], ["column_types", "ColumnType"], ["window_types", "WindowType"], ["door_types", "DoorType"]];
    for (const [collection, kind] of lookups) if (model[collection]?.[id]) return kind;
    const opening = model.openings?.[id];
    if (opening) return opening.kind.Window ? "Window" : opening.kind.Door ? "Door" : "Void";
    return undefined;
  };
  for (const [id, sets] of Object.entries<any>(properties)) {
    const kind = target(id);
    kinds[id] = kind ?? "";
    for (const found of Object.values<any>(TEMPLATES).filter((row) => kind && row.applies_to.includes(kind)))
      for (const [property, value] of Object.entries<any>(sets[found.name] ?? {})) {
        const definition = found.properties.find((row: any) => row.name === property);
        const problem = definition ? violates(definition, value) : null;
        if (problem) throw new Error(`${id} ${found.name}.${property} breaks its definition (${problem})`);
      }
  }
  return { classification_systems: SYSTEMS, property_templates: TEMPLATES, properties, classifications };
}
