import {sha256V1} from "../🔏️key/🟦️.ts";
import {sortedCodePointsV1, type UnitPlanV1, type UnitV1, type UnitsPolicyV1} from "../🕸️graph/🟦️.ts";

/** 🏷️ A target or rule name that is a pure function of the unit id: readable ASCII slug plus eight hex digits of the id digest, so ids never collide after slugging. */
export function unitTargetNameV1(id: string): string {
  const slug = id.normalize("NFKD").replace(/[^A-Za-z0-9]+/g, "-").replace(/^-+|-+$/g, "").toLowerCase().slice(0, 48);
  return `unit-${slug || "x"}-${sha256V1(id).slice(0, 8)}`;
}
/** 📄️ Paths of the two small files every unit publishes below the output root: `iface.json` (interface digest, closure and artifact) and `out.json` (output digest). Dependents hash these and never the unit's sources or bulk outputs. */
export const unitInterfacePathV1 = (outputRoot: string, id: string): string => `${outputRoot}/${unitTargetNameV1(id)}/iface.json`;
export const unitOutputPathV1 = (outputRoot: string, id: string): string => `${outputRoot}/${unitTargetNameV1(id)}/out.json`;
/** 📋️ The spec file of one unit: the unit itself, which `units exec` reads. One file per unit keeps the inputs of a unit independent of every other unit. */
export const unitSpecPathV1 = (specRoot: string, id: string): string => `${specRoot}/${unitTargetNameV1(id)}.json`;
/** 📋️ The spec files of a plan (path to text). */
export function unitSpecsV1(plan: UnitPlanV1, specRoot: string): Readonly<Record<string, string>> {
  return Object.fromEntries(plan.order.map(id => [unitSpecPathV1(specRoot, id), JSON.stringify(plan.byId.get(id)!, null, 2) + "\n"] as const));
}

/** 🎯️ One Nx target of a unit. `inputs` names the unit's own files and spec and, for each consumed dependency, only the published interface or output file (`dependentTasksOutputFiles`), so Nx cuts off by itself when a dependency's closure did not change. */
export type NxUnitTargetV1 = {
  readonly name: string;
  readonly unit: string;
  readonly executor: "nx:run-commands";
  readonly command: readonly string[];
  readonly cache: boolean;
  readonly inputs: readonly (string | {readonly dependentTasksOutputFiles: string; readonly transitive: false})[];
  readonly outputs: readonly string[];
  readonly dependsOn: readonly string[];
};
/** 🎯️ Emits the Nx targets of a plan, in dependency order. The command is `[...command, "exec", "--spec", <spec file>, "--out-root", outputRoot]`; `units exec` finds the files of its dependencies by the naming convention below the output root. */
export function nxUnitTargetsV1(plan: UnitPlanV1, options: {readonly outputRoot: string; readonly specRoot: string; readonly command: readonly string[]}): readonly NxUnitTargetV1[] {
  return plan.order.map(id => {
    const unit = plan.byId.get(id)!, consumed = new Set(unit.dependencies.map(dependency => dependency.use));
    const inputs: NxUnitTargetV1["inputs"][number][] = [...[...unit.sources, ...unit.inputs, unitSpecPathV1(options.specRoot, id)].map(path => `{workspaceRoot}/${path}`)];
    if (consumed.has("interface") || consumed.has("both")) inputs.push({dependentTasksOutputFiles: "**/iface.json", transitive: false});
    if (consumed.has("implementation") || consumed.has("both")) inputs.push({dependentTasksOutputFiles: "**/out.json", transitive: false});
    return {
      name: unitTargetNameV1(id), unit: id, executor: "nx:run-commands", command: [...options.command, "exec", "--spec", unitSpecPathV1(options.specRoot, id), "--out-root", options.outputRoot], cache: true, inputs,
      outputs: [`{workspaceRoot}/${unitOutputPathV1(options.outputRoot, id)}`, ...(unit.interface ? [`{workspaceRoot}/${unitInterfacePathV1(options.outputRoot, id)}`] : [])],
      dependsOn: unit.dependencies.map(dependency => unitTargetNameV1(dependency.unit)).sort(),
    };
  });
}

