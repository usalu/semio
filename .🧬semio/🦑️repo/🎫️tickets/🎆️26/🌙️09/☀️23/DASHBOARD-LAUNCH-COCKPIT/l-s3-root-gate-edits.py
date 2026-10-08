"""L-S3 root gate: exact-string edits of the root `📜️script.ts`.

Usage: python3 l-s3-root-gate-edits.py <check|A|C> — run from the repository root.
Every edit replaces one anchor that must occur exactly once in the live file; a block anchor is cut
between two unique markers and must equal the same cut of the pre-edit snapshot, so a peer's change
inside an anchor aborts the whole step before anything is written. One step is one write.
"""
import sys

ROOT_SCRIPT = "📜️script.ts"
SNAPSHOT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/🗑️generated/launch-s3/root-script.before.ts"

PLAYGROUND_FILE_LINE = 'const INTERACTIVITY_ALL_APP_PLAYGROUND_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";\n'

A_CONSTANTS = r'''/** 🛝️ The generated playground catalog: one row per variant, emitted by the plugin registry from the
 * `[[package.metadata.semio.playground]]` rows of the crate that composes it. The dashboard starts a
 * variant from exactly this row — React and WGPU Wasm on the two declared `ports`, WGPU native by the
 * variant name alone — so a variant without two ports cannot be started in each renderer. */
''' + PLAYGROUND_FILE_LINE + r'''/** 🛤️ The script whose `VerifyScript.run` registers every verification verb: this file. */
const INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE = "📜️script.ts";
/** 📑️ The root project manifest. Its `verify` target forwards every argument to the verify router, so
 * the dashboard offers exactly the argument forms that target declares as a `choice` parameter.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json `#/$defs/TargetDeclaration` */
const INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE = "📋️project.json";
const INTERACTIVITY_ALL_APP_VERIFY_COMMAND = "bun ./📜️script.ts verify";
const INTERACTIVITY_ALL_APP_VERIFY_DECLARATION = "targets.verify.metadata.semio.dashboard.parameters";
/** 🚥️ Argument forms that are gates in their own right although the router registers no sub-verb for
 * them: a flag or a mode that their verb's handler reads. Every sub-verb the router does register is
 * derived from [[INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE]] and is never restated here. */
const INTERACTIVITY_ALL_APP_VERIFY_ARGUMENT_GATES: readonly (readonly string[])[] = [["interactivity", "apps", "--actions"], ["dependencies", "literal-external"]];
'''

A_PLAYGROUND_TYPE_OLD = 'type InteractivityAllAppPlayground = { variant: string; pluginId: string; appId?: string };\n'
A_PLAYGROUND_TYPE_NEW = 'type InteractivityAllAppPlayground = { variant: string; pluginId: string; cratePath: string; appId?: string; ports?: { react: number; wgpu: number } };\n'

A_READER_OLD = r'''    const appId = line.match(/\bapp:\s*"([^"]+)"/)?.[1];
    if (variant === "" || pluginId === "") failures.push(`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: malformed generated playground row`);
    else rows.push({ variant, pluginId, ...(appId ? { appId } : {}) });
'''
A_READER_NEW = r'''    const cratePath = line.match(/\bcratePath:\s*"([^"]+)"/)?.[1] ?? "";
    const appId = line.match(/\bapp:\s*"([^"]+)"/)?.[1];
    if (variant === "" || pluginId === "" || cratePath === "") {
      failures.push(`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: malformed generated playground row`);
      continue;
    }
    const declared = line.match(/\bports:\s*\{\s*react:\s*(\d+),\s*wgpu:\s*(\d+)\s*\}/u);
    const ports = { react: Number(declared?.[1]), wgpu: Number(declared?.[2]) };
    const startable = [ports.react, ports.wgpu].every((port) => Number.isInteger(port) && port >= 1 && port <= 65535) && ports.react !== ports.wgpu;
    if (!startable) failures.push(`${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}: variant ${JSON.stringify(variant)} declares no distinct React and WGPU port, so the dashboard cannot start it in each renderer; declare \`ports = { react = <port>, wgpu = <port> }\` on its \`[[package.metadata.semio.playground]]\` row in ${cratePath}/Cargo.toml and run \`@semio-tech/plugin-registry:generate\``);
    rows.push({ variant, pluginId, cratePath, ...(appId ? { appId } : {}), ...(startable ? { ports } : {}) });
'''

