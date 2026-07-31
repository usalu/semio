/** @type {import('dependency-cruiser').IConfiguration} */
const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];

const PLUGINS = ["🎞️animate", "🏛️architect", "🧱️block", "📐️cad", "🕸️dag", "🖍️draw", "🔋️energy", "🏗️fem", "🌊️flow", "📋️forms", "🌍️gis", "📜️imperative", "📏️layout", "💠️lowpoly", "➗️mathematical", "📕️norm", "🗒️note", "📖️playbook", "🌀️procedural", "🏭️process", "🧩️puzzle", "🖨️raster", "💡️reasoning", "📸️remodel", "🎬️sequence", "🎥️shooting", "🪵️sourcing", "🪐️space", "🔱️trinity", "🌿️vcs", "✒️writer"];

function crossTechnologyRules() {
  const rules = [];
  for (const from of TECHNOLOGIES) {
    for (const to of TECHNOLOGIES) {
      if (from === to) continue;
      rules.push({
        name: `no-cross-technology-${from}-to-${to}`,
        severity: "error",
        comment: "Relative imports must not cross top-level technology folders; use @semio-tech packages",
        from: { path: `^${from}/` },
        to: {
          path: `^${to}/`,
          dependencyTypes: ["local"],
        },
      });
    }
  }
  return rules;
}

/** 🔌️ Plugins must not relative-import a sibling plugin's implementation — cross-plugin sharing goes
 * through @semio-tech packages or a framework/s module. `flow` is exempt (media-graph canvas embed). */
function crossPluginRules() {
  const rules = [];
  for (const from of PLUGINS) {
    for (const to of PLUGINS) {
      if (from === to || to === "🌊️flow") continue;
      rules.push({
        name: `no-cross-plugin-${from}-to-${to}`,
        severity: "error",
        comment: "Relative imports must not cross plugin folders; use @semio-tech packages",
        from: { path: `^✏️s/🔌️plugin/${from}/` },
        to: {
          path: `^✏️s/🔌️plugin/${to}/`,
          dependencyTypes: ["local"],
        },
      });
    }
  }
  return rules;
}

module.exports = {
  forbidden: [
    {
      name: "no-circular",
      severity: "error",
      comment: "No circular dependencies",
      from: {},
      to: { circular: true },
    },
    {
      name: "not-to-unlisted",
      severity: "error",
      comment: "Only depend on packages declared in the nearest package.json",
      from: {},
      to: {
        dependencyTypes: ["npm-no-pkg", "npm-unknown"],
      },
    },
    {
      name: "no-escaping-relative-imports",
      severity: "error",
      comment: "Relative imports must not escape the owning package root",
      from: {},
      to: {
        dependencyTypes: ["local"],
        path: "^\\.\\./\\.\\./\\.\\./\\.\\./",
      },
    },
    {
      name: "framework-no-plugin-packages",
      severity: "error",
      comment: "🧰️framework must not import plugin app packages — shells derive from app contributions",
      from: { path: "^🧰️framework/" },
      to: {
        path: PLUGINS.map((p) => `^✏️s/🔌️plugin/${p}/`).concat(PLUGINS.map((p) => `^@semio-tech/${p.replace(/^[^a-zA-Z]+/, "")}-`)),
      },
    },
    {
      name: "ui-no-framework-packages",
      severity: "error",
      comment: "🧰️framework/🔨️module/🖱️ui must stay presentational and business-logic free — no OS/plugin/framework coupling",
      from: { path: "^🧰️framework/🔨️module/🖱️ui/" },
      to: {
        path: ["^🧰️framework/", "^@semio-tech/framework-"],
        pathNot: ["^🧰️framework/🔨️module/🖱️ui/", "^@semio-tech/ui-"],
      },
    },
    {
      name: "renderer-hosts-only-ui",
      severity: "error",
      comment: "the react renderer host may depend only on ui/styling, framework-core protocol types, react, and itself — never os-shell, ui-interpreter, or app packages",
      from: { path: "^🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/⚡️implementation/🟦️typescript/🧑️‍🎨️engine/⚛️react/" },
      to: {
        pathNot: ["^🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/⚡️implementation/🟦️typescript/🧑️‍🎨️engine/⚛️react/", "^@semio-tech/ui-", "^@semio-tech/framework-core", "^react$", "^react/", "^react-dom$", "^react-dom/", "^node:"],
      },
    },
    {
      name: "no-generated-edits-upstream",
      severity: "error",
      comment: "only the plugin registry itself may import its generated plugin catalog directly — other consumers must go through generated/🟦️plugins.ts",
      from: { pathNot: "^🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🟦️typescript/📇️registry/" },
      to: { path: "^🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🟦️typescript/📇️registry/🤖️generated/plugins\\.json$" },
    },
    ...crossTechnologyRules(),
    ...crossPluginRules(),
  ],
  options: {
    doNotFollow: {
      path: "node_modules|dist|target|storybook-static|\\.git|\\.nx|\\.🦑️repo|\\.repo",
    },
    tsPreCompilationDeps: true,
    combinedDependencies: true,
    enhancedResolveOptions: {
      exportsFields: ["exports"],
      conditionNames: ["import", "require", "node", "default"],
    },
  },
};
