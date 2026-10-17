/** 🧭️ Trusted bootstrap materialization, rotation and publication of the current trusted catalog.
 * @see ../../../📦️packages/🦀️rust/📜️script.ts
 */
import { withTrustedCatalogCleanupV1 } from "../🧹️settlement/🟦️.ts";
import { readTrustedCatalogPublicationProvenanceV1 } from "../🟦️.ts";
import { trustedCargoCompilationPlanV1 } from "../📦️cargo/🟦️.ts";
import { readCompletedCargoPublicationReferenceV1, type CompletedCargoPublicationReferenceV1 } from "@semio-tech/process/artifacts/native-build/receipt/retention/reference";
import { withTrustedCargoCompilationPacketsV1 } from "../📦️cargo/📥️reading/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import { withRepositoryArtifactDestinationV1, stageRepositoryArtifacts } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { withDescriptorEmitterV1, withDescriptorEmitterTaskV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { type CompilerAdmissionControl } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { withRequestedIssuedResourceControlV1, requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { parseSpaceArtifactCreationStatusJsonV1 } from "@semio-tech/framework-os/modules/directory/io/text/space-artifact-creation-v1";
import { trustedCatalogByteClaimV1, createTrustedCatalogProvenanceDirectoryV1, writeTrustedCatalogPublicationProvenanceV1, type ActorProducerV1, type PackageProducerV1, type GenerationProducerV1 } from "../🟦️.ts";
import { observePhysicalFileV1, readPhysicalFileV1, type FileObservationControlV1 } from "@semio-tech/framework/filesystem/observation";
import { withExactCargoLawOutputV1 } from "@semio-tech/process/testing/cargo/exact";
import { withCheckInProcessFixtureV1 } from "../📦️cargo/🧫️fixture/📥️reading/🟦️.ts";
import { buildBudgetMs } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { chmodSync, closeSync, existsSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, renameSync, rmSync, writeSync } from "node:fs";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { setTimeout as waitForHubPhaseV1 } from "node:timers/promises";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { spawn } from "node:child_process";
import { buildRepositoryCargoArtifacts } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { decodePackValue, encodePackValue, packValueToExactJson } from "@semio-tech/framework-os";
import type { PackValue } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, parseDocumentOpenPlanV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { DOCUMENT_BROWSER_ACTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema/browser-actor";
import { sealSpaceArtifactCreateV1 } from "@semio-tech/framework-os/modules/directory/schema/space-artifact-creation-v1";
import { produceFreshComponentV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";
import { type FreshBuildControlV1, type FreshComponentReceiptV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧾️source-epoch/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts";
import { buildClosedBrowserActorArtifactV1, type ClosedBrowserActorArtifactV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts";
import { TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧬️schema/🟦️.ts";
import { runRepositoryExactCargoLaws, readStableBuildFile, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { hubSchemaExport } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../../🏗️build/🛂staging-environment/🟦️.ts";
import { issueLocalCredential } from "../../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS, type LocalHubRun, type LocalHubHostV1, withLocalHubHostTaskV1, withLocalHubHostCandidateV1, withLocalHubRunTaskV1, withCapturedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { captureTrustedBootstrapCodecsV1, trustedBootstrapBuildControl, type TrustedBootstrapBundleV1, trustedBootstrapCarryPluginModule, type TrustedBootstrapCurrentPointerV1, trustedBootstrapFileObservationControlV1, trustedBootstrapFsyncDirectory, trustedBootstrapGenerationReceipt, type TrustedBootstrapGenerationReceiptV1, trustedBootstrapHubOpenTargetsV1, type TrustedBootstrapMaterializationV1, trustedBootstrapPluginModuleV1, type TrustedBootstrapPublishedV1, trustedBootstrapRecordReceipt, trustedBootstrapVerifyGeneration, trustedBootstrapWriteNew, UNLIMITED_TRUSTED_BUILD_DEADLINE_MS } from "../🗃️bootstrap-bundle/🟦️.ts";
import { documentOpenNeutralObject, documentOpenNeutralText, trustedBootstrapBrowserActorV1, type TrustedBootstrapBrowserActorV1, trustedBootstrapClosureEncoding, type TrustedBootstrapCodec, trustedBootstrapDependencyEncoding, trustedBootstrapDescriptorClaims, type TrustedBootstrapDescriptorClaimsV1, trustedBootstrapExecutionProtocol, type TrustedBootstrapOpenTargetV1, trustedBootstrapPluginModuleBlobPath, type TrustedBootstrapPluginModuleRecordV1, trustedBootstrapProfileEncoding, trustedBootstrapResolveClosure, trustedBootstrapResolveDependencies, trustedBootstrapTupleOrder } from "../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapComponentCodecRowsV1, trustedBootstrapDescriptorKindsV1, trustedBootstrapHostedKindsV1, trustedBootstrapLinkedUnownedKindsV1, type TrustedBootstrapPackageSpecV1, trustedBootstrapPreflightDescriptorsV1, trustedBootstrapProfileIdV1, trustedBootstrapReadRegular } from "../🧭️bootstrap-selection/🟦️.ts";
import { proveBrowserActorGisChildV1 } from "../🌍️gis-browser-proof/🟦️.ts";
import { liveDirectoryCommandRequestId, postLiveDirectoryCommand } from "../../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { webSha256Hex } from "../../../🚀️local-relay/🛰️servers/🟦️.ts";

/** 🏗️ Produces one immutable closed N-package generation without loading or registering codecs. */

export async function withMaterializedTrustedCatalogBundleV1<T>(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,selection:readonly TrustedBootstrapPackageSpecV1[],consume:(receipt:TrustedBootstrapMaterializationV1,phase:CompilerAdmissionControl)=>Promise<T>):Promise<T> {
  if(typeof consume!=="function")throw Error("Actual retained trusted catalog consumer required");
  selection=Object.freeze(selection.map(spec=>Object.freeze({...spec})));
  return withLocalHubHostTaskV1(host,phaseControl,context=>{
  const {invocation,workspace:repoRoot,dataRoot,binaryPath:hubBinary}=context;
  const budget=buildBudgetMs(),limit=budget>0?Math.min(budget,UNLIMITED_TRUSTED_BUILD_DEADLINE_MS):UNLIMITED_TRUSTED_BUILD_DEADLINE_MS;
  return withRequestedIssuedResourceControlV1(context.control,limit,async compiler=>{
  trustedBootstrapPreflightDescriptorsV1(repoRoot, selection);
  const trustedRoot = join(dataRoot, "trusted-catalog");
  mkdirSync(trustedRoot, { recursive: true, mode: 0o700 });
  if (lstatSync(trustedRoot).isSymbolicLink() || !lstatSync(trustedRoot).isDirectory()) throw new Error("trusted catalog root must be a regular private directory");
  if (process.platform !== "win32") chmodSync(trustedRoot, 0o700);
  const nonce = randomBytes(16).toString("hex");
  const buildRoot = join(trustedRoot, `build-${nonce}`);
  const stageRoot = join(trustedRoot, `staging-${nonce}`);
  mkdirSync(buildRoot, { mode: 0o700 });
  mkdirSync(stageRoot, { mode: 0o700 });
  const observationStage = join(trustedRoot, `provenance-staging-${nonce}`);
  mkdirSync(observationStage, { mode: 0o700 });
  const control={invocation,compiler,diagnosticsRoot:buildRoot,cancelled:()=>compiler.signal.aborted,remainingMs:()=>{const remaining=checkResourceLeaseControl(compiler);if(remaining===null)throw Error("Finite actual catalog compiler phase required");return remaining;},checkpoint:(stage:string,completed:number,total:number)=>{checkResourceLeaseControl(compiler);}};
  const checkBuild = () => {
    checkResourceLeaseControl(control.compiler);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog build cancelled");
  };
  const observationControl=trustedBootstrapFileObservationControlV1(control);
  return withTrustedCatalogCleanupV1(async()=>{
    const { codecs: linkedCodecs, gisVersion } = captureTrustedBootstrapCodecsV1(repoRoot, () => {
      if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted codec capture cancelled");
      control.checkpoint("capture-codecs", 0, 8);
    });
    const requests = selection.map((spec) => ({ pluginId: spec.pluginId, cargoPackage: spec.cargoPackage, componentPackageId: spec.componentPackageId, outputName: spec.outputName, componentProfile: "release" as const }));
    const receipts = new Map<string, FreshComponentReceiptV1>();
    const compilerInvocations = new Map<string, readonly CompletedCargoPublicationReferenceV1[]>();
    const actorProducers = new Map<string, Omit<ActorProducerV1,"compiler"> & {compilerName:string}>();
    const descriptorClaims = new Map<string, TrustedBootstrapDescriptorClaimsV1>();
    const browserActors = new Map<string, TrustedBootstrapBrowserActorV1>();
    const pluginModules = new Map<string, TrustedBootstrapPluginModuleRecordV1>();
    const descriptorJson = new Map<string, Record<string, any>>();
    const codecs: Record<string, readonly TrustedBootstrapCodec[]> = {};
    const openTargets: TrustedBootstrapOpenTargetV1[] = [];
    const declaredOpenTargets = new Map<string, (codecs: readonly TrustedBootstrapCodec[], unowned: ReadonlySet<string>, unbound: ReadonlySet<string>) => Promise<readonly TrustedBootstrapOpenTargetV1[]>>();
    const unownedKinds = new Map<string, ReadonlySet<string>>();
    for (const request of requests) {
      const spec = selection.find((candidate) => candidate.pluginId === request.pluginId)!;
      const target = join(buildRoot, `${request.pluginId}-target`);
      const stage = join(stageRoot, "packages", request.pluginId);
      mkdirSync(target, { recursive: true, mode: 0o700 });
      mkdirSync(stage, { recursive: true, mode: 0o700 });
      let derivedActor: ClosedBrowserActorArtifactV1 | undefined;
      try {
        const {receipt,derived:componentSha256}=await produceFreshComponentV1(repoRoot,request,stage,control,async(lease,evidence)=>lease.consume(async component=>{
          const digest=createHash("sha256").update(component).digest("hex"),retained:CompletedCargoPublicationReferenceV1[]=[];
          for(const row of evidence.observations)await evidence.withCompilation(row.kind,row.pass,async view=>{
            if(view.observationSha256!==row.sha256||createHash("sha256").update(row.bytes).digest("hex")!==row.sha256||view.observation!==new TextDecoder("utf-8",{fatal:true}).decode(row.bytes))throw Error("Exact unchanged completed compiler observation required");
            const index=view.files.get("index.json"),receipt=view.files.get("receipt.json");if(!index||!receipt||receipt.sha256!==view.observationSha256)throw Error("Complete original compiler packet required");
            const reference=readCompletedCargoPublicationReferenceV1({version:1,mode:view.origin.mode,kind:row.kind,pass:row.pass,receiptSha256:receipt.sha256,indexSha256:index.sha256,originalArtifact:row.originalArtifact});
            await view.recheck();
            await withRepositoryCacheStoreV1(repoRoot,"products",invocation,view.control,(store,phase)=>{const plan=trustedCargoCompilationPlanV1(store.root,reference);return withRepositoryArtifactDestinationV1({workspace:repoRoot,invocation,storage:store.storage,control:store.control,phaseControl:phase,directory:plan.path},destination=>stageRepositoryArtifacts(destination,plan.owner,new Map([...view.files].map(([name,file])=>[name,file.path]))));});
            await view.recheck();retained.push(reference);
          });
          if(retained.filter(row=>row.kind==="component"&&row.originalArtifact?.sha256===digest).length!==1||retained.filter(row=>row.kind==="emitter"&&row.originalArtifact!==null).length!==1)throw Error("One actual bound component and emitter completion required");
          compilerInvocations.set(request.pluginId,Object.freeze(retained));return digest;
        }));
        if (componentSha256 !== receipt.component.sha256 || (derivedActor && derivedActor.componentSha256 !== componentSha256)) throw new Error(`fresh ${request.pluginId} derivation differs from its verified component`);
        if (receipt.pluginId !== request.pluginId || receipt.packageId !== request.componentPackageId) throw new Error(`fresh ${request.pluginId} receipt identity changed after production`);
        const descriptor = trustedBootstrapReadRegular(join(stage, "descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "fresh compiled descriptor protocol", checkBuild);
        try {
          if (descriptor.byteLength !== receipt.descriptor.byteLength || createHash("sha256").update(descriptor).digest("hex") !== receipt.descriptor.sha256) throw new Error("fresh descriptor protocol bytes differ from the producer receipt");
          descriptorClaims.set(request.pluginId, trustedBootstrapDescriptorClaims(descriptor));
          descriptorJson.set(request.pluginId, packValueToExactJson(decodePackValue(descriptor)) as Record<string, any>);
          const descriptorCopy = Uint8Array.from(descriptor);
          declaredOpenTargets.set(request.pluginId, async (rows, unowned, unbound) => {
            try {
              return await trustedBootstrapHubOpenTargetsV1(host,control.compiler,request.pluginId,descriptorCopy,rows,unowned,unbound);
            } finally {
              descriptorCopy.fill(0);
            }
          });
        } finally {
          descriptor.fill(0);
        }
        // 🧬️ The package's codec closure. A package the hub LINKS Rust codecs for must carry exactly
        // the rows its compiled provider previews, so those come from the same registry the provider
        // is generated from; every other package answers for itself, through the `codecs` subcommand
        // of the descriptor emitter this build already produced, on these exact component bytes.
        const manifestKinds = trustedBootstrapDescriptorKindsV1(descriptorJson.get(request.pluginId)!).map((kind) => [String(kind.id), String(kind.schema)] as const);
        const hostedKinds = trustedBootstrapHostedKindsV1(request.pluginId, descriptorJson.get(request.pluginId)!, codecs);
        if (spec.linkedCodecRegistry) {
          const linked = linkedCodecs[request.pluginId as "gis" | "stdio"];
          if (!linked || linked.length === 0) throw new Error(`trusted codec capture carries no linked closure for ${request.pluginId}`);
          codecs[request.pluginId] = linked;
          unownedKinds.set(request.pluginId, trustedBootstrapLinkedUnownedKindsV1(manifestKinds, linked));
        } else if (manifestKinds.length === 0 && hostedKinds.pairs.length > 0) {
          codecs[request.pluginId] = [];
          unownedKinds.set(request.pluginId, hostedKinds.unowned);
        } else {
          const answered=await withDescriptorEmitterV1(repoRoot,invocation,control.compiler,emitter=>withDescriptorEmitterTaskV1(emitter,invocation,async tool=>{const result=await trustedBootstrapComponentCodecRowsV1(invocation,tool.control,repoRoot,tool.path,join(stage,"component.wasm"),join(target,"component-codecs.json"),manifestKinds);await tool.recheck();return result;}));
          codecs[request.pluginId] = answered.rows;
          unownedKinds.set(request.pluginId, answered.unowned);
        }
        const openTargetsForPackage=declaredOpenTargets.get(request.pluginId);if(!openTargetsForPackage)throw Error("Actual retained descriptor answer required");
        for(const declared of await openTargetsForPackage([...codecs[request.pluginId]!,...hostedKinds.rows],unownedKinds.get(request.pluginId)??new Set(),hostedKinds.unbound))openTargets.push(declared);
        const pluginOpensDocuments = openTargets.some((declared) => declared.pluginId === request.pluginId);
        if (pluginOpensDocuments) {
          const componentBytes = trustedBootstrapReadRegular(join(stage, "component.wasm"), DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, `fresh ${request.pluginId} component for closed actor`, checkBuild);
          try {
            derivedActor = await buildClosedBrowserActorArtifactV1(componentBytes, invocation, control.compiler, { cancelled: () => control.cancelled() || control.remainingMs() <= 0, progress: (phase, completed, total) => control.checkpoint(`actor-${phase}`, completed, total) });
          } finally {
            componentBytes.fill(0);
          }
          if (derivedActor.componentSha256 !== componentSha256) throw new Error(`fresh ${request.pluginId} closed actor source differs from its verified component`);
        }
        const actor = trustedBootstrapBrowserActorV1(
          derivedActor
            ? {
                kind: "closed-browser-actor",
                schema: derivedActor.schema,
                codegenPolicy: derivedActor.codegenPolicy,
                sha256: derivedActor.sha256,
                sourceComponentSha256: componentSha256,
                sourceDescriptorByteSha256: receipt.descriptor.sha256,
                policySha256: derivedActor.policySha256,
                importInterfaces: derivedActor.importInterfaces,
                byteLength: derivedActor.byteLength,
                path: `packages/${request.pluginId}/browser/closed-actor.mjs`,
              }
            : { kind: "none" },
          { componentSha256, descriptorByteSha256: receipt.descriptor.sha256 },
          pluginOpensDocuments ? "wasm" : "react",
        );
        if (derivedActor) {
          mkdirSync(join(stage, "browser"), { mode: 0o700 });
          trustedBootstrapWriteNew(join(stage, "browser", "closed-actor.mjs"), derivedActor.bytes, checkBuild);
          trustedBootstrapFsyncDirectory(join(stage, "browser"));
          const compilerName = derivedActor.producer.compiler.sha256 + ".mjs";
          const compilerRoot = join(observationStage, "compilers"); mkdirSync(compilerRoot, {recursive:true,mode:0o700});
          const compilerPath = join(compilerRoot,compilerName);
          if (!existsSync(compilerPath)) trustedBootstrapWriteNew(compilerPath,derivedActor.producer.compiler.bytes,checkBuild);
          const compilerClaim = await observePhysicalFileV1(compilerPath,observationControl);
          if (compilerClaim.sha256 !== derivedActor.producer.compiler.sha256 || compilerClaim.byteLength !== derivedActor.producer.compiler.byteLength) throw new Error("trusted actor retained compiler changed");
          actorProducers.set(request.pluginId,{schema:"semio.os.closed-browser-actor-producer/v1",actor:await trustedCatalogByteClaimV1(stageRoot,join(stage,"browser","closed-actor.mjs"),observationControl),component:await trustedCatalogByteClaimV1(stageRoot,join(stage,"component.wasm"),observationControl),descriptor:await trustedCatalogByteClaimV1(stageRoot,join(stage,"descriptor.semio"),observationControl),policyCanonical:derivedActor.policyCanonical,policySha256:derivedActor.policySha256,runtime:derivedActor.producer.runtime,inputs:derivedActor.producer.inputs,compilerName});

        }
        browserActors.set(request.pluginId, actor);
        pluginModules.set(request.pluginId, await trustedBootstrapPluginModuleV1(repoRoot, request, stage, stageRoot, target, receipt, control, checkBuild));
        receipts.set(request.pluginId, receipt);
      } finally {
        derivedActor?.bytes.fill(0);
        derivedActor?.producer.compiler.bytes.fill(0);
      }
    }
    const gis = receipts.get("gis");
    if (gis && gis.version !== gisVersion) throw new Error("fresh GIS component version differs from captured native codecs");
    for (const spec of selection) {
      const rows = codecs[spec.pluginId] ?? [];
      if (rows.length === 0) throw new Error(`trusted ${spec.pluginId} closure carries no artifact codec`);
      // Openability is derived from verified descriptor editor targets (TC5); the table bit is advisory.
    }
    // 🔢️ `validate_bundle` requires the selected closure in canonical identity order, which is the
    // same order `trusted_profile_generation` frames it in; the REQUESTED order only names the
    // profile, so `--packages stdio,gis` keeps minting `local-stdio-gis-open-v1`.
    const selectedClosure = selection
      .map((spec) => ({ pluginId: spec.pluginId, packageId: spec.componentPackageId, version: receipts.get(spec.pluginId)!.version }))
      .sort((left, right) => trustedBootstrapTupleOrder([left.pluginId, left.packageId, left.version], [right.pluginId, right.packageId, right.version]));
    const dependencies = trustedBootstrapResolveClosure(trustedBootstrapProfileIdV1(selection), descriptorClaims, selectedClosure);
    const packageSummary = selectedClosure.map((identity) => {
      const receipt = receipts.get(identity.pluginId)!;
      return {
        pluginId: identity.pluginId,
        packageId: identity.packageId,
        version: identity.version,
        role: "plugin",
        componentSha256: receipt.component.sha256,
        componentBlake3: receipt.component.blake3,
        descriptorSha256: receipt.descriptor.sha256,
        receipt: receipt.componentReceipt,
        dependencies: dependencies.get(identity.pluginId)!,
        executionProtocol: descriptorClaims.get(identity.pluginId)!.executionProtocol,
        browserActor: browserActors.get(identity.pluginId)!,
        pluginModule: pluginModules.get(identity.pluginId)!,
        codecCount: codecs[identity.pluginId]!.length,
        targetCount: openTargets.filter((declared) => declared.pluginId === identity.pluginId).length,
      };
    });
    const profileSummary = { id: trustedBootstrapProfileIdV1(selection), selectedClosure, packages: packageSummary, openTargets };
    const selectedClosureSha256 = createHash("sha256").update(trustedBootstrapClosureEncoding(profileSummary)).digest("hex");
    const generationId = createHash("sha256").update(trustedBootstrapProfileEncoding(profileSummary, codecs)).digest("hex");
    const file = (plugin: string, receipt: FreshComponentReceiptV1) => ({
      pluginId: plugin,
      packageId: receipt.packageId,
      version: receipt.version,
      role: "plugin",
      dependencies: dependencies.get(plugin)!,
      component: { path: `packages/${plugin}/component.wasm`, byteLength: receipt.component.byteLength, sha256: receipt.component.sha256, blake3: receipt.component.blake3 },
      descriptor: { path: `packages/${plugin}/descriptor.semio`, byteLength: receipt.descriptor.byteLength, sha256: receipt.descriptor.sha256 },
      receipt: receipt.componentReceipt,
      executionProtocol: descriptorClaims.get(plugin)!.executionProtocol,
      browserActor: browserActors.get(plugin)!,
      pluginModule: pluginModules.get(plugin)!,
      nativeCodecs: codecs[plugin],
      openTargets: openTargets.filter((declared) => declared.pluginId === plugin).map(({ pluginId: _owner, ...target }) => target),
    });
    const bundle = {
      schemaVersion: 4,
      profiles: [
        {
          id: profileSummary.id,
          selectedClosure,
          selectedClosureSha256,
          openTargets: openTargets.map(({ pluginId: owner, ...target }) => ({ package: selectedClosure.find((identity) => identity.pluginId === owner)!, target })),
          generationId,
        },
      ],
      packages: selectedClosure.map((identity) => file(identity.pluginId, receipts.get(identity.pluginId)!)),
    };
    const bundleBytes = Buffer.from(`${JSON.stringify(bundle)}\n`, "utf8");
    if (bundleBytes.byteLength > 4 * 1024 * 1024) throw new Error("trusted catalog bundle exceeds 4 MiB");
    const bundlePath = join(stageRoot, "trusted-catalog.json");
    trustedBootstrapWriteNew(bundlePath, bundleBytes, checkBuild);
    for (const identity of selectedClosure) trustedBootstrapFsyncDirectory(join(stageRoot, "packages", identity.pluginId));
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
    trustedBootstrapFsyncDirectory(stageRoot);
    if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog bootstrap cancelled before publication");
    const references=[...compilerInvocations.values()].flat();
    return await withTrustedCargoCompilationPacketsV1(repoRoot,invocation,control.compiler,references,observationControl,dataRoot,async(packets,actualPhase)=>{
    for(const request of requests){const rows=compilerInvocations.get(request.pluginId);const receipt=receipts.get(request.pluginId);if(!rows||!receipt||rows.filter(row=>row.kind==="component"&&row.originalArtifact?.sha256===receipt.component.sha256).length!==1)throw Error("Exact actual model compiler binding required before adoption");}
    const generations = join(trustedRoot, "generations");
    mkdirSync(generations, { recursive: true, mode: 0o700 });
    if (lstatSync(generations).isSymbolicLink() || !lstatSync(generations).isDirectory()) throw new Error("trusted generations root must be a regular directory");
    const generationRoot = join(generations, generationId);
    const checkGeneration = () => {
      checkBuild();
      if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted catalog generation cancelled before publication");
      control.checkpoint("verify-generation", 8, 8);
    };
    if (existsSync(generationRoot)) {
      trustedBootstrapVerifyGeneration(
        generationRoot,
        bundleBytes,
        new Map(bundle.packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])),
        checkGeneration,
      );
      rmSync(stageRoot, { recursive: true, force: true });
    } else {
      trustedBootstrapVerifyGeneration(stageRoot, bundleBytes, new Map(bundle.packages.map((record) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), checkGeneration);
      renameSync(stageRoot, generationRoot);
      trustedBootstrapFsyncDirectory(generations);
    }
    const bundleSha256 = createHash("sha256").update(bundleBytes).digest("hex");
    const observationParent = join(trustedRoot,"provenance","generations",generationId); createTrustedCatalogProvenanceDirectoryV1(dataRoot,observationParent);
    const observationRoot = join(observationParent,nonce); renameSync(observationStage,observationRoot);
    const packageProducers: PackageProducerV1[] = [];
    for(const identity of selectedClosure){
      const request=requests.find(row=>row.pluginId===identity.pluginId)!,target=join(buildRoot,`${identity.pluginId}-target`),packageRoot=join(generationRoot,"packages",identity.pluginId),metadataRoot=join(observationRoot,"packages",identity.pluginId);
      const invocations=compilerInvocations.get(identity.pluginId);if(!invocations?.length)throw Error("Captured actual compiler observations required");
      const cargoInvocations=invocations;
      let browserActor: PackageProducerV1["browserActor"] = null;
      const actor=actorProducers.get(identity.pluginId);
      if(actor){const {compilerName,...record}=actor;const actorRecord:ActorProducerV1={...record,compiler:await observePhysicalFileV1(join(observationRoot,"compilers",compilerName),observationControl)};const actorPath=join(metadataRoot,"browser-producer.json");trustedBootstrapWriteNew(actorPath,Buffer.from(JSON.stringify(actorRecord)+"\n"),checkBuild);browserActor=await observePhysicalFileV1(actorPath,observationControl);}
      packageProducers.push({pluginId:identity.pluginId,packageId:identity.packageId,cargoPackage:request.cargoPackage,component:await trustedCatalogByteClaimV1(generationRoot,join(packageRoot,"component.wasm"),observationControl),descriptor:await trustedCatalogByteClaimV1(generationRoot,join(packageRoot,"descriptor.semio"),observationControl),cargoInvocations,browserActor});
    }
    const producer:GenerationProducerV1={schema:"semio.hub.trusted-catalog-producer/v1",profileId:profileSummary.id,generationId,bundle:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,"trusted-catalog.json"),observationControl),packages:packageProducers};
    const producerPath=join(observationRoot,"producer.json");trustedBootstrapWriteNew(producerPath,Buffer.from(JSON.stringify(producer)+"\n"),checkBuild);trustedBootstrapFsyncDirectory(observationRoot);
    const materialized={ profileId: profileSummary.id, generationId, bundleSha256, bundlePath: join(generationRoot, "trusted-catalog.json"), producerPath };
    for(const packet of packets)await packet.recheck();
    const result=await consume(materialized,actualPhase);
    for(const packet of packets)await packet.recheck();
    return result;
    });
  },[()=>rmSync(stageRoot,{recursive:true,force:true}),()=>rmSync(buildRoot,{recursive:true,force:true}),()=>rmSync(observationStage,{recursive:true,force:true})]);
  });
  });
}

/** 🧿️ Rebinds captured rotation bytes to the already retained current receipt before decoding. */
export function trustedBootstrapReadCurrentBundle(current: TrustedBootstrapBundleV1, check: () => void): Record<string, any> {
  const bytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "trusted rotation source bundle", check);
  try {
    if (createHash("sha256").update(bytes).digest("hex") !== current.bundleSha256) throw new Error("trusted rotation source bundle differs from its retained digest");
    const bundle = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    if (bundle.schemaVersion !== 4 || bundle.profiles?.length !== 1 || !Array.isArray(bundle.packages) || bundle.packages.length === 0 || bundle.profiles[0]?.id !== current.profileId || bundle.profiles[0]?.generationId !== current.generationId)
      throw new Error("trusted rotation source differs from the exact retained profile generation");
    return bundle;
  } finally {
    bytes.fill(0);
  }
}

/** 🌉️ Executes the cold-map laws against the exact verified GIS bytes retained by one generation. */
async function proveTrustedGisColdMapComponentV1(invocation: ScriptInvocation, phaseControl:CompilerAdmissionControl, repoRoot: string, current: TrustedBootstrapMaterializationV1): Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const check = () => {
    if (phaseControl.signal.aborted || (checkResourceLeaseControl(phaseControl)??Infinity) <= 0) throw new Error("trusted GIS cold-map law cancelled");
  };
  {
    const bundle = trustedBootstrapReadCurrentBundle(current, check);
    const packages = new Map<string, Record<string, any>>((bundle.packages ?? []).map((record: Record<string, any>) => [String(record.pluginId), record]));
    const gis = packages.get("gis");
    if (
      packages.size === 0 ||
      !gis ||
      gis.packageId !== "semio:gis" ||
      gis.component?.path !== "packages/gis/component.wasm" ||
      gis.descriptor?.path !== "packages/gis/descriptor.semio" ||
      !/^[a-f0-9]{64}$/u.test(gis.component.sha256 ?? "") ||
      !/^[a-f0-9]{64}$/u.test(gis.descriptor.sha256 ?? "")
    ) {
      throw new Error("trusted GIS cold-map law requires the exact verified bundle owner");
    }
    const generationRoot = dirname(current.bundlePath);
    const componentPath = join(generationRoot, "packages/gis/component.wasm");
    const descriptorPath = join(generationRoot, "packages/gis/descriptor.semio");
    const verifyRetainedBytes = (): void => {
      trustedBootstrapVerifyPublication(dirname(dirname(dirname(generationRoot))), current);
      const component = trustedBootstrapReadRegular(componentPath, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "trusted GIS cold-map component", check);
      const descriptor = trustedBootstrapReadRegular(descriptorPath, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "trusted GIS cold-map descriptor", check);
      try {
        if (
          component.byteLength !== gis.component.byteLength ||
          descriptor.byteLength !== gis.descriptor.byteLength ||
          createHash("sha256").update(component).digest("hex") !== gis.component.sha256 ||
          createHash("sha256").update(descriptor).digest("hex") !== gis.descriptor.sha256
        ) {
          throw new Error("trusted GIS cold-map bytes differ from the retained generation receipt");
        }
      } finally {
        component.fill(0);
        descriptor.fill(0);
      }
    };
    verifyRetainedBytes();
    const receipts = await runRepositoryExactCargoLaws({control:phaseControl,preparation: configuredRepositoryCargoPreparationPolicyV1(),cwd: repoRoot,groups: [
        {
          package: "semio-hub-gis",
          target: { kind: "test", name: "component_cold_map_patch" },
          laws: ["genuine_gis_component_cold_loads_and_patches_the_exact_tiled_map_surface", "genuine_gis_component_rejects_stale_cold_authority_before_loading"],
        },
      ],env: {
        ...process.env,
        RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432",
        SEMIO_GIS_COMPONENT_WASM: componentPath,
        SEMIO_GIS_COMPONENT_SHA256: gis.component.sha256,
        SEMIO_GIS_DESCRIPTOR_SHA256: gis.descriptor.sha256,
      },nativeEnv: { RUST_MIN_STACK: process.env.SEMIO_TEST_NATIVE_RUST_MIN_STACK ?? "268435456" },invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 120_000, lawMilliseconds: 300_000 },progress(event) {
        console.log(`trusted-gis-cold-map ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`);
      }});
    verifyRetainedBytes();
    for (const receipt of receipts) console.log(`trusted-gis-cold-map-receipt: ${JSON.stringify(receipt)}`);
    console.log(`trusted-gis-cold-map-component: generation=${current.generationId} component=${gis.component.sha256} descriptor=${gis.descriptor.sha256} exact=2; candidate publication and browser remain separate`);
  }
}

/** 🪢️ Executes the closed GIS actor of the exact published generation without rebuilding its parents. */
export async function proveTrustedGisClosedActorV1(repoRoot: string, dataRoot: string, current: TrustedBootstrapMaterializationV1, artifactRoot: string): Promise<void> {
  const work = mkdtempSync(join(artifactRoot, "retained-gis-browser-"));
  const owner = trustedBootstrapBuildControl(Math.min(buildBudgetMs() || 300_000, 300_000));
  const check = (): void => {
    if (owner.control.cancelled() || owner.control.remainingMs() <= 0) throw new Error("retained GIS browser proof cancelled");
  };
  let bundleBytes: Uint8Array | undefined, descriptor: Uint8Array | undefined, actorBytes: Uint8Array | undefined;
  try {
    const initialCurrent = trustedBootstrapCurrent(dataRoot);
    if (!initialCurrent || initialCurrent.generationId !== current.generationId || initialCurrent.bundleSha256 !== current.bundleSha256) throw new Error("retained GIS browser has no exact published selection");
    const bundle = trustedBootstrapReadCurrentBundle(current, check);
    bundleBytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "retained GIS browser bundle", check);
    if (createHash("sha256").update(bundleBytes).digest("hex") !== current.bundleSha256) throw new Error("retained GIS browser bundle changed");
    const packages = new Map<string, Record<string, any>>(bundle.packages.map((record: Record<string, any>) => [String(record.pluginId), record]));
    const gis = packages.get("gis");
    if (
      packages.size === 0 || !packages.has("stdio") || !gis || gis.packageId !== "semio:gis" ||
      gis.component?.path !== "packages/gis/component.wasm" || gis.descriptor?.path !== "packages/gis/descriptor.semio" ||
      gis.browserActor?.kind !== "closed-browser-actor" || gis.browserActor.path !== "packages/gis/browser/closed-actor.mjs" ||
      gis.browserActor.sourceComponentSha256 !== gis.component.sha256 ||
      gis.browserActor.sourceDescriptorByteSha256 !== gis.descriptor.sha256
    ) throw new Error("retained GIS browser parent identity differs");
    const receipts = new Map<string, TrustedBootstrapGenerationReceiptV1>();
    for (const plugin of ["gis", "stdio"] as const) {
      const record = packages.get(plugin)!;
      if (record.packageId !== `semio:${plugin}` || record.component?.path !== `packages/${plugin}/component.wasm` || record.descriptor?.path !== `packages/${plugin}/descriptor.semio`)
        throw new Error("retained GIS browser generation closure differs");
      receipts.set(plugin, trustedBootstrapGenerationReceipt(record));
    }
    const generationRoot = dirname(current.bundlePath);
    const verifyRetained = (): void => {
      check();
      for (const root of [dataRoot, join(dataRoot, "trusted-catalog"), join(dataRoot, "trusted-catalog", "generations")]) {
        const info = lstatSync(root);
        if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("retained GIS browser current traverses a non-directory");
      }
      const published = trustedBootstrapCurrent(dataRoot);
      if (!published || published.currentSha256 !== initialCurrent.currentSha256 || published.publicationRevision !== initialCurrent.publicationRevision || published.profileId !== current.profileId || published.generationId !== current.generationId || published.bundleSha256 !== current.bundleSha256 || published.bundlePath !== current.bundlePath)
        throw new Error("retained GIS browser current generation changed");
      trustedBootstrapVerifyGeneration(generationRoot, bundleBytes!, receipts, check);
    };
    verifyRetained();
    descriptor = trustedBootstrapReadRegular(join(generationRoot, "packages/gis/descriptor.semio"), DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "retained GIS browser descriptor", check);
    actorBytes = trustedBootstrapReadRegular(join(generationRoot, "packages/gis/browser/closed-actor.mjs"), DOCUMENT_BROWSER_ACTOR_MAX_BYTES, "retained GIS browser actor", check);
    if (descriptor.byteLength !== gis.descriptor.byteLength) throw new Error("retained GIS browser descriptor length changed");
    const proof = await proveBrowserActorGisChildV1(repoRoot, work, {
      bytes: actorBytes, sha256: gis.browserActor.sha256, byteLength: gis.browserActor.byteLength,
      componentSha256: gis.component.sha256, descriptorByteSha256: gis.descriptor.sha256,
    }, descriptor, owner.control);
    verifyRetained();
    console.log("trusted-retained-gis-browser-receipt: " + JSON.stringify({
      profileId: current.profileId, generationId: current.generationId, bundleSha256: current.bundleSha256,
      componentSha256: gis.component.sha256, descriptorSha256: gis.descriptor.sha256,
      actorSha256: gis.browserActor.sha256, actorByteLength: gis.browserActor.byteLength,
      normalizedDescriptor: true, ...proof,
    }));
    console.log("trusted-retained-gis-browser: exact published actor described in Chromium; no authenticated worker, cold-pair application, Shell renderer, Map mutation or two-peer claim");
  } finally {
    actorBytes?.fill(0);
    descriptor?.fill(0);
    bundleBytes?.fill(0);
    owner.close();
    rmSync(work, { recursive: true, force: true });
  }
}

/** 🧳️ Copies one already verified immutable selection below an isolated candidate's own data root. */
function stageTrustedBootstrapCandidateCurrent(candidateDataRoot: string, current: TrustedBootstrapBundleV1, check: () => void): void {
  const bundleBytes = trustedBootstrapReadRegular(current.bundlePath, 4 * 1024 * 1024, "trusted candidate source bundle", check);
  const stageRoot = join(candidateDataRoot, "trusted-catalog", `staging-${randomBytes(16).toString("hex")}`);
  try {
    if (createHash("sha256").update(bundleBytes).digest("hex") !== current.bundleSha256) throw new Error("trusted candidate source bundle differs from its retained digest");
    const bundle = trustedBootstrapReadCurrentBundle(current, check);
    const packages = new Map<string, any>(bundle.packages.map((record: any) => [record.pluginId, record]));
    if (packages.size === 0 || !packages.has("stdio")) throw new Error("trusted candidate source carries no package closure rooted at stdio");
    const receipts = new Map<string, TrustedBootstrapGenerationReceiptV1>();
    for (const plugin of packages.keys()) {
      const record = packages.get(plugin)!;
      if (record.packageId !== `semio:${plugin}` || record.component?.path !== `packages/${plugin}/component.wasm` || record.descriptor?.path !== `packages/${plugin}/descriptor.semio`)
        throw new Error(`trusted candidate ${plugin} path identity differs`);
      receipts.set(plugin, trustedBootstrapGenerationReceipt(record));
    }
    const sourceRoot = dirname(current.bundlePath);
    trustedBootstrapVerifyGeneration(sourceRoot, bundleBytes, receipts, check);
    const trustedRoot = join(candidateDataRoot, "trusted-catalog");
    const generationsRoot = join(trustedRoot, "generations");
    mkdirSync(generationsRoot, { recursive: true, mode: 0o700 });
    for (const directory of [candidateDataRoot, trustedRoot, generationsRoot]) {
      const info = lstatSync(directory);
      if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("trusted candidate owner root traverses a link or non-directory");
      if (process.platform !== "win32") chmodSync(directory, 0o700);
    }
    const generationRoot = join(generationsRoot, current.generationId);
    if (existsSync(generationRoot)) {
      trustedBootstrapVerifyGeneration(generationRoot, bundleBytes, receipts, check);
    } else {
      // 🧳️ Derived from the receipts the bundle itself carries, exactly the way
      // `trustedBootstrapVerifyGeneration` derives the closure it then checks this staging against.
      // Spelled as the fixed gis+stdio pair (ticket 26/09/18 slice TC3c generalised
      // `withMaterializedTrustedCatalogBundleV1` and left this copy behind) the candidate staged two
      // packages out of a three-package selection and the very next line's fence answered
      // `generation directory is not the exact regular closure` — TC3e measured that at 13:48:13 on
      // 2026-09-22, seven minutes into a hold, with the note package already published upstream.
      const stagedPlugins = [...receipts.keys()].sort();
      const stagedCarriesActor = (plugin: string): boolean => receipts.get(plugin)!.browserActor.kind === "closed-browser-actor";
      const files: { relative: string; maximum: number }[] = [];
      mkdirSync(join(stageRoot, "plugin-modules"), { recursive: true, mode: 0o700 });
      for (const plugin of stagedPlugins) {
        mkdirSync(stagedCarriesActor(plugin) ? join(stageRoot, "packages", plugin, "browser") : join(stageRoot, "packages", plugin), { recursive: true, mode: 0o700 });
        files.push({ relative: `packages/${plugin}/component.wasm`, maximum: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES });
        files.push({ relative: `packages/${plugin}/descriptor.semio`, maximum: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
        files.push({ relative: receipts.get(plugin)!.pluginModule.path, maximum: TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES });
        if (stagedCarriesActor(plugin)) files.push({ relative: `packages/${plugin}/browser/closed-actor.mjs`, maximum: DOCUMENT_BROWSER_ACTOR_MAX_BYTES });
      }
      for (const blob of readdirSync(join(sourceRoot, "plugin-modules")).sort()) files.push({ relative: trustedBootstrapPluginModuleBlobPath(blob), maximum: TRUSTED_PLUGIN_MODULE_FILE_MAX_BYTES });
      trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes, check);
      for (const file of files) {
        const bytes = trustedBootstrapReadRegular(join(sourceRoot, file.relative), file.maximum, `trusted candidate ${file.relative}`, check);
        try {
          trustedBootstrapWriteNew(join(stageRoot, file.relative), bytes, check);
        } finally {
          bytes.fill(0);
        }
      }
      for (const plugin of stagedPlugins) {
        if (stagedCarriesActor(plugin)) trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin, "browser"));
        trustedBootstrapFsyncDirectory(join(stageRoot, "packages", plugin));
      }
      trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
      trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
      trustedBootstrapFsyncDirectory(stageRoot);
      trustedBootstrapVerifyGeneration(stageRoot, bundleBytes, receipts, check);
      renameSync(stageRoot, generationRoot);
      trustedBootstrapFsyncDirectory(generationsRoot);
    }
    stageTrustedBootstrapProbePointer(candidateDataRoot, current);
    const staged = trustedBootstrapCurrent(candidateDataRoot);
    if (!staged || staged.profileId !== current.profileId || staged.generationId !== current.generationId || staged.bundleSha256 !== current.bundleSha256 || staged.bundlePath !== join(generationRoot, "trusted-catalog.json"))
      throw new Error("trusted candidate current pointer differs from its isolated generation");
  } finally {
    bundleBytes.fill(0);
    rmSync(stageRoot, { recursive: true, force: true });
  }
}

