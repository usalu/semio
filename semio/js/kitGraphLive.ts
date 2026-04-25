/**
 * Live kit reads use GraphQL fields on `kitStore` / `designForId` / `pieceForId` (no `readKitCommands` batch).
 * VCS uses typed root mutations; shape of each result is the same tagged JSON as `KitStoreCommandResult`.
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

function storePayload(cmd: unknown): { tag: string; value: unknown } {
  if (cmd == null || typeof cmd !== "object" || Array.isArray(cmd)) {
    throw new Error("kit store command: expected object");
  }
  const o = cmd as Record<string, unknown>;
  const keys = Object.keys(o);
  if (keys.length !== 1) {
    throw new Error("kit store command: expected a single tagged variant");
  }
  const tag = keys[0]!;
  return { tag, value: o[tag] };
}

/** Maps `KitStoreCommand` JSON to typed root mutations; returns the tagged `KitStoreCommandResult` JSON. */
export async function kitGraphqlExecuteStoreCommand(handle: KitGraphqlHandle, cmd: unknown): Promise<unknown> {
  const { tag, value } = storePayload(cmd);
  const data = await kitGraphqlRun(handle, (() => {
    switch (tag) {
      case "newSession":
        return { query: `mutation { newSession }` };
      case "endSession": {
        const id = (value as { id?: string } | null)?.id;
        if (typeof id !== "string") throw new Error("endSession: need id");
        return { query: `mutation($id: String!) { endSession(id: $id) }`, variables: { id } };
      }
      case "newAlternative": {
        const v = value as { fromCheckpoint?: string | null; name: string } | null;
        if (v == null || typeof v.name !== "string") throw new Error("newAlternative");
        return {
          query: `mutation($fromCheckpoint: String, $name: String!) { newAlternative(fromCheckpoint: $fromCheckpoint, name: $name) }`,
          variables: { fromCheckpoint: v.fromCheckpoint ?? null, name: v.name },
        };
      }
      case "executeSessionCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        const id = v?.id;
        const sc = v?.commands;
        if (typeof id !== "string" || !Array.isArray(sc)) throw new Error("executeSessionCommands");
        return {
          query: `mutation($sessionId: String!, $sessionCommands: [JSON!]!) { executeSessionCommands(sessionId: $sessionId, sessionCommands: $sessionCommands) }`,
          variables: { sessionId: id, sessionCommands: sc },
        };
      }
      case "executeKitCheckpointCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        if (typeof v?.id !== "string" || !Array.isArray(v?.commands)) throw new Error("executeKitCheckpointCommands");
        return {
          query: `mutation($checkpointId: String!, $commands: [JSON!]!) { executeKitCheckpointCommands(checkpointId: $checkpointId, commands: $commands) }`,
          variables: { checkpointId: v.id, commands: v.commands },
        };
      }
      case "executeKitAlternativeCommands": {
        const v = value as { id?: string; commands?: unknown[] } | null;
        if (typeof v?.id !== "string" || !Array.isArray(v?.commands)) throw new Error("executeKitAlternativeCommands");
        return {
          query: `mutation($alternativeId: String!, $commands: [JSON!]!) { executeKitAlternativeCommands(alternativeId: $alternativeId, commands: $commands) }`,
          variables: { alternativeId: v.id, commands: v.commands },
        };
      }
      case "attachBackbone": {
        const cfg = (value as { config?: unknown } | null)?.config;
        return {
          query: `mutation($config: JSON!) { attachBackbone(config: $config) }`,
          variables: { config: cfg },
        };
      }
      case "detachBackbone":
        return { query: `mutation { detachBackbone }` };
      case "setActiveCheckpoint": {
        const id = (value as { id?: string | null } | null)?.id ?? null;
        return {
          query: `mutation($id: String) { setActiveCheckpoint(id: $id) }`,
          variables: { id },
        };
      }
      case "listConflicts":
        return { query: `mutation { listConflicts }` };
      case "resolveConflict": {
        const v = value as { id?: string; strategy?: unknown } | null;
        if (typeof v?.id !== "string") throw new Error("resolveConflict");
        return {
          query: `mutation($id: String!, $strategy: JSON!) { resolveConflict(id: $id, strategy: $strategy) }`,
          variables: { id: v.id, strategy: v.strategy },
        };
      }
      case "backboneStatus":
        return { query: `mutation { backboneStatus }` };
      case "syncNow":
        return { query: `mutation { syncNow }` };
      case "batch": {
        const cmds = (value as { commands?: unknown[] } | null)?.commands;
        if (!Array.isArray(cmds)) throw new Error("batch.commands");
        return { query: `mutation($commands: [JSON!]!) { kitStoreBatch(commands: $commands) }`, variables: { commands: cmds } };
      }
      default:
        throw new Error(`[DEBUG] kitGraphqlExecuteStoreCommand: unhandled ${tag}`);
    }
  })());
  const root = kitGraphqlFirstData(data);
  const op = Object.keys(root)[0];
  if (op === undefined) throw new Error("kitGraphql: empty mutation data");
  return root[op];
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

