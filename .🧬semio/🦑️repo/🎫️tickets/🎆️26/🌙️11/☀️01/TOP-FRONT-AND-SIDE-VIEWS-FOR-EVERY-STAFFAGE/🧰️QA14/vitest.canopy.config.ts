import base from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts";

const dirs = ["🪢️tied-canopy", "🌡️catalog-physics", "🏗️structure-oracle", "📏️print-suitability", "🪞️representation-copied", "🧭️representation-resolution", "🧊️three-views", "✅️asset-validation", "🧬️schema-conformance", "♾️physical-invariance", "🧊️thermal-scenarios", "🎭️cast-export", "🧊️axial-twist"];

export default {
  ...base,
  test: {
    ...base.test,
    include: dirs.map((d) => `../../🧪️tests/${d}/🟦️.ts`),
  },
};
