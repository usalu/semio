import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🧊️ wfc3d snapshot — the persisted WFC problem over an arbitrary 3d slot graph. Mirrors the Rust
 * `🦀️.rs` sibling; `TileMedia3d` is INTERNALLY tagged on `kind` (`#[value(tag = "kind")]`), so a
 * medium rides as `{ kind, ...its own fields }`. */

export const WFC3D_DOCUMENT_SCHEMA = "s.wfc.wfc3d";

export interface Color {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}

export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

export type TileMedia3d =
  | { kind: "mesh"; positions: Binary64[]; indices: number[]; color?: Color }
  | { kind: "meshChild"; child: ArtifactChildHandle };

export interface Tile {
  id: string;
  label?: string;
  weight: Binary64;
  media: TileMedia3d;
}

export interface Slot3d {
  id: string;
  x: Binary64;
  y: Binary64;
  z: Binary64;
  width: Binary64;
  height: Binary64;
  depth: Binary64;
  pinnedTileId?: string;
}

export interface SlotEdge {
  id: string;
  fromSlotId: string;
  toSlotId: string;
  relation: string;
}

/** ⛓️ One adjacency rule over an UNORDERED tile pair. The rules ARE the compatibility table — an
 * ALLOW-LIST — so a pair no rule mentions is forbidden. `allowed: false` always beats an admitting
 * rule, and an absent `relation` states the rule for every relation at once. */
export interface GraphRule {
  id: string;
  tileAId: string;
  tileBId: string;
  relation?: string;
  allowed: boolean;
}

export interface Wfc3dSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  seed: bigint;
  /** @state artifact */
  slots: Slot3d[];
  /** @state artifact */
  edges: SlotEdge[];
  /** @state artifact */
  tiles: Tile[];
  /** @state artifact */
  rules: GraphRule[];
}
