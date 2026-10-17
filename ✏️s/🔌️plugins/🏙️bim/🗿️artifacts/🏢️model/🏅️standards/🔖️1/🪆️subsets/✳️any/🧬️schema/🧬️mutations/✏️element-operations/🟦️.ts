/** ✏️ BIM model direct mutations that edit selected elements: transform, copy, place, rename, properties, classification, storey and phase. */
import type { Phase, Point2, PropertyValue } from "../../🟦️.ts";

export interface MoveElements {
  mutation: "moveElements";
  ids: string[];
  vector: Point2;
}

export interface RotateElements {
  mutation: "rotateElements";
  ids: string[];
  pivot: Point2;
  angle: number;
}

export interface DeleteElements {
  mutation: "deleteElements";
  ids: string[];
}

export interface RenameElement {
  mutation: "renameElement";
  id: string;
  name: string;
}

export interface SetElementProperty {
  mutation: "setElementProperty";
  id: string;
  pset: string;
  property: string;
  value: PropertyValue;
}

export interface RemoveElementProperty {
  mutation: "removeElementProperty";
  id: string;
  pset: string;
  property: string;
}

export interface SetElementClassification {
  mutation: "setElementClassification";
  id: string;
  system: string;
  code: string;
}

export interface RemoveElementClassification {
  mutation: "removeElementClassification";
  id: string;
  system: string;
}

export interface PlaceElements {
  mutation: "placeElements";
  placements: Record<string, unknown>;
}

export interface SetElementStorey {
  mutation: "setElementStorey";
  id: string;
  storey: string;
}

export interface SetElementPhase {
  mutation: "setElementPhase";
  id: string;
  phase: Phase;
}

export interface CopyElements {
  mutation: "copyElements";
  ids: string[];
  vector: Point2;
  prefix: string;
}

export interface MirrorElements {
  mutation: "mirrorElements";
  ids: string[];
  line_start: Point2;
  line_end: Point2;
  prefix?: string;
}

export interface ArrayElements {
  mutation: "arrayElements";
  ids: string[];
  prefix: string;
  pattern: Record<string, unknown>;
}

export interface AlignElements {
  mutation: "alignElements";
  ids: string[];
  axis: string;
  edge: string;
  target: number;
}
