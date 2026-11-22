/** 🏠 One already authorized worker folder binding and its accepted first Store frontier. */
export type BoundBackboneFirstArchive = Readonly<{
  binding: Readonly<{ kind: "folder"; dataClass: "persistedLocalOnly"; path: string }>;
  schema: string;
  actor: string;
  clientInstanceId: string;
  ownerEpoch: number;
  source: NativeFolderJournalScope["source"];
  frontier: Readonly<{ documentId: string; generation: bigint; revision: Uint8Array; extent: number }>;
}>;

/** 🔑 Receives the server’s live worker binding and the native folder writer. */
export async function captureBoundBackboneFirstArchive(
  authority: BoundBackboneFirstArchive,
  current: (scope: BoundBackboneFirstArchive) => boolean,
  journal: NativeFolderJournal,
  signal: AbortSignal,
): Promise<FolderFirstArchiveCapture> {
  const { binding, actor, clientInstanceId, ownerEpoch, source, frontier } = authority;
  const validToken = (value: string) => /^[0-9a-f]{16}$/u.test(value) && value !== "0000000000000000";
  if (binding.kind !== "folder" || binding.dataClass !== "persistedLocalOnly" || !isAbsolute(binding.path) ||
      binding.path.includes("\0") || !actor || !clientInstanceId || !Number.isSafeInteger(ownerEpoch) || ownerEpoch < 1 ||
      !Number.isInteger(source.instanceId) || source.instanceId < 1 || source.instanceId > 0xffffffff ||
      !validToken(source.operationId) || !validToken(source.generation) ||
      source.sequence < 1n || source.sequence > 0xffffffffffffffffn) throw canonicalBootstrapMirrorError("invalid");
  const sealed: BoundBackboneFirstArchive = {
    ...authority, binding: { ...binding }, source: { ...source, declaration: { ...source.declaration } },
    frontier: { ...frontier, revision: Uint8Array.from(frontier.revision) },
  };
  if (!current(sealed)) throw canonicalBootstrapMirrorError("conflict");
  await mkdir(sealed.binding.path, { recursive: true });
  const folder = await realpath(sealed.binding.path);
  const dbPath = join(folder, ".semio", "canonical-bootstrap.db");
  const db = existsSync(dbPath) ? await backboneDbHandleFor(dbPath) : null;
  const owner = db?.query("SELECT state FROM canonical_bootstrap_owner WHERE document_id = ?1");
  const live = () => {
    if (!current(sealed)) return false;
    if (!owner) return !existsSync(dbPath);
    const row = owner.get(sealed.frontier.documentId) as { state?: string } | null;
    return !row || row.state === "generic";
  };
  const capture = await journal.capture(folder, sealed, signal, live);
  return {
    write: (offset, page) => capture.write(offset, page),
    async publish(digest, publishSignal) {
      if (await realpath(sealed.binding.path) !== folder || !live()) throw canonicalBootstrapMirrorError("conflict");
      return capture.publish(digest, publishSignal);
    },
    retire: () => capture.retire(),
  };
}