/** 🧪️ Independent WebCrypto receipt oracle exercises the production rotation capture boundary. */
export async function proveTrustedRotationSourceFixture(fixture: Record<string, any>): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { writeFileSync } = await import("node:fs");
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"));
  const evidence = mkdtempSync(join(artifactRoot, "rotation-source-"));
  for (const law of fixture.rotation.sourceCases) {
    const path = join(evidence, `${law.change}.json`);
    const bundle = { schemaVersion: 4, profiles: [{ id: fixture.profile.id, generationId: fixture.profile.generationId }], packages: [{}, {}] };
    const bytes = Buffer.from(JSON.stringify(bundle));
    const sha256 = await webSha256Hex(bytes);
    writeFileSync(path, bytes, { flag: "wx" });
    const receipt = { profileId: fixture.profile.id, generationId: fixture.profile.generationId, bundleSha256: sha256, bundlePath: path };
    if (law.change === "wrong-generation") receipt.generationId = "42".repeat(32);
    if (law.change === "wrong-profile") receipt.profileId = "foreign";
    if (law.change === "replaced-bundle") writeFileSync(path, JSON.stringify({ ...bundle, packages: [{ changed: true }, {}] }));
    let checks = 0,
      accepted = false;
    try {
      trustedBootstrapReadCurrentBundle(receipt, () => {
        checks++;
        if (law.change === "cancel-before-read") throw new Error("cancelled");
        if (law.change === "leaf-replaced-during-read" && checks === 2) {
          renameSync(path, `${path}.old`);
          writeFileSync(path, bytes, { flag: "wx" });
        }
      });
      accepted = true;
    } catch {}
    assert.equal(accepted, law.accepted, law.change);
  }
  for (const law of fixture.rotation.writeCases) {
    const path = join(evidence, `${law.id}.mjs`);
    const bytes = Buffer.alloc(law.byteLength, 42),
      previous = Buffer.from("previous owner");
    if (law.existing) writeFileSync(path, previous, { flag: "wx" });
    let checks = 0,
      accepted = false;
    try {
      trustedBootstrapWriteNew(path, bytes, () => {
        checks++;
        if (checks === law.cancelCheck) throw new Error("cancelled");
      });
      accepted = true;
    } catch {}
    assert.equal(accepted, law.accepted, law.id);
    assert.equal(existsSync(path), law.retained !== "absent", `${law.id} retained owner`);
    if (law.retained !== "absent") {
      const retained = readFileSync(path),
        expected = law.retained === "old" ? previous : bytes;
      assert.equal(createHash("sha256").update(retained).digest("hex"), await webSha256Hex(expected), `${law.id} exact bytes`);
    }
  }
  console.log(`trusted-rotation-source: WebCrypto=1 cases=${fixture.rotation.sourceCases.length} writes=${fixture.rotation.writeCases.length} evidence=${evidence}; capture/write ownership only, no generation publication`);
}

