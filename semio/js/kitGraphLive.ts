/**
 * `executeRead` facades: wires nested `readKit*Commands` JSON batches so
 * @semio/react hooks can read domain state without duplicating path wiring.
 */

import type {
  IdDto,
  ReadCommandBatch,
  ReadCommandBatchResult,
  ReadDesignCommand,
  ReadDesignCommandOutput,
  ReadKitCommand,
  ReadKitCommandOutput,
  ReadPieceCommand,
  ReadPieceCommandOutput,
  ReadTypeCommand,
  ReadTypeCommandOutput,
} from "./readCommandTypes";

/** Any client exposing `executeRead` (e.g. `KitStoreClient`). */
export type KitExecuteRead = {
  executeRead(commands: ReadCommandBatch): Promise<ReadCommandBatchResult>;
};

export function idDto(id: string): IdDto {
  return { id };
}

function assertSingleResult(results: ReadCommandBatchResult): ReadKitCommandOutput {
  if (results.length !== 1) {
    throw new Error(`read batch: expected 1 result, got ${results.length}`);
  }
  return results[0]!;
}

/** Top-level `ReadKitCommand` (single item batch). */
export async function readKit(
  client: KitExecuteRead,
  command: ReadKitCommand
): Promise<ReadKitCommandOutput> {
  return assertSingleResult(await client.executeRead([command]));
}

export async function readKitDesign(
  client: KitExecuteRead,
  designId: string,
  command: ReadDesignCommand
): Promise<ReadDesignCommandOutput> {
  const out = await readKit(client, {
    readKitDesignCommands: {
      id: idDto(designId),
      commands: [command],
    },
  });
  if (!("readKitDesignCommands" in out) || out.readKitDesignCommands == null) {
    throw new Error("read path: expected readKitDesignCommands");
  }
  return out.readKitDesignCommands.results[0]!;
}

export async function readKitDesignPiece(
  client: KitExecuteRead,
  designId: string,
  pieceId: string,
  command: ReadPieceCommand
): Promise<ReadPieceCommandOutput> {
  const d0 = await readKitDesign(client, designId, {
    readDesignPieceCommands: {
      id: idDto(pieceId),
      commands: [command],
    },
  });
  if (!("readDesignPieceCommands" in d0) || d0.readDesignPieceCommands == null) {
    throw new Error("read path: expected readDesignPieceCommands");
  }
  return d0.readDesignPieceCommands.results[0]!;
}

export async function readKitType(
  client: KitExecuteRead,
  typeId: string,
  command: ReadTypeCommand
): Promise<ReadTypeCommandOutput> {
  const out = await readKit(client, {
    readKitTypeCommands: {
      id: idDto(typeId),
      commands: [command],
    },
  });
  if (!("readKitTypeCommands" in out) || out.readKitTypeCommands == null) {
    throw new Error("read path: expected readKitTypeCommands");
  }
  return out.readKitTypeCommands.results[0]!;
}

/** `executeRead` for one piece field (nested `readKitDesign` → `readDesignPiece` → field). */
export class LivePieceView {
  constructor(
    private readonly client: KitExecuteRead,
    readonly designId: string,
    readonly pieceId: string
  ) {}

  read(command: ReadPieceCommand): Promise<ReadPieceCommandOutput> {
    return readKitDesignPiece(this.client, this.designId, this.pieceId, command);
  }

  async readFlatPlane(): Promise<unknown> {
    const out = await this.read({ readPieceFlatPlaneCommand: null });
    if (!("readPieceFlatPlaneCommand" in out) || out.readPieceFlatPlaneCommand == null) {
      throw new Error("readPieceFlatPlaneCommand: missing output");
    }
    return out.readPieceFlatPlaneCommand.flatPlane;
  }

  async readFlatCenter(): Promise<unknown> {
    const out = await this.read({ readPieceFlatCenterCommand: null });
    if (!("readPieceFlatCenterCommand" in out) || out.readPieceFlatCenterCommand == null) {
      throw new Error("readPieceFlatCenterCommand: missing output");
    }
    return out.readPieceFlatCenterCommand.flatCenter;
  }

  async readParentConnectionFull(): Promise<unknown | null | undefined> {
    const out = await this.read({ readPieceParentConnectionFullCommand: null });
    if (!("readPieceParentConnectionFullCommand" in out) || out.readPieceParentConnectionFullCommand == null) {
      throw new Error("readPieceParentConnectionFullCommand: missing output");
    }
    return out.readPieceParentConnectionFullCommand.connection;
  }
}

