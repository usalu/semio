const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/💡️inference-port/🟦️.ts";
const spec = {
  file, function: "registerGisInferencePortTests",
  kit: kit("🧰", "inference-port-test-kit", "createGisInferencePortTestKit", "🏭", "The shared GIS Map inference port test kit: the destructured vitest, the worker law fakes and the production seams.", "Creates the shared inference port test kit of one registration from the injected dependencies.", "GisInferencePortTestKit"),
  groups: [
    g("🔁", "successor-opening-refusal", "registerSuccessorOpeningRefusalTests", "🧬", "The retained inference job successor opening refusal test.", "Registers the test that retains the preceding inference job when a successor opening is refused.", 20, 20),
    {
      dir: dir("🗺", "gis-map-inference-port"), doc: doc("🗺", "The gis map inference port describe block: its wrapper, shared fixtures and parts."), name: "registerGisMapInferencePortTests", emoji: "🛰", fdoc: "Registers the gis map inference port describe block from its helpers and parts.",
      describeLine: 80,
      helpers: { dir: dir("🛠", "inference-port-helpers"), doc: doc("🛠", "The inference port harness, lease fields, receipt and page bodies shared by the inference port parts."), name: "createInferencePortHelpers", emoji: "🔧", fdoc: "Creates the inference port fixtures shared by the inference port parts.", typeName: "InferencePortHelpers" },
      parts: [
        g("🪪", "receipt-and-lease-admission", "registerReceiptAndLeaseAdmissionTests", "📜", "The exact receipt wire, session authority and live lease admission tests.", "Registers the exact receipt and events wire, session authority, lease admission and cancellation retention tests.", 81, 341),
        g("🔒", "sealed-owner-retention", "registerSealedOwnerRetentionTests", "🧊", "The sealed inference owner retention, Undo owner and successor proof tests.", "Registers the sealed owner retention, Undo owner, successor broker proof and canonical response tests.", 364, 506),
        g("🔑", "broker-session-authority", "registerBrokerSessionAuthorityTests", "🗝", "The verified broker session authority and Directory bootstrap tests.", "Registers the verified broker session, physical reopen, bootstrap page correlation and unverified response tests.", 522, 647),
        g("🔌", "broker-port-retirement-and-queue", "registerBrokerPortRetirementTests", "🚪", "The attached broker port retirement and queued request tests.", "Registers the attached broker port retirement, old queued request and retired inference work tests.", 715, 775),
        g("🏁", "cancel-stale-approval-and-vocabulary", "registerCancelStaleApprovalTests", "🧾", "The cancel, stale base, approval, preview and vocabulary tests.", "Registers the cancel click, stale base, exact hash approval, cross job preview, route rejection and bilingual vocabulary tests.", 815, 939),
      ],
    },
  ],
  rootKeep: [],
};
write(spec, process.argv[2]);
