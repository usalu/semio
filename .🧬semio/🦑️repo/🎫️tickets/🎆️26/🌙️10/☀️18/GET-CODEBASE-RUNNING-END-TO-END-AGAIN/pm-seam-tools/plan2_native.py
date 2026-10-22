"""🌍️ Round 2, concern 5: the native dependency synchronisation builds its commands from the managers of the other ecosystems (python, rust, go) instead of literal tool argv. Proven against the live text and against the text the langs-conc set leaves behind (python is an alternative pair of edits, the rest is common to both)."""
def declare(edit, paths):
    D = paths.DEPENDENCIES
    N = D + "/🏗️native/🟦️.ts"
    def both(id, group, old, new, count=1):
        edit(N, id, group, old, new, count)
        edit(N, id, group, old, new, count, base="langs-conc", emit=False)
    both("native-import", "glue", 'import { wasmBindgenVersion } from "../../../🦀️cargo/🕸️wasm-bindgen/🟦️.ts";\n',
         'import { wasmBindgenVersion } from "../../../🦀️cargo/🕸️wasm-bindgen/🟦️.ts";\nimport { selectPackageManagerV1 } from "../🔌️managers/📚️catalog/🟦️.ts";\n')
    both("native-render", "glue", "  const run = async (command: string, args: string[], capture = false, environment?: NodeJS.ProcessEnv): Promise<string> => {",
         '  const render = (ecosystem: string, operation: "lock" | "sync" | "provision", request: Record<string, unknown> = {}): [string, string[]] => {\n    const invocation = selectPackageManagerV1(ecosystem).render(operation, { platform: process.platform, ...request });\n    return [invocation.executable, [...invocation.arguments]];\n  };\n'
         "  const run = async (command: string, args: string[], capture = false, environment?: NodeJS.ProcessEnv): Promise<string> => {")
    both("native-rust-lock", "glue", 'await run("cargo", ["update", "--workspace", "--manifest-path", owner.manifest])', 'await run(...render("rust", "lock", { manifest: owner.manifest }))')
    both("native-go", "glue", 'await run("go", ["mod", "download"], false,', 'await run(...render("go", "sync"), false,')
    edit(N, "native-python", "glue", '  if (kind === "python") await run("uv", ["sync", "--locked", "--all-packages", "--all-groups"]);\n',
         '  if (kind === "python") await run(...render("python", "sync", { scope: "all" }));\n', alt="native-python")
    edit(N, "native-python-planned", "glue", '    await run("uv", [...plan.installArguments], false, planned);\n    await run("uv", [...plan.syncArguments], false, planned);\n',
         '    await run(...render("python", "provision", { settings: { series: plan.series } }), false, planned);\n    await run(...render("python", "sync", { settings: { project: plan.root }, lists: { groups: plan.groups } }), false, planned);\n', alt="native-python", base="langs-conc")
