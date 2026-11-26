async function readConfig(api, sequence, policy, wire) {
  const command = wire.encodeAppCommand({ ReadConfig: { seq: sequence } });
  const [page, ...extra] = wire.createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: BigInt(sequence), command });
  if (!page || extra.length) throw Error("ReadConfig should fit exactly one ingress page");
  const budget = { maxEffects: 64, maxPatchBytes: 1 << 20, maxFrames: 64 };
  let admitted = false, complete = false, config;
  for (let turn = 0; turn < policy.maximumTurns; turn++) {
    const output = await api.poll([], admitted ? undefined : page, undefined, budget);
    if (output.status.tag === "faulted" || output.commandIngress?.tag === "fault") throw Error("BIM ReadConfig command ingress failed");
    if (output.uiPatches.length || output.presence.length || output.uiPatchReceipt || output.lifecycleReceipt) throw Error("BIM ReadConfig emitted output this bridge probe cannot route");
    for (const effect of output.effects) {
      const bytes = wire.shellFrameBytes(effect, 1);
      if (!bytes) throw Error("BIM ReadConfig emitted an effect other than an app-channel reply");
      const frame = wire.decodeAppFrame(bytes);
      if (!("Config" in frame) || frame.Config.in_reply_to !== sequence || config) throw Error("BIM ReadConfig did not return one correlated Config reply");
      config = frame.Config;
    }
    const ingress = output.commandIngress?.tag;
    admitted ||= ingress === "page-accepted" || ingress === "command-pending" || ingress === "command-complete";
    complete ||= ingress === "command-complete";
    if (complete && output.status.tag === "idle" && output.nextWake == null) {
      if (!config) throw Error("BIM ReadConfig completed without returning its configuration");
      return;
    }
  }
  throw Error("BIM ReadConfig did not settle");
}

export async function exercise(createActorApi, decodeActorInstanceLifecycle, applicationIds, policy, clock, loadCore, sequence, wire) {
  for (const appId of applicationIds) {
    const actor = appId.split("#").at(-1);
    const api = await createActorApi(actor, 1n, { operation: 1n, generation: 1n, grant: policy.grant, fuel: policy.fuel, maximumElapsedUs: BigInt(policy.lifetimeUs) }, clock, { receipt() {}, hold(raw: unknown) { throw Error(JSON.stringify(raw, (_, v) => typeof v === "bigint" ? String(v) : v)); } }, loadCore);
    const payload = { instance: 1, purpose: { tag: "ordinary" }, activationGeneration: 1n, requestSequence: 1, appId, actor, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() };
    const events: { kind: string; payload: unknown }[] = [{ kind: "instance-open", payload }];
    let lifetime, closeGeneration, closing = false, retired = false, settled = false;
    for (let turn = 0; turn < policy.maximumTurns; turn++) {
      const result = await api.poll(events.splice(0,1), undefined, undefined, { maxEffects: 64, maxPatchBytes: 1 << 20, maxFrames: 64 });
      if (result.status.tag === "faulted") throw Error("BIM actor faulted: " + JSON.stringify(result.status, (_, v) => typeof v === "bigint" ? String(v) : v));
      if (result.effects.length || result.uiPatches.length || result.presence.length || result.uiPatchReceipt) throw Error("BIM lifecycle emitted output requiring OS routing; this isolated harness cannot settle it");
      if (result.lifecycleReceipt) {
        const receipt = decodeActorInstanceLifecycle(result.lifecycleReceipt);
        if (receipt.lifetime.instanceId !== 1 || receipt.lifetime.activationGeneration !== 1n) throw Error("BIM receipt addressed another activation");
        if (receipt.requestSequence !== (closing ? 2 : 1)) throw Error("BIM receipt acknowledged a different request");
        if (receipt.kind === "captured") lifetime = receipt.lifetime;
        if (closing) {
          if (!lifetime || receipt.lifetime.guestLifetime !== lifetime.guestLifetime) throw Error("BIM close changed the captured lifetime");
          if (closeGeneration !== undefined && receipt.closeGeneration !== closeGeneration) throw Error("BIM close generation changed");
          closeGeneration = receipt.closeGeneration;
        }
        if (receipt.kind === "retired") retired = true;
        events.push({ kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt } });
      }
      if (result.status.tag === "idle" && result.nextWake == null && !events.length) {
        if (!lifetime) throw Error("BIM actor settled without a captured lifetime");
        if (closing) { settled = true; break; }
        await readConfig(api, sequence, policy, wire);
        console.log(appId, "returned configuration through the browser command bridge");
        const artifactKind = appId.split("@")[0];
        const documentId = "artifact-0123456789abcdef0123456789abcdef";
        const callCodec = async request => {
          const result = await api.codec({ artifactKind, ...request });
          if (!("ok" in result)) throw Error("BIM " + request.operation + " refused: " + new TextDecoder().decode(result.fault));
          return result.ok;
        };
        const fingerprint = await callCodec({ operation: "pack-schema-hash" });
        if (fingerprint.length !== 32) throw Error("BIM snapshot fingerprint must contain 32 bytes");
        const pair = await callCodec({ operation: "genesis", documentId });
        if (!pair.pack.length || !pair.spr.length) throw Error("BIM genesis did not return the document and its snapshot");
        const mirror = await callCodec({ operation: "print-mirror", pair });
        if (!mirror.some(text => text.includes(documentId))) throw Error("BIM document mirror lost its minted identity");
        closing = true;
        events.push({ kind: "instance-close", payload: { kind: "close", lifetime, requestSequence: 2 } });
      }
    }
    if (!retired || !settled) throw Error("BIM actor failed to settle after retirement");
    if (!(await api.closeOriginal()).sourceClosed) throw Error("BIM actor context remained open");
    console.log(appId, "opened, acknowledged and retired");
  }
}
