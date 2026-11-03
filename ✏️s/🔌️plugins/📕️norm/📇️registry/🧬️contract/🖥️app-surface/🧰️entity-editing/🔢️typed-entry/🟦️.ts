/** 🔢️ Unit-aware typed entry, TypeScript twin of `./🦀️.rs`: the text typed during a drag or into a marker cell (`35 cm`, `1,5m`, `12`) becomes the drag parameter of the handle, in the unit the bound field is shown in.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`markerCells`) by both implementations. */
import type { Diagram2d, Handle, Marker } from "../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";
import { HandleDragToolV1, invertAffineV1, parseEntryV1, type FieldWriteV1 } from "../../🕹️direct-manipulation/🛠️handle-drag-tool/🟦️.ts";
import { markerGripV1, markerRestV1 } from "../../🪟️visual-windows/📈️diagrams2d/🪧marker-grips/🟦️.ts";

const UNITS: readonly (readonly [string, string, number])[] = [
  ["m", "L", 1], ["cm", "L", 0.01], ["mm", "L", 0.001], ["km", "L", 1000],
  ["N", "F", 1], ["kN", "F", 1e3], ["MN", "F", 1e6],
  ["Pa", "S", 1], ["kPa", "S", 1e3], ["MPa", "S", 1e6], ["GPa", "S", 1e9],
  ["Nm", "M", 1], ["kNm", "M", 1e3], ["MNm", "M", 1e6],
  ["deg", "A", 1], ["°", "A", 1], ["rad", "A", 57.29577951308232],
  ["%", "R", 0.01], ["", "R", 1],
];

const unitOf = (symbol: string): readonly [string, number] | undefined => {
  const row = UNITS.find(([name]) => name === symbol);
  return row === undefined ? undefined : [row[1], row[2]];
};

/** 🔢️ A typed quantity converted into `target` (the unit symbol of the bound field); a bare number is taken as already in `target`, a unit of another dimension or an unknown unit is `undefined`. */
export function parseQuantityEntryV1(text: string, target: string): number | undefined {
  const trimmed = text.trim();
  const split = trimmed.search(/[\p{Alphabetic}°%]/u);
  const cut = split < 0 ? trimmed.length : split;
  const value = parseEntryV1(trimmed.slice(0, cut));
  if (value === undefined) return undefined;
  const symbol = trimmed.slice(cut).trim();
  if (symbol === "") return value;
  const from = unitOf(symbol);
  const to = unitOf(target);
  if (from === undefined || to === undefined || to[0] !== from[0]) return undefined;
  return (value * from[1]) / to[1];
}

/** 🎯️ The drag parameter `s` the typed text means for `handle`: converted into the unit of its first bound field, mapped back through the field binding and clamped to the limits. `undefined` when the text does not parse or the handle binds no field. */
export function entryToSV1(handle: Handle, text: string): number | undefined {
  const binding = handle.writes[0];
  if (binding === undefined) return undefined;
  const value = parseQuantityEntryV1(text, binding.unit);
  if (value === undefined) return undefined;
  const s = invertAffineV1(binding, value);
  return handle.limits === null ? s : Math.min(Math.max(s, handle.limits.min), handle.limits.max);
}

/** ✍️ The field writes of a value typed into a marker cell: the same unit-aware parse, limits, field quantisation and single commit as a pointer drag released at that value; `undefined` when the text does not parse in the unit of the field, the marker has no binding, or the value equals the current one. */
export function markerRowCommitV1(diagram: Diagram2d, marker: Marker, text: string): readonly FieldWriteV1[] | undefined {
  const grip = markerGripV1(diagram, marker);
  const rest = markerRestV1(diagram, marker);
  if (grip === undefined || rest === undefined) return undefined;
  const target = entryToSV1(grip, text);
  if (target === undefined) return undefined;
  const tool = new HandleDragToolV1([grip]);
  const [dragging] = tool.step({ kind: "armed", handle: grip.id, origin: rest }, { kind: "move", s: target, modifiers: { shift: false, alt: false, ctrl: true } });
  const [, effect] = tool.step(dragging, { kind: "release" });
  return effect.kind === "commit" ? effect.writes : undefined;
}
