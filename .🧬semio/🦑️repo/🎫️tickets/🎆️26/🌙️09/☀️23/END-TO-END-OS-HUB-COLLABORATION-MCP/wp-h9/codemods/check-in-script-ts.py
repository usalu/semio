"""📌️ One-off: move the hub process probes from the checkpoint-publications upload route to the Check In command."""
p = "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:120], s.count(old))
    s = s.replace(old, new)
def replace_between(start_marker, end_marker, new):
    global s
    a = s.index(start_marker)
    b = s.index(end_marker, a)
    s = s[:a] + new + s[b:]

rep("  CHECKPOINT_PUBLICATION_PAIR_MAX_BYTES,\n  parseCheckpointPublicationCommandV1,\n", "  documentCheckInCanonicalJson,\n  parseDocumentCheckInStatusV1,\n  parseDocumentCheckInV1,\n  type DocumentCheckInStatusV1,\n")

replace_between("type CheckpointPublicationProcessFixtureV1 = {", "async function startMcpBoundWorkspaceChild(", '''/** 🧫️ One real GIS Map ledger edit as the native fixture emitted it: the envelopes a replica's store
 * produced for that edit, in causal order, with their operation payloads. */
type CheckInProcessEnvelopeV1 = {
  readonly mutationId: string;
  readonly dependencies: readonly string[];
  readonly diffSchema: string;
  readonly diff: string;
  readonly inverseSchema: string;
  readonly inverse: string;
};

type CheckInProcessFixtureV1 = {
  readonly schema: "semio.hub.check-in-process-fixture/v1";
  readonly profileId: string;
  readonly generationId: string;
  readonly documentId: string;
  readonly package: { readonly pluginId: string; readonly packageId: string; readonly version: string; readonly componentSha256: string };
  readonly artifact: { readonly kind: "s.gis.gismap"; readonly schema: "gis.map"; readonly packSchemaHash: string };
  readonly surfaceId: string;
  readonly payload: { readonly envelopes: { readonly path: string; readonly count: number; readonly sha256: string } };
};

function checkInProcessFixture(): { readonly root: string; readonly fixture: CheckInProcessFixtureV1; readonly envelopes: readonly CheckInProcessEnvelopeV1[] } {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("check-in process requires its ticket-owned artifact root");
  const root = join(resolve(artifactRoot), "check-in-process-fixture");
  const fixture = JSON.parse(readFileSync(join(root, "fixture.json"), "utf8")) as CheckInProcessFixtureV1;
  if (
    fixture.schema !== "semio.hub.check-in-process-fixture/v1" ||
    fixture.profileId !== "gis-map-integration-fixtures" ||
    !/^[0-9a-f]{64}$/u.test(fixture.generationId) ||
    fixture.artifact.kind !== "s.gis.gismap" ||
    fixture.artifact.schema !== "gis.map" ||
    !/^[0-9a-f]{64}$/u.test(fixture.artifact.packSchemaHash)
  )
    throw new Error("check-in process fixture identity is invalid");
  const path = resolve(root, fixture.payload.envelopes.path);
  if (relative(root, path).startsWith("..")) throw new Error("check-in process envelopes escaped their fixture root");
  const bytes = readFileSync(path);
  if (createHash("sha256").update(bytes).digest("hex") !== fixture.payload.envelopes.sha256) throw new Error("check-in process envelopes differ from their native receipt");
  const envelopes = JSON.parse(bytes.toString("utf8")) as CheckInProcessEnvelopeV1[];
  if (!Array.isArray(envelopes) || envelopes.length !== fixture.payload.envelopes.count || envelopes.some((envelope) => envelope.diffSchema !== fixture.artifact.schema || envelope.inverseSchema !== fixture.artifact.schema))
    throw new Error("check-in process envelopes are not the fixture's GIS Map ledger edit");
  return { root, fixture, envelopes: Object.freeze(envelopes) };
}

''')

rep("async function waitForCheckpointSocketFrame<T>(", "async function waitForDocumentSocketFrame<T>(")
s = s.replace("waitForCheckpointSocketFrame(", "waitForDocumentSocketFrame(")
rep('''    if (socket.readyState >= WebSocket.CLOSING) throw new Error(`checkpoint publication socket closed before ${label}`);
    if (Date.now() >= deadline) throw new Error(`checkpoint publication socket ${label} deadline exceeded`);''', '''    if (socket.readyState >= WebSocket.CLOSING) throw new Error(`document socket closed before ${label}`);
    if (Date.now() >= deadline) throw new Error(`document socket ${label} deadline exceeded`);''')

