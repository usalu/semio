/** @type {import('dependency-cruiser').IConfiguration} */
const TECHNOLOGIES = ["compose", "framework", "flow", "layout", "puzzle", "ui", "draw", "note", "sequence", "s"];

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
      name: "framework-no-app-packages",
      severity: "error",
      comment: "framework must not import app packages — shells derive from app contributions",
      from: { path: "^framework/" },
      to: {
        path: [
          "^draw/",
          "^note/",
          "^flow/",
          "^layout/",
          "^puzzle/",
          "^sequence/",
          "^writer/",
          "^raster/",
          "^forms/",
          "^shooting/",
          "^lowpoly/",
          "^procedural/",
          "^trinity/",
          "^gis/",
          "^vcs/",
          "^cad/",
          "^mathematical/",
          "^reasoning/",
          "^imperative/",
          "^@semio-tech/draw-",
          "^@semio-tech/note-",
          "^@semio-tech/flow-",
          "^@semio-tech/layout-",
          "^@semio-tech/puzzle-",
          "^@semio-tech/sequence-",
          "^@semio-tech/writer-",
          "^@semio-tech/raster-",
          "^@semio-tech/forms-",
          "^@semio-tech/shooting-",
          "^@semio-tech/lowpoly-",
          "^@semio-tech/procedural-",
          "^@semio-tech/trinity-",
          "^@semio-tech/gis-",
          "^@semio-tech/vcs-",
          "^@semio-tech/cad-",
          "^@semio-tech/mathematical-",
          "^@semio-tech/reasoning-",
          "^@semio-tech/imperative-",
        ],
      },
    },
    {
      name: "s-no-app-packages-except-flow-media",
      severity: "error",
      comment: "s must not import app packages except flow-react for media-graph canvas",
      from: { path: "^s/" },
      to: {
        path: [
          "^draw/",
          "^note/",
          "^layout/",
          "^puzzle/",
          "^sequence/",
          "^writer/",
          "^raster/",
          "^forms/",
          "^shooting/",
          "^lowpoly/",
          "^procedural/",
          "^trinity/",
          "^gis/",
          "^vcs/",
          "^cad/",
          "^mathematical/",
          "^reasoning/",
          "^imperative/",
          "^@semio-tech/draw-",
          "^@semio-tech/note-",
          "^@semio-tech/layout-",
          "^@semio-tech/puzzle-",
          "^@semio-tech/sequence-",
          "^@semio-tech/writer-",
          "^@semio-tech/raster-",
          "^@semio-tech/forms-",
          "^@semio-tech/shooting-",
          "^@semio-tech/lowpoly-",
          "^@semio-tech/procedural-",
          "^@semio-tech/trinity-",
          "^@semio-tech/gis-",
          "^@semio-tech/vcs-",
          "^@semio-tech/cad-",
          "^@semio-tech/mathematical-",
          "^@semio-tech/reasoning-",
          "^@semio-tech/imperative-",
        ],
      },
    },
    {
      name: "ui-no-framework-packages",
      severity: "error",
      comment: "ui must stay presentational and business-logic free — no OS/plugin/framework coupling",
      from: { path: "^ui/" },
      to: {
        path: ["^framework/", "^@semio-tech/framework-"],
      },
    },
    {
      name: "renderer-hosts-only-ui",
      severity: "error",
      comment: "framework/renderer/react/components hosts may depend only on ui/styling, framework/core protocol types, react, and each other — never os-shell, ui-interpreter, or app packages",
      from: { path: "^framework/renderer/react/components/" },
      to: {
        pathNot: ["^framework/renderer/react/components/", "^@semio-tech/ui-", "^@semio-tech/framework-core", "^react$", "^react/", "^react-dom$", "^react-dom/", "^node:"],
      },
    },
    {
      name: "no-generated-edits-upstream",
      severity: "error",
      comment: "only framework/plugin/registry may import its generated plugin catalog directly — other consumers must go through generated/plugins.ts",
      from: { pathNot: "^framework/plugin/registry/" },
      to: { path: "^framework/plugin/registry/generated/plugins\\.json$" },
    },
    ...crossTechnologyRules(),
  ],
  options: {
    doNotFollow: {
      path: "node_modules|dist|target|storybook-static|\\.git|\\.nx|\\.repo",
    },
    tsPreCompilationDeps: true,
    combinedDependencies: true,
    enhancedResolveOptions: {
      exportsFields: ["exports"],
      conditionNames: ["import", "require", "node", "default"],
    },
  },
};
