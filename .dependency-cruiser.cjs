/** @type {import('dependency-cruiser').IConfiguration} */
const fs = require("fs");
const path = require("path");

const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];

/** 🔌️ Derived from the live `✏️s/🔌️plugins` directory listing rather than hardcoded, so the
 * cross-plugin isolation matrix below self-corrects as plugins are added, renamed, or removed. */
const PLUGINS = fs
  .readdirSync(path.join(__dirname, "✏️s/🔌️plugins"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name);

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
        from: { path: `^✏️s/🔌️plugins/${from}/` },
        to: {
          path: `^✏️s/🔌️plugins/${to}/`,
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
      // 🧭️ Matches the RESOLVED dependency path, not the raw import specifier — dependency-cruiser is
      // invoked from the repo root, so this only fires when a resolved local import lands 4+ levels
      // above the repo root, i.e. actually escapes the checkout. A `📜️script.ts`'s own relative
      // specifier (`../../../../../../../🧰️framework/…`, 6-8 `../` segments to reach repo-lib) resolves
      // to an ordinary in-repo path and never trips this — do not "fix" it into a specifier-depth rule,
      // that would break every script.ts in the repo (see ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).
      name: "no-escaping-relative-imports",
      severity: "error",
      comment: "Relative imports must not resolve to a path outside the repo checkout",
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
        path: PLUGINS.map((p) => `^✏️s/🔌️plugins/${p}/`).concat(PLUGINS.map((p) => `^@semio-tech/${p.replace(/^[^a-zA-Z]+/, "")}-`)),
      },
    },
    {
      name: "ui-no-framework-packages",
      severity: "error",
      comment: "🧰️framework/🔨️modules/🖱️ui must stay presentational and business-logic free — no OS/plugin/framework coupling",
      from: { path: "^🧰️framework/🔨️modules/🖱️ui/" },
      to: {
        path: ["^🧰️framework/", "^@semio-tech/framework-"],
        pathNot: ["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-"],
      },
    },
    {
      // 🧭️ Was stale since before this rule's own introduction: it matched
      // `📺️renderer/⚡️implementations/🟦️typescript/🧑️‍🎨️engine/⚛️react/`, but the real dir order has always
      // been `🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/` — zero paths ever matched `from`, so
      // this rule has never fired. Repointed (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE) at the
      // FUTURE co-located shape (`📦️packages/🟦️typescript/🎯️targets/⚛️react/` + `🧱️elements/`) rather
      // than the current dead path — stays a deliberate no-op until that restructure's W4 move lands,
      // at which point it starts enforcing for real. Do not "fix" it back to the current dir; that would
      // just trade one dead path for another about to be deleted.
      name: "renderer-hosts-only-ui",
      severity: "error",
      comment: "the react renderer host may depend only on ui/styling, framework-core protocol types, react, and itself — never os-shell, ui-interpreter, or app packages",
      from: { path: "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/(📦️packages/🟦️typescript/🎯️targets/⚛️react|🧱️elements)/" },
      to: {
        pathNot: [
          "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/(📦️packages/🟦️typescript/🎯️targets/⚛️react|🧱️elements)/",
          "^@semio-tech/ui-",
          "^@semio-tech/framework-core",
          "^react$",
          "^react/",
          "^react-dom$",
          "^react-dom/",
          "^node:",
        ],
      },
    },
    {
      name: "no-generated-edits-upstream",
      severity: "error",
      comment: "only the plugin registry itself may import its generated plugin catalog directly — other consumers must go through generated/🟦️plugins.ts",
      from: { pathNot: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/" },
      to: { path: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/🔣️plugins\\.json$" },
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
