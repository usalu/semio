import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT, toolJobRustBlock, toolJobImmutableOperationRootsExact, toolJobPeerCommitAuthorityExact, toolJobPeerInteractionRootsExact } from "../../../../../../../../../../../../../📜️script.ts";

/** 🧹️ Cross-checks CAD domain retirement byte counts independently of its Rust ownership cursor. */
export function cadPresenceRetirementSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️retirement.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/CadPresenceRetirementLaws" });
  if (!validate(fixture)) throw new Error(`CAD presence retirement schema: ${JSON.stringify(validate.errors)}`);
  const counts = new Map<string, number>();
  for (const law of fixture.cases) {
    const bytes = [law.activeUtility, law.engagementStep, law.engagementPane].reduce((sum, part) => sum + (part === null ? 0 : Buffer.byteLength(part.unit.repeat(part.repeat), "utf8")), 0);
    if (counts.has(law.name) || bytes !== law.expectedBytes) throw new Error(`CAD presence byte oracle: ${law.name}`);
    counts.set(law.name, bytes);
  }
  for (const law of fixture.storeCases) {
    if (!counts.has(law.local) || law.peers.some((peer) => !counts.has(peer.presence))) throw new Error(`CAD presence unknown fixture root: ${law.name}`);
    const bytes = counts.get(law.local)! + law.peers.reduce((sum, peer) => sum + counts.get(peer.presence)! + Buffer.byteLength(peer.actor, "utf8"), 0);
    if (bytes !== law.expectedBytes) throw new Error(`CAD presence roster byte oracle: ${law.name}`);
  }
  for (const hostile of [{ ...fixture, grant: { maximumItems: 2, maximumBytes: 4096 } }, { ...fixture, grant: { maximumItems: 1, maximumBytes: 65536 } }]) {
    if (validate(hostile)) throw new Error("CAD presence schema accepted an enlarged production grant");
  }
  const storeBase = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence");
  const storeFixture = JSON.parse(readFileSync(join(storeBase, "🧹️retirement.json"), "utf8"));
  const storeSchema = JSON.parse(readFileSync(join(storeBase, "🧬️schema/🧹️retirement.schema.json"), "utf8"));
  const validateStore = new Ajv({ strict: true, allErrors: true }).compile(storeSchema);
  if (!validateStore(storeFixture)) throw new Error(`presence Store retirement schema: ${JSON.stringify(validateStore.errors)}`);
  for (const law of storeFixture.cases) {
    if (law.expectedSnapshots !== law.peers.length + 1 || law.expectedActorBytes !== law.peers.reduce((sum, peer) => sum + Buffer.byteLength(peer.actor, "utf8"), 0)) throw new Error(`presence Store retirement oracle: ${law.name}`);
  }
  const distinctPeers = new Map<string, { actor: string }>([...storeFixture.overlap.first, ...storeFixture.overlap.second].map((peer) => [JSON.stringify(peer), peer]));
  if (distinctPeers.size !== storeFixture.overlap.expectedPeerSnapshots || [...distinctPeers.values()].reduce((sum, peer) => sum + Buffer.byteLength(peer.actor), 0) !== storeFixture.overlap.expectedActorBytes) throw new Error("presence overlapping-roster ownership oracle");
  const readerReturn = storeFixture.readerReturn;
  let readerAliases = 1;
  let registryAliases = 1;
  let returned = false;
  for (const event of readerReturn.eventOrder) {
    if (event === "reader-alias-released") readerAliases--;
    else if (event === "return-published") { if (readerAliases !== 0 || registryAliases !== 1) throw new Error("read return precedes exact alias release"); returned = true; }
    else if (event === "registry-owner-taken") { if (!returned || readerAliases !== 0) throw new Error("read registry take lacks exclusive payload authority"); }
    else if (event === "final-owner-retired") registryAliases--;
  }
  if (readerAliases !== 0 || registryAliases !== 0) throw new Error("read ownership oracle retains an alias");
  for (const hostile of [
    { ...storeFixture, readerReturn: { ...readerReturn, eventOrder: ["return-published", "reader-alias-released", "registry-owner-taken", "final-owner-retired"] } },
    { ...storeFixture, readerReturn: { ...readerReturn, contendedTransferPreservesUnreturned: false } },
    { ...storeFixture, readerReturn: { ...readerReturn, transferPublishesReturn: true } },
  ]) if (validateStore(hostile)) throw new Error("presence read schema accepted premature return or lost transfer authority");
  const storeSource = readFileSync(join(storeBase, "../🦀️.rs"), "utf8");
  const exactReadReturn = (source: string): boolean => {
    const block = (needle: string): string => { const start = source.indexOf(needle); return start < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", start))?.body ?? ""; };
    const release = block("fn return_snapshot_read<T:");
    const transfer = block("fn into_typed<T:");
    const drop = release.indexOf("drop(owner.take());");
    const publish = release.indexOf("lease.return_now()");
    return drop >= 0 && publish > drop && release.indexOf("after_alias_release();") > drop
      && (source.match(/return_snapshot_read\(&mut self\.owner, &mut self\.lease, \|\| \{\}\)/g) ?? []).length === 4
      && transfer.includes("registry.try_take(lease.index, lease.generation)") && !transfer.includes("return_now")
      && transfer.includes("self.owner = Some(owner);") && transfer.includes("self.lease = Some(lease);");
  };
  if (!exactReadReturn(storeSource)) throw new Error("opaque snapshot read return/transfer lost its exact final-owner ordering");
  const readHostiles = [
    storeSource.replace("drop(owner.take());\n    after_alias_release();", "after_alias_release();"),
    storeSource.replace("let guard = match registry.try_take(lease.index, lease.generation)", "let _ = lease.return_now();\n        let guard = match registry.try_take(lease.index, lease.generation)"),
    storeSource.replace("self.lease = Some(lease);", "drop(lease);"),
  ];
  for (const hostile of readHostiles) if (hostile === storeSource || exactReadReturn(hostile)) throw new Error("opaque snapshot read source guard accepted a premature return or lost retained lease");
  const exactPeerRelease = (source: string): boolean => {
    const cursorStart = source.indexOf("impl<P: Send + Sync + 'static> PresencePeersRetirement<P>");
    const cursor = cursorStart < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", cursorStart))?.body ?? "";
    const publicationStart = source.indexOf("impl<P: Send + Sync + 'static> PresencePeersPublication<P>");
    const publication = publicationStart < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", publicationStart))?.body ?? "";
    return cursor.includes("*self.entry = Arc::into_inner(waiting)") && !cursor.includes("Arc::try_unwrap(waiting)")
      && cursor.includes("Arc::try_unwrap(root)") && cursor.includes("self.owned_root.as_mut()")
      && source.includes("*retirement.root = Some(previous)") && source.includes("presence peer entry requires exact final-owner retirement")
      && source.includes("actor: std::mem::ManuallyDrop<String>") && source.includes("presence: std::mem::ManuallyDrop<Option<Arc<P>>>")
      && !source.includes("impl<P> Clone for PresencePeersRoot<P>") && !source.includes("pub fn clone_aliases(&self)")
      && (publication.match(/PresencePeersRetirement::new\(PresencePeersRetiredEntries::one/g) ?? []).length === 2;
  };
  if (!exactPeerRelease(storeSource)) throw new Error("presence peer roots lost exact shared-entry final-owner retirement");
  const peerHostiles = [
    storeSource.replace("*self.entry = Arc::into_inner(waiting)", "*self.entry = Arc::try_unwrap(waiting).ok()"),
    storeSource.replace("*retirement.root = Some(previous)", "drop(previous)"),
    storeSource.replace("fn clone_aliases(&self)", "pub fn clone_aliases(&self)"),
  ];
  for (const hostile of peerHostiles) if (hostile === storeSource || exactPeerRelease(hostile)) throw new Error("presence peer guard accepted shared-owner waiting, implicit root drop, or public owner cloning");
  const replacements = storeFixture.localReplacements;
  const localOracle = new Ajv({ strict: true }).compile({ const: { ...replacements, capturedValues: replacements.values.slice(0, -1), expectedRetiredWhileOpen: replacements.values.length - 1, expectedFinalSnapshots: replacements.values.length } });
  if (!localOracle(replacements) || storeFixture.localCapture.expectedValueWhileOpen !== storeFixture.localCapture.value || !storeFixture.localCapture.expectedWorkerTerminal) throw new Error("presence local capture/replacement independent owner ledger");
  const retirementSource = readFileSync(join(storeBase, "♻️retirement/🦀️.rs"), "utf8");
  const cadSource = readFileSync(join(base, "♻️retirement/🦀️.rs"), "utf8");
  const exactLocal = (store: string, retirement: string, cad: string): boolean =>
    toolJobImmutableOperationRootsExact(store)
    && store.includes("pub base: ArtifactEphemeralBaseRead<P>")
    && store.includes("Presence(SnapshotRead<P>)")
    && store.includes("Arc::ptr_eq(installed, &root_retirement_factory)")
    && store.includes("previous.return_to_registry()")
    && retirement.includes("advance_returned_local(reads, &mut self.active_returned")
    && retirement.includes("self.active_returned_local.take()")
    && retirement.includes("presence store requires its exact detached terminal-empty owner before Drop")
    && retirement.includes("SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }")
    && retirement.includes("MountedWorkerJobSession::try_new(job, params)")
    && retirement.includes("owner.maintenance_local_reads_step(1, 4096)")
    && cad.includes("*self.owned = Arc::into_inner(root)") && !cad.includes("Arc::try_unwrap(root)");
  if (!exactLocal(storeSource, retirementSource, cadSource)) throw new Error("presence local read, live return, detached close or final-owner authority is incomplete");
  const localHostiles = [
    [storeSource.replace("pub base: ArtifactEphemeralBaseRead<P>", "pub base: Arc<P>"), retirementSource, cadSource],
    [storeSource.replace("self.local_reads.try_issue(owner.clone())", "unregistered_read(owner.clone())"), retirementSource, cadSource],
    [storeSource.replace("Arc::ptr_eq(installed, &root_retirement_factory)", "true"), retirementSource, cadSource],
    [storeSource, retirementSource.replace("self.active_returned_local.take()", "None"), cadSource],
    [storeSource, retirementSource, cadSource.replace("Arc::into_inner(root)", "Arc::try_unwrap(root).ok()")],
  ];
  for (const [store, retirement, cad] of localHostiles) if (exactLocal(store, retirement, cad)) throw new Error("presence local guard accepted a raw alias, foreign factory, lost returned owner or shared wait");
  const closeBinding = storeFixture.closeFactoryBinding;
  const closeCounts = ["local", "peer"].reduce((counts, lane) => ({ ...counts, [lane]: counts[lane] + 1 }), { local: 0, peer: 0, foreign: 0 });
  if (JSON.stringify(closeCounts) !== JSON.stringify({ local: closeBinding.expectedLocal, peer: closeBinding.expectedPeer, foreign: closeBinding.expectedForeign })) throw new Error("Presence close factory oracle substituted an installed owner");
  const closeSchemaHostiles = [
    { ...storeFixture, closeFactoryBinding: { ...closeBinding, expectedForeign: 2 } },
    { ...storeFixture, closeFactoryBinding: { ...closeBinding, expectedLocal: 0 } },
    { ...storeFixture, closeFactoryBinding: { ...closeBinding, returnReadAfterDetach: false } },
  ];
  for (const hostile of closeSchemaHostiles) if (validateStore(hostile)) throw new Error("Presence close schema admitted foreign or lost returned-read ownership");
  const exactCloseFactories = (source: string): boolean => {
    const start = source.indexOf("pub fn begin_retirement(");
    const open = source.indexOf("{", start);
    const signature = source.slice(start, open);
    const body = start < 0 ? "" : toolJobRustBlock(source, open)?.body ?? "";
    const closeStart = source.indexOf("pub fn close_step(");
    const close = closeStart < 0 ? "" : toolJobRustBlock(source, source.indexOf("{", closeStart))?.body ?? "";
    return !signature.includes("factory:") && body.includes("let Some(local_factory) = self.local_retirement_factory.as_ref() else")
      && body.includes("let local_factory = local_factory.clone();") && body.includes("let peer_factory = self.peer_retirement_factory.clone();")
      && body.includes("!self.peers.is_empty() && self.peer_retirement_factory.is_none()")
      && close.includes("advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref()")
      && close.includes('self.local_factory.as_ref().expect("detached local root retains its installed factory").retire(local)')
      && close.includes('self.peer_factory.as_ref().expect("detached nonempty peer root retains its installed factory").clone()');
  };
  if (!exactCloseFactories(retirementSource)) throw new Error("Presence close lost exact original local/peer factory separation");
  const closeSourceHostiles = [
    retirementSource.replace("terminal_is_empty: fn(&P) -> bool,", "terminal_is_empty: fn(&P) -> bool, factory: Arc<dyn SnapshotRetirementFactory<P>>,"),
    retirementSource.replace("let local_factory = local_factory.clone();", "let local_factory = self.peer_retirement_factory.clone().unwrap();"),
    retirementSource.replace("let peer_factory = self.peer_retirement_factory.clone();", "let peer_factory = self.local_retirement_factory.clone();"),
    retirementSource.replace("advance_returned_local(reads, &mut self.active_returned, self.local_factory.as_ref()", "advance_returned_local(reads, &mut self.active_returned, self.peer_factory.as_ref()"),
    retirementSource.replace("!self.peers.is_empty() && self.peer_retirement_factory.is_none()", "false"),
  ];
  for (const hostile of closeSourceHostiles) if (hostile === retirementSource || exactCloseFactories(hostile)) throw new Error("Presence close guard admitted factory substitution or wrong returned-read retirement");
  const closeFactoryChecks = 2 + closeSchemaHostiles.length + closeSourceHostiles.length;
  const commitFixture = JSON.parse(readFileSync(join(storeBase, "📌️peer-commit.json"), "utf8"));
  const commitSchema = JSON.parse(readFileSync(join(storeBase, "🧬️schema/📌️peer-commit.schema.json"), "utf8"));
  const validateCommit = new Ajv({ strict: true, allErrors: true }).compile(commitSchema);
  if (!validateCommit(commitFixture)) throw new Error("Presence peer commit fixture violates strict schema");
  for (const law of commitFixture.cases) if (law.accepted !== (law.sameStore && law.sameFactory && !law.stale) || law.expectedSnapshots !== 3 + Number(law.stale)) throw new Error(`Presence peer commit independent identity oracle: ${law.name}`);
  const commitSchemaHostiles = [{ ...commitFixture, maximumBytes: 8192 }, { ...commitFixture, cases: commitFixture.cases.map(law => ({ ...law, accepted: true })) }];
  for (const hostile of commitSchemaHostiles) if (validateCommit(hostile)) throw new Error("Presence peer commit schema admitted forged freshness");
  const exactPeerCommit = toolJobPeerCommitAuthorityExact;
  if (!exactPeerCommit(storeSource, retirementSource)) throw new Error("Presence peer commit lost exact base/factory join or retained alias handoff");
  const commitSourceHostiles = [
    [storeSource.replace("!Arc::ptr_eq(&self.peers, &commit.base_root)", "false"), retirementSource],
    [storeSource.replace("Arc::ptr_eq(factory, &commit.factory)", "true"), retirementSource],
    [storeSource.replace("*retirement.root = Some(previous);\n        drop(commit.base_root);", "drop(commit.base_root);\n        *retirement.root = Some(previous);"), retirementSource],
    [storeSource, retirementSource.replace("base_root: std::mem::ManuallyDrop::new(Some(self.base_root))", "base_root: std::mem::ManuallyDrop::new(None)")],
    [storeSource, retirementSource.replace("if self.base_root.take().is_some()", "if false")],
  ];
  for (const [store, retirement] of commitSourceHostiles) if (exactPeerCommit(store, retirement)) throw new Error("Presence peer commit guard admitted foreign/stale publication or lost base ownership");
  const commitChecks = 2 + commitFixture.cases.length + commitSchemaHostiles.length + commitSourceHostiles.length;
  const peerFixture = JSON.parse(readFileSync(join(storeBase, "🛂️peer-admission.json"), "utf8"));
  const peerSchema = JSON.parse(readFileSync(join(storeBase, "🧬️schema/🛂️peer-admission.schema.json"), "utf8"));
  const validatePeer = new Ajv({ strict: true, allErrors: true }).compile(peerSchema);
  if (!validatePeer(peerFixture)) throw new Error(`peer admission fixture schema: ${JSON.stringify(validatePeer.errors)}`);
  for (const law of peerFixture.cases) {
    const bytes = Buffer.byteLength(law.actor.unit.repeat(law.actor.repeat), "utf8");
    if (bytes !== law.expectedActorBytes || law.accepted !== (law.state === "ready" && bytes > 0 && bytes <= 256) || law.actor.minimumCapacity <= peerFixture.maximumBytes) throw new Error(`peer actor admission independent byte oracle: ${law.name}`);
  }
  if (validatePeer({ ...peerFixture, requiresCapacitySizedByteGrant: true })) throw new Error("peer actor fixture admitted capacity-sized byte credit");
  const rejectionSource = readFileSync(join(storeBase, "🚫️rejection/🦀️.rs"), "utf8");
  const pluginSource = readFileSync(join(storeBase, "../../🔌️plugin/🦀️.rs"), "utf8");
  const exactRejectedActor = (store: string, rejection: string, plugin: string): boolean => {
    const start = store.indexOf("pub fn adopt(&mut self, actor: String, presence: P");
    const adopt = start < 0 ? "" : toolJobRustBlock(store, store.indexOf("{", start))?.body ?? "";
    return (adopt.match(/return Err\(PresencePeerAdmissionRejected::new\("[^"\n]+", actor, presence, self\.factory\.clone\(\)\)\)/g) ?? []).length === 5
      && !store.includes("pub fn retire_rejected(")
      && rejection.includes("pub fn into_retirement(mut self) -> Box<dyn ErasedSnapshotRetirement>")
      && rejection.includes("self.factory.take().expect(\"rejected admission retains its minting publication factory\")")
      && rejection.includes("actor: std::mem::ManuallyDrop<String>")
      && rejection.includes("Some(actor.into_bytes())")
      && rejection.includes("let released_bytes = actor.len().min(maximum_bytes)")
      && rejection.includes("actor.truncate(actor.len() - released_bytes)")
      && rejection.includes("drop(self.actor.take())")
      && !rejection.includes("capacity() > maximum_bytes")
      && plugin.includes("Some(rejected.into_retirement())");
  };
  if (!exactRejectedActor(storeSource, rejectionSource, pluginSource)) throw new Error("peer rejection lost its exact actor/presence owner or byte cursor");
  const rejectedHostiles = [
    [storeSource.replace('"presence peer actor is empty or exceeds its fixed byte authority", actor, presence', '"presence peer actor is empty or exceeds its fixed byte authority", String::new(), presence'), rejectionSource, pluginSource],
    [storeSource, rejectionSource.replace("actor.len().min(maximum_bytes)", "actor.capacity()"), pluginSource],
    [storeSource, rejectionSource.replace("drop(self.actor.take())", "return Err(\"capacity() > maximum_bytes\".into())"), pluginSource],
    [storeSource, rejectionSource, pluginSource.replace("Some(rejected.into_retirement())", "None")],
    [storeSource.replaceAll("actor, presence, self.factory.clone()", "actor, presence, foreign_factory.clone()"), rejectionSource, pluginSource],
    [storeSource, rejectionSource.replace('self.factory.take().expect("rejected admission retains its minting publication factory")', "foreign_factory"), pluginSource],
  ];
  for (const [store, rejection, plugin] of rejectedHostiles) if (exactRejectedActor(store, rejection, plugin)) throw new Error("peer rejection guard accepted dropped identity, false byte credit or missing mounted owner");
  if (validatePeer({ ...peerFixture, factoryBinding: { ...peerFixture.factoryBinding, expectedForeignRetirements: 1 } })) throw new Error("peer rejection fixture accepted a foreign factory");
  const channelSource = readFileSync(join(storeBase, "../../📡️spr/🧵️channel/🦀️.rs"), "utf8");
  const captureProof = (plugin: string, store: string, channel: string, retirement: string): boolean => toolJobPeerInteractionRootsExact(plugin, store, channel, retirement);
  if (!captureProof(pluginSource, storeSource, channelSource, retirementSource)) throw new Error("peer capture census rejected its real exact helper/base/factory authority");
  const captureHostiles = [
    [pluginSource.replace("self.start_typed_command_operation(command, admission, meta, operation_id, None).await", "self.foreign_command_operation(command, admission, meta, operation_id, None).await"), storeSource, channelSource, retirementSource],
    [pluginSource.replace("let presence_peers = self.presence_store.peers_root();", "let presence_peers = self.presence_store.peers().await;"), storeSource, channelSource, retirementSource],
    [pluginSource, storeSource.replace("!Arc::ptr_eq(&self.peers, &commit.base_root)", "false"), channelSource, retirementSource],
    [pluginSource, storeSource.replace("Arc::ptr_eq(factory, &commit.factory)", "true"), channelSource, retirementSource],
    [pluginSource, storeSource.replace("base_root: Arc<PresencePeersRoot<P>>,", "pub base_root: Arc<PresencePeersRoot<P>>,"), channelSource, retirementSource],
    [pluginSource, storeSource.replace("pub struct PresencePeersCommit<P> {", "pub struct PresencePeersCommit<P> { pub factory_override: Arc<dyn SnapshotRetirementFactory<P>>,"), channelSource, retirementSource],
    [pluginSource, storeSource.replace("*retirement.root = Some(previous);\n        drop(commit.base_root);", "drop(commit.base_root);\n        *retirement.root = Some(previous);"), channelSource, retirementSource],
    [pluginSource, storeSource, channelSource, retirementSource.replace("base_root: std::mem::ManuallyDrop::new(Some(self.base_root))", "base_root: std::mem::ManuallyDrop::new(None)")],
    [pluginSource.replace("self.validate_peer_roster_publication(seq, generation, &cancel)", "self.accept_unchecked_roster(seq, generation, &cancel)"), storeSource, channelSource, retirementSource],
    [pluginSource, storeSource, channelSource.replace("pub fn admit_page(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage)", "pub fn decode_before_admission(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage)"), retirementSource],
  ];
  for (const [plugin, store, channel, retirement] of captureHostiles) {
    if (plugin === pluginSource && store === storeSource && channel === channelSource && retirement === retirementSource) throw new Error("peer capture hostile missed its exact source target");
    if (captureProof(plugin, store, channel, retirement)) throw new Error("peer capture census admitted a forged helper, public authority, stale root, foreign factory or bypassed ingress");
  }
  return 1 + fixture.cases.length + fixture.storeCases.length + 2 + 1 + storeFixture.cases.length + 30 + peerFixture.cases.length + closeFactoryChecks + commitChecks + 1 + captureHostiles.length;
}
