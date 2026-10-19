const { R, f, ph, write, dir, doc } = require("./lib_ps.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts";
write({
  file, function: "createBrowserBundleTests", keep: [], resolve: true,
  shared: { dir: dir("🧰", "browser-bundle-test-types"), doc: doc("🧰", "The shared browser bundle test types: the build control the actor bundle tests pass.") },
  files: [
    f("🧱", "dependency-bindings", "The destructured browser bundle test dependencies the test phases share.", [
      ph("bindBrowserBundleDependencies", "🔗", "Binds the browser bundle script API the tests drive.", 9, 9),
    ]),
    f("🏭", "factory-capsule-and-sources", "The closed component factory, codegen capsule, codegen sources and codegen policy tests.", [
      ph("defineComponentFactoryTest", "🏗", "Defines the closed browser component factory test.", 10, 10),
      ph("defineCodegenCapsuleTest", "💊", "Defines the browser codegen capsule test.", 102, 102),
      ph("defineCodegenSourcesTest", "📜", "Defines the browser codegen sources test.", 142, 142),
      ph("defineCodegenPolicyTest", "🛂", "Defines the browser codegen policy test.", 190, 190),
    ]),
    f("📦", "manifest-bundle-and-activation", "The async task return lift, the actor codegen manifest, the closed actor bundle and the host activation tests.", [
      ph("defineAsyncTaskReturnLiftTest", "↩", "Defines the async task return lift test.", 242, 242),
      ph("defineActorCodegenManifestTest", "🧾", "Defines the browser actor codegen manifest test.", 251, 251),
      ph("defineClosedActorBundleTest", "🎁", "Defines the closed browser actor bundle test.", 266, 266),
      ph("defineHostActivationTest", "🚀", "Defines the browser host activation test.", 492, 492),
    ]),
  ],
}, process.argv[2]);