/** 🔄️ Reissues the zero-target Stdio descriptor as a distinct immutable, fully verified generation. */
export async function materializeTrustedStdioGisRotation(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,repoRoot: string, dataRoot: string, current: TrustedBootstrapPublishedV1): Promise<TrustedBootstrapMaterializationV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  const observation:FileObservationControlV1={maxBytes:134217728,maxWork:65536,chunkBytes:1048576,cancelled:()=>phaseControl.signal.aborted,remainingMs:()=>checkResourceLeaseControl(phaseControl)??Infinity,onProgress:()=>{checkResourceLeaseControl(phaseControl);}};
  const sourceProducer=await readTrustedCatalogPublicationProvenanceV1(dataRoot,observation);
  const retained = trustedBootstrapCurrent(dataRoot);
  if (!retained || JSON.stringify(retained) !== JSON.stringify(current)) throw new Error("trusted stdio+GIS rotation did not start from the retained current generation");
  const trustedRoot = join(dataRoot, "trusted-catalog");
  const generationsRoot = join(trustedRoot, "generations");
  const stageRoot = join(trustedRoot, `rotation-${randomBytes(16).toString("hex")}`);

  const checkGeneration = () => {
    if (phaseControl.signal.aborted || (checkResourceLeaseControl(phaseControl)??Infinity) <= 0) throw new Error("trusted rotation generation cancelled before publication");
    checkResourceLeaseControl(phaseControl);
  };
  return withTrustedCatalogCleanupV1(async()=>{
    const bundle = trustedBootstrapReadCurrentBundle(current, checkGeneration);
    const profile = bundle.profiles[0]!;
    mkdirSync(stageRoot, { mode: 0o700 });
    const codecs = {} as Record<"gis" | "stdio", TrustedBootstrapCodec[]>;
    for (const plugin of ["gis", "stdio"] as const) {
      const record = bundle.packages.find((candidate: any) => candidate.pluginId === plugin);
      if (!record || record.packageId !== `semio:${plugin}` || record.role !== "plugin" || !Array.isArray(record.nativeCodecs)) throw new Error(`trusted rotation lost ${plugin} package authority`);
      codecs[plugin] = record.nativeCodecs;
      const sourceRoot = join(generationsRoot, current.generationId, "packages", plugin);
      const destinationRoot = join(stageRoot, "packages", plugin);
      mkdirSync(destinationRoot, { recursive: true, mode: 0o700 });
      const sourceReceipt = trustedBootstrapGenerationReceipt(record);
      const component = trustedBootstrapReadRegular(join(sourceRoot, "component.wasm"), DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, `${plugin} rotation component`, checkGeneration);
      let descriptor = trustedBootstrapReadRegular(join(sourceRoot, "descriptor.semio"), 4 * 1024 * 1024, `${plugin} rotation descriptor`, checkGeneration);
      let actorBytes: Uint8Array | undefined;
      try {
        if (
          component.byteLength !== record.component.byteLength ||
          createHash("sha256").update(component).digest("hex") !== record.component.sha256 ||
          descriptor.byteLength !== record.descriptor.byteLength ||
          createHash("sha256").update(descriptor).digest("hex") !== record.descriptor.sha256
        )
          throw new Error(`trusted rotation ${plugin} source differs from its retained receipt`);
        const claims = trustedBootstrapDescriptorClaims(descriptor);
        const dependencies = trustedBootstrapResolveDependencies(claims, profile.selectedClosure);
        if (claims.identity.pluginId !== plugin || claims.identity.packageId !== record.packageId || claims.identity.version !== record.version || claims.executionProtocol.appChannelVersion !== trustedBootstrapExecutionProtocol(record.executionProtocol).appChannelVersion || !trustedBootstrapDependencyEncoding(dependencies).equals(trustedBootstrapDependencyEncoding(record.dependencies))) throw new Error("trusted rotation descriptor identity, protocol or dependencies differ from its retained receipt");
        record.executionProtocol = claims.executionProtocol;
        record.dependencies = dependencies;
        const actor = trustedBootstrapBrowserActorV1(record.browserActor, { componentSha256: record.component.sha256, descriptorByteSha256: record.descriptor.sha256 }, plugin === "gis" ? "wasm" : "react");
        record.browserActor = actor;
        if (actor.kind === "closed-browser-actor") {
          if (actor.path !== "packages/gis/browser/closed-actor.mjs") throw new Error("rotation actor path differs from fixed GIS closure");
          actorBytes = readStableBuildFile(join(sourceRoot, "browser", "closed-actor.mjs"), actor.byteLength, { remaining: actor.byteLength }, checkGeneration);
          if (actorBytes.byteLength !== actor.byteLength || createHash("sha256").update(actorBytes).digest("hex") !== actor.sha256) throw new Error("rotation actor differs from its verified receipt");
          mkdirSync(join(destinationRoot, "browser"), { mode: 0o700 });
          trustedBootstrapWriteNew(join(destinationRoot, "browser", "closed-actor.mjs"), actorBytes, checkGeneration);
          trustedBootstrapFsyncDirectory(join(destinationRoot, "browser"));
        }
        if (plugin === "stdio") {
          const value = decodePackValue(descriptor) as unknown as Record<string, any>;
          if (value.packageId !== record.packageId || value.manifest?.pluginId !== plugin || value.manifest?.version !== record.version || value.role !== record.role || value.execution !== "isolated" || "hashes" in value)
            throw new Error("trusted rotation Stdio descriptor identity changed");
          value.manifest.label = `Stdio trusted rotation ${randomBytes(8).toString("hex")}`;
          const updatedDescriptor = Buffer.from(encodePackValue(value));
          descriptor.fill(0);
          descriptor = updatedDescriptor;
          const json = Buffer.from(JSON.stringify(packValueToExactJson(value as PackValue)), "utf8");
          try {
            const pair = verifySemanticDescriptorPairBytesV1(json, descriptor);
            if (pair.pluginId !== plugin || pair.packageId !== record.packageId || pair.version !== record.version || pair.role !== "plugin" || pair.execution !== "isolated") throw new Error("trusted rotation Stdio descriptor identity changed");
          } finally {
            json.fill(0);
          }
          record.descriptor.byteLength = descriptor.byteLength;
          record.descriptor.sha256 = createHash("sha256").update(descriptor).digest("hex");
          record.receipt = { ...trustedBootstrapRecordReceipt({ ...record, descriptor: sourceReceipt.descriptor }), semanticDescriptorSha256: record.descriptor.sha256 };
        }
        trustedBootstrapWriteNew(join(destinationRoot, "component.wasm"), component, checkGeneration);
        trustedBootstrapWriteNew(join(destinationRoot, "descriptor.semio"), descriptor, checkGeneration);
        record.pluginModule = trustedBootstrapCarryPluginModule(join(generationsRoot, current.generationId), stageRoot, sourceReceipt, record, descriptor, checkGeneration);
        trustedBootstrapFsyncDirectory(destinationRoot);
      } finally {
        component.fill(0);
        descriptor.fill(0);
        actorBytes?.fill(0);
      }
    }
    const packageSummary = bundle.packages.map((record: any) => ({
      pluginId: record.pluginId,
      packageId: record.packageId,
      version: record.version,
      role: record.role,
      componentSha256: record.component.sha256,
      componentBlake3: record.component.blake3,
      descriptorSha256: record.descriptor.sha256,
      receipt: record.receipt,
      dependencies: record.dependencies,
      executionProtocol: record.executionProtocol,
      browserActor: record.browserActor,
      pluginModule: record.pluginModule,
      codecCount: record.nativeCodecs.length,
      targetCount: record.openTargets.length,
    }));
    const packageTargets = bundle.packages.reduce((total: number, record: any) => total + record.openTargets.length, 0);
    if (!Array.isArray(profile.openTargets) || profile.openTargets.length !== packageTargets) throw new Error("trusted rotation source profile does not open every package target");
    const profileSummary = { id: profile.id, selectedClosure: profile.selectedClosure, packages: packageSummary, openTargets: profile.openTargets.map((selection: any) => ({ pluginId: selection.package.pluginId, ...selection.target })) };
    const generationId = createHash("sha256").update(trustedBootstrapProfileEncoding(profileSummary, codecs)).digest("hex");
    if (generationId === current.generationId) throw new Error("trusted rotation did not change the full profile generation");
    profile.generationId = generationId;
    const bundleBytes = Buffer.from(`${JSON.stringify(bundle)}\n`, "utf8");
    if (bundleBytes.byteLength > 4 * 1024 * 1024) throw new Error("trusted rotation bundle exceeds 4 MiB");
    trustedBootstrapWriteNew(join(stageRoot, "trusted-catalog.json"), bundleBytes, checkGeneration);
    trustedBootstrapFsyncDirectory(join(stageRoot, "packages"));
    trustedBootstrapFsyncDirectory(join(stageRoot, "plugin-modules"));
    trustedBootstrapFsyncDirectory(stageRoot);
    const generationRoot = join(generationsRoot, generationId);
    if (existsSync(generationRoot)) throw new Error("trusted rotation generation already exists");
    trustedBootstrapVerifyGeneration(stageRoot, bundleBytes, new Map(bundle.packages.map((record: any) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), checkGeneration);
    renameSync(stageRoot, generationRoot);
    trustedBootstrapFsyncDirectory(generationsRoot);
    const observationRoot=join(trustedRoot,"provenance","generations",generationId,randomBytes(16).toString("hex"));createTrustedCatalogProvenanceDirectoryV1(dataRoot,observationRoot);
    const packages:PackageProducerV1[]=[];
    for(const original of sourceProducer.generation.packages){const record=bundle.packages.find((row:any)=>row.pluginId===original.pluginId);if(!record)throw Error("Rotation lost actual original package producer");let browserActor:PackageProducerV1["browserActor"]=null;if(original.browserActor){const input=await readPhysicalFileV1(original.browserActor.path,observation);if(input.claim.sha256!==original.browserActor.sha256||input.claim.byteLength!==original.browserActor.byteLength)throw Error("Rotation original actor producer changed");const path=join(observationRoot,original.pluginId+"-browser.json");trustedBootstrapWriteNew(path,input.bytes,checkGeneration);browserActor=await observePhysicalFileV1(path,observation);}packages.push({...original,component:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,record.component.path),observation),descriptor:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,record.descriptor.path),observation),browserActor});}
    const producer:GenerationProducerV1={...sourceProducer.generation,generationId,bundle:await trustedCatalogByteClaimV1(generationRoot,join(generationRoot,"trusted-catalog.json"),observation),packages};
    const producerPath=join(observationRoot,"producer.json");trustedBootstrapWriteNew(producerPath,Buffer.from(JSON.stringify(producer)+"\n"),checkGeneration);
    return { profileId: profile.id, generationId, bundleSha256: createHash("sha256").update(bundleBytes).digest("hex"), bundlePath: join(generationRoot, "trusted-catalog.json"),producerPath };
  },[()=>rmSync(stageRoot,{recursive:true,force:true})]);
}

