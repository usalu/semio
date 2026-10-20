
/** 🧭️ One root command whose module closure the law holds to the registry's declaration. */
export type ZeroTouchCommandV1 = Readonly<{ id: string; module: string; requires: "none" | "dependencies" }>;

/** 🪜️ Walks the full closure of every command module: a workspace package specifier is refused for every command; an external package is refused for `none` and allowed for `dependencies`, whose module is only loaded after the router has provisioned the dependencies; a `none` command reaches a seam by a dynamic import only. */
export function zeroTouchCommandFindingsV1(commands: readonly ZeroTouchCommandV1[], seams: readonly string[], tree: ZeroTouchTreeV1): Readonly<{ files: Readonly<Record<string, number>>; findings: readonly ZeroTouchFindingV1[] }> {
  const findings: ZeroTouchFindingV1[] = [], files: Record<string, number> = {}, seamModules = new Set(seams);
  for (const command of commands) {
    const closure = `command:${command.id}`, seen = new Set<string>(), pending = [command.module];
    if (tree.read(command.module) === undefined) findings.push({ closure, kind: "missing-entry", file: command.module, specifier: command.module, detail: "The command module does not exist." });
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file)) continue;
      seen.add(file);
      const text = tree.read(file);
      if (text === undefined) continue;
      for (const { specifier, kind } of scanModuleSpecifiersV1(file, text)) {
        if (specifier.startsWith(".")) {
          const target = relativeTarget(tree, file, specifier);
          if (target === undefined) continue;
          if (command.requires === "dependencies" || !seamModules.has(target)) pending.push(target);
          else if (kind === "static") findings.push({ closure, kind: "seam-static", file, specifier, detail: `The post-install module ${target} may only be loaded by a dynamic import.` });
        } else if (specifier.startsWith("node:") || specifier.startsWith("bun:") || specifier === "bun" || isBuiltin(specifier)) continue;
        else if (Object.keys(tree.packages).some((name) => specifier === name || specifier.startsWith(`${name}/`))) {
          const target = workspaceSpecifierTargetV1(tree.packages, specifier);
          findings.push({ closure, kind: "workspace-specifier", file, specifier, detail: target === undefined ? "Import the owner module by a relative path; no export of the package resolves this specifier." : `Import the owner module by the relative path to ${target}.` });
        } else if (command.requires === "none") findings.push({ closure, kind: "external-package", file, specifier, detail: `The command declares requires none but reaches the external package ${specifier}; declare requires dependencies or remove the import.` });
      }
    }
    files[closure] = seen.size;
  }
  findings.sort((left, right) => compareBytes(left.closure, right.closure) || compareBytes(left.file, right.file) || compareBytes(left.specifier, right.specifier));
  return { files, findings };
}

/** 📋️ Reads the command rows the law needs from the registry text; the registry schema owns the complete shape. */
export function readZeroTouchCommandsV1(text: string): readonly ZeroTouchCommandV1[] {
  const value = JSON.parse(text) as { commands?: unknown };
  if (!Array.isArray(value.commands) || value.commands.length === 0) throw Error("Root command registry required");
  const ids = new Set<string>();
  return value.commands.map((row: Record<string, unknown>) => {
    if (typeof row.id !== "string" || typeof row.module !== "string" || (row.requires !== "none" && row.requires !== "dependencies") || ids.has(row.id)) throw Error("Root command registry row required");
    ids.add(row.id);
    return { id: row.id, module: row.module, requires: row.requires };
  });
}

export const ZERO_TOUCH_COMMANDS_FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json";

/** ✅️ Checks every command of the root registry against the checkout on disk; the seams of the closure roster are the post-install modules. */
export function checkZeroTouchCommandsV1(root: string): ReturnType<typeof zeroTouchCommandFindingsV1> {
  const tree = diskZeroTouchTreeV1(root), registry = tree.read(ZERO_TOUCH_COMMANDS_FILE), roster = tree.read(ZERO_TOUCH_CLOSURES_FILE);
  if (registry === undefined || roster === undefined) throw Error("Root command registry and closure roster required");
  return zeroTouchCommandFindingsV1(readZeroTouchCommandsV1(registry), readZeroTouchRosterV1(roster).seams.map((seam) => seam.module), tree);
}
