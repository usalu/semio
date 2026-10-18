const source = `
const STUB = /^(@semio-tech\\/(framework-surface-rs|framework-editor-rs|framework-os-node-graph-rs|puzzle-wasm|flow-core)(\\/.*)?|[^?]*\\.wasm(\\?.*)?)$/u;
export function canonicalWasmBuildVitePlugin() {
  return { name: "wbs-wasm-stub", enforce: "pre",
    resolveId(id) { if (STUB.test(id)) return { id: "\\0wbs-stub:" + id, syntheticNamedExports: true }; },
    load(id) { if (id.startsWith("\\0wbs-stub:")) return "export default {};"; } };
}
`;
Bun.plugin({
  name: "wbs-wasm-stub",
  setup(build) {
    build.onLoad({ filter: /caching\/.*vite\/.*wasm\/[^/]*\.ts$/u }, (args) => {
      if (!args.path.endsWith("/🟦️.ts") || args.path.includes("/📦️delivery/")) return undefined;
      return { contents: source, loader: "ts" };
    });
  },
});