export function trustedBootstrapPublicationRevision(value: unknown): string {
  if (typeof value !== "string" || !/^[1-9][0-9]{0,19}$/u.test(value) || BigInt(value) > 18_446_744_073_709_551_615n) throw new Error("trusted publication revision is not canonical nonzero u64");
  return value;
}

/** 📌️ Captures one canonical pointer token without following a link or reading its selection. */
export function trustedBootstrapReadCurrentPointer(dataRoot: string): Readonly<{ pointer: TrustedBootstrapCurrentPointerV1; sha256: string }> | undefined {
  const pointerPath = join(dataRoot, "trusted-catalog", "current.json");
  try {
    const info = lstatSync(pointerPath);
    if (info.isSymbolicLink() || !info.isFile()) throw new Error("trusted catalog current pointer is not a regular leaf");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
  const bytes = Buffer.from(trustedBootstrapReadRegular(pointerPath, 64 * 1024, "trusted catalog current pointer", () => {}));
  try {
    const row = documentOpenNeutralObject(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)), ["profileId", "generationId", "bundleSha256", "publicationRevision"]);
    if (typeof row.generationId !== "string" || typeof row.bundleSha256 !== "string" || !/^[0-9a-f]{64}$/u.test(row.generationId) || !/^[0-9a-f]{64}$/u.test(row.bundleSha256)) throw new Error("trusted catalog current digest is invalid");
    const pointer = Object.freeze({ profileId: documentOpenNeutralText(row.profileId), generationId: row.generationId, bundleSha256: row.bundleSha256, publicationRevision: trustedBootstrapPublicationRevision(row.publicationRevision) });
    if (!bytes.equals(Buffer.from(`${JSON.stringify(pointer)}\n`, "utf8"))) throw new Error("trusted catalog current pointer is not exact canonical metadata");
    return Object.freeze({ pointer, sha256: createHash("sha256").update(bytes).digest("hex") });
  } finally {
    bytes.fill(0);
  }
}

