/** 📸️ Grid2dSnapshot schema — real facet mirror of the Rust `🦀️.rs` sibling. Tiles carry their own
 * 2D media (inline bitmap, inline vector, or a composed `s.stdio.semio@v1/image` child); the solved
 * assignment is an inference over this document and never appears here. */
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}

/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

export interface WfcColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface WfcPoint2 {
  x: number;
  y: number;
}

export type WfcPathSegment =
  | { kind: "moveTo"; to: WfcPoint2 }
  | { kind: "lineTo"; to: WfcPoint2 }
  | { kind: "quadTo"; ctrl: WfcPoint2; to: WfcPoint2 }
  | { kind: "cubicTo"; ctrl1: WfcPoint2; ctrl2: WfcPoint2; to: WfcPoint2 }
  | { kind: "close" };

export interface WfcVectorPath {
  segments: WfcPathSegment[];
  fill?: WfcColor;
  stroke?: WfcColor;
  strokeWidth: number;
}

export type WfcTileMedia2d =
  | { kind: "bitmap"; width: number; height: number; palette: WfcColor[]; pixels: string }
  | { kind: "vector"; paths: WfcVectorPath[] }
  | { kind: "image"; child: ArtifactChildHandle };

export interface WfcTile2d {
  id: string;
  label?: string;
  weight: number;
  media: WfcTileMedia2d;
}

export type WfcDirection2d = "LEFT" | "RIGHT" | "TOP" | "BOTTOM";

export interface WfcAdjacencyRule2d {
  id: string;
  tileAId: string;
  tileBId: string;
  direction: WfcDirection2d;
  allowed: boolean;
}

export interface WfcPinnedCell2d {
  x: number;
  y: number;
  tileId: string;
}

export interface WfcCell2d {
  x: number;
  y: number;
}

export interface Grid2dSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ seed: number;
  /** @state artifact */ width: number;
  /** @state artifact */ height: number;
  /** @state artifact */ cellWidth: number;
  /** @state artifact */ cellHeight: number;
  /** @state artifact */ periodicX: boolean;
  /** @state artifact */ periodicY: boolean;
  /** @state artifact */ tiles: WfcTile2d[];
  /** @state artifact */ rules: WfcAdjacencyRule2d[];
  /** @state artifact */ pinned: WfcPinnedCell2d[];
  /** @state artifact */ masked: WfcCell2d[];
}

export const WFC_GRID2D_DOCUMENT_SCHEMA = "s.wfc.grid2d";

/** 🧭️ The `(dx, dy)` step a direction takes from tile A to tile B — `y` grows downward. */
export function directionOffset(direction: WfcDirection2d): [number, number] {
  switch (direction) {
    case "LEFT":
      return [-1, 0];
    case "RIGHT":
      return [1, 0];
    case "TOP":
      return [0, -1];
    default:
      return [0, 1];
  }
}

/** 🔁️ The direction that carries B back to A. */
export function oppositeDirection(direction: WfcDirection2d): WfcDirection2d {
  switch (direction) {
    case "LEFT":
      return "RIGHT";
    case "RIGHT":
      return "LEFT";
    case "TOP":
      return "BOTTOM";
    default:
      return "TOP";
  }
}