A_FUNCTIONS_ANCHOR = '/** 🧭️Derives generated playground launch identities from the schema-owned launch seed. */\n'
A_FUNCTIONS = r'''/** 🕹️Proves every owner-qualified app context has a playground variant the dashboard can start in React, WGPU Wasm and WGPU native. */
function interactivityAllAppPlaygroundCoverageFailures(descriptors: readonly InteractivityAllAppDescriptor[], playgrounds: readonly InteractivityAllAppPlayground[]): string[] {
  const roleNeutral = (id: string) => id.replace(/#(?:editor|viewer)$/u, "#surface");
  const failures: string[] = [];
  for (const descriptor of descriptors.filter((row) => row.kind === "app")) for (const appId of descriptor.appIds) {
    const candidates = playgrounds.filter((playground) => playground.pluginId === descriptor.pluginId && (!playground.appId || roleNeutral(playground.appId) === roleNeutral(appId)));
    if (!candidates.some((playground) => playground.ports !== undefined)) failures.push(`${descriptor.file}: ${appId} has no owner-qualified playground variant with a React and a WGPU port in ${INTERACTIVITY_ALL_APP_PLAYGROUND_FILE}; declare a \`[[package.metadata.semio.playground]]\` row (variant, app, ports) in the Cargo.toml of the crate that composes plugin ${JSON.stringify(descriptor.pluginId)} and run \`@semio-tech/plugin-registry:generate\``);
  }
  return failures;
}

/** 🚏️Reads the sub-verbs `VerifyScript.run` registers from the router's own route heads, so no second list of verification verbs exists. */
function interactivityAllAppVerifyRoutesFromSource(source: string): { routes: string[][]; failures: string[] } {
  const lines = source.split("\n");
  const start = lines.indexOf("export class VerifyScript extends Script {");
  const end = start < 0 ? -1 : lines.indexOf("  }", start);
  if (end < 0) return { routes: [], failures: [`${INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE}: \`VerifyScript.run\` was not found, so no verification verb can be derived`] };
  const routes = new Map<string, string[]>();
  const failures: string[] = [];
  for (let index = start; index < end; index += 1) {
    const line = lines[index]!;
    if (!/^ {4}\S/u.test(line) || line.startsWith("    if (await dispatchOwnedScriptRoute(") || !(line.startsWith("    if (") || line.includes("segments"))) continue;
    const head = line.match(/^ {4}if \(segments\[0\] === "([^"\s]+)"(?: && segments\[1\] === "([^"\s]+)")?\) (?:\{|return;)$/u);
    const route = head ? [head[1]!, ...(head[2] === undefined ? [] : [head[2]])] : [];
    if (!head) failures.push(`${INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE}:${index + 1}: unreadable verify route head; write \`if (segments[0] === "<verb>") {\` or \`if (segments[0] === "<verb>" && segments[1] === "<sub-verb>") {\` so the verb can be derived`);
    else if (routes.has(route.join(" "))) failures.push(`${INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE}:${index + 1}: verify ${route.join(" ")} is registered more than once, so its later arm never runs`);
    else routes.set(route.join(" "), route);
  }
  if (routes.size === 0 && failures.length === 0) failures.push(`${INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE}: \`VerifyScript.run\` registers no verification verb`);
  return { routes: [...routes.values()], failures };
}

/** 🗳️Reads the argument forms the root project's `verify` target declares in its one argument-owning dashboard `choice` parameter. */
function interactivityAllAppVerifyFormsFromSource(source: string): { forms: string[][]; failures: string[] } {
  const record = (value: unknown): Record<string, unknown> | undefined => value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : undefined;
  const declaration = `${INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE}: ${INTERACTIVITY_ALL_APP_VERIFY_DECLARATION}`;
  let manifest: Record<string, unknown> | undefined;
  try {
    manifest = record(JSON.parse(source));
  } catch {
    manifest = undefined;
  }
  if (!manifest) return { forms: [], failures: [`${INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE}: not a JSON project manifest, so no verification verb is declared`] };
  const target = record(record(manifest.targets)?.verify);
  if (!target) return { forms: [], failures: [`${INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE}: targets.verify is missing, so no dashboard command reaches the verify router`] };
  const failures: string[] = [];
  const options = record(target.options);
  if (options?.command !== INTERACTIVITY_ALL_APP_VERIFY_COMMAND || options.forwardAllArgs !== true) failures.push(`${INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE}: targets.verify.options must run \`${INTERACTIVITY_ALL_APP_VERIFY_COMMAND}\` with \`forwardAllArgs: true\`, or no declared argument form reaches the verify router`);
  const parameters = record(record(record(target.metadata)?.semio)?.dashboard)?.parameters;
  if (!Array.isArray(parameters)) return { forms: [], failures: [...failures, `${declaration} is missing; declare one \`choice\` parameter there whose \`values[].args\` are the verify argument forms a developer can start`] };
  const forms = new Map<string, string[]>();
  let owners = 0;
  for (const [index, parameterValue] of parameters.entries()) {
    const parameter = record(parameterValue);
    if (parameter?.kind !== "choice" || !Array.isArray(parameter.values)) continue;
    let owned = false;
    for (const [valueIndex, value] of parameter.values.entries()) {
      const args = record(value)?.args;
      if (args === undefined || (Array.isArray(args) && args.length === 0)) continue;
      owned = true;
      if (!Array.isArray(args) || args.some((word) => typeof word !== "string" || word === "")) failures.push(`${declaration}[${index}].values[${valueIndex}].args must be a list of non-empty words`);
      else if (forms.has(JSON.stringify(args))) failures.push(`${declaration} declares args ${JSON.stringify(args)} more than once`);
      else forms.set(JSON.stringify(args), args as string[]);
    }
    if (owned) owners += 1;
  }
  if (owners !== 1) failures.push(`${declaration} must declare the verify argument forms in exactly one \`choice\` parameter, found ${owners}`);
  return { forms: [...forms.values()], failures };
}

/** 🔭️Proves every verification verb the router registers and every required argument gate is a declared argument form, and every declared form reaches a registered verb. Unreadable routers and missing declarations are their readers' failures. */
function interactivityAllAppVerifyReachFailures(routes: readonly (readonly string[])[], forms: readonly (readonly string[])[]): string[] {
  if (routes.length === 0 || forms.length === 0) return [];
  const declaration = `${INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE}: ${INTERACTIVITY_ALL_APP_VERIFY_DECLARATION}`;
  const declared = new Set(forms.map((form) => JSON.stringify(form)));
  const answered = (form: readonly string[]) => routes.some((route) => route.length <= form.length && route.every((word, index) => form[index] === word));
  const failures: string[] = [];
  for (const gate of INTERACTIVITY_ALL_APP_VERIFY_ARGUMENT_GATES) if (!answered(gate)) failures.push(`${INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE}: required argument form \`verify ${gate.join(" ")}\` extends no sub-verb \`VerifyScript.run\` registers`);
  for (const form of [...routes, ...INTERACTIVITY_ALL_APP_VERIFY_ARGUMENT_GATES]) {
    if (!declared.has(JSON.stringify(form))) failures.push(`${declaration} declares no choice value for \`verify ${form.join(" ")}\`, so no developer can start it from the dashboard; add { "id": ${JSON.stringify(form.map((word) => word.replace(/^-+/u, "")).join("-"))}, "args": ${JSON.stringify(form)} } to its values`);
  }
  for (const form of forms) if (!answered(form)) failures.push(`${declaration} declares args ${JSON.stringify(form)}, which no sub-verb \`VerifyScript.run\` registers in ${INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE} answers; remove the value or register the verb`);
  return failures;
}

'''

