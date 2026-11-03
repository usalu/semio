/** 🪧 Marker grips, TypeScript twin of `./🦀️.rs`: a diagram marker bound to a field is the same kind of grip as a 3D handle, so the keyboard, the typed value and the table twin reach it through the one handle drag statechart.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`markerCells`) by both implementations. */
import type { Diagram2d, Handle, Marker } from "../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";
import { invertAffineV1 } from "../../../🕹️direct-manipulation/🛠️handle-drag-tool/🟦️.ts";

/** 🏷️ The id prefix of the grip of a marker, `marker:<diagram>:<marker>`. */
export const MARKER_GRIP_PREFIX_V1 = "marker:";

/** 🆔️ The handle id of a marker of a diagram. */
export const markerGripIdV1 = (diagram: Diagram2d, marker: Marker): string => `${MARKER_GRIP_PREFIX_V1}${diagram.id}:${marker.id}`;

/** 📐️ The coordinate of a marker that its bound field drives: the one on the first axis shown in the unit of the binding, else the first. */
export function boundCoordinateV1(diagram: Diagram2d, marker: Marker): number | undefined {
  if (marker.binding === null) return undefined;
  const unit = marker.binding.unit;
  const found = diagram.axes.findIndex((axis) => axis.unit === unit);
  return (found < 0 ? 0 : found) === 0 ? marker.at.x : marker.at.y;
}

/** 🕹️ The grip of a marker with a binding: a dimension-line handle at the marker writing the bound field, named by the label of the marker, limited to the domain of the axis shown in the unit of the field; `undefined` for a marker without a binding. */
export function markerGripV1(diagram: Diagram2d, marker: Marker): Handle | undefined {
  const binding = marker.binding;
  if (binding === null) return undefined;
  const domain = diagram.axes.find((axis) => axis.unit === binding.unit)?.domain ?? null;
  const limits = domain === null ? null : { min: Math.min(invertAffineV1(binding, domain.min), invertAffineV1(binding, domain.max)), max: Math.max(invertAffineV1(binding, domain.min), invertAffineV1(binding, domain.max)) };
  return {
    id: markerGripIdV1(diagram, marker),
    entity: diagram.focus ?? { kind: "group", id: diagram.id, path: "" },
    kind: { kind: "dimensionLine" },
    anchor: { x: marker.at.x, y: marker.at.y, z: 0 },
    grip: "diamond",
    writes: [binding],
    snap: { grid: null, increment: null, targets: [], catalogue: [], tolerance: 0 },
    limits,
    accessibleName: marker.label,
  };
}

/** 📍️ The drag parameter a marker grip rests at: the inverse of the field map at the bound coordinate. */
export function markerRestV1(diagram: Diagram2d, marker: Marker): number | undefined {
  const coordinate = boundCoordinateV1(diagram, marker);
  return marker.binding === null || coordinate === undefined ? undefined : invertAffineV1(marker.binding, coordinate);
}