/** Maps one `ReadKitCommand` to field-based GraphQL (no `readKitCommands`). */
export async function kitGraphqlMapReadCommand(handle: KitGraphqlHandle, c: ReadKitCommand): Promise<ReadKitCommandOutput> {
  if ("readKitTypeIdsCommand" in c && c.readKitTypeIdsCommand == null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { typeIds } }` }),
    ) as { kitStore?: { typeIds?: string[] } };
    const typeIds = d.kitStore?.typeIds;
    if (!Array.isArray(typeIds)) throw new Error("typeIds");
    return { readKitTypeIdsCommand: { typeIds: typeIds.map((id) => idDto(id)) } };
  }
  if ("readKitTypesMetadataCommand" in c && c.readKitTypesMetadataCommand == null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { typesMetadata } }` }),
    ) as { kitStore?: { typesMetadata?: unknown } };
    return { readKitTypesMetadataCommand: { types: d.kitStore?.typesMetadata as any } };
  }
  if ("readKitDesignIdsCommand" in c && c.readKitDesignIdsCommand == null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { designIds } }` }),
    ) as { kitStore?: { designIds?: string[] } };
    const designIds = d.kitStore?.designIds;
    if (!Array.isArray(designIds)) throw new Error("designIds");
    return { readKitDesignIdsCommand: { designIds: designIds.map((id) => idDto(id)) } };
  }
  if ("readKitDesignsMetadataCommand" in c && c.readKitDesignsMetadataCommand == null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { designsMetadata } }` }),
    ) as { kitStore?: { designsMetadata?: unknown } };
    return { readKitDesignsMetadataCommand: { designs: d.kitStore?.designsMetadata as any } };
  }
  if ("readKitColoredConnectorsCommand" in c && c.readKitColoredConnectorsCommand == null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { coloredConnectors } }` }),
    ) as { kitStore?: { coloredConnectors?: unknown } };
    return { readKitColoredConnectorsCommand: { rows: d.kitStore?.coloredConnectors as any } };
  }
  if ("readKitNameCommand" in c && c.readKitNameCommand == null) {
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, { query: `query { kitStore { name } }` }),
    ) as { kitStore?: { name?: string } };
    if (d.kitStore?.name == null) throw new Error("kit name");
    return { readKitNameCommand: { name: d.kitStore.name } };
  }
  if ("readKitDesignCommands" in c && c.readKitDesignCommands) {
    const { id, commands } = c.readKitDesignCommands;
    const out: ReadDesignCommandOutput[] = [];
    for (const sub of commands) {
      out.push(await mapDesignRead(handle, id.id, sub));
    }
    return { readKitDesignCommands: { results: out } };
  }
  if ("readKitTypeCommands" in c && c.readKitTypeCommands) {
    const { id, commands } = c.readKitTypeCommands;
    const out: ReadTypeCommandOutput[] = [];
    for (const sub of commands) {
      out.push(await mapTypeRead(handle, id.id, sub));
    }
    return { readKitTypeCommands: { results: out } };
  }
  throw new Error(`[DEBUG] kitGraphql: unsupported read command ${Object.keys(c).join(",")}`);
}

async function mapDesignRead(handle: KitGraphqlHandle, designId: string, cmd: ReadDesignCommand): Promise<ReadDesignCommandOutput> {
  if (cmd.readDesignClusterableGroupsCommand) {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { clusterableGroups(selection: $sel) } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId, sel: cmd.readDesignClusterableGroupsCommand.selection.map((x) => x.id) } })) as {
      kitStore?: { designForId?: { clusterableGroups?: string[][] } | null };
    };
    const g = d.kitStore?.designForId?.clusterableGroups;
    if (!Array.isArray(g)) throw new Error("clusterableGroups");
    return { readDesignClusterableGroupsCommand: { groups: g.map((row) => row.map((id) => idDto(id))) } };
  }
  if (cmd.readDesignIncludedDesignsCommand == null) {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { includedDesigns } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId } })) as {
      kitStore?: { designForId?: { includedDesigns?: unknown } | null };
    };
    return { readDesignIncludedDesignsCommand: { designs: d.kitStore?.designForId?.includedDesigns as any } };
  }
  if (cmd.readDesignQualitySumCommand) {
    const qid = cmd.readDesignQualitySumCommand.qualityId.id;
    const q = `query($id: String!, $q: String!) { kitStore { designForId(id: $id) { qualitySum(qualityId: $q) } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId, q: qid } })) as {
      kitStore?: { designForId?: { qualitySum?: number } | null };
    };
    const s = d.kitStore?.designForId?.qualitySum;
    if (typeof s !== "number") throw new Error("qualitySum");
    return { readDesignQualitySumCommand: { sum: s } };
  }
  if (cmd.readDesignReplaceableCatalogCommand) {
    const q = `query($id: String!, $sel: [String!]!) { kitStore { designForId(id: $id) { replaceableCatalog(selection: $sel) { typeIds designIds } } } }`;
    const d = kitGraphqlFirstData(
      await kitGraphqlRun(handle, {
        query: q,
        variables: { id: designId, sel: cmd.readDesignReplaceableCatalogCommand.selection.map((x) => x.id) },
      }),
    ) as { kitStore?: { designForId?: { replaceableCatalog?: { typeIds: string[]; designIds: string[] } } | null } };
    const rc = d.kitStore?.designForId?.replaceableCatalog;
    if (rc == null) throw new Error("replaceableCatalog");
    return {
      readDesignReplaceableCatalogCommand: {
        types: rc.typeIds.map((t) => idDto(t)),
        designs: rc.designIds.map((x) => idDto(x)),
      },
    };
  }
  if (cmd.readDesignIncludedDesignIdsCommand == null) {
    const q = `query($id: String!) { kitStore { designForId(id: $id) { includedDesignIds } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: designId } })) as {
      kitStore?: { designForId?: { includedDesignIds?: string[] } | null };
    };
    const ids = d.kitStore?.designForId?.includedDesignIds;
    if (!Array.isArray(ids)) throw new Error("includedDesignIds");
    return { readDesignIncludedDesignIdsCommand: { designIds: ids.map((x) => idDto(x)) } };
  }
  if (cmd.readDesignPieceCommands) {
    return {
      readDesignPieceCommands: {
        results: [await mapPieceRead(handle, designId, cmd.readDesignPieceCommands.id.id, cmd.readDesignPieceCommands.commands[0]!)],
      },
    };
  }
  throw new Error(`[DEBUG] mapDesignRead: ${Object.keys(cmd).join(",")}`);
}

async function mapPieceRead(
  handle: KitGraphqlHandle,
  designId: string,
  pieceId: string,
  cmd: ReadPieceCommand,
): Promise<ReadPieceCommandOutput> {
  if (cmd.readPieceFlatPlaneCommand == null) {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatPlane } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { d: designId, p: pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { flatPlane?: unknown } | null } | null };
    };
    return { readPieceFlatPlaneCommand: { flatPlane: d.kitStore?.designForId?.pieceForId?.flatPlane as any } };
  }
  if (cmd.readPieceFlatCenterCommand == null) {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { flatCenter } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { d: designId, p: pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { flatCenter?: unknown } | null } | null };
    };
    return { readPieceFlatCenterCommand: { flatCenter: d.kitStore?.designForId?.pieceForId?.flatCenter as any } };
  }
  if (cmd.readPieceParentConnectionFullCommand == null) {
    const q = `query($d: String!, $p: String!) { kitStore { designForId(id: $d) { pieceForId(id: $p) { parentConnectionFull } } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { d: designId, p: pieceId } })) as {
      kitStore?: { designForId?: { pieceForId?: { parentConnectionFull?: unknown } | null } | null };
    };
    return {
      readPieceParentConnectionFullCommand: {
        connection: d.kitStore?.designForId?.pieceForId?.parentConnectionFull as any,
      },
    };
  }
  throw new Error(`[DEBUG] mapPieceRead: ${Object.keys(cmd).join(",")}`);
}