A_EXPORTS_ANCHOR = '  interactivityAllAppPlaygroundLaunchNames,\n'
A_EXPORTS = r'''  INTERACTIVITY_ALL_APP_PLAYGROUND_FILE,
  INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE,
  INTERACTIVITY_ALL_APP_VERIFY_ARGUMENT_GATES,
  type InteractivityAllAppPlayground,
  interactivityAllAppPlaygroundsFromSource,
  interactivityAllAppPlaygroundCoverageFailures,
  interactivityAllAppVerifyRoutesFromSource,
  interactivityAllAppVerifyFormsFromSource,
  interactivityAllAppVerifyReachFailures,
'''

C_DISCOVERY_TYPE_NEW = r'''type InteractivityAllAppDiscovery = {
  descriptors: InteractivityAllAppDescriptor[];
  appCount: number;
  actionCount: number;
  migratedActionCount: number;
  missingActionCount: number;
  playgroundCoveredAppCount: number;
  playgroundMissingAppCount: number;
  duplicateAppIds: { id: string; files: string[] }[];
  playgrounds: InteractivityAllAppPlayground[];
  verifyRoutes: string[][];
  verifyForms: string[][];
  failures: string[];
};
'''

C_DISCOVERY_DOC_OLD = '/** 🧭️Discovers every plugin descriptor and every launch-derived product without a maintained product allowlist. */\n'
C_DISCOVERY_DOC_NEW = '/** 🧭️Discovers every plugin descriptor, its playground coverage and the declared verification verbs without a maintained product allowlist. */\n'

