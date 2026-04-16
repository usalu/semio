# Step 2: thin Design, delete KitImpl monolith (marker-based), add thin Kit + fix KitLike / DTO.
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
INDEX = ROOT / "semio/js/index.ts"


def replace_export_class(src: str, name: str, new_block: str) -> str:
    m = re.search(rf"^export class {re.escape(name)}(?:\s+implements\s+[\w.]+)?\s*\{{", src, re.M)
    if not m:
        raise SystemExit(f"missing class {name}")
    start = m.start()
    i = m.end() - 1
    depth = 0
    while i < len(src):
        c = src[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                i += 1
                break
        i += 1
    return src[:start] + new_block + src[i:]


THIN_DESIGN = r'''export class Design {
  id!: string;
  name!: string;
  families?: FamilyId[];
  isAbstract?: boolean;
  folder?: string;
  pieces?: Piece[];
  _connections?: Connection[];
  stats?: Stat[];
  props?: Prop[];
  layers?: Layer[];
  activeLayer?: LayerId;
  groups?: Group[];
  canScale?: boolean;
  canMirror?: boolean;
  unit?: string;
  location?: LocationId;
  authors?: AuthorId[];
  concepts?: ConceptId[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;

  constructor(plain: DesignPlain | Design) {
    const wire: DesignPlain = plain instanceof Design ? plain.toPlain() : plain;
    const p = DesignSchema.parse(wire);
    const { connections: _wcon, pieces: _wp, ...rest } = p;
    Object.assign(this, rest);
    this.pieces = p.pieces?.map((x) => new Piece(x));
    this._connections = p.connections?.map((x) => new Connection(x, this));
    this.stats = p.stats?.map((x) => new Stat(x));
    this.props = p.props?.map((x) => new Prop(x));
    this.layers = p.layers?.map((x) => new Layer(x));
    this.groups = p.groups?.map((x) => new Group(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(plain: DesignPlain): Design {
    return new Design(plain);
  }

  findPiece(lookup: string | { name: string }): Piece | undefined {
    const key = typeof lookup === "string" ? lookup : lookup.name;
    const byId = this.pieces?.find((p) => p.id === key);
    if (byId) return byId;
    return this.pieces?.find((p) => p.name === key);
  }

  requirePiece(lookup: string | { name: string }): Piece {
    const piece = this.findPiece(lookup);
    const label = typeof lookup === "string" ? lookup : lookup.name;
    if (!piece) throw new Error(`Piece ${label} not found in design ${this.name}`);
    return piece;
  }

  findConnection(connectionId: string): Connection | undefined {
    return this._connections?.find((c) => c.id === connectionId);
  }

  requireConnection(connectionId: string): Connection {
    return findConnection(this._connections ?? [], connectionId);
  }

  getPieces(): readonly Piece[] {
    return this.pieces ?? [];
  }

  getConnections(): readonly Connection[] {
    return this._connections ?? [];
  }

  connections(): readonly Connection[] {
    return this.getConnections();
  }

  toPlain(): DesignPlain {
    return DesignSchema.parse({
      ...(this as unknown as DesignPlain),
      pieces: this.pieces?.map((x) => x.toPlain()),
      connections: this._connections?.map((x) => x.toPlain()),
      stats: this.stats?.map((x) => x.toPlain()),
      props: this.props?.map((x) => x.toPlain()),
      layers: this.layers?.map((x) => x.toPlain()),
      groups: this.groups?.map((x) => x.toPlain()),
      attributes: this.attributes?.map((x) => x.toPlain()),
    });
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Design {
    return new Design(DesignSchema.parse(JSON.parse(json)));
  }

  toMeta(): DesignMeta {
    return DesignMetaSchema.parse(this.toPlain());
  }

  toShallow(): DesignShallow {
    const plain = this.toPlain();
    return DesignShallowSchema.parse({
      ...plain,
      pieces: this.pieces?.map((p) => PieceMetaSchema.parse(p.toPlain())),
      connections: this._connections?.map((c) => ConnectionMetaSchema.parse(c.toPlain())),
      stats: this.stats?.map((s) => StatMetaSchema.parse(s.toPlain())),
      props: this.props?.map((p) => PropMetaSchema.parse(p.toPlain())),
      layers: this.layers?.map((l) => LayerMetaSchema.parse(l.toPlain())),
      groups: this.groups?.map((g) => GroupMetaSchema.parse(g.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetaSchema.parse(a.toPlain())),
    });
  }

  static createId(id: string): DesignId {
    return { id };
  }

  static areSameId(a: DesignId, b: DesignId): boolean {
    return a.id === b.id;
  }
}
'''

THIN_KIT = r'''
// #region KitEntity
/**
 * Thin {@link KitData} view: serialization + plain DTOs only. Domain mutations use {@link KitStoreClient} (WASM).
 */
export class Kit {
  id!: string;
  name!: string;
  version?: string;
  types?: Type[];
  designs?: Design[];
  tags?: Tag[];
  concepts?: Concept[];
  families?: Family[];
  qualities?: Quality[];
  files?: File[];
  folders?: Folder[];
  authors?: Author[];
  remote?: string;
  homepage?: string;
  license?: string;
  preview?: string;
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;

  constructor(data: KitData) {
    const p = KitSchema.parse(data);
    Object.assign(this, p);
    this.types = p.types?.map((t) => new Type(t));
    this.designs = p.designs?.map((d) => new Design(d));
    this.tags = p.tags?.map((t) => new Tag(t));
    this.concepts = p.concepts?.map((c) => new Concept(c));
    this.families = p.families?.map((f) => new Family(f));
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.files = p.files?.map((f) => new File(f));
    this.folders = p.folders?.map((f) => new Folder(f));
    this.authors = p.authors?.map((a) => new Author(a));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(data: KitData): Kit {
    return new Kit(data);
  }

  toPlain(): KitData {
    return KitSchema.parse({
      ...(this as unknown as KitData),
      types: this.types?.map((t) => t.toPlain()),
      designs: this.designs?.map((d) => d.toPlain()),
      tags: this.tags?.map((t) => t.toPlain()),
      concepts: this.concepts?.map((c) => c.toPlain()),
      families: this.families?.map((f) => f.toPlain()),
      qualities: this.qualities?.map((q) => q.toPlain()),
      files: this.files?.map((f) => f.toPlain()),
      folders: this.folders?.map((f) => f.toPlain()),
      authors: this.authors?.map((a) => a.toPlain()),
      attributes: this.attributes?.map((a) => a.toPlain()),
    });
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Kit {
    return Kit.fromPlain(KitSchema.parse(JSON.parse(json)));
  }

  toJSON(): KitData {
    return this.toPlain();
  }

  static createId(id: string): KitId {
    return { id };
  }

  static areSameId(a: KitId, b: KitId): boolean {
    return a.id === b.id;
  }
}

/**
 * Wire DTO or thin {@link Kit} instance.
 */
export type KitLike = Kit | KitData;
// #endregion KitEntity
'''


def delete_kitimpl_monolith(s: str) -> str:
    kit_like = "export type KitLike = KitImpl | KitData;"
    if kit_like not in s:
        raise SystemExit("expected KitLike = KitImpl line")
    start = s.index(kit_like)
    sa = s.index("Storage-agnostic kit store contracts MUST be defined here.")
    end = s.rfind("\n// #region", 0, sa)
    if end < 0 or end <= start:
        raise SystemExit("could not find KitStore region start before Storage-agnostic")
    return s[:start] + THIN_KIT.lstrip("\n") + s[end:]


def main() -> None:
    s = INDEX.read_text(encoding="utf-8")
    s = replace_export_class(s, "Design", THIN_DESIGN)
    s = delete_kitimpl_monolith(s)
    s = s.replace("  kit: KitImpl;", "  kit: Kit;")
    s = s.replace("  replace(next: KitImpl, meta?: { origin?: string }): void;", "  replace(next: Kit, meta?: { origin?: string }): void;")
    s = s.replace(
        "  const dto = JSON.parse(JSON.stringify(asKitInstance(opts.initialKit).toJSON())) as ReturnType<KitImpl[\"toJSON\"]>;",
        "  const dto = JSON.parse(JSON.stringify(opts.initialKit)) as KitData;",
    )
    INDEX.write_text(s, encoding="utf-8")
    print("ok: Design + Kit monolith + KitLike + DTO + KitStore types")


if __name__ == "__main__":
    main()