s = s.replace("createCheckpointPublicationProcessGenesis", "createCheckInProcessGenesis")
s = s.replace("CheckpointPublicationProcessFixtureV1", "CheckInProcessFixtureV1")
rep('''  ) throw new Error(`checkpoint process genesis was not durably accepted: ${accepted.status}`);''', '''  ) throw new Error(`check-in process genesis was not durably accepted: ${accepted.status}`);''')
rep('''    if (!["accepted", "preparing", "indeterminate"].includes(status.phase) || Date.now() >= deadline) throw new Error(`checkpoint process genesis reached ${status.phase} before Ready`);''', '''    if (!["accepted", "preparing", "indeterminate"].includes(status.phase) || Date.now() >= deadline) throw new Error(`check-in process genesis reached ${status.phase} before Ready`);''')
rep('''    if (response.status !== 200 && response.status !== 202) throw new Error(`checkpoint process genesis status failed: ${response.status}`);
    if (status.catalogGenerationId !== request.expectedCatalogGenerationId) throw new Error("checkpoint process genesis status crossed its selected catalog generation");''', '''    if (response.status !== 200 && response.status !== 202) throw new Error(`check-in process genesis status failed: ${response.status}`);
    if (status.catalogGenerationId !== request.expectedCatalogGenerationId) throw new Error("check-in process genesis status crossed its selected catalog generation");''')
rep('''  ) throw new Error("checkpoint process genesis Ready coordinates differ from the selected GIS target");''', '''  ) throw new Error("check-in process genesis Ready coordinates differ from the selected GIS target");''')

replace_between("async function commitCheckpointPublicationProcessMutation(", "function jsonRpcResponses(", '''/** ✍️ Commits the fixture's real GIS Map ledger edit through an authenticated document socket and
 * returns the committed head the hub acknowledged — the head a Check In then names. */
async function commitCheckInProcessEdit(
  run: LocalHubRun,
  capability: string,
  spaceId: string,
  fixture: CheckInProcessFixtureV1,
  envelopes: readonly CheckInProcessEnvelopeV1[],
): Promise<{ readonly plan: ReturnType<typeof parseDocumentOpenPlanV1>; readonly frontier: WireFrontierSummary }> {
  const documentRoot = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.documentId)}`;
  const planResponse = await fetch(`http://127.0.0.1:${run.port}${documentRoot}/open-plan`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId: fixture.documentId }, requestedSurfaceId: fixture.surfaceId, clientInstanceId: "check-in-process" }),
    signal: AbortSignal.timeout(5_000),
  });
  const plan = parseDocumentOpenPlanV1(await planResponse.json().catch(() => undefined), Date.now());
  if (!planResponse.ok || plan.scope.spaceId !== spaceId || plan.scope.documentId !== fixture.documentId || plan.catalog.generationId !== fixture.generationId) throw new Error("check-in process received a substituted GIS Map plan");
  const grantIntent = parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
  const grantResponse = await fetch(`http://127.0.0.1:${run.port}${documentRoot}/socket-grants`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify(grantIntent),
    signal: AbortSignal.timeout(5_000),
  });
  const grant = parseDocumentSocketGrantReceiptV1(await grantResponse.json().catch(() => undefined));
  if (!grantResponse.ok) throw new Error(`check-in process socket grant failed: ${grantResponse.status}`);
  const scope = `${spaceId}/${fixture.documentId}`;
  const socket = new WebSocket(`ws://127.0.0.1:${run.port}/scopes/${encodeURIComponent(scope)}/document/ws?surface=${encodeURIComponent(fixture.surfaceId)}`, ["semio.session.v1", capability]);
  socket.binaryType = "arraybuffer";
  const frames: Record<string, any>[] = [];
  let socketError: unknown;
  socket.onmessage = (event) => {
    try {
      frames.push(decodeServerFrame(new Uint8Array(event.data as ArrayBuffer)).frame as unknown as Record<string, any>);
    } catch (error) {
      socketError = error;
    }
  };
  socket.onerror = (event) => {
    socketError = event;
  };
  try {
    await new Promise<void>((resolveOpen, rejectOpen) => {
      const timer = setTimeout(() => rejectOpen(new Error("check-in process socket open deadline exceeded")), 5_000);
      socket.onopen = () => {
        clearTimeout(timer);
        resolveOpen();
      };
    });
    if (socket.protocol !== grant.protocol) throw new Error("check-in process socket did not negotiate its exact protocol");
    const packSchemaHash = Buffer.from(fixture.artifact.packSchemaHash, "hex");
    socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: fixture.artifact.schema, pack_schema_hash: Array.from(packSchemaHash), resume_token: null, frontier: null } }, "command"));
    await waitForDocumentSocketFrame(socket, frames, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), "Welcome");
    await waitForDocumentSocketFrame(socket, frames, (frame) => ("Session" in frame && frame.Session.actor === grant.actorId ? frame.Session : undefined), "verified Session actor");
    if (socketError) throw socketError;
    const documentId = `v1:${Buffer.byteLength(spaceId)}:${Buffer.byteLength(fixture.documentId)}:${spaceId}${fixture.documentId}`;
    const physicalMs = Date.now();
    const wire: WireMutationEnvelope[] = envelopes.map((envelope, index) => ({
      mutation_id: envelope.mutationId,
      document_id: documentId,
      actor: grant.actorId,
      dependencies: [...envelope.dependencies],
      diff: { schema: envelope.diffSchema, payload: Array.from(Buffer.from(envelope.diff, "base64")) },
      inverse: { schema: envelope.inverseSchema, payload: Array.from(Buffer.from(envelope.inverse, "base64")) },
      timestamp: { actor: 1, physical_ms: physicalMs, logical: index + 1 },
    }));
    socket.send(encodeClientFrame({ Commands: { batch_id: 1, envelopes: wire } }, "command"));
    const ack = await waitForDocumentSocketFrame(socket, frames, (frame) => ("Ack" in frame && frame.Ack.batch_id === 1 ? frame.Ack : undefined), "persisted command acknowledgement");
    const accepted = ack.stages.some((stage: any) => stage === "Persisted") && ack.stages.some((stage: any) => stage?.Applied?.outcome === "Accepted");
    const tip = envelopes[envelopes.length - 1]!.mutationId;
    if (!accepted || ack.frontier.head_edit_id !== tip || ack.frontier.head_edit_ordinal !== envelopes.length || ack.frontier.last_commit_seq !== 1)
      throw new Error("check-in process edit was not durably accepted at its exact first frontier");
    return { plan, frontier: ack.frontier as WireFrontierSummary };
  } finally {
    if (socket.readyState < WebSocket.CLOSING) socket.close(1000, "edit committed");
  }
}

''')