C_DISCOVERY_BODY_NEW = r'''  const playgrounds = interactivityAllAppPlaygroundsFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_PLAYGROUND_FILE));
  failures.push(...playgrounds.failures);
  const playgroundCoverageFailures = interactivityAllAppPlaygroundCoverageFailures(descriptors, playgrounds.rows);
  failures.push(...playgroundCoverageFailures);
  const verifyRoutes = interactivityAllAppVerifyRoutesFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_VERIFY_ROUTER_FILE));
  const verifyForms = interactivityAllAppVerifyFormsFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_VERIFY_MANIFEST_FILE));
  failures.push(...verifyRoutes.failures, ...verifyForms.failures, ...interactivityAllAppVerifyReachFailures(verifyRoutes.routes, verifyForms.forms));
  const descriptorAppCount = descriptors.reduce((count, descriptor) => count + descriptor.appIds.length, 0);
  return {
    descriptors: descriptors.sort((left, right) => left.file.localeCompare(right.file)),
    appCount: descriptorAppCount,
    actionCount: descriptors.reduce((count, descriptor) => count + descriptor.actions.length, 0),
    migratedActionCount: descriptors.reduce((count, descriptor) => count + descriptor.actions.filter((action) => action.disposition === "migrated").length, 0),
    missingActionCount: descriptors.reduce((count, descriptor) => count + descriptor.actions.filter((action) => action.disposition !== "migrated").length, 0),
    playgroundCoveredAppCount: descriptorAppCount - playgroundCoverageFailures.length,
    playgroundMissingAppCount: playgroundCoverageFailures.length,
    duplicateAppIds,
    playgrounds: playgrounds.rows,
    verifyRoutes: verifyRoutes.routes,
    verifyForms: verifyForms.forms,
'''

C_AUDIT_LOG_OLD = "    console.log(`[verify interactivity apps] ${apps.descriptors.length} descriptor(s), ${apps.appCount} app declaration(s), ${apps.launchOnlyProducts.length} launch-only product surface(s), ${apps.surfaceCount} total surface(s), ${apps.actionCount} action row(s), ${apps.launchCoveredAppCount} launch-covered app context(s), ${apps.launchMissingAppCount} missing launch context(s), ${apps.launches.length} dev launch surface(s), ${appSelfTests} hostile/oracle self-test(s).`);\n"
C_AUDIT_LOG_NEW = "    console.log(`[verify interactivity apps] ${apps.descriptors.length} descriptor(s), ${apps.appCount} app declaration(s), ${apps.actionCount} action row(s), ${apps.playgrounds.length} playground variant(s), ${apps.playgroundCoveredAppCount} playground-covered app context(s), ${apps.playgroundMissingAppCount} missing playground context(s), ${apps.verifyRoutes.length} verify route(s), ${apps.verifyForms.length} declared verify argument form(s), ${appSelfTests} hostile/oracle self-test(s).`);\n"

