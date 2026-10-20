const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧠️resident-pool-and-metadata/🟦️.tsx";
write({
  file, function: "registerResidentPoolAndMetadataTests",
  kit: kit("🧰", "resident-test-kit", "createResidentTestKit", "🏭", "The shared owned resident test kit: the destructured vitest, the document limits and the typed wire fixtures.", "Creates the shared resident test kit of one registration from the injected dependencies.", "ResidentTestKit"),
  groups: [{
    dir: dir("🧾", "typedwire-suite"), doc: doc("🧾", "The TypedWire describe block: its wrapper, page fixture helper and parts."), name: "registerTypedWireSuite", emoji: "📡", fdoc: "Registers the TypedWire describe block from its helpers and parts.",
    describeLine: 13,
    helpers: { dir: dir("🛠", "page-fixture"), doc: doc("🛠", "The resident page fixture shared by the resident page and reader parts."), name: "createResidentPageHelpers", emoji: "🔧", fdoc: "Creates the resident page fixture shared by the resident parts.", typeName: "ResidentPageHelpers" },
    parts: [
      g("🏊", "pool-slot-and-scope", "registerPoolSlotAndScopeTests", "🪣", "The owned resident pool, slot contract, construction and scope tests.", "Registers the resident pool, slot contract, construction and scope admission, cancellation and foreign composition tests.", 16, 167),
      g("📦", "composition-and-page", "registerCompositionAndPageTests", "🗂", "The resident composition, payload admission and page producer tests.", "Registers the resident composition, payload admission, page binding, producer and fault tests.", 176, 299),
      g("📄", "page-lifecycle", "registerPageLifecycleTests", "♻", "The resident page retirement, quarantine, child, fault and inventory tests.", "Registers the page retirement, cancellation, quarantine, child refusal, constructor fault, body fault and inventory tests.", 310, 367),
      g("👓", "reader-binding", "registerReaderBindingTests", "📖", "The resident reader binding, window, held handle and fault tests.", "Registers the reader binding fault and grant, revocation, quarantine, admission, retirement, window, held and fault tests.", 374, 474),
      g("🧱", "builder-copied-and-input", "registerBuilderCopiedAndInputTests", "✂", "The resident builder, copied range and active input tests.", "Registers the builder admission, grant, cancellation, fault, inventory, copied range and active input tests.", 487, 596),
      g("🔏", "evidence-binding-payload", "registerEvidenceBindingPayloadTests", "🧮", "The resident evidence, builder binding and payload tests.", "Registers the evidence brands, admission, retirement, quarantine, builder binding, payload and metadata tests.", 603, 739),
    ],
  }],
  rootKeep: [],
}, process.argv[2]);