export function trustedBootstrapCurrent(dataRoot: string): TrustedBootstrapPublishedV1 | undefined {
  const current = trustedBootstrapReadCurrentPointer(dataRoot);
  if (!current) return undefined;
  const pointer = current.pointer;
  if (!/^local-(?:[a-z0-9]+-)+open-v1$/u.test(pointer.profileId)) throw new Error("trusted catalog current profile is not a local selection this builder mints");
  const bundlePath = join(dataRoot, "trusted-catalog", "generations", pointer.generationId, "trusted-catalog.json");
  const receipt = Object.freeze({ profileId: pointer.profileId, generationId: pointer.generationId, bundleSha256: pointer.bundleSha256, bundlePath, publicationRevision: pointer.publicationRevision, currentSha256: current.sha256 });
  trustedBootstrapReadCurrentBundle(receipt, () => {});
  return receipt;
}

/** 🔎️ Rechecks all exact generation bytes without mutating any selection. */
export function trustedBootstrapVerifyPublication(dataRoot: string, receipt: TrustedBootstrapBundleV1): void {
  const bundlePath = join(dataRoot, "trusted-catalog", "generations", receipt.generationId, "trusted-catalog.json");
  const retained = { ...receipt, bundlePath };
  const selected = trustedBootstrapReadCurrentBundle(retained, () => {});
  const bundleBytes = trustedBootstrapReadRegular(bundlePath, 4 * 1024 * 1024, "trusted publication bundle", () => {});
  try {
    if (createHash("sha256").update(bundleBytes).digest("hex") !== receipt.bundleSha256) throw new Error("trusted publication bundle changed before its final fence");
    trustedBootstrapVerifyGeneration(dirname(bundlePath), bundleBytes, new Map(selected.packages.map((record: any) => [record.pluginId, trustedBootstrapGenerationReceipt(record)])), () => {});
  } finally {
    bundleBytes.fill(0);
  }
}

