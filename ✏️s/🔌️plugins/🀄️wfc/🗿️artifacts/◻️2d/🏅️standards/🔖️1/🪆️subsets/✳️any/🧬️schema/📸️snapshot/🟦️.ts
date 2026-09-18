// 🌊️ WFC 2D snapshot — the TypeScript twin of `🦀️.rs`, ported field by field from the Rust records
// (never generated). The JSON Schema leaf beside this file is normative; this mirrors it.

/** 🎨 One straight-alpha sRGB colour, 0–255 per channel. */
export type Wfc2dColor = { readonly r: number; readonly g: number; readonly b: number; readonly a: number };

/** ✏️ One path command in TILE SPACE (0..1 on both axes), externally tagged. */
export type Wfc2dPathSegment =
  | "Close"
  | { readonly Move: { readonly to: readonly [number, number] } }
  | { readonly Line: { readonly to: readonly [number, number] } }
  | { readonly Quad: { readonly ctrl: readonly [number, number]; readonly to: readonly [number, number] } }
  | { readonly Cubic: { readonly ctrl1: readonly [number, number]; readonly ctrl2: readonly [number, number]; readonly to: readonly [number, number] } };

/** 🖍️ One filled/stroked subpath of a vector tile. */
export type Wfc2dVectorPath = {
  readonly segments: readonly Wfc2dPathSegment[];
  readonly fill?: Wfc2dColor;
  readonly stroke?: Wfc2dColor;
  readonly strokeWidth: number;
};

/** 🖼️ What a tile looks like, externally tagged. */
export type Wfc2dTileMedia =
  | "Empty"
  | { readonly Bitmap: { readonly width: number; readonly height: number; readonly palette: readonly Wfc2dColor[]; readonly pixels: string } }
  | { readonly Vector: { readonly paths: readonly Wfc2dVectorPath[] } }
  | { readonly Image: { readonly child: { readonly childId: string; readonly target: unknown } } };

/** 🀄️ One placeable tile — the WFC pattern alphabet. */
export type Wfc2dTile = { readonly id: string; readonly label?: string; readonly weight: number; readonly media: Wfc2dTileMedia };

/** 📍 One position the solver must fill — a free rectangle, never a grid cell. */
export type Wfc2dSlot = {
  readonly id: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly pinnedTileId?: string;
};

/** 🔗 One adjacency edge between two slots, naming its relation class. */
export type Wfc2dSlotEdge = { readonly id: string; readonly fromSlotId: string; readonly toSlotId: string; readonly relation: string };

/** ⛓️ One adjacency permission between two tile ids; `relation` absent means every class. */
export type Wfc2dRule = { readonly id: string; readonly tileAId: string; readonly tileBId: string; readonly relation?: string; readonly allowed: boolean };

/** 🌊️ The persisted WFC problem. Collections are kept in canonical ascending `id` order. */
export type Wfc2dSnapshot = {
  readonly schema: string;
  readonly seed: number;
  readonly slots: readonly Wfc2dSlot[];
  readonly edges: readonly Wfc2dSlotEdge[];
  readonly tiles: readonly Wfc2dTile[];
  readonly rules: readonly Wfc2dRule[];
};

export const WFC_2D_DOCUMENT_SCHEMA = "s.wfc.wfc2d";
export const WFC_2D_DEFAULT_RELATION = "adjacent";

/** 🌱️ The empty document this subset boots with. */
export function defaultWfc2dSnapshot(): Wfc2dSnapshot {
  return { schema: WFC_2D_DOCUMENT_SCHEMA, seed: 0, slots: [], edges: [], tiles: [], rules: [] };
}

/** 🔢 The position `id` occupies in an ascending-`id` collection — the Rust `ordered_index` twin. */
export function orderedIndex<T>(items: readonly T[], id: string, key: (item: T) => string): number {
  const at = items.findIndex((item) => key(item) > id);
  return at === -1 ? items.length : at;
}

/** 🔗 Every distinct edge relation string, ascending — the model's relation universe. */
export function wfc2dRelations(snapshot: Wfc2dSnapshot): readonly string[] {
  return [...new Set(snapshot.edges.map((edge) => edge.relation))].sort();
}
