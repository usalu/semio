import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
// 🌊️ WFC 2D snapshot — the TypeScript twin of `🦀️.rs`, ported field by field from the Rust records
// (never generated). The JSON Schema leaf beside this file is normative; this mirrors it.

/** 🎨 One straight-alpha sRGB colour, 0–255 per channel. */
export type Wfc2dColor = { readonly r: number; readonly g: number; readonly b: number; readonly a: number };

/** ✏️ One path command in TILE SPACE (0..1 on both axes), externally tagged. */
export type Wfc2dPathSegment =
  | "Close"
  | { readonly Move: { readonly to: readonly [Binary64, Binary64] } }
  | { readonly Line: { readonly to: readonly [Binary64, Binary64] } }
  | { readonly Quad: { readonly ctrl: readonly [Binary64, Binary64]; readonly to: readonly [Binary64, Binary64] } }
  | { readonly Cubic: { readonly ctrl1: readonly [Binary64, Binary64]; readonly ctrl2: readonly [Binary64, Binary64]; readonly to: readonly [Binary64, Binary64] } };

/** 🖍️ One filled/stroked subpath of a vector tile. */
export type Wfc2dVectorPath = {
  readonly segments: readonly Wfc2dPathSegment[];
  readonly fill?: Wfc2dColor;
  readonly stroke?: Wfc2dColor;
  readonly strokeWidth: Binary64;
};

/** 🖼️ What a tile looks like, externally tagged. */
export type Wfc2dTileMedia =
  | "Empty"
  | { readonly Bitmap: { readonly width: number; readonly height: number; readonly palette: readonly Wfc2dColor[]; readonly pixels: string } }
  | { readonly Vector: { readonly paths: readonly Wfc2dVectorPath[] } }
  | { readonly Image: { readonly child: { readonly childId: string; readonly target: {readonly artifactId:string;readonly dialect:{readonly artifactKind:string;readonly standard:string;readonly subset:string}} } } };

/** 🀄️ One placeable tile — the WFC pattern alphabet. */
export type Wfc2dTile = { readonly id: string; readonly label?: string; readonly weight: Binary64; readonly media: Wfc2dTileMedia };

/** 📍 One position the solver must fill — a free rectangle, never a grid cell. */
export type Wfc2dSlot = {
  readonly id: string;
  readonly x: Binary64;
  readonly y: Binary64;
  readonly width: Binary64;
  readonly height: Binary64;
  readonly pinnedTileId?: string;
};

/** 🔗 One adjacency edge between two slots, naming its relation class. */
export type Wfc2dSlotEdge = { readonly id: string; readonly fromSlotId: string; readonly toSlotId: string; readonly relation: string };

/** ⛓️ One adjacency permission between two tile ids; `relation` absent means every class. */
export type Wfc2dRule = { readonly id: string; readonly tileAId: string; readonly tileBId: string; readonly relation?: string; readonly allowed: boolean };

/** 🌊️ The persisted WFC problem. Collections are kept in canonical ascending `id` order. */
export type Wfc2dSnapshot = {
  readonly schema: string;
  readonly seed: bigint;
  readonly slots: readonly Wfc2dSlot[];
  readonly edges: readonly Wfc2dSlotEdge[];
  readonly tiles: readonly Wfc2dTile[];
  readonly rules: readonly Wfc2dRule[];
};

export const WFC_2D_DOCUMENT_SCHEMA = "s.wfc.wfc2d";
export const WFC_2D_DEFAULT_RELATION = "adjacent";

/** 🌱️ The empty document this subset boots with. */
export function defaultWfc2dSnapshot(): Wfc2dSnapshot {
  return { schema: WFC_2D_DOCUMENT_SCHEMA, seed: 0n, slots: [], edges: [], tiles: [], rules: [] };
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

/** 🧮️ Compares explicit owned media fields, including exact scalar words. */
export function wfc2dMediaEqual(a:Wfc2dTileMedia,b:Wfc2dTileMedia):boolean{
 const color=(x:Wfc2dColor|undefined,y:Wfc2dColor|undefined)=>x===undefined?y===undefined:y!==undefined&&x.r===y.r&&x.g===y.g&&x.b===y.b&&x.a===y.a;
 const point=(x:readonly[Binary64,Binary64],y:readonly[Binary64,Binary64])=>x[0].bits===y[0].bits&&x[1].bits===y[1].bits;
 const segment=(x:Wfc2dPathSegment,y:Wfc2dPathSegment):boolean=>{if(typeof x==='string'||typeof y==='string')return x===y;if('Move'in x)return'Move'in y&&point(x.Move.to,y.Move.to);if('Line'in x)return'Line'in y&&point(x.Line.to,y.Line.to);if('Quad'in x)return'Quad'in y&&point(x.Quad.to,y.Quad.to)&&point(x.Quad.ctrl,y.Quad.ctrl);return'Cubic'in y&&point(x.Cubic.to,y.Cubic.to)&&point(x.Cubic.ctrl1,y.Cubic.ctrl1)&&point(x.Cubic.ctrl2,y.Cubic.ctrl2);};
 if(typeof a==='string'||typeof b==='string')return a===b;
 if('Bitmap'in a){if(!('Bitmap'in b))return false;const x=a.Bitmap,y=b.Bitmap;return x.width===y.width&&x.height===y.height&&x.pixels===y.pixels&&x.palette.length===y.palette.length&&x.palette.every((v,i)=>color(v,y.palette[i]));}
 if('Image'in a){if(!('Image'in b))return false;const x=a.Image.child,y=b.Image.child;return x.childId===y.childId&&x.target.artifactId===y.target.artifactId&&x.target.dialect.artifactKind===y.target.dialect.artifactKind&&x.target.dialect.standard===y.target.dialect.standard&&x.target.dialect.subset===y.target.dialect.subset;}
 if(!('Vector'in b))return false;const x=a.Vector.paths,y=b.Vector.paths;return x.length===y.length&&x.every((p,i)=>{const q=y[i]!;return p.strokeWidth.bits===q.strokeWidth.bits&&color(p.fill,q.fill)&&color(p.stroke,q.stroke)&&p.segments.length===q.segments.length&&p.segments.every((v,j)=>segment(v,q.segments[j]!));});
}