async function mapTypeRead(handle: KitGraphqlHandle, typeId: string, cmd: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
  if (cmd.readTypeBestRepresentationCommand) {
    const tags = cmd.readTypeBestRepresentationCommand.tagIds;
    const q = `query($id: String!, $tags: [String!]!) { kitStore { typeForId(id: $id) { bestRepresentation(tagIds: $tags) } } }`;
    const d = kitGraphqlFirstData(await kitGraphqlRun(handle, { query: q, variables: { id: typeId, tags } })) as {
      kitStore?: { typeForId?: { bestRepresentation?: unknown } | null };
    };
    return { readTypeBestRepresentationCommand: { representation: d.kitStore?.typeForId?.bestRepresentation as any } };
  }
  throw new Error(`[DEBUG] mapTypeRead: ${Object.keys(cmd).join(",")}`);
}

export async function kitGraphqlExecuteRead(handle: KitGraphqlHandle, batch: ReadCommandBatch): Promise<ReadCommandBatchResult> {
  const out: ReadCommandBatchResult = [];
  for (const c of batch) {
    out.push(await kitGraphqlMapReadCommand(handle, c));
  }
  return out;
}

/** Top-level `ReadKitCommand` (single item batch). */
export async function readKit(client: KitExecuteRead, command: ReadKitCommand): Promise<ReadKitCommandOutput> {
  return assertSingleResult(await client.executeRead([command]));
}