const quote = (value: string): string => JSON.stringify(value);
const list = (values: readonly string[]): string => values.length ? "[" + values.map(quote).join(", ") + "]" : "[]";
/** 🏗️ The Starlark rule every unit uses. A unit consumes the `iface.json` of each interface dependency (direct only: the file carries the closure) and the `out.json` files of the whole implementation closure of each implementation dependency, so Bazel's own action cache cuts off at an unchanged interface. `dep_srcs` lists dependency sources for tools that read them (policy `bazel: sources`). */
export const UNITS_BZL_V1 = `UnitInfo = provider(fields = ["iface", "out", "outs"])

def _unit_impl(ctx):
    out = ctx.actions.declare_file(ctx.label.name + "/out.json")
    iface = ctx.actions.declare_file(ctx.label.name + "/iface.json") if ctx.attr.has_interface else None
    iface_files = [dep[UnitInfo].iface for dep in ctx.attr.iface_deps]
    impl_outs = depset(transitive = [dep[UnitInfo].outs for dep in ctx.attr.impl_deps])
    args = ctx.actions.args()
    args.add_all(["units", "exec", "--spec", ctx.file.spec, "--out", out])
    if iface:
        args.add_all(["--iface", iface])
    args.add_all(iface_files, before_each = "--iface-dep")
    args.add_all(impl_outs, before_each = "--impl-dep")
    inputs = depset(
        ctx.files.srcs + ctx.files.dep_srcs + [ctx.file.spec] + iface_files,
        transitive = [impl_outs],
    )
    ctx.actions.run(
        executable = ctx.executable.runner,
        arguments = [args],
        inputs = inputs,
        outputs = [out] + ([iface] if iface else []),
        env = ctx.attr.env,
        use_default_shell_env = True,
        mnemonic = "RepoUnit",
        progress_message = "Unit " + ctx.attr.unit,
    )
    all_outs = depset([out], transitive = [dep[UnitInfo].outs for dep in ctx.attr.iface_deps + ctx.attr.impl_deps])
    return [DefaultInfo(files = depset([out] + ([iface] if iface else []))), UnitInfo(iface = iface if iface else out, out = out, outs = all_outs)]

repo_unit = rule(
    implementation = _unit_impl,
    attrs = {
        "unit": attr.string(mandatory = True),
        "spec": attr.label(allow_single_file = True, mandatory = True),
        "srcs": attr.label_list(allow_files = True),
        "dep_srcs": attr.label_list(allow_files = True),
        "iface_deps": attr.label_list(providers = [UnitInfo]),
        "impl_deps": attr.label_list(providers = [UnitInfo]),
        "has_interface": attr.bool(default = False),
        "toolchain": attr.string(mandatory = True),
        "env": attr.string_dict(),
        "runner": attr.label(allow_single_file = True, executable = True, cfg = "exec", mandatory = True),
    },
)
`;
/** 🏗️ Emits a BUILD file text with one `repo_unit` per unit, in dependency order. Sources and inputs are file labels relative to the package (the BUILD file sits at the workspace root). `declared` edges order and select in this model but add no Bazel dependency: they never enter the inputs. A unit marked `SEMIO_BAZEL_INPUTS=sources` in its environment (or, unmarked, whose dependencies have an interface kind with `bazel: sources`) also lists the sources of its whole dependency closure; `extraInputs` adds files a tool needs without keying them (Go: go.mod); `environment` is the process environment of every action (the language of the texts); the unit's own environment travels in its spec. */
export function bazelUnitsV1(plan: UnitPlanV1, options: {readonly runner: string; readonly load: string; readonly specRoot: string; readonly policy: UnitsPolicyV1; readonly environment?: Readonly<Record<string, string>>; readonly extraInputs?: (unit: UnitV1) => readonly string[]}): string {
  const lines = [`load(${quote(options.load)}, "repo_unit")`, ""], extra = options.extraInputs ?? (() => []);
  const closure = new Map<string, Set<string>>();
  const files = (id: string): Set<string> => {
    let found = closure.get(id);
    if (!found) {
      const unit = plan.byId.get(id)!;
      found = new Set([...unit.sources, ...unit.inputs, ...extra(unit)]);
      for (const dependency of unit.dependencies) for (const path of files(dependency.unit)) found.add(path);
      closure.set(id, found);
    }
    return found;
  };
  for (const id of plan.order) {
    const unit: UnitV1 = plan.byId.get(id)!, iface = unit.dependencies.filter(dependency => dependency.use === "interface" || dependency.use === "both").map(dependency => ":" + unitTargetNameV1(dependency.unit)).sort(), impl = unit.dependencies.filter(dependency => dependency.use === "implementation" || dependency.use === "both").map(dependency => ":" + unitTargetNameV1(dependency.unit)).sort();
    const environment = options.environment ?? {}, env = sortedCodePointsV1(Object.keys(environment)).map(key => `${quote(key)}: ${quote(environment[key]!)}`).join(", ");
    const own = new Set([...unit.sources, ...unit.inputs, ...extra(unit)]);
    const readsSources = unit.action.env.SEMIO_BAZEL_INPUTS === "sources" || (unit.action.env.SEMIO_BAZEL_INPUTS === undefined && unit.dependencies.some(dependency => { const kind = plan.byId.get(dependency.unit)!.interface; return dependency.use !== "declared" && kind !== null && options.policy.interfaceKinds[kind.kind]?.bazel === "sources"; }));
    const depSources = readsSources ? sortedCodePointsV1([...files(id)].filter(path => !own.has(path))) : [];
    lines.push("repo_unit(", `    name = ${quote(unitTargetNameV1(id))},`, `    unit = ${quote(id)},`, `    spec = ${quote(`${options.specRoot}/${unitTargetNameV1(id)}.json`)},`, `    srcs = ${list(sortedCodePointsV1(own))},`);
    if (depSources.length) lines.push(`    dep_srcs = ${list(depSources)},`);
    if (iface.length) lines.push(`    iface_deps = ${list(iface)},`);
    if (impl.length) lines.push(`    impl_deps = ${list(impl)},`);
    if (unit.interface) lines.push("    has_interface = True,");
    lines.push(`    toolchain = ${quote(unit.toolchain.id)},`);
    if (env) lines.push(`    env = {${env}},`);
    lines.push(`    runner = ${quote(options.runner)},`, ")", "");
  }
  return lines.join("\n");
}