replace_between("type CheckpointPublicationProcessResultV1 = ", "async function activeMcpDocumentConnections(", '''/** 📌️ Checks in one acknowledged head through the hub's Check In command and follows its status to
 * `ready`, then proves a lost-response retry of the same request answers the identical checkpoint. */
async function checkInProcessHeadV1(run: LocalHubRun, capability: string, spaceId: string, documentId: string, frontier: WireFrontierSummary): Promise<DocumentCheckInStatusV1> {
  const chainSha256 = Buffer.from(frontier.chain_hash).toString("hex");
  const request = parseDocumentCheckInV1(
    documentCheckInCanonicalJson({
      schema: "semio.hub.document-check-in/v1",
      requestId: randomBytes(16).toString("hex"),
      head: { documentId, headEditOrdinal: frontier.head_edit_ordinal, headEditId: frontier.head_edit_id, lastCommitSeq: frontier.last_commit_seq, chainSha256 },
    }),
  );
  if (!request) throw new Error("check-in process could not seal its canonical request");
  const route = `http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/check-ins`;
  const post = async (): Promise<DocumentCheckInStatusV1> => {
    const response = await fetch(route, { method: "POST", headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" }, body: documentCheckInCanonicalJson(request), signal: AbortSignal.timeout(10_000) });
    const status = parseDocumentCheckInStatusV1(await response.text());
    if (![200, 202].includes(response.status) || !status || status.requestId !== request.requestId) throw new Error(`check-in process command was not accepted: ${response.status}`);
    return status;
  };
  let status = await post();
  const deadline = Date.now() + 120_000;
  while (status.phase !== "ready") {
    if (status.phase === "failed" || status.phase === "cancelled" || Date.now() >= deadline) throw new Error(`check-in process ended ${status.phase}${status.refusal ? ` (${status.refusal})` : ""}`);
    await Bun.sleep(20);
    const response = await fetch(`${route}/${request.requestId}`, { headers: { authorization: `Bearer ${capability}` }, signal: AbortSignal.timeout(5_000) });
    const next = parseDocumentCheckInStatusV1(await response.text());
    if (![200, 202].includes(response.status) || !next || next.requestId !== request.requestId || next.progress.completedUnits < status.progress.completedUnits) throw new Error(`check-in process status regressed: ${response.status}`);
    status = next;
  }
  if (JSON.stringify(status.ready?.baseline) !== JSON.stringify(request.head)) throw new Error("check-in process checkpoint baseline is not the named head");
  const replay = await post();
  if (replay.phase !== "ready" || JSON.stringify(replay.ready) !== JSON.stringify(status.ready)) throw new Error("check-in lost-response retry did not answer its identical checkpoint");
  return status;
}

''')

