/** 🪟️ Diagrams window marker cells, TypeScript twin of the marker rows of `./🦀️.rs`: a marker bound to a field is a row with an inline editor cell that shows the bound value in the unit of the field, takes `350 cm` style text (Enter or leaving the cell commits, Escape restores the shown value) and is named by the marker and the unit for a screen reader.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`markerCells`) by both implementations; the commit rule is `markerRowCommitV1`. */
import type { Diagram2d, Marker } from "../../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";
import { valueAtV1 } from "../../../🕹️direct-manipulation/🛠️handle-drag-tool/🟦️.ts";
import { markerGripV1, markerRestV1 } from "../🪧marker-grips/🟦️.ts";

/** 🌐️ An interface language of the cell chrome. */
export type CellLocaleV1 = "en" | "de";

/** 🖊️ The id of the inline editor in the marker row `index` of the table twin of the diagram surface `base`. */
export const markerCellIdV1 = (base: string, index: number): string => `${base}.marker.m${index}.input`;

/** 🖊️ The id of the marker row `index`. */
export const markerRowIdV1 = (base: string, index: number): string => `${base}.marker.m${index}`;

/** 🖊️ The value to twelve significant digits without float noise or trailing zeros (`2.5`, `0.000125`); the text re-parses to the same number, so confirming an untouched cell commits nothing. */
export function shownValueV1(value: number): string {
  if (value === 0 || !Number.isFinite(value)) return "0";
  const places = Math.min(Math.max(11 - Math.floor(Math.log10(Math.abs(value))), 0), 15);
  const text = value.toFixed(places);
  return text.includes(".") ? text.replace(/0+$/u, "").replace(/\.$/u, "") : text;
}

/** 🖊️ What the inline editor of a marker row shows and says. */
export type MarkerCellV1 = Readonly<{ id: string; text: string; unit: string; name: string; placeholder: string; commit: "blur" }>;

/** 🖊️ The editor cell of marker `index`, `undefined` for a marker bound to no field (its row is read-only). */
export function markerCellV1(base: string, index: number, diagram: Diagram2d, marker: Marker, locale: CellLocaleV1): MarkerCellV1 | undefined {
  const grip = markerGripV1(diagram, marker);
  const rest = markerRestV1(diagram, marker);
  if (grip === undefined || rest === undefined) return undefined;
  const binding = grip.writes[0];
  const unit = binding?.unit ?? "";
  const shown = shownValueV1(binding === undefined ? rest : valueAtV1(binding, rest));
  const german = locale === "de";
  return {
    id: markerCellIdV1(base, index),
    text: `${shown} ${unit}`.trim(),
    unit,
    name: `${marker.label[locale]}, ${german ? "Wert bearbeiten" : "edit value"}`,
    placeholder: unit === "" ? (german ? "Wert" : "value") : `${german ? "Wert in" : "value in"} ${unit}`,
    commit: "blur",
  };
}
