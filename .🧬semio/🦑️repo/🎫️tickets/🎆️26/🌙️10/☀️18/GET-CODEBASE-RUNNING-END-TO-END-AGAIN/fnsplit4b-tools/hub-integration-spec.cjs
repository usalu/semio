const { R, g, dir, doc, write } = require("./lib_ks.cjs");
const file = R + "🌎️hub/🧪️tests/🤝️integration/🟦️.ts";
write({
  file, describeLine: 134,
  shared: { dir: dir("🧰", "hub-integration-fixtures"), doc: doc("🧰", "The shared hub integration test fixtures: the end to end switch, the test timeout and the scope export validator.") },
  parts: [
    g("🔐", "bootstrap-auth-and-trusted-catalog", "registerBootstrapAuthAndTrustedCatalogTests", "🪪", "The local bootstrap, auth capability and trusted catalog bundle contract tests.", "Registers the bootstrap, credential and readiness schema, the auth capability and socket grant, and the trusted catalog bundle tests.", 135, 272),
    g("🧾", "checkpoint-and-authority-adapter", "registerCheckpointAndAuthorityAdapterTests", "🧮", "The checkpoint event and authority adapter vector tests.", "Registers the checkpoint event, identity and cap, and the authority adapter SHA-256 vector tests.", 380, 452),
    g("📦", "artifact-chunk-cas-boundaries", "registerArtifactChunkCasBoundaryTests", "🧱", "The artifact chunk CAS boundary derivation test.", "Registers the artifact chunk CAS boundary test with AJV and WebCrypto.", 473, 473),
    g("🔁", "lag-rebootstrap-and-loopback-ports", "registerLagRebootstrapAndLoopbackPortTests", "🔌", "The lag rebootstrap control and loopback port tests.", "Registers the typed lag rebootstrap control, the rebindable loopback port and the HTTP listener debug binary tests.", 727, 795),
  ],
}, process.argv[2]);
