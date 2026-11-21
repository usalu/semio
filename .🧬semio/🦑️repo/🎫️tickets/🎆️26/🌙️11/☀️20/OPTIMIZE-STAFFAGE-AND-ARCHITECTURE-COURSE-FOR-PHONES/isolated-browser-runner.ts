import { existsSync, readFileSync } from "node:fs";
import * as nodeModule from "node:module";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import type { InlineConfig, Plugin, ViteDevServer } from "vite";

const ticket = dirname(fileURLToPath(import.meta.url));
const workspace = resolve(ticket, "../../../../../../..");
const scratchDefault = join(ticket, "🗑️generated/w");
const bare = /^[a-z@][^:\\]*$/iu;

interface WorkspacePackage {
  readonly name?: string;
  readonly workspaces?: readonly string[];
}

function manifest(path: string): WorkspacePackage {
  return JSON.parse(readFileSync(path, "utf8")) as WorkspacePackage;
}

function workspaceResolvers(): Map<string, NodeJS.Require> {
  const resolvers = new Map<string, NodeJS.Require>();
  for (const owner of [workspace, join(workspace, "🎓️teaching")]) {
    for (const member of manifest(join(owner, "package.json")).workspaces ?? []) {
      const memberManifest = join(owner, member, "package.json");
      if (!existsSync(memberManifest)) continue;
      const { name } = manifest(memberManifest);
      if (name?.startsWith("@semio-tech/")) resolvers.set(name, nodeModule.createRequire(memberManifest));
    }
  }
  return resolvers;
}

let packageResolvers: Map<string, NodeJS.Require> | undefined;

function findWorkspacePackage(id: string): string | undefined {
  if (!id.startsWith("@semio-tech/")) return undefined;
  const name = id.split("/").slice(0, 2).join("/");
  const own = (packageResolvers ??= workspaceResolvers()).get(name);
  try {
    return own?.resolve(id);
  } catch {
    return undefined;
  }
}

/** 🧭️ Resolves Vite browser dependencies from the isolated install while keeping the live source tree. */
export function isolatedVitePackageResolver(scratch = scratchDefault): Plugin {
  const scratchManifest = join(scratch, "package.json");
  return {
    name: "phone-ticket-isolated-vite-packages",
    enforce: "pre",
    async resolveId(id, importer, options) {
      if (!bare.test(id) || importer === undefined || importer.startsWith(scratch) || nodeModule.isBuiltin(id)) return;
      const own = findWorkspacePackage(id);
      if (own !== undefined) return own;
      if (id.startsWith("@semio-tech/")) return;
      return await this.resolve(id, scratchManifest, { ...options, skipSelf: true });
    },
  };
}

/** 🔌️ Keeps bare Node dependencies on the scratch installation for config, tests and browser harness modules. */
export function registerIsolatedDependencies(scratch: string): () => void {
  const manifestPath = join(scratch, "package.json");
  const scratchManifest = pathToFileURL(manifestPath).href;
  const scratchURL = pathToFileURL(`${scratch}/`).href;
  if (typeof Bun !== "undefined") {
    const dependency = nodeModule.createRequire(manifestPath);
    Bun.plugin({
      name: "phone-ticket-isolated-node-packages",
      setup(build) {
        build.onResolve({ filter: bare }, ({ path, importer }) => {
          if (importer?.startsWith(scratch) || nodeModule.isBuiltin(path)) return;
          return { path: findWorkspacePackage(path) ?? dependency.resolve(path) };
        });
      },
    });
    return () => undefined;
  }
  const hooks = nodeModule.registerHooks({
    load(url, context, nextLoad) {
      if (!url.startsWith("file:") || url.startsWith(scratchURL) || !url.endsWith(".ts")) return nextLoad(url, context);
      const file = fileURLToPath(url);
      const source = nodeModule.stripTypeScriptTypes(readFileSync(file, "utf8"), { mode: "transform", sourceMap: true, sourceUrl: file });
      return { format: "module", source, shortCircuit: true };
    },
    resolve(specifier, context, nextResolve) {
      if (!bare.test(specifier) || nodeModule.isBuiltin(specifier)) return nextResolve(specifier, context);
      const own = findWorkspacePackage(specifier);
      if (own !== undefined) return nextResolve(pathToFileURL(own).href, context);
      const requested = specifier.split("/").slice(0, specifier.startsWith("@") ? 2 : 1).join("/");
      const wasi = process.env.SEMIO_PHONE_VERIFY_WASM === "true";
      const replacement = !wasi ? undefined : requested === "rollup" ? "@rollup/wasm-node" : requested === "lightningcss" ? "lightningcss-wasm" : undefined;
      const name = replacement !== undefined && existsSync(join(scratch, "node_modules", replacement)) ? replacement : requested;
      const insideScratch = name === requested && context.parentURL?.startsWith(scratchURL);
      let directory = insideScratch ? dirname(fileURLToPath(context.parentURL!)) : scratch;
      while (directory !== scratch && !existsSync(join(directory, "node_modules", name))) directory = dirname(directory);
      if (!existsSync(join(directory, "node_modules", name))) throw Object.assign(new Error(`Package ${name} is absent from the isolated installation`), { code: "MODULE_NOT_FOUND" });
      const from = insideScratch ? context : { ...context, parentURL: scratchManifest };
      return nextResolve(name + specifier.slice(requested.length), from);
    },
  });
  return () => hooks.deregister();
}

