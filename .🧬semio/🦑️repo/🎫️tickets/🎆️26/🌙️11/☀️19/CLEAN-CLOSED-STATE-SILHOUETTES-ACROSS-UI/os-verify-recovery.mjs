import { existsSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { isBuiltin, registerHooks } from "node:module";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const packageRoot = resolve(process.argv[2]);
const checkout = resolve(import.meta.dirname, "../../../../../../..");
const modules = join(packageRoot, "node_modules");
const anchor = join(packageRoot, "package.json");
const lock = JSON.parse(readFileSync(join(checkout, "bun.lock"), "utf8").replace(/,\s*([}\]])/g, "$1"));
for (const [folder, { name }] of Object.entries(lock.workspaces)) {
  if (!folder || !name) continue;
  const target = join(checkout, folder);
  const link = join(modules, name);
  if (!existsSync(join(target, "package.json")) || existsSync(link)) continue;
  mkdirSync(dirname(link), { recursive: true });
  symlinkSync(target, link, "junction");
}

/** 📦 Resolve pinned dependencies from the isolated install for this preview process. */
function installed(specifier) {
  if (!/^(?:@[^/]+\/)?[a-zA-Z0-9_]/.test(specifier) || specifier.includes(":") || isBuiltin(specifier)) return false;
  const parts = specifier.split("/");
  const name = parts.slice(0, specifier.startsWith("@") ? 2 : 1).join("/");
  return existsSync(join(modules, name, "package.json"));
}

const parentURL = pathToFileURL(anchor).href;
registerHooks({
  resolve(specifier, context, nextResolve) {
    if (installed(specifier)) return nextResolve(specifier, { ...context, parentURL });
    if (/^(?:@[^/]+\/)?[a-zA-Z0-9_]/.test(specifier) && !specifier.includes(":") && !isBuiltin(specifier) &&
        !context.parentURL?.includes("/node_modules/")) {
      throw new Error(`Isolated dependency missing: ${specifier} from ${context.parentURL}`);
    }
    return nextResolve(specifier, context);
  },
});

const configuration = resolve(import.meta.dirname, "../../☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/vite-round3-os.config.ts");
const [{ createServer }, { default: preview }] = await Promise.all([import("vite"), import(pathToFileURL(configuration))]);
const environment = { command: "serve", mode: "prototype" };
const config = await preview(environment);
const entry = pathToFileURL(join(config.root, "main.tsx")).pathname;
const html = readFileSync(join(config.root, "index.html"), "utf8");
writeFileSync(join(packageRoot, "index.html"), html.replace("./main.tsx", "/@fs" + entry));
const server = await createServer({
  ...config,
  root: packageRoot,
  configFile: false,
  mode: environment.mode,
  resolve: { ...config.resolve, dedupe: [] },
  optimizeDeps: { noDiscovery: true, include: [
    "react", "react-dom", "react-dom/client", "react/jsx-runtime", "react/jsx-dev-runtime",
    "@react-three/fiber", "@react-three/drei", "@xyflow/react",
    "@dnd-kit/core", "@dnd-kit/sortable", "@dnd-kit/utilities", "react-resizable-panels", "react-i18next",
    "scheduler", "zustand/traditional",
    "use-sync-external-store/shim", "use-sync-external-store/shim/with-selector.js",
  ] },
  plugins: [{
    name: "os-preview-isolated-dependencies",
    enforce: "pre",
    resolveId(specifier, importer, options) {
      if (importer !== anchor && installed(specifier)) return this.resolve(specifier, anchor, { ...options, skipSelf: true });
    },
  }, ...config.plugins],
  server: { ...config.server, host: "127.0.0.1", port: 5602, strictPort: true, hmr: false, watch: null },
});
await server.listen();
server.printUrls();
