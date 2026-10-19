const R = "/Users/ueli/Documents/semio/";
const LIB = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const catalog = await import(LIB + "🏗️authoring/🚀️launch-catalog/🟦️.ts");
const env = { BUN_RUNTIME_TRANSPILER_CACHE_PATH: "0", NODE_DISABLE_COMPILE_CACHE: "1" };
const registry = [
  { variant: "alpha", app: "s.alpha.alpha@1/*#editor", ports: { react: 7001, wgpu: 7101 } },
  { variant: "beta", ports: { react: 7002, wgpu: 7102 } },
  { variant: "gamma", ports: { react: 7003, wgpu: 7103 }, userPorts: { react: [7201, 7202], wgpu: [7301, 7302] } },
];
const literal = (name: string, command: string, group: string, order: number, extra: object = {}) => ({ name, type: "node-terminal", request: "launch", command, cwd: "${workspaceFolder}", env: { ...env, ...extra }, presentation: { group, order } });
const seed = {
  version: "0.2.0",
  configurations: [
    literal("🧪️literal build", "bun repo run demo:build", "4_build", 905.5),
    "@generated:alpha:react",
    literal("🧪️literal gate", "bun repo run demo:check", "4_gate", 930, { UNSET_ME: null }),
    "@generated:alpha:wgpu",
    "@generated:beta:react",
    "@generated:gamma:react",
    "@generated:gamma:wgpu",
    "@generated:gamma:users",
    "@generated:crate-routes",
    literal("🧪️literal tail", "bun repo run ${input:demoTarget}:test", "4_test", 910),
  ],
  compounds: [{ name: "🧭️compound gamma", configurations: ["🛠️dev🧪️gamma⚛️react", "🛠️dev🧪️gamma👤️1⚛️react"], stopAll: true }],
  inputs: [{ id: "demoTarget", type: "pickString", description: "Demo project", options: ["demo"] }],
  devLaunchers: {
    alpha: { namePrefix: "🧪️alpha", order: 10, wgpuOrder: 10.1 },
    beta: { namePrefix: "🧪️beta", order: 11 },
    gamma: { namePrefix: "🧪️gamma", order: 12, wgpuOrder: 12.1, env: { GAMMA_HUB: "http://127.0.0.1:1" }, users: { namePrefixPattern: "🧪️gamma👤️{N}", env: { GAMMA_DATA: "${workspaceFolder}/data/user{N}" } } },
  },
  crateRoutes: { subjects: [{ name: "demo-core", shape: "lib" }, { name: "demo-core-tests", project: "@semio-tech/demo-core-tests-rs", shape: "tests" }, { name: "demo-wide", shape: "package", targets: ["test"], arguments: { test: ["--test", "wide", "--offline"] } }] },
};
const expected = catalog.expandLaunchCatalog(seed, registry);
const problems = [
  { name: "a variant without a launcher row", patch: [{ op: "remove", path: "/seed/devLaunchers/beta" }], problem: "beta" },
  { name: "a launcher row for a variant the registry does not declare", patch: [{ op: "remove", path: "/registry/0" }], problem: "names a variant the playground registry does not declare" },
  { name: "a registry variant without a launcher row", patch: [{ op: "add", path: "/registry/-", value: { variant: "delta", ports: { react: 7004, wgpu: 7104 } } }], problem: "playground variant delta has no devLaunchers row" },
  { name: "a launcher row without its react placeholder", patch: [{ op: "remove", path: "/seed/configurations/1" }], problem: "devLaunchers.alpha has no @generated:alpha:react placeholder" },
  { name: "a placeholder twice", patch: [{ op: "add", path: "/seed/configurations/-", value: "@generated:beta:react" }], problem: "placeholder @generated:beta:react appears twice" },
  { name: "a wgpu placeholder without wgpuOrder", patch: [{ op: "add", path: "/seed/configurations/-", value: "@generated:beta:wgpu" }], problem: "@generated:beta:wgpu needs wgpuOrder" },
  { name: "a users placeholder without user ports", patch: [{ op: "add", path: "/seed/devLaunchers/beta/users", value: { namePrefixPattern: "🧪️beta👤️{N}", env: {} } }, { op: "add", path: "/seed/configurations/-", value: "@generated:beta:users" }], problem: "@generated:beta:users needs a users template and user ports" },
  { name: "a catalog without the crate anchor", patch: [{ op: "remove", path: "/seed/configurations/8" }], problem: "must appear exactly once" },
  { name: "a literal name twice", patch: [{ op: "add", path: "/seed/configurations/-", value: literal("🧪️literal build", "bun repo run demo:build", "4_build", 906) }], problem: "configuration name 🧪️literal build appears 2 times" },
  { name: "a literal that takes a generated name", patch: [{ op: "add", path: "/seed/configurations/-", value: literal("🛠️dev🧪️beta⚛️react", "bun repo run demo:build", "4_build", 906) }], problem: "configuration name 🛠️dev🧪️beta⚛️react appears 2 times" },
  { name: "a compound that names a missing configuration", patch: [{ op: "add", path: "/seed/compounds/0/configurations/-", value: "🧪️ghost" }], problem: "names the missing configuration 🧪️ghost" },
  { name: "a command that uses an undeclared input", patch: [{ op: "replace", path: "/seed/inputs", value: [] }], problem: "uses the undeclared input demoTarget" },
  { name: "a literal with a machine specific path", patch: [{ op: "add", path: "/seed/configurations/-", value: literal("🧪️absolute", "bun /Users/someone/Documents/semio/script.ts", "4_build", 907) }], problem: "machine specific absolute path" },
  { name: "a literal without the build environment", patch: [{ op: "add", path: "/seed/configurations/-", value: { ...literal("🧪️bare", "bun repo run demo:build", "4_build", 908), env: {} } }], problem: "env" },
  { name: "a crate subject twice", patch: [{ op: "add", path: "/seed/crateRoutes/subjects/-", value: { name: "demo-core", shape: "tests" } }], problem: "crate subject demo-core appears twice" },
  { name: "a crate subject that is no crate", patch: [{ op: "add", path: "/seed/crateRoutes/subjects/-", value: { name: "Bad Name", shape: "lib" } }], problem: "seed" },
  { name: "a seed key the contract does not know", patch: [{ op: "add", path: "/seed/projectLaunchers", value: {} }], problem: "seed" },
];
const register = { subjects: [{ name: "demo-core", shape: "lib" }, { name: "demo-new", shape: "lib" }, { name: "demo-new", shape: "tests" }, { name: "demo-new-tests", shape: "tests" }], addedNames: ["demo-new", "demo-new-tests"] };
const fixture = { registry, manifest: { path: "Cargo.toml", text: "[package]\nname = \"demo\"\n\n[[package.metadata.semio.playground]]\nvariant = \"gamma\"\nports = { react = 7003, wgpu = 7103 }\nuser_ports = { react = [7201, 7202], wgpu = [7301, 7302] }\n\n[[package.metadata.semio.playground]]\nvariant = \"alpha\"\napp = \"s.alpha.alpha@1/*#editor\"\nports = { react = 7001, wgpu = 7101 }\n" }, seed, expected, problems, register };
await Bun.write(LIB + "🧪️tests/🧫️fixtures/🚀️launch-catalog/🔣️.json", JSON.stringify(fixture, null, 2) + "\n");
console.log(expected.configurations.map((c: any) => `${c.name} @${c.presentation?.group}/${c.presentation?.order}`).join("\n"));
