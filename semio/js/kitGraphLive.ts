/**
 * `executeRead` facades: wires nested `readKit*Commands` JSON batches so
 * @semio/react hooks can read domain state without duplicating path wiring.
 * GraphQL: reads go through `kitStore.readKitCommands`; VCS/backbone through `kitStoreExecute`.
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

//#region 🔖KitGraphqlWire

/** WASM [`KitStoreHandle::execute`] shape: streams JSON-stringified GraphQL responses. */
export type KitGraphqlHandle = {
  execute(requestJson: string, onMessage: (line: string) => void): Promise<void>;
};

export async function kitGraphqlRun(
  handle: KitGraphqlHandle,
  body: { query: string; variables?: Record<string, unknown>; operationName?: string },
): Promise<unknown[]> {
  const out: unknown[] = [];
  await handle.execute(JSON.stringify(body), (line: string) => {
    out.push(JSON.parse(line));
  });
  return out;
}

export function kitGraphqlFirstData(msgs: unknown[]): Record<string, unknown> {
  for (const m of msgs) {
    if (m == null || typeof m !== "object") continue;
    const r = m as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
    if (Array.isArray(r.errors) && r.errors.length > 0) {
      throw new Error(r.errors[0]?.message ?? "GraphQL error");
    }
    if (r.data != null && typeof r.data === "object") {
      return r.data as Record<string, unknown>;
    }
  }
  throw new Error("kitGraphql: no data in response");
}

export async function kitGraphqlExecuteRead(handle: KitGraphqlHandle, batch: ReadCommandBatch): Promise<ReadCommandBatchResult> {
  const q = `query ($batch: JSON!) { kitStore { readKitCommands(batch: $batch) } }`;
  const msgs = await kitGraphqlRun(handle, { query: q, variables: { batch: [...batch] } });
  const data = kitGraphqlFirstData(msgs);
  const store = data.kitStore as Record<string, unknown> | undefined;
  const inner = store?.readKitCommands;
  if (!Array.isArray(inner)) throw new Error("readKitCommands: expected array");
  return inner as ReadCommandBatchResult;
}

export async function kitGraphqlExecuteStoreCommand(handle: KitGraphqlHandle, cmd: unknown): Promise<unknown> {
  const q = `mutation ($command: JSON!) { kitStoreExecute(command: $command) }`;
  const msgs = await kitGraphqlRun(handle, { query: q, variables: { command: cmd } });
  const data = kitGraphqlFirstData(msgs);
  if (!("kitStoreExecute" in data)) throw new Error("kitGraphql: missing kitStoreExecute");
  return data.kitStoreExecute;
}

/** Fan-out kit events from `subscription { eventStream }`; cancel stops invoking `sink` (underlying stream may continue). */
export function kitGraphqlSubscribeLoop(handle: KitGraphqlHandle, sink: (payload: unknown) => void): () => void {
  let cancelled = false;
  void handle
    .execute(JSON.stringify({ query: "subscription { eventStream }" }), (line: string) => {
      if (cancelled) return;
      try {
        const msg = JSON.parse(line) as { data?: { eventStream?: unknown } | null; errors?: unknown[] };
        if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return;
        if (msg.data && "eventStream" in msg.data && msg.data.eventStream !== undefined) {
          sink(msg.data.eventStream);
        }
      } catch {
        /* ignore */
      }
    })
    .catch(() => {});
  return () => {
    cancelled = true;
  };
}

//#endregion 🔖KitGraphqlWire

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
