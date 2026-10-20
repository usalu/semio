const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts";
write({
  file, function: "registerTests1",
  kit: kit("🧰", "brepjs-test-kit", "createBrepjsTestKit", "🏭", "The shared brepjs test kit: the destructured vitest, the brepjs kernel dependencies and the bootstrapped cad modules.", "Creates the shared brepjs test kit of one registration from the injected dependencies.", "BrepjsTestKit"),
  groups: [{
    dir: dir("🧱", "brepjs-suite"), doc: doc("🧱", "The cad-js brepjs describe block: its wrapper, kernel fixture and parts."), name: "registerBrepjsSuite", emoji: "🏛", fdoc: "Registers the cad-js brepjs describe block from its kernel fixture and parts.",
    describeLine: 32,
    helpers: { dir: dir("🛠", "kernel-fixture"), doc: doc("🛠", "The brepjs kernel fixture shared by the brepjs parts."), name: "createBrepjsKernelHelpers", emoji: "🔧", fdoc: "Creates the brepjs kernel fixture shared by the brepjs parts.", typeName: "BrepjsKernelHelpers" },
    parts: [
      g("📦", "box-and-extrude", "registerBoxAndExtrudeTests", "🧊", "The brepjs box volume, tessellation, solid sync and extrude tests.", "Registers the box volume, tessellate, solid sync, extrude wire diff and extrude curve command tests.", 35, 128),
      g("🕸", "tessellation-cache", "registerTessellationCacheTests", "🧵", "The brepjs sheared shell sync, tessellation cache and dispose tests.", "Registers the sheared shell sync, face entity mapping, cached mesh, dispose and face bucket tests.", 155, 215),
      g("📐", "topology-and-measures", "registerTopologyAndMeasureTests", "📏", "The brepjs topology preview, aabb, measure and adjacency tests.", "Registers the topology preview, moved shell aabb, distance, area, volume, hull fuse, adjacency and aabb difference tests.", 226, 354),
      g("〰", "curve-commands", "registerCurveCommandTests", "🌀", "The brepjs arc, circle, sphere, control point, interpolate, typology and fixture roundtrip tests.", "Registers the arc, circle, sphere, control point curve, worker argument, interpolate curve, typology and concrete forest fixture tests.", 363, 490),
    ],
  }],
  kitInit: [30],
  rootTail: [515, 516],
  rootKeep: [],
}, process.argv[2]);