# MCP proof
rep('''  const { root: fixtureRoot, fixture: loadedFixture, pack, spr, diff, inverse } = checkpointPublicationProcessFixture();''', '''  const { root: fixtureRoot, fixture: loadedFixture, envelopes } = checkInProcessFixture();''')
rep('''  const retained: Buffer[] = [pack, spr, diff, inverse];
''', '')
rep('''    const { plan, frontier } = await commitCheckpointPublicationProcessMutation(run, author.capability, spaceId, fixture, diff, inverse);
    const { command, receipt } = await publishCheckpointPublicationProcessPairV1(run, author.capability, spaceId, fixture, plan, frontier, pack, spr);
''', '''    const { plan, frontier } = await commitCheckInProcessEdit(run, author.capability, spaceId, fixture, envelopes);
    const checkedIn = await checkInProcessHeadV1(run, author.capability, spaceId, fixture.documentId, frontier);
''')
rep('''        resource?.descriptorDigestV1 !== command.descriptorDigestV1 ||
        resource?.activeCheckpointId !== receipt.checkpoint.checkpointId ||
        JSON.stringify(resource?.frontier) !== JSON.stringify(receipt.checkpoint.baselineFrontier) ||
        !decodedPack.equals(pack) ||
        !decodedSpr.equals(spr) ||''', '''        resource?.descriptorDigestV1 !== plan.descriptorDigestV1 ||
        resource?.activeCheckpointId !== checkedIn.ready?.checkpointId ||
        resource?.frontier?.headEditOrdinal !== checkedIn.ready?.baseline.headEditOrdinal ||
        resource?.frontier?.headEditId !== checkedIn.ready?.baseline.headEditId ||
        resource?.frontier?.lastCommitSeq !== checkedIn.ready?.baseline.lastCommitSeq ||
        decodedPack.byteLength === 0 ||
        decodedSpr.byteLength === 0 ||''')
rep('''        throw new Error("checkpoint publication MCP resource did not project the exact independently verified GIS Map pair");''', '''        throw new Error("check-in MCP resource did not project the hub-materialized GIS Map checkpoint");''')
rep('''  } finally {
    retained.forEach((bytes) => bytes.fill(0));
    if (child) terminateOwnedChildTree(child);
    await finishLocalHub(run);
  }
}''', '''  } finally {
    if (child) terminateOwnedChildTree(child);
    await finishLocalHub(run);
  }
}''')
s = s.replace("proveCheckpointPublicationMcpProcess", "proveCheckInMcpProcess")

# GIS proposal process
rep('''  const payload = checkpointPublicationProcessFixture();''', '''  const payload = checkInProcessFixture();''')
rep('''    documentId: `gis-map-collaboration-${randomBytes(8).toString("hex")}`,
    mutationId: `gis-map-base-${randomBytes(8).toString("hex")}`,
''', '''    documentId: `gis-map-collaboration-${randomBytes(8).toString("hex")}`,
''')
rep('''  const retained = [payload.pack, payload.spr, payload.diff, payload.inverse];
''', '')
rep('''    const { plan, frontier } = await commitCheckpointPublicationProcessMutation(run, authorA.capability, spaceId, fixture, payload.diff, payload.inverse);
    const initialPublication = await publishCheckpointPublicationProcessPairV1(run, authorA.capability, spaceId, fixture, plan, frontier, payload.pack, payload.spr);
''', '''    const { frontier } = await commitCheckInProcessEdit(run, authorA.capability, spaceId, fixture, payload.envelopes);
    const initialCheckIn = await checkInProcessHeadV1(run, authorA.capability, spaceId, fixture.documentId, frontier);
''')
rep('''initialA.activeCheckpointId !== initialPublication.receipt.checkpoint.checkpointId''', '''initialA.activeCheckpointId !== initialCheckIn.ready?.checkpointId''')
rep('''    const publication = source.indexOf("await publishCheckpointPublicationProcessPairV1(");''', '''    const publication = source.indexOf("await checkInProcessHeadV1(");''')
rep('''      pack.byteLength + spr.byteLength > CHECKPOINT_PUBLICATION_PAIR_MAX_BYTES ||''', '''      pack.byteLength + spr.byteLength > GIS_MAP_PROCESS_PAIR_MAX_BYTES ||''')
open(p, "w", encoding="utf-8").write(s)
print("ok")
