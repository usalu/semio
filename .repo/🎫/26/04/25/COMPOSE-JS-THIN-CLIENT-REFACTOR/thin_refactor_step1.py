# Temporary script: thin-client mechanical edits for compose/js/index.ts (run from repo root).
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]  # c:/git/compose
INDEX = ROOT / "compose/js/index.ts"


def replace_export_class(src: str, name: str, new_block: str) -> str:
    m = re.search(rf"^export class {re.escape(name)}(?:\s+implements\s+[\w.]+)?\s*\{{", src, re.M)
    if not m:
        raise SystemExit(f"missing class {name}")
    start = m.start()
    i = m.end() - 1  # position of {
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


def main() -> None:
    s = INDEX.read_text(encoding="utf-8")

    thin_type = r'''export class Type {
  id!: string;
  name!: string;
  families?: FamilyId[];
  isAbstract?: boolean;
  folder?: string;
  representations?: Representation[];
  connectors?: Connector[];
  props?: Prop[];
  stock?: number;
  virtual?: boolean;
  unit?: string;
  createdAt?: string;
  updatedAt?: string;
  location?: LocationId;
  authors?: AuthorId[];
  concepts?: ConceptId[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  lifecycle?: EntityLifecycle;
  deletedByUserId?: string;
  deletedByDisplayName?: string;
  deletedAt?: string;
  deletedInChangeId?: string;

  constructor(plain: TypePlain) {
    const p = TypeSchema.parse(plain);
    Object.assign(this, p);
    this.representations = p.representations?.map((m) => new Representation(m));
    this.connectors = p.connectors?.map((c) => new Connector(c));
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(plain: TypePlain): Type {
    return new Type(plain);
  }

  findConnector(connectorId: string): Connector | undefined {
    return this.connectors?.find((c) => c.id === connectorId);
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Type {
    return Type.fromPlain(TypeSchema.parse(JSON.parse(json)));
  }

  toPlain(): TypePlain {
    return TypeSchema.parse({ ...(this as unknown as TypePlain) });
  }

  toMeta(): TypeMeta {
    return TypeMetaSchema.parse(this.toPlain());
  }

  toShallow(): TypeShallow {
    const plain = this.toPlain();
    return TypeShallowSchema.parse({
      ...plain,
      representations: this.representations?.map((m) => RepresentationMetaSchema.parse(m.toPlain())),
      connectors: this.connectors?.map((c) => ConnectorMetaSchema.parse(c.toPlain())),
      props: this.props?.map((p) => PropMetaSchema.parse(p.toPlain())),
      attributes: this.attributes?.map((a) => AttributeMetaSchema.parse(a.toPlain())),
    });
  }

  static createId(id: string): TypeId {
    return { id };
  }

  static areSameId(a: TypeId, b: TypeId): boolean {
    return a.id === b.id;
  }
}
'''

    thin_layer = r'''export class Layer implements LayerPlain {
  id!: string;
  path!: string;
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
  description?: string;
  attributes?: Attribute[];
  constructor(plain: LayerPlain) {
    const p = LayerSchema.parse(plain);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromPlain(plain: LayerPlain): Layer {
    return new Layer(plain);
  }
  toPlain(): LayerPlain {
    return LayerSchema.parse(this as unknown as LayerPlain);
  }
  serialize(): string {
    return JSON.stringify(this.toPlain());
  }
  static deserialize(json: string): Layer {
    return new Layer(LayerSchema.parse(JSON.parse(json)));
  }
  static createId(id: string): LayerId {
    return { id };
  }
  static areSameId(a: LayerId, b: LayerId): boolean {
    return a.id === b.id;
  }
}
'''

    thin_piece = r'''export class Piece {
  id!: string;
  name?: string;
  type?: TypeId;
  design?: DesignId;
  plane?: Plane;
  center?: Coordinate;
  scale?: number;
  mirrorPlane?: Plane;
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
  description?: string;
  props?: Prop[];
  attributes?: Attribute[];

  constructor(plain: PiecePlain) {
    const p = PieceSchema.parse(plain);
    Object.assign(this, p);
    this.plane = p.plane ? new Plane(p.plane) : undefined;
    this.center = p.center ? new Coordinate(p.center) : undefined;
    this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined;
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }

  static fromPlain(plain: PiecePlain): Piece {
    return new Piece(plain);
  }

  wireTypeId(): TypeId | undefined {
    return this.type;
  }

  wireDesignAsPieceId(): DesignId | undefined {
    return this.design;
  }

  serialize(): string {
    return JSON.stringify(this.toPlain());
  }

  static deserialize(json: string): Piece {
    return new Piece(PieceSchema.parse(JSON.parse(json)));
  }

  toPlain(): PiecePlain {
    return PieceSchema.parse({
      ...(this as unknown as PiecePlain),
      type: this.wireTypeId(),
      design: this.wireDesignAsPieceId(),
    });
  }

  toMeta(): PieceMeta {
    return PieceMetaSchema.parse(this.toPlain());
  }

  toShallow(): PieceShallow {
    const plain = this.toPlain();
    return PieceShallowSchema.parse(plain);
  }

  static createId(id: string): PieceId {
    return { id };
  }

  static areSameId(a: PieceId, b: PieceId): boolean {
    return a.id === b.id;
  }
}
'''

    s = replace_export_class(s, "Type", thin_type)
    s = replace_export_class(s, "Layer", thin_layer)
    s = replace_export_class(s, "Piece", thin_piece)

    INDEX.write_text(s, encoding="utf-8")
    print("ok: Type, Layer, Piece replaced")


if __name__ == "__main__":
    main()