/** 🧪️ Seeds only a never-published isolated probe; this path cannot replace a current pointer. */
function stageTrustedBootstrapProbePointer(dataRoot: string, receipt: TrustedBootstrapBundleV1): void {
  trustedBootstrapVerifyPublication(dataRoot, receipt);
  const pointer = Buffer.from(`${JSON.stringify({ profileId: receipt.profileId, generationId: receipt.generationId, bundleSha256: receipt.bundleSha256, publicationRevision: "1" })}\n`, "utf8");
  const trustedRoot = join(dataRoot, "trusted-catalog");
  trustedBootstrapWriteNew(join(trustedRoot, "current.json"), pointer, () => {});
  trustedBootstrapFsyncDirectory(trustedRoot);
}

/** 🧬️ Resolves the sole scope-owned publication receipt contract for the production decoder. */
function trustedCatalogPublicationReceiptContract(): (value: unknown) => boolean {
  return hubSchemaExport(resolve(dirname(fileURLToPath(import.meta.url)), "../../../.."), "schema://hub.artifact-authority.trusted-catalog/TrustedCatalogPublicationReceiptV1");
}

/** 🧾️ Preserves both native outcomes while matching the exact request and next canonical revision. */
export function trustedBootstrapPublicationReceipt(bytes: Uint8Array, command: Readonly<Record<string, unknown>>, expectedRevision: string): Readonly<{ currentSha256: string; publicationRevision: string; outcome: "durable" | "replaced-unconfirmed" }> {
  if (bytes.byteLength === 0 || bytes.byteLength > 4096) throw new Error("trusted publication receipt exceeds its bound; reconcile current");
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  const row = documentOpenNeutralObject(JSON.parse(text), ["schema", "requestId", "profileId", "generationId", "bundleSha256", "publicationRevision", "currentSha256", "outcome"]);
  if (!trustedCatalogPublicationReceiptContract()(row)) throw new Error("trusted publication receipt does not satisfy its scope-owned contract; reconcile current");
  if (["requestId", "profileId", "generationId", "bundleSha256"].some((key) => row[key] !== command[key]) || row.publicationRevision !== expectedRevision || text !== JSON.stringify(row) + "\n")
    throw new Error("trusted publication receipt is not exact; reconcile current");
  const publicationRevision = trustedBootstrapPublicationRevision(row.publicationRevision);
  const pointer = { profileId: row.profileId, generationId: row.generationId, bundleSha256: row.bundleSha256, publicationRevision };
  const currentSha256 = createHash("sha256").update(JSON.stringify(pointer) + "\n").digest("hex");
  if (row.currentSha256 !== currentSha256) throw new Error("trusted publication receipt token differs from its exact pointer; reconcile current");
  return Object.freeze({ currentSha256, publicationRevision, outcome: row.outcome });
}

/** 🚦️ Keeps a validated visible replacement distinct from missing or malformed process evidence. */
export class TrustedBootstrapPublicationUnconfirmed extends Error {
  constructor(readonly receipt: ReturnType<typeof trustedBootstrapPublicationReceipt>, readonly evidenceRoot: string) {
    super(`trusted publication replacement is visible but durability is unconfirmed; do not activate or retry; evidence=${evidenceRoot}`);
  }
}

type TrustedPublicationChildResultV1 = Readonly<{ bytes: Buffer; status: number | null; signal: string | null; failed: string }>;