/** ▶️ Imports the ticket-owned browser harness with the isolated package resolver in effect. */
export async function run(scratch: string, file: string): Promise<unknown> {
  const deregister = registerIsolatedDependencies(scratch);
  try {
    return await import(pathToFileURL(resolve(scratch, file)).href);
  } finally {
    deregister();
  }
}

/** 🌐️ Starts a ticket browser server with the live application config and scratch-owned transform state. */
export async function serve(scratch: string, configFile: string, port: number, middleware = false): Promise<ViteDevServer> {
  if (!Number.isSafeInteger(port) || port < 1 || port > 65535) throw new Error("Expected a valid local port");
  const deregister = registerIsolatedDependencies(scratch);
  try {
    const { createServer } = await import("vite") as typeof import("vite");
    const mode = "prototype";
    const loaded = await import(pathToFileURL(resolve(scratch, configFile)).href);
    const config: InlineConfig = typeof loaded.default === "function" ? await loaded.default({ command: "serve", mode }) : loaded.default;
    const trace = (phase: string) => {
      if (process.env.SEMIO_PHONE_VERIFY_VITE_TRACE === "true") console.info(`[DEBUG] isolated Vite: ${phase}`);
    };
    const milestones: Plugin = {
      name: "phone-ticket-vite-startup",
      configResolved() { trace("config resolved"); },
      configureServer() { trace("server configured"); },
    };
    const isolatedDevClient: Plugin = {
      name: "phone-ticket-vite-dev-client",
      enforce: "post",
      transform(code, id) {
        if (!id.endsWith("/vite/dist/client/client.mjs")) return;
        const connection = "transport.connect(createHMRHandler(handleMessage));";
        if (!code.includes(connection)) throw Error("The installed Vite development client uses a different HMR startup");
        return code.replace(connection, "");
      },
    };
    const withoutDiscovery = process.env.SEMIO_PHONE_VERIFY_NO_DISCOVERY === "true";
    trace("creating server");
    const server = await createServer({
      ...config,
      configFile: false,
      envFile: false,
      mode,
      cacheDir: join(scratch, `vite-${port}`),
      plugins: [milestones, isolatedVitePackageResolver(scratch), ...(config.plugins ?? []), ...(middleware ? [isolatedDevClient] : [])],
      ...(withoutDiscovery ? { optimizeDeps: { noDiscovery: true, include: [] } } : {}),
      server: {
        ...config.server,
        host: "127.0.0.1",
        port,
        strictPort: true,
        ...(middleware ? { middlewareMode: true, hmr: false, ws: false, watch: null } : {}),
        fs: { ...config.server?.fs, allow: [...new Set([workspace, scratch, ...(config.server?.fs?.allow ?? [])])] },
      },
    });
    if (middleware) {
      const close = server.close.bind(server);
      server.close = async () => {
        try { await close(); } finally { deregister(); }
      };
      trace("middleware ready");
      return server;
    }
    server.httpServer?.once("close", deregister);
    trace("listening");
    await server.listen();
    server.printUrls();
    return server;
  } catch (error) {
    deregister();
    throw error;
  }
}

if (import.meta.main) {
  const [action, file, argument, port] = process.argv.slice(2);
  if (file === undefined || (action !== "run" && action !== "serve" && action !== "browser")) throw new Error("Expected run <script>, serve <config> <port>, or browser <config> <script> <port>");
  const scratch = process.env.SEMIO_PHONE_VERIFY_SCRATCH ?? scratchDefault;
  if (action === "run") await run(scratch, file);
  else if (action === "serve") await serve(scratch, file, Number(argument));
  else {
    if (argument === undefined) throw new Error("The browser action needs a browser script");
    const server = await serve(scratch, file, Number(port), true);
    const { routeViteRequests } = await import("./isolated-vite-route.ts");
    globalThis.routeTicketBrowser = (target, origin) => routeViteRequests(server, target, origin);
    try { await run(scratch, argument); } finally { await server.close(); }
  }
}