export async function readKitDesign(
  client: KitExecuteRead,
  designId: string,
  command: ReadDesignCommand,
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
  command: ReadPieceCommand,
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
  command: ReadTypeCommand,
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
    readonly pieceId: string,
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
    readonly designId: string,
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
    readonly typeId: string,
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

  /** @emoji 📌 Type ids in kit graph order (`ReadKitTypeIdsCommand`). */
  async readTypeIds(): Promise<readonly string[]> {
    const out = await readKit(this.client, { readKitTypeIdsCommand: null });
    if (!("readKitTypeIdsCommand" in out) || out.readKitTypeIdsCommand == null) {
      throw new Error("readKitTypeIdsCommand: missing output");
    }
    return out.readKitTypeIdsCommand.typeIds.map((r) => r.id);
  }

  /** @emoji 📌 Per-type metadata rows (`ReadKitTypesMetadataCommand`). */
  async readTypesMetadata(): Promise<ReadonlyArray<unknown>> {
    const out = await readKit(this.client, { readKitTypesMetadataCommand: null });
    if (!("readKitTypesMetadataCommand" in out) || out.readKitTypesMetadataCommand == null) {
      throw new Error("readKitTypesMetadataCommand: missing output");
    }
    return out.readKitTypesMetadataCommand.types;
  }

  /** @emoji 📌 Design ids in kit graph order (`ReadKitDesignIdsCommand`). */
  async readDesignIds(): Promise<readonly string[]> {
    const out = await readKit(this.client, { readKitDesignIdsCommand: null });
    if (!("readKitDesignIdsCommand" in out) || out.readKitDesignIdsCommand == null) {
      throw new Error("readKitDesignIdsCommand: missing output");
    }
    return out.readKitDesignIdsCommand.designIds.map((r) => r.id);
  }

  /** @emoji 📌 Per-design metadata rows (`ReadKitDesignsMetadataCommand`). */
  async readDesignsMetadata(): Promise<ReadonlyArray<unknown>> {
    const out = await readKit(this.client, { readKitDesignsMetadataCommand: null });
    if (!("readKitDesignsMetadataCommand" in out) || out.readKitDesignsMetadataCommand == null) {
      throw new Error("readKitDesignsMetadataCommand: missing output");
    }
    return out.readKitDesignsMetadataCommand.designs;
  }

  async readColoredConnectors(): Promise<ReadonlyArray<unknown>> {
    const out = await readKit(this.client, { readKitColoredConnectorsCommand: null });
    if (!("readKitColoredConnectorsCommand" in out) || out.readKitColoredConnectorsCommand == null) {
      throw new Error("readKitColoredConnectorsCommand: missing output");
    }
    return out.readKitColoredConnectorsCommand.rows;
  }
}