/** `executeRead` for design-scoped fields (e.g. clusterable groups, quality sum). */
export class LiveDesignView {
  constructor(
    private readonly client: KitExecuteRead,
    readonly designId: string
  ) {}

  read(command: ReadDesignCommand): Promise<ReadDesignCommandOutput> {
    return readKitDesign(this.client, this.designId, command);
  }

  async readClusterableGroups(selection: ReadonlyArray<string>): Promise<ReadonlyArray<ReadonlyArray<IdDto>>> {
    const out = await this.read({
      readDesignClusterableGroupsCommand: { selection: selection.map(idDto) },
    });
    if (!("readDesignClusterableGroupsCommand" in out) || out.readDesignClusterableGroupsCommand == null) {
      throw new Error("readDesignClusterableGroupsCommand: missing output");
    }
    return out.readDesignClusterableGroupsCommand.groups;
  }

  async readIncludedDesigns(): Promise<ReadonlyArray<unknown>> {
    const out = await this.read({ readDesignIncludedDesignsCommand: null });
    if (!("readDesignIncludedDesignsCommand" in out) || out.readDesignIncludedDesignsCommand == null) {
      throw new Error("readDesignIncludedDesignsCommand: missing output");
    }
    return out.readDesignIncludedDesignsCommand.designs;
  }

  async readQualitySum(qualityId: string): Promise<number> {
    const out = await this.read({
      readDesignQualitySumCommand: { qualityId: idDto(qualityId) },
    });
    if (!("readDesignQualitySumCommand" in out) || out.readDesignQualitySumCommand == null) {
      throw new Error("readDesignQualitySumCommand: missing output");
    }
    return out.readDesignQualitySumCommand.sum;
  }

  async readReplaceableCatalog(selection: ReadonlyArray<string>): Promise<{ types: string[]; designs: string[] }> {
    const out = await this.read({
      readDesignReplaceableCatalogCommand: { selection: selection.map(idDto) },
    });
    if (!("readDesignReplaceableCatalogCommand" in out) || out.readDesignReplaceableCatalogCommand == null) {
      throw new Error("readDesignReplaceableCatalogCommand: missing output");
    }
    const row = out.readDesignReplaceableCatalogCommand;
    return {
      types: row.types.map((t) => t.id),
      designs: row.designs.map((d) => d.id),
    };
  }

  async readIncludedDesignIds(): Promise<string[]> {
    const out = await this.read({ readDesignIncludedDesignIdsCommand: null });
    if (!("readDesignIncludedDesignIdsCommand" in out) || out.readDesignIncludedDesignIdsCommand == null) {
      throw new Error("readDesignIncludedDesignIdsCommand: missing output");
    }
    return out.readDesignIncludedDesignIdsCommand.designIds.map((d) => d.id);
  }
}

export class LiveTypeView {
  constructor(
    private readonly client: KitExecuteRead,
    readonly typeId: string
  ) {}

  read(command: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
    return readKitType(this.client, this.typeId, command);
  }

  async readBestRepresentation(tagIds: ReadonlyArray<string>): Promise<unknown | null | undefined> {
    const out = await this.read({
      readTypeBestRepresentationCommand: { tagIds: [...tagIds] },
    });
    if (!("readTypeBestRepresentationCommand" in out) || out.readTypeBestRepresentationCommand == null) {
      throw new Error("readTypeBestRepresentationCommand: missing output");
    }
    return out.readTypeBestRepresentationCommand.representation;
  }
}

/**
 * Root of live read facades. Construct with a `KitStoreClient` and navigate
 * `piece` / `design` / `type` for `executeRead` calls.
 */
export class LiveKitRoot {
  constructor(readonly client: KitExecuteRead) {}

  piece(designId: string, pieceId: string): LivePieceView {
    return new LivePieceView(this.client, designId, pieceId);
  }

  design(designId: string): LiveDesignView {
    return new LiveDesignView(this.client, designId);
  }

  type(typeId: string): LiveTypeView {
    return new LiveTypeView(this.client, typeId);
  }

  async readColoredConnectors(): Promise<ReadonlyArray<unknown>> {
    const out = await readKit(this.client, { readKitColoredConnectorsCommand: null });
    if (!("readKitColoredConnectorsCommand" in out) || out.readKitColoredConnectorsCommand == null) {
      throw new Error("readKitColoredConnectorsCommand: missing output");
    }
    return out.readKitColoredConnectorsCommand.rows;
  }
}