C_APPS_LOG_OLD = "    else console.log(`[verify interactivity apps] descriptors=${report.descriptors.length} extensions=${report.descriptors.filter((descriptor) => descriptor.kind === \"extension\").length} apps=${report.appCount} launchOnlyProducts=${report.launchOnlyProducts.length} surfaces=${report.surfaceCount} actions=${report.actionCount} migratedActions=${report.migratedActionCount} missingActions=${report.missingActionCount} launchCoveredApps=${report.launchCoveredAppCount} launchMissingApps=${report.launchMissingAppCount} launches=${report.launches.length} failures=${report.failures.length} selfTests=${selfTests}`);\n"
C_APPS_LOG_NEW = "    else console.log(`[verify interactivity apps] descriptors=${report.descriptors.length} extensions=${report.descriptors.filter((descriptor) => descriptor.kind === \"extension\").length} apps=${report.appCount} actions=${report.actionCount} migratedActions=${report.migratedActionCount} missingActions=${report.missingActionCount} playgroundVariants=${report.playgrounds.length} playgroundCoveredApps=${report.playgroundCoveredAppCount} playgroundMissingApps=${report.playgroundMissingAppCount} verifyRoutes=${report.verifyRoutes.length} verifyForms=${report.verifyForms.length} failures=${report.failures.length} selfTests=${selfTests}`);\n"

C_SERVED_OLD = " * It is a command segment rather than an env var so it stays reachable from `launch.json`, which is\n * how every dev here starts things and which carries no `env` field.\n"
C_SERVED_NEW = " * It is a command segment rather than an env var so it stays reachable as a plain argument form: a\n * dashboard parameter value, an Nx target or a shell selects it with the command alone.\n"
C_RENDERER_OLD = ' * `SEMIO_RENDERER = "wgpu"` assignment can never overrule a react launch row.\n'
C_RENDERER_NEW = ' * `SEMIO_RENDERER = "wgpu"` assignment can never overrule a start that chose react.\n'
C_POLICY_OLD = " * eval (see `.vscode/launch.json`'s `⚖️gate…` entries) that bypassed Nx entirely. `verify policy-breach\n"
C_POLICY_NEW = " * eval that bypassed Nx entirely. `verify policy-breach\n"


def between(text: str, start: str, end: str, include_end: bool) -> str:
    """Cuts the block from a unique start marker to the first end marker after it."""
    assert text.count(start) == 1, f"start marker occurs {text.count(start)}x: {start[:70]!r}"
    begin = text.index(start)
    stop = text.index(end, begin)
    return text[begin:stop + (len(end) if include_end else 0)]


