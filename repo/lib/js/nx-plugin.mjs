import { dirname, join, relative } from "node:path";

/**
 * @param {string} workspaceRoot
 * @param {string} absScript
 */
function extraLintInputs(workspaceRoot, absScript) {
  const base = absScript.split(/[/\\]/).pop() ?? "";
  if (base === "lint.script.ts") return [];
  const m = base.match(/^(.+)\.([^.]+)\.lint\.script\.ts$/);
  if (!m) return [];
  const targetAbs = join(dirname(absScript), `${m[1]}.${m[2]}`);
  const tr = relative(workspaceRoot, targetAbs).split("\\").join("/");
  return [`{workspaceRoot}/${tr}`];
}

/**
 * @param {string[]} configFiles
 * @param {unknown} _options
 * @param {{ workspaceRoot: string }} context
 */
function lintScriptProjects(configFiles, _options, context) {
  const { workspaceRoot } = context;
  /** @param {string} p */
  const nxPath = (p) => p.split("\\").join("/");

  return configFiles
    .filter(
      (configFile) =>
        !configFile.includes("node_modules") &&
        !configFile.includes(".repo") &&
        !configFile.includes("/dist/") &&
        !configFile.includes("\\dist\\"),
    )
    .map((configFile) => {
      const abs = join(workspaceRoot, configFile);
      const rel = nxPath(relative(workspaceRoot, abs));
      const projectRoot = dirname(abs);
      const slug = rel.replace(/[^a-zA-Z0-9_-]+/g, "_");
      const projectName = `breach-${slug}`;
      const extraInputs = extraLintInputs(workspaceRoot, abs);
      return [
        configFile,
        {
          projects: {
            [projectName]: {
              name: projectName,
              root: nxPath(relative(workspaceRoot, projectRoot)) || ".",
              targets: {
                lint: {
                  executor: "nx:run-commands",
                  options: {
                    cwd: workspaceRoot,
                    command: `bun repo/lib/js/bin/lint.ts "${rel}"`,
                  },
                  inputs: [`{workspaceRoot}/${rel}`, ...extraInputs],
                  outputs: [`{workspaceRoot}/.repo/cache/breaches`],
                  cache: true,
                },
              },
            },
          },
        },
      ];
    });
}

export default {
  name: "@repo/lint-scripts-file",
  createNodesV2: ["**/*lint.script.ts", lintScriptProjects],
};
