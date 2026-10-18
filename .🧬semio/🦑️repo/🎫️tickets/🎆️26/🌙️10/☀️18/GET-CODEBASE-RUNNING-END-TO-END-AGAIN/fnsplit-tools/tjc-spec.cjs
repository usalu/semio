// builds tjc.json for psplit.cjs (emoji carry U+FE0F explicitly)
const V = "️";
const R = "/Users/ueli/Documents/semio/";
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts";
const dir = (e, n) => e + V + n;
const doc = (e, t) => `/** ${e}${V} ${t} */`;
const f = (e, n, t, phases) => ({ dir: dir(e, n), doc: doc(e, t), phases });
const ph = (name, emoji, d, from, to) => ({ name, emoji, doc: d, from, to });
const spec = {
  file,
  function: "toolJobCoverageSelfTests",
  helpers: {
    dir: dir("🧮", "source-evaluation"),
    doc: doc("🧮", "The tool job source evaluator and the exact bounded first step proof builder shared by the proof catalog phases."),
    items: [{ line: 30, as: "evaluateToolJobSources", emoji: "⚗", doc: "Evaluates the tool job rows that remain unmigrated for a map of owner sources." }, { line: 253, as: "exactBoundedFirstStepProof", emoji: "🪙", doc: "Builds the exact bounded first step proof source of one tool for an owner file and controller." }],
  },
  keep: [57, 676, 1860, 1861],
  files: [
    f("🧾", "proof-catalog-and-admission", "The interactive job evaluation fixtures, the proof catalog evaluation checks and the admission order and bounded preparation checks.", [
      ph("interactiveJobEvaluationFixtures", "🔎", "Runs the fixture sources that must leave at least one unmigrated tool job row and returns them for the final count.", 49, 56),
      ph("proofCatalogEvaluationChecks", "📚", "Checks scan then monolith rows, duplicate owners, valid, typed and shared catalogs, executable owners, qualified schemas and forged proof rows.", 257, 343),
      ph("admissionOrderAndBoundedPreparationChecks", "🚦", "Checks reserved route bypass, decode before lookup, contract mismatch, cancellation ownership, qualified proof order, alias fallback, reserved terminals and bounded full operation preparation.", 344, 402),
    ]),
    f("🧯", "payload-store-progress-and-mounted-session", "The process global payload store inventory, the actor progress overlay and the mounted fem2d session checks.", [
      ph("globalPayloadStoreInventoryChecks", "🗃", "Checks the process global payload owner inventory and the exact shared fixed authority exemption.", 58, 75),
      ph("actorProgressOverlayChecks", "📶", "Checks the actor, shard and wgpu job progress overlay sources and every single-edit mutation of them.", 86, 162),
      ph("fem2dMountedSessionChecks", "🔩", "Checks the mounted fem2d session sources and every single-edit mutation of them.", 163, 251),
    ]),
    f("📤", "publication-contracts", "The retained, ephemeral, fresh turn, host only, config and presence publication contract checks.", [
      ph("retainedPublicationChecks", "📰", "Checks the retained store and plugin publication bounds.", 403, 461),
      ph("ephemeralPublicationChecks", "💨", "Checks the retained ephemeral one item publication and publisher bounds.", 463, 510),
      ph("freshTurnAndHostOnlyPublicationChecks", "🔔", "Checks publication freshness before every turn and the host only, config and presence publication contracts.", 512, 568),
    ]),
    f("🛑", "cancellation-close-and-output-ownership", "The cancellation, runtime close, snapshot, reactor and segmented output ownership checks.", [
      ph("typedRouteCancellationAndCloseChecks", "🧷", "Checks typed route guards, cancellation handle shapes, close witnesses, runtime close budgets and live cleanup ownership.", 570, 611),
      ph("snapshotAndReactorCloseChecks", "🧹", "Checks snapshot retirement witnesses, owned disposers and reactor close, admission and starvation.", 612, 639),
      ph("segmentedOutputAndChildRootChecks", "📦", "Checks segmented output chunks, media output credit, operation roots and the fixed child registry.", 640, 675),
    ]),
    f("🗄", "artifact-store-and-envelope-codec", "The artifact store structural owner, history ledger, edit message ledger and owned envelope codec checks.", [
      ph("artifactStoreStructuralAndEnvelopeCodecChecks", "🏛", "Checks the structural store owners, the VCS history reservation, the edit message ledger and the owned envelope codec callers.", 677, 715),
    ]),
    f("✍", "caller-retained-document-envelopes", "The writer, jack, trinity rewrite and gis map caller retained document envelope checks.", [
      ph("writerEnvelopeCallerRetainedChecks", "📝", "Checks the writer envelope caller retained route and returns the writer sources the other documents derive from.", 716, 769),
      ph("jackEnvelopeCallerRetainedChecks", "🔌", "Checks the jack envelope caller retained route.", 770, 808),
      ph("trinityRewriteEnvelopeCallerRetainedChecks", "🔁", "Checks the trinity rewrite envelope caller retained route.", 809, 877),
      ph("gisMapEnvelopeCallerRetainedChecks", "🗺", "Checks the gis map envelope caller retained route.", 878, 932),
    ]),
    f("🖼", "raster-envelope", "The raster envelope caller retained sources and their route, mutation, loop restoration and rejected output identity checks.", [
      ph("rasterEnvelopeSourceFixtures", "🧬", "Builds the raster codec and editor sources.", 933, 1103),
      ph("rasterEnvelopeRouteAndMutationChecks", "🎞", "Checks the raster envelope route, signature suspension and single-edit mutations.", 1122, 1208),
      ph("rasterEnvelopeLoopRestorationChecks", "♻", "Checks that restoring a whole loop in the raster serializers is refused.", 1209, 1253),
      ph("rasterRejectedOutputIdentityChecks", "🪪", "Checks the rejected output identity, binding and retirement order of the raster parameter and asset maps.", 1254, 1304),
    ]),
    f("🎨", "drawing-envelope", "The drawing envelope caller retained sources and their route, semantic digest and mutation checks.", [
      ph("drawingEnvelopeSourceFixtures", "🏗", "Builds the drawing codec and editor sources.", 1305, 1513),
      ph("drawingEnvelopeRouteAndSemanticChecks", "🔍", "Checks the drawing envelope route, the production editor and the semantic digest field omissions.", 1536, 1595),
      ph("drawingEnvelopeMutationChecks", "🧪", "Checks the drawing single-edit mutations of the codec, the editor and the plugin sources.", 1600, 1684),
    ]),
    f("🤝", "peer-presence-and-command-ingress", "The peer presence roster, the paged command ingress and the kernel request queue checks.", [
      ph("peerPresenceRosterChecks", "👥", "Checks that the peer presence roster stays fixed, typed, validated and committed with its generation.", 1685, 1698),
      ph("commandIngressAndKernelRequestChecks", "📥", "Checks the paged command ingress, the command batch owners and the kernel request and close queues.", 1699, 1744),
    ]),
    f("🔭", "universal-and-layout-retained-ownership", "The universal retained session ownership and the layout export and cold relay retained ownership checks.", [
      ph("universalRetainedOwnershipChecks", "🌐", "Checks the job, native I/O, wgpu and animate retained session ownership and every single-edit mutation of them.", 1745, 1818),
      ph("layoutAndColdRelayRetainedChecks", "📡", "Checks the layout export, layout wasm and guest cold relay retained ownership.", 1819, 1859),
    ]),
  ],
};
require("fs").writeFileSync(process.argv[2], JSON.stringify(spec, null, 1));
