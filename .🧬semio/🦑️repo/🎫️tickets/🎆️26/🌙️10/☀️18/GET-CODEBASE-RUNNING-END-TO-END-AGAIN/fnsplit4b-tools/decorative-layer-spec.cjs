const { R, g, dir, doc, write } = require("./lib_ks.cjs");
const file = R + "🧰️framework/🛍️products/🐾️pets/🧪️tests/🫥️decorative-layer/🟦️.tsx";
write({
  file, describeLine: 239,
  sharedRewrite: [['dirname(fileURLToPath(import.meta.url)), "../../', 'dirname(fileURLToPath(import.meta.url)), "../../../']],
  shared: { dir: dir("🧰", "decorative-layer-fixtures"), doc: doc("🧰", "The shared decorative layer test fixtures: the stylesheet, the menagerie, the page geometry, the pointer and the event ledgers.") },
  parts: [
    g("♿", "species-and-accessibility", "registerSpeciesAndAccessibilityTests", "🦋", "The shiny species, menagerie admission and accessibility tests.", "Registers the shiny species update, menagerie admission, hidden element, assistive technology and keyboard exposure tests.", 240, 322),
    g("🎨", "pointer-and-stylesheet-policy", "registerPointerAndStylesheetPolicyTests", "🖱", "The pointer event and transition stylesheet policy tests and the pet placement tests.", "Registers the pointer event and transition stylesheet laws, the surface standing and the placement tests.", 334, 402),
    g("🎞", "still-and-living-stage-behaviour", "registerStillAndLivingStageTests", "🐾", "The still and living stage painting, pointer rest and surface follow tests.", "Registers the still stage paint, the pointer rest on still and living stages, the page move follow and the calm stage tests.", 423, 507),
    g("📡", "events-casts-and-time", "registerEventsCastsAndTimeTests", "⏱", "The prop events, cast reporting, time scale and rotation tests.", "Registers the changed props events, the on stage report, the time scale, the cast rotation and the unknown scene tests.", 554, 682),
    g("🌙", "narrow-hidden-and-forced-colours", "registerNarrowHiddenAndForcedColoursTests", "🌗", "The narrow stage, hidden document, inert page and forced colours tests.", "Registers the narrow stage, hidden document, inert dialog, hidden start and forced colours tests.", 694, 785),
    g("🧹", "teardown-and-mounting", "registerTeardownAndMountingTests", "🔚", "The teardown, strict mode, seeded mount and failure containment tests.", "Registers the clean teardown, the strict mode double mount, the seeded stage, the silent failure end and the look hand over tests.", 801, 907),
  ],
}, process.argv[2]);