/** 👶️ Retains one direct publication child until close with bounded evidence and cancellation. */
export async function runTrustedPublicationChild(binaryPath:string,args:readonly string[],input:Uint8Array,dataRoot:string,evidenceRoot:string,control:Pick<FreshBuildControlV1,"cancelled"|"remainingMs">&{progressed?:()=>void}):Promise<TrustedPublicationChildResultV1>{
 if(!isAbsolute(binaryPath)||!isAbsolute(dataRoot)||!isAbsolute(evidenceRoot)||input.byteLength===0||input.byteLength>4096)throw Error("publication child authority exceeds its boundary");
 return new Promise((accept,reject)=>{
  let stdout:number|undefined,stderr:number|undefined,child:ReturnType<typeof spawn>,timer:ReturnType<typeof setInterval>|undefined,closed=false,outputBytes=0,diagnosticBytes=0,failed="";const chunks:Buffer[]=[],failures:unknown[]=[];
  const record=(error:unknown)=>{if(!failures.some(value=>Object.is(value,error)))failures.push(error);};
  const closeFiles=()=>{for(const descriptor of[stdout,stderr])if(descriptor!==undefined)try{closeSync(descriptor);}catch(error){record(error);}stdout=stderr=undefined;};
  const settleFailure=()=>failures.length>1?new AggregateError(failures,"Publication operation and evidence close failed"):failures[0];
  try{stdout=openSync(join(evidenceRoot,"publication.stdout.json"),"wx",0o600);stderr=openSync(join(evidenceRoot,"publication.stderr.txt"),"wx",0o600);child=spawn(binaryPath,[...args],{env:{...process.env,OS_HUB_DATA:dataRoot},shell:false,windowsHide:true,stdio:["pipe","pipe","pipe"]});}catch(error){record(error);closeFiles();reject(settleFailure());return;}
  child.once("close",(status,signal)=>{closed=true;clearInterval(timer);closeFiles();if(failures.length){for(const chunk of chunks)chunk.fill(0);reject(settleFailure());return;}const bytes=Buffer.concat(chunks);for(const chunk of chunks)chunk.fill(0);accept({bytes,status,signal,failed});});
  const terminate=(reason:string)=>{failed||=reason;if(!closed)try{terminateOwnedChildTree(child);}catch(error){record(error);}};
  const guard=(body:()=>void)=>{if(closed)return;try{body();}catch(error){record(error);terminate("publication phase or evidence setup failed");}};
  guard(()=>{
   child.once("error",()=>terminate("publication child could not start"));
   child.stdin!.on("error",()=>terminate("publication command transport failed"));
   child.stdout!.on("data",(chunk:Buffer)=>guard(()=>{control.progressed?.();const retained=Buffer.from(chunk.subarray(0,Math.max(0,4096-outputBytes)));if(retained.length){writeSync(stdout!,retained);chunks.push(retained);}outputBytes+=chunk.length;if(outputBytes>4096)terminate("publication stdout exceeded its receipt bound");}));
   child.stderr!.on("data",(chunk:Buffer)=>guard(()=>{control.progressed?.();const retained=chunk.subarray(0,Math.max(0,65536-diagnosticBytes));if(retained.length)writeSync(stderr!,retained);diagnosticBytes+=chunk.length;if(diagnosticBytes>65536)terminate("publication diagnostics exceeded their bound");}));
   timer=setInterval(()=>guard(()=>{if(control.cancelled())terminate("publication cancelled");else if(control.remainingMs()<=0)terminate("publication timed out");}),100);
   if(control.cancelled()||control.remainingMs()<=0){terminate("publication phase already exhausted");return;}
   child.stdin!.end(input);
  });
 });
}

/** 🚥️ Classifies complete transport evidence before trusting any receipt prefix. */
export function trustedPublicationChildReceipt(result: TrustedPublicationChildResultV1, command: Readonly<Record<string, unknown>>, revision: string, evidenceRoot: string): ReturnType<typeof trustedBootstrapPublicationReceipt> {
  if (result.failed || result.signal || (result.status !== 0 && result.status !== 1)) throw new Error(`publication process outcome is indeterminate; reconcile current, do not retry or activate; evidence=${evidenceRoot}`);
  const receipt = trustedBootstrapPublicationReceipt(result.bytes, command, revision);
  if (receipt.outcome === "replaced-unconfirmed" && result.status === 1) throw new TrustedBootstrapPublicationUnconfirmed(receipt, evidenceRoot);
  if (receipt.outcome !== "durable" || result.status !== 0) throw new Error(`publication receipt and exit status disagree; reconcile current, do not retry or activate; evidence=${evidenceRoot}`);
  return receipt;
}

/** 🔐️ Delegates the sole live compare-and-swap to the exact native Hub binary. */
async function publishTrustedBootstrapCurrent(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,receipt:TrustedBootstrapMaterializationV1,evidenceRoot:string,expected:ReturnType<typeof trustedBootstrapReadCurrentPointer>):Promise<TrustedBootstrapPublishedV1>{
  return withLocalHubHostTaskV1(host,phaseControl,context=>withRequestedIssuedResourceControlV1(context.control,0,async actual=>{
  const {invocation,dataRoot,binaryPath}=context;
  trustedBootstrapVerifyPublication(dataRoot, receipt);
  const publicationRevision = trustedBootstrapPublicationRevision((BigInt(expected?.pointer.publicationRevision ?? "0") + 1n).toString());
  const command = Object.freeze({ schema: "semio.hub.trusted-catalog-publication/v1", requestId: randomBytes(16).toString("hex"), profileId: receipt.profileId, generationId: receipt.generationId, bundleSha256: receipt.bundleSha256, expectedCurrentSha256: expected?.sha256 ?? null });
  const input = Buffer.from(JSON.stringify(command) + "\n");
  if (!isAbsolute(dataRoot) || !isAbsolute(binaryPath) || input.length > 4096) throw new Error("trusted publication transport requires exact bounded authority");
  let lastProgressMs=performance.now();
  const control={cancelled:()=>actual.signal.aborted,remainingMs:()=>Math.min(checkResourceLeaseControl(actual)??Infinity,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS-(performance.now()-lastProgressMs)),progressed:()=>{checkResourceLeaseControl(actual);lastProgressMs=performance.now();}};
  let processResult: TrustedPublicationChildResultV1 | undefined;
  try {
    processResult = await runTrustedPublicationChild(binaryPath, ["trusted-catalog", "publish"], input, dataRoot, evidenceRoot, control);
    const published = trustedPublicationChildReceipt(processResult, command, publicationRevision, evidenceRoot);
    const current = trustedBootstrapCurrent(dataRoot);
    if (!current || current.currentSha256 !== published.currentSha256) throw new Error("trusted current changed after native publication; do not activate the candidate plan");
    await writeTrustedCatalogPublicationProvenanceV1(dataRoot,receipt.producerPath,{maxBytes:134217728,maxWork:65536,chunkBytes:1048576,cancelled:()=>actual.signal.aborted,remainingMs:()=>checkResourceLeaseControl(actual)??Infinity,onProgress:()=>{checkResourceLeaseControl(actual);}});
    return current;
  } finally {
    processResult?.bytes.fill(0);
    input.fill(0);

  }
  }));
}

