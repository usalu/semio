/** 🔧️ Ticket-local writer for the schema and catalogue edits of the GAPS work.
 *
 * `bun 🔧️gaps-schema.ts petri`  — registers `notation-petri` and its two demo tables, and re-points
 *                                 the `petri-net` chart kind onto the dedicated Petri-net family.
 * `bun 🔧️gaps-schema.ts keys`   — fills `x-semio-family-options.<family>.options` from the
 *                                 handcrafted vocabulary in `🔧️gaps-key-docs.json`.
 *
 * Both actions read, mutate and write back with `JSON.stringify(…, null, 2)` plus a trailing
 * newline, which is byte-for-byte the on-disk format of both documents.
 */
import { readFileSync, writeFileSync } from "node:fs";

const PRODUCT = "C:/git/semio/🧰️framework/🛍️products/📓️print";
const SCHEMA = `${PRODUCT}/🧬️schema/🔣️.json`;
const CATALOG = `${PRODUCT}/🖼️assets/🔣️viz-catalog.json`;

type Localized = { en: string; de: string };
type Option = { type?: string; default?: unknown; enum?: string[]; description: Localized };
type Schema = {
  "x-semio-family-options": Record<string, { owner: string; options: Record<string, Option> }>;
  "x-semio-demo-tables": { name: string; columns: string[]; description: Localized }[];
};

function readJson<T>(path: string): T {
  return JSON.parse(readFileSync(path, "utf8")) as T;
}

