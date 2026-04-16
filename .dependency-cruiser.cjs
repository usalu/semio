/**
 * @type {import("dependency-cruiser").IConfiguration}
 * @emoji 🧱 Enforces strict semio JS stack layering (sketchpad → react → js; no sketchpad → js).
 */
module.exports = {
  forbidden: [
    {
      name: "no-sketchpad-to-semio-js",
      severity: "error",
      comment: "Sketchpad MUST consume kits only through @semio/react (see semio/sketchpad/AGENTS.md).",
      from: { path: "^semio/sketchpad" },
      to: { path: "^semio/js" },
    },
    {
      name: "no-react-to-rs-wasm-pkg",
      severity: "error",
      comment: "React MUST load WASM only via @semio/js / worker; no direct @semio/rs-wasm imports in TS sources.",
      from: { path: "^semio/react" },
      to: { path: "^semio/rs/pkg" },
    },
    {
      name: "no-js-to-react-or-sketchpad",
      severity: "error",
      comment: "Domain client must not depend on UI bundles.",
      from: { path: "^semio/js" },
      to: { path: "^semio/react" },
    },
    {
      name: "no-js-to-sketchpad",
      severity: "error",
      comment: "Domain client must not depend on sketchpad.",
      from: { path: "^semio/js" },
      to: { path: "^semio/sketchpad" },
    },
  ],
  options: {
    doNotFollow: { path: "node_modules" },
    tsPreCompilationDeps: true,
  },
};
