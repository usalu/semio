const { R, g, dir, doc, write } = require("./lib_ks.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🧪️tests/📞️app-channel-client/🟦️.ts";
write({
  file, describeLine: 30,
  shared: { dir: dir("🧰", "app-channel-client-fixtures"), doc: doc("🧰", "The shared app channel client test fixtures: the command sequence reader, the fake and scripted channel handles and the settle helper.") },
  parts: [
    g("🔌", "local-interaction-wire-and-lifecycle", "registerLocalInteractionWireTests", "🧬", "The local interaction outer wire, presence snapshot and client lifecycle tests.", "Registers the strict outer wire, presence snapshot, ACK ownership, sequence admission, reopened client and exhausted admission tests.", 31, 230),
    g("🏁", "typed-operation-completion", "registerTypedOperationCompletionTests", "✅", "The unsolicited typed operation completion, progress and shared vector tests.", "Registers the unsolicited completion, progress scope and history patch, shared completion vector and archive merge vector tests.", 261, 351),
    g("📨", "command-and-document-archive-frames", "registerCommandAndDocumentArchiveFrameTests", "🗃", "The command sequencing, document configure and archive frame tests.", "Registers the command sequence, child heads, configure and read, archive load and cancel, window config, cache candidate and transaction tests.", 369, 566),
    g("🔀", "merge-policy-and-fault-decoding", "registerMergePolicyAndFaultDecodingTests", "🧯", "The merge policy, conflict, unsolicited report and fault decoding tests.", "Registers the merge policy and conflict vectors, the unsolicited report frames and the fault and wire decoders.", 597, 654),
    g("🎛", "composer-surface-and-outcome-streams", "registerComposerSurfaceAndOutcomeStreamTests", "🌊", "The composer surface, retire and outcome stream tests.", "Registers the composer surface, retire, ended outcome stream, error outcome and undecodable frame tests.", 691, 782),
  ],
}, process.argv[2]);