function writeJson(path: string, value: unknown): void {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

/** 🔤 Sorts an object's keys so a generated block reads in a stable order. */
function sortKeys<T>(value: Record<string, T>): Record<string, T> {
  return Object.fromEntries(Object.entries(value).sort(([left], [right]) => left.localeCompare(right)));
}

//#region 🔖️Petri
const PETRI_OPTIONS: Record<string, Option> = {
  variant: {
    type: "string",
    description: {
      en: "Sub-kind the family renders; the catalogue slug of the chart kind. The family must draw a different geometry for every variant it declares.",
      de: "Untertyp, den die Familie zeichnet; der Katalog-Slug der Diagrammart. Die Familie muss für jede deklarierte Variante eine andere Geometrie zeichnen.",
    },
  },
  data: {
    type: "string",
    default: "demo-petri",
    description: {
      en: "Node table, columns id/label/kind/row/col/tokens.",
      de: "Knotentabelle, Spalten id/label/kind/row/col/tokens.",
    },
  },
  edges: {
    type: "string",
    default: "demo-petri-edges",
    description: {
      en: "Arc table, columns from/to/kind/label.",
      de: "Kantentabelle, Spalten from/to/kind/label.",
    },
  },
  id: {
    type: "string",
    default: "id",
    description: { en: "Column holding the node identity the arcs refer to.", de: "Spalte mit der Knotenkennung, auf die sich die Kanten beziehen." },
  },
  label: {
    type: "string",
    default: "label",
    description: { en: "Column holding the caption printed under a place or transition.", de: "Spalte mit der Beschriftung unter Stelle oder Transition." },
  },
  kind: {
    type: "string",
    default: "kind",
    description: { en: "Column separating the two node kinds; `place` draws a circle, `transition` a bar.", de: "Spalte, die die beiden Knotenarten trennt; `place` zeichnet einen Kreis, `transition` einen Balken." },
  },
  tokens: {
    type: "string",
    default: "tokens",
    description: { en: "Column holding the token count of a place; a positive count draws the marking dot.", de: "Spalte mit der Markenzahl einer Stelle; eine positive Zahl zeichnet den Markenpunkt." },
  },
  row: {
    type: "string",
    default: "row",
    description: { en: "Column holding the layout row of a node.", de: "Spalte mit der Layoutzeile eines Knotens." },
  },
  col: {
    type: "string",
    default: "col",
    description: { en: "Column holding the layout column of a node.", de: "Spalte mit der Layoutspalte eines Knotens." },
  },
  routing: {
    type: "string",
    default: "straight",
    enum: ["straight", "orthogonal", "curved"],
    description: { en: "How an arc is routed between a place and a transition.", de: "Wie eine Kante zwischen Stelle und Transition geführt wird." },
  },
  width: {
    type: "number",
    description: { en: "Frame width in millimetres; unset takes the figure's own width.", de: "Rahmenbreite in Millimetern; ohne Angabe gilt die Breite der Abbildung." },
  },
  height: {
    type: "number",
    description: { en: "Frame height in millimetres; unset takes the figure's own height.", de: "Rahmenhöhe in Millimetern; ohne Angabe gilt die Höhe der Abbildung." },
  },
};

const PETRI_TABLES = [
  {
    name: "demo-petri",
    columns: ["id", "label", "kind", "row", "col", "tokens"],
    description: {
      en: "Places and transitions of a small Petri net, with the token marking of every place.",
      de: "Stellen und Transitionen eines kleinen Petri-Netzes, mit der Markierung jeder Stelle.",
    },
  },
  {
    name: "demo-petri-edges",
    columns: ["from", "to", "kind", "label"],
    description: {
      en: "Arcs of the demo Petri net, each from a place to a transition or back.",
      de: "Kanten des Beispiel-Petri-Netzes, jeweils von einer Stelle zu einer Transition oder zurück.",
    },
  },
];

function registerPetri(): void {
  const schema = readJson<Schema>(SCHEMA);
  schema["x-semio-family-options"]["notation-petri"] = { owner: "DIAGRAMS", options: PETRI_OPTIONS };
  schema["x-semio-family-options"] = sortKeys(schema["x-semio-family-options"]);
  for (const table of PETRI_TABLES) {
    if (schema["x-semio-demo-tables"].some((entry) => entry.name === table.name)) continue;
    schema["x-semio-demo-tables"].push(table);
  }
  schema["x-semio-demo-tables"].sort((left, right) => left.name.localeCompare(right.name));
  writeJson(SCHEMA, schema);

  const catalog = readJson<{ kinds: { slug: string; family: string; options: Record<string, unknown>; data: string }[] }>(CATALOG);
  const entry = catalog.kinds.find((kind) => kind.slug === "petri-net");
  if (entry === undefined) throw new Error("no petri-net chart kind");
  entry.family = "notation-petri";
  entry.options = { variant: "petri-net", kind: "kind", tokens: "tokens" };
  entry.data = "demo-petri";
  writeJson(CATALOG, catalog);
  console.log("[DEBUG] notation-petri registered; petri-net re-pointed");
}
//#endregion 🔖️Petri

//#region 🔖️Keys
type KeyDocs = Record<string, Record<string, Option>>;

function fillKeys(): void {
  const schema = readJson<Schema>(SCHEMA);
  const docs = readJson<KeyDocs>(`${import.meta.dir}/🔧️gaps-key-docs.json`);
  let added = 0;
  let removed = 0;
  for (const [family, options] of Object.entries(docs)) {
    const entry = schema["x-semio-family-options"][family];
    if (entry === undefined) throw new Error(`no schema entry for family ${family}`);
    for (const [key, option] of Object.entries(options)) {
      if (entry.options[key] === undefined) added += 1;
      entry.options[key] = entry.options[key] ?? option;
    }
    for (const key of Object.keys(entry.options)) {
      if (options[key] === undefined) {
        delete entry.options[key];
        removed += 1;
      }
    }
    entry.options = sortKeys(entry.options);
  }
  writeJson(SCHEMA, schema);
  console.log(`[DEBUG] family options: ${added} added, ${removed} removed`);
}
//#endregion 🔖️Keys

const action = process.argv[2];
if (action === "petri") registerPetri();
else if (action === "keys") fillKeys();
else throw new Error(`unknown action ${String(action)}`);