async function proveTrustedStdioGisCandidatePlan(run: LocalHubRun, receipt: TrustedBootstrapMaterializationV1, envelope: Record<string, any>): Promise<Record<string, any>> {
  return withLocalHubRunTaskV1(run,async control=>{
  const bundle = JSON.parse(readFileSync(receipt.bundlePath, "utf8"));
  const profile = bundle.profiles?.find((candidate: any) => candidate.id === receipt.profileId);
  const selected = bundle.packages?.find((candidate: any) => candidate.pluginId === "gis");
  // 🎯️ The GIS target is selected by its OWNER, not by index: a generation's open targets are a SET
  // now (ticket 26/09/18), so `[0]` would silently probe whichever package happens to sort first.
  const target = profile?.openTargets?.find((row: any) => row?.package?.pluginId === "gis" && row?.target?.role === "editor")?.target;
  if (!profile || !selected || !target) throw new Error("trusted stdio+GIS candidate bundle lost its exact GIS target");
  const headers = { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" };
  const created = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "Trusted GIS Bootstrap Probe", spaceKind: "studio", visibility: "private" });
  const createdBody = created.status === 202 ? (JSON.parse(created.text) as Record<string, any>) : undefined;
  const spaceId = createdBody?.events?.find((candidate: any) => candidate?.body?.kind === "space.created")?.body?.spaceId;
  if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("trusted stdio+GIS candidate could not create its private probe space");
  // 🌱️ The probe document is created through the Hub's OWN server-owned creation transaction, the
  // same one `createCheckInProcessGenesis` drives. A bare `announce-document` registers
  // a descriptor and nothing else: the open-plan issuer also requires an ACTIVE ARTIFACT CHECKPOINT
  // (`🏗️bootstrap/🦀️.rs:3099-3104`), so it answered `404 not-found` for every announced-only document
  // and this proof could never have passed. The creation transaction establishes the genesis
  // checkpoint, which is also what a real client does before it opens anything.
  const creationRoute = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const creationRequest = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: receipt.generationId, kindId: target.artifactKind, name: "Trusted GIS Bootstrap Probe Map" });
  const accepted = await fetch(`http://127.0.0.1:${run.port}${creationRoute}`, { method: "POST", headers, signal:AbortSignal.any([control.signal,AbortSignal.timeout(5000)]), body: JSON.stringify(creationRequest) });
  const acceptedText = await accepted.text();
  if (![200, 202].includes(accepted.status)) throw new Error(`trusted stdio+GIS candidate could not create its GIS Map probe: HTTP ${accepted.status} ${JSON.stringify(acceptedText.slice(0, 512))}`);
  let creation = parseSpaceArtifactCreationStatusJsonV1(acceptedText);
  if (creation.requestId !== creationRequest.requestId || creation.spaceId !== spaceId || creation.catalogGenerationId !== creationRequest.expectedCatalogGenerationId)
    throw new Error(`trusted stdio+GIS candidate GIS Map probe answered another creation: ${JSON.stringify(acceptedText.slice(0, 512))}`);
  const creationDeadline = Date.now() + 120_000;
  while (creation.phase !== "ready") {
    if (!["accepted", "preparing", "indeterminate"].includes(creation.phase) || Date.now() >= creationDeadline) throw new Error(`trusted stdio+GIS candidate creation reached ${creation.phase} before Ready`);
    checkResourceLeaseControl(control);await waitForHubPhaseV1(20,undefined,{signal:control.signal});
    const polled = await fetch(`http://127.0.0.1:${run.port}${creationRoute}/${encodeURIComponent(creationRequest.requestId)}`, { headers: { authorization: headers.authorization }, signal:AbortSignal.any([control.signal,AbortSignal.timeout(5000)]) });
    if (polled.status !== 200 && polled.status !== 202) throw new Error(`trusted stdio+GIS candidate creation status failed: ${polled.status}`);
    creation = parseSpaceArtifactCreationStatusJsonV1(await polled.text());
    if (creation.catalogGenerationId !== creationRequest.expectedCatalogGenerationId) throw new Error("trusted stdio+GIS candidate creation crossed its selected catalog generation");
  }
  const ready = creation.ready;
  if (!ready || ready.kindId !== target.artifactKind || ready.artifactSchema !== target.artifactSchema || !/^artifact-[0-9a-f]{32}$/u.test(ready.artifactId))
    throw new Error("trusted stdio+GIS candidate creation Ready coordinates differ from the selected GIS target");
  const documentId = ready.artifactId;
  const response = await fetch(`http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
    method: "POST",
    headers,
    signal:AbortSignal.any([control.signal,AbortSignal.timeout(2000)]),
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: target.surfaceId, clientInstanceId: "trusted-bootstrap-candidate" }),
  });
  // 🩺️ The refusal body is a two-field `document-open-plan-error/v1`, so parsing it first reported
  // every hub refusal as the parser's generic `document-open.invalid-fields` and threw the actual
  // `code` away. Read the status FIRST and name the code.
  const planText = await response.text();
  if (!response.ok) throw new Error(`trusted stdio+GIS candidate was refused its GIS Map plan: ${response.status} ${planText.slice(0, 256)}`);
  const plan = parseDocumentOpenPlanV1(JSON.parse(planText), Date.now());
  if (
    !response.ok ||
    plan.scope.spaceId !== spaceId ||
    plan.scope.documentId !== documentId ||
    plan.catalog.generationId !== receipt.generationId ||
    JSON.stringify(plan.package) !==
      JSON.stringify({
        pluginId: selected.pluginId,
        packageId: selected.packageId,
        version: selected.version,
        componentSha256: selected.component.sha256,
        componentBlake3: selected.component.blake3,
        descriptorByteSha256: selected.descriptor.sha256,
        executionProtocol: trustedBootstrapExecutionProtocol(selected.executionProtocol),
        componentReceipt: trustedBootstrapRecordReceipt(selected),
      }) ||
    JSON.stringify(plan.artifact) !== JSON.stringify({ kind: target.artifactKind, schema: target.artifactSchema, packSchemaHash: target.packSchemaHash }) ||
    JSON.stringify(plan.parentDialect) !== JSON.stringify(target.parentDialect) ||
    JSON.stringify(plan.surface) !== JSON.stringify({ surfaceId: target.surfaceId, appId: target.appId, windowKindId: target.windowKindId, role: target.role, rendererTarget: target.rendererTarget }) ||
    JSON.stringify(plan.grant) !== JSON.stringify(target.grant)
  )
    throw new Error("trusted stdio+GIS candidate issued a substituted GIS Map plan");
  return plan;
  });
}

async function proveTrustedStdioGisStalePlanRejected(run: LocalHubRun, stalePlan: Record<string, any>, envelope: Record<string, any>): Promise<void> {
  return withLocalHubRunTaskV1(run,async control=>{
  try {
    const response = await fetch(`http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(stalePlan.scope.spaceId)}/documents/${encodeURIComponent(stalePlan.scope.documentId)}/socket-grants`, {
      method: "POST",
      headers: { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" },
      signal:AbortSignal.any([control.signal,AbortSignal.timeout(2000)]),
      body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: stalePlan.receipt }),
    });
    const body = (await response.json().catch(() => undefined)) as Record<string, any> | undefined;
    if (response.status !== 401 || JSON.stringify(body) !== JSON.stringify({ schema: "semio.hub.document-open-plan-error/v1", code: "denied" })) throw new Error("prior-generation authenticated plan was not terminally denied before fresh issuance");
  } finally {
    stalePlan.receipt = "";
  }
  });
}

/** 🧪️ Qualifies the real Hub publication command with explicit synthetic guest fixture bytes. */
export async function proveTrustedPublicationCli(invocation: ScriptInvocation, control:CompilerAdmissionControl, repoRoot: string, hubRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  await runRepositoryExactCargoLaws({preparation: configuredRepositoryCargoPreparationPolicyV1(),cwd: repoRoot,...exactCargoStageEnvironments(),groups: [{
    package: "semio-hub",
    target: { kind: "bin", name: "os-hub" },
    cargoArgs: ["--features", "integration-fixtures"],
    laws: ["trusted_catalog_command::tests::trusted_publication_transport_matches_neutral_arguments_and_input_bounds"],
  },
  {
    package: "semio-hub-boot-laws",
    target: { kind: "lib" },
    cargoArgs: ["--features", "integration-fixtures"],
    laws: ["tests::check_in_process_fixture_emits_verified_gis_ledger_and_catalog"],
  }],invocation,control,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 120_000, lawMilliseconds: 180_000 },consume:outputs=>withExactCargoLawOutputV1(outputs,{version:1,receiptIndex:outputs.receipts.findIndex(row=>row.package==="semio-hub-boot-laws")},output=>buildRepositoryCargoArtifacts(join(hubRoot,"Cargo.toml"),["--features","integration-fixtures","--bin","os-hub"],repoRoot,{producer:"artifact",invocation,control:output.control,network:repositoryCargoNetworkV1(invocation,repoRoot),consume:capture=>withCapturedLocalHubHostV1(invocation,output.control,capture,(host,context)=>withCheckInProcessFixtureV1(host,context.control,outputs,{version:1,purpose:"publication"},async(data,actualPhase)=>{
  const {dataRoot,binaryPath,diagnosticsRoot:evidence}=context;
  const bundle=trustedBootstrapReadRegular(join(dataRoot,"trusted-catalog/generations",data.fixture.generationId,"trusted-catalog.json"),4*1024*1024,"publication CLI retained fixture",()=>{checkResourceLeaseControl(actualPhase);});
  let bundleSha256:string;try{bundleSha256=createHash("sha256").update(bundle).digest("hex");}finally{bundle.fill(0);}
  const command={schema:"semio.hub.trusted-catalog-publication/v1",requestId:"12".repeat(16),profileId:data.fixture.profileId,generationId:data.fixture.generationId,bundleSha256,expectedCurrentSha256:null as string|null};
  const run=async(name:string,value:Record<string,unknown>):Promise<TrustedPublicationChildResultV1>=>withLocalHubHostTaskV1(host,actualPhase,owner=>withRequestedIssuedResourceControlV1(owner.control,40000,async phase=>{
   if(!/^[a-z-]{1,32}$/u.test(name))throw Error("Closed publication case name required");const caseRoot=join(evidence,name);checkResourceLeaseControl(phase);mkdirSync(caseRoot);
   const input=JSON.stringify(value)+"\n";if(Buffer.byteLength(input)>4096)throw Error("Bounded publication command required");
   const result=await captureOwnedCommandResultV1(binaryPath,["trusted-catalog","publish"],owner.root,{control:phase,env:{...owner.environment,OS_HUB_DATA:dataRoot},input:{version:1,text:input},output:{maximumBytes:69632,maximumLines:2048}});
   const bytes=Buffer.from(result.stdout);if(bytes.length>4096||Buffer.byteLength(result.stderr)>65536){bytes.fill(0);throw Error("Bounded publication terminal output required");}return{bytes,status:result.status,signal:result.signal,failed:""};
  }));
  const first = await run("first", command);
  assert.equal(first.failed, ""); assert.equal(first.signal, null);
  const receipt = trustedBootstrapPublicationReceipt(first.bytes, command, "1");
  assert.equal(first.status, receipt.outcome === "durable" ? 0 : 1);
  const initial = trustedBootstrapReadCurrentPointer(dataRoot); assert(initial);
  assert.equal(initial.sha256, receipt.currentSha256); assert.equal(initial.pointer.publicationRevision, "1");
  command.expectedCurrentSha256 = initial.sha256; command.requestId = "34".repeat(16);
  const next = await run("next", command);
  assert.equal(next.failed, ""); assert.equal(next.signal, null);
  const nextReceipt = trustedBootstrapPublicationReceipt(next.bytes, command, "2");
  assert.equal(next.status, nextReceipt.outcome === "durable" ? 0 : 1);
  const current = trustedBootstrapReadCurrentPointer(dataRoot); assert(current);
  assert.equal(current.sha256, nextReceipt.currentSha256); assert.notEqual(current.sha256, initial.sha256);
  const retained = trustedBootstrapReadRegular(join(dataRoot, "trusted-catalog/current.json"), 65536, "CLI current", () => {});
  for (const [name, value] of [
    ["stale", { ...command, requestId: "56".repeat(16) }],
    ["caller-path", { ...command, expectedCurrentSha256: current.sha256, bundlePath: "/untrusted" }],
    ["missing-token", { ...command, expectedCurrentSha256: undefined }],
  ] as const) {
    const result = await run(name, value);
    assert.equal(result.status, 1); assert.equal(result.signal, null); assert.equal(result.failed, ""); assert.equal(result.bytes.length, 0);
    assert.deepEqual(trustedBootstrapReadRegular(join(dataRoot, "trusted-catalog/current.json"), 65536, "CLI retained current", () => {}), retained);
    result.bytes.fill(0);
  }
  first.bytes.fill(0); next.bytes.fill(0); retained.fill(0);
  console.log(`trusted publication CLI: real-Hub=1 compiled-provider=1 revisions=1,2 stale=refused invalid=2 outcome=${nextReceipt.outcome} evidence=${evidence}; synthetic guest fixture, no guest execution or activation claim`);
 }))})),progress(event){console.log(`trusted-publication-cli-native ${event.stage}: ${event.law??""} artifacts=${event.artifactDir}`);}});
}

/** 🗂️ Allocates private qualification diagnostics outside immutable generations and current. */
function trustedBootstrapValidationRoot(dataRoot: string): string {
  const trustedRoot = join(dataRoot, "trusted-catalog");
  for (const directory of [dataRoot, trustedRoot]) {
    const info = lstatSync(directory);
    if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("trusted validation parent must be a regular directory");
  }
  const root = join(trustedRoot, "validation");
  if (!existsSync(root)) mkdirSync(root, { mode: 0o700 });
  const info = lstatSync(root);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("trusted validation root must be a regular directory");
  if (process.platform !== "win32") chmodSync(root, 0o700);
  return mkdtempSync(join(root, "gis-"));
}

/** 🟢️ Publishes current metadata only after exact GIS proof and isolated candidate validation. */
export async function validateAndPublishTrustedStdioGisCandidate(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,receipt:TrustedBootstrapMaterializationV1,stalePlan?:Record<string,any>):Promise<Record<string,any>>{
 return withLocalHubHostTaskV1(host,phaseControl,async context=>{
  const {invocation,workspace:repoRoot,dataRoot,control}=context,expectedCurrent=trustedBootstrapReadCurrentPointer(dataRoot),validationRoot=trustedBootstrapValidationRoot(dataRoot);
  await proveTrustedGisColdMapComponentV1(invocation,control,repoRoot,receipt);
  return withLocalHubHostCandidateV1(host,control,async(candidateHost,candidateContext)=>{
   const profile:LocalProfile={profileId:"trusted-bootstrap-probe",subject:"trusted-bootstrap-subject",displayName:"Trusted Bootstrap Probe",allowedClientClasses:["native"]};
   stageTrustedBootstrapCandidateCurrent(candidateContext.dataRoot,receipt,()=>{checkResourceLeaseControl(candidateContext.control);});
   const candidate=await startLocalHub(candidateHost,[profile],{capture:true});
   let envelope:Record<string,any>|undefined;
   return withTrustedCatalogCleanupV1(()=>withLocalHubRunTaskV1(candidate,async actual=>{
    const readiness=await waitForReadiness(candidate,false,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
    if(readiness.artifactAuthority?.ready!==true||readiness.features?.openPlan!==true||readiness.features?.openPlanExchange!==true)throw Error("trusted stdio+GIS candidate did not expose one verified document-open authority");
    envelope=await issueLocalCredential(candidate,profile.profileId,"native");
    if(stalePlan)await proveTrustedStdioGisStalePlanRejected(candidate,stalePlan,envelope);
    const plan=await proveTrustedStdioGisCandidatePlan(candidate,receipt,envelope);
    await publishTrustedBootstrapCurrent(host,actual,receipt,validationRoot,expectedCurrent);
    return plan;
   }),[()=>{if(envelope)envelope.capability="";},()=>finishLocalHub(candidate)]);
  });
 });
}
