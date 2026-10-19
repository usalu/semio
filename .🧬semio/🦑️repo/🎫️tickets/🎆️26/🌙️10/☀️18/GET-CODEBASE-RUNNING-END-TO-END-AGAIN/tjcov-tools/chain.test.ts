import { test } from "bun:test";
const R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const T = `${R}/🧪️tests/🔬️tool-job-coverage`;
const J = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const items: [string, string, string][] = [
  [`${T}/🤝️peer-presence-and-command-ingress/🟦️.ts`, "peerPresenceRosterChecks", ""],
  [`${T}/🤝️peer-presence-and-command-ingress/🟦️.ts`, "commandIngressAndKernelRequestChecks", ""],
  [`${T}/🧯️payload-store-progress-and-mounted-session/🟦️.ts`, "fem2dMountedSessionChecks", ""],
  [`${T}/🧯️payload-store-progress-and-mounted-session/🟦️.ts`, "globalPayloadStoreInventoryChecks", ""],
  [`${T}/🧯️payload-store-progress-and-mounted-session/🟦️.ts`, "actorProgressOverlayChecks", ""],
  [`${T}/🧾️proof-catalog-and-admission/🟦️.ts`, "proofCatalogEvaluationChecks", ""],
  [`${T}/🧾️proof-catalog-and-admission/🟦️.ts`, "admissionOrderAndBoundedPreparationChecks", ""],
  [`${T}/📤️publication-contracts/🟦️.ts`, "retainedPublicationChecks", ""],
  [`${T}/📤️publication-contracts/🟦️.ts`, "ephemeralPublicationChecks", ""],
  [`${T}/📤️publication-contracts/🟦️.ts`, "freshTurnAndHostOnlyPublicationChecks", ""],
  [`${T}/🛑️cancellation-close-and-output-ownership/🟦️.ts`, "typedRouteCancellationAndCloseChecks", ""],
  [`${T}/🛑️cancellation-close-and-output-ownership/🟦️.ts`, "snapshotAndReactorCloseChecks", ""],
  [`${T}/🛑️cancellation-close-and-output-ownership/🟦️.ts`, "segmentedOutputAndChildRootChecks", ""],
  [`${T}/🗄️artifact-store-and-envelope-codec/🟦️.ts`, "artifactStoreStructuralAndEnvelopeCodecChecks", ""],
  [`${T}/🔭️universal-and-layout-retained-ownership/🟦️.ts`, "universalRetainedOwnershipChecks", ""],
  [`${T}/🔭️universal-and-layout-retained-ownership/🟦️.ts`, "layoutAndColdRelayRetainedChecks", ""],
  [`${R}/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts`, "toolJobLatestWinsSelfTests", ""],
  [`${R}/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts`, "toolJobFactoryProofJoinSelfTests", ""],
  [`${R}/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts`, "toolJobOwnerFactoryResolutionSelfTests", ""],
  [`${R}/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts`, "toolJobCheckpointSelfTests", ""],
  [`${R}/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts`, "toolJobScalarConfigCohortSelfTests", ""],
];
const only = process.env.ONLY?.split(",");
for (const [file, name] of items) {
  if (only && !only.includes(name)) continue;
  test(name, async () => { const m = await import(file); m[name](); }, 180000);
}