def edits(step: str, live: str, snapshot: str) -> list[tuple[str, str, str]]:
    """Returns (name, anchor, replacement) for one step; block anchors must equal their snapshot cut."""
    def block(name: str, start: str, end: str, include_end: bool) -> str:
        cut = between(live, start, end, include_end)
        assert cut == between(snapshot, start, end, include_end), f"{name}: a peer changed this block since the snapshot"
        return cut
    if step == "A":
        return [
            ("A1 constants: playground docstring and verify sources", PLAYGROUND_FILE_LINE, A_CONSTANTS),
            ("A2 playground row type carries crate and renderer ports", A_PLAYGROUND_TYPE_OLD, A_PLAYGROUND_TYPE_NEW),
            ("A3 playground reader reads crate and renderer ports", A_READER_OLD, A_READER_NEW),
            ("A4 coverage, router, declaration and reach functions", A_FUNCTIONS_ANCHOR, A_FUNCTIONS + A_FUNCTIONS_ANCHOR),
            ("A5 exports of the new readers and laws", A_EXPORTS_ANCHOR, A_EXPORTS_ANCHOR + A_EXPORTS),
        ]
    assert step == "C"
    discovery_type = between(live, "type InteractivityAllAppDiscovery = {\n", "\n};\n", True)
    discovery_body = between(live, "  const launch = interactivityAllAppLaunchesFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_LAUNCH_FILE));\n", "    launchOnlyProducts,\n", True)
    assert discovery_type == between(snapshot, "type InteractivityAllAppDiscovery = {\n", "\n};\n", True)
    assert discovery_body == between(snapshot, "  const launch = interactivityAllAppLaunchesFromSource(policyReadFileSafe(repoRoot, INTERACTIVITY_ALL_APP_LAUNCH_FILE));\n", "    launchOnlyProducts,\n", True)
    return [
        ("C1 launch row capacity", "const INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY = 512;\n", ""),
        ("C2 dev command restatements and launch file paths", block("C2", "/** 🚀️ The browser dev command every generated playground launcher carries.", 'const INTERACTIVITY_ALL_APP_LAUNCH_SEED_FILE = ".vscode/🧩️launch.seed.jsonc";\n', True), ""),
        ("C3 required launch rows", block("C3", "/** ⚖️ Launch rows every `.vscode/launch.json` must register exactly once", '  { name: "⚖️gate🪆️composed-child-refs", command: "bun nx run workspace:verify -- composed-child-refs" },\n];\n', True), ""),
        ("C4 launch row type", "type InteractivityAllAppLaunch = { name: string; command: string; cwd: string; env: Readonly<Record<string, string>> };\n", ""),
        ("C5 discovery report shape", discovery_type, C_DISCOVERY_TYPE_NEW),
        ("C6 launch name, coverage and file readers", block("C6", "/** 🧭️Derives generated playground launch identities from the schema-owned launch seed. */\n", "/** 🪪️ Whether a repo-relative path is a plugin-descriptor COORDINATE", False), ""),
        ("C7a discovery docstring", C_DISCOVERY_DOC_OLD, C_DISCOVERY_DOC_NEW),
        ("C7b discovery reads the catalog, the router and the manifest", discovery_body, C_DISCOVERY_BODY_NEW),
        ("C8a audit ledger line", C_AUDIT_LOG_OLD, C_AUDIT_LOG_NEW),
        ("C8b audit deny message", "(all-app discovery/launch registration, unlisted blocking bridges", "(all-app discovery/dashboard declaration, unlisted blocking bridges"),
        ("C8c apps ledger line", C_APPS_LOG_OLD, C_APPS_LOG_NEW),
        ("C8d apps omitted-failures message", "additional discovery or launch-registration failure(s) omitted.`);\n", "additional discovery or dashboard-declaration failure(s) omitted.`);\n"),
        ("C8e apps failure message", "discovery or launch-registration failure(s).`);\n", "discovery or dashboard-declaration failure(s).`);\n"),
        ("C9a exports of the required rows and the launch type", "  INTERACTIVITY_ALL_APP_REQUIRED_GATES,\n  type InteractivityAllAppLaunch,\n", ""),
        ("C9b exports of the launch readers", "  interactivityAllAppLaunchesFromSource,\n  INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY,\n  interactivityAllAppLaunchCoverageFailures,\n  interactivityAllAppPlaygroundLaunchNames,\n", ""),
        ("C10a `dev … served` docstring", C_SERVED_OLD, C_SERVED_NEW),
        ("C10b renderer selection docstring", C_RENDERER_OLD, C_RENDERER_NEW),
        ("C10c policy-breach gates docstring", C_POLICY_OLD, C_POLICY_NEW),
    ]


def main() -> None:
    mode = sys.argv[1]
    live = open(ROOT_SCRIPT, encoding="utf8").read()
    snapshot = open(SNAPSHOT, encoding="utf8").read()
    steps = ["A", "C"] if mode == "check" else [mode]
    for step in steps:
        try:
            planned = edits(step, live, snapshot)
        except (AssertionError, ValueError) as error:
            print(f"step {step}: not applicable to the live file ({error})")
            continue
        result, total = live, 0
        for name, anchor, replacement in planned:
            count = result.count(anchor)
            delta = replacement.count("\n") - anchor.count("\n")
            print(f"step {step} | {name} | anchor occurrences={count} | anchor lines={anchor.count(chr(10))} | line delta={delta:+d}")
            assert count == 1, f"{name}: anchor is not unique"
            result = result.replace(anchor, replacement)
            total += delta
        assert result.count("\n") - live.count("\n") == total
        print(f"step {step}: {live.count(chr(10))} -> {result.count(chr(10))} lines ({total:+d})")
        if mode != "check":
            assert open(ROOT_SCRIPT, encoding="utf8").read() == live, "the live file changed while the step was planned"
            with open(ROOT_SCRIPT, "w", encoding="utf8") as handle:
                handle.write(result)
            print(f"step {step}: written")


main()
