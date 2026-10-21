import { pathToFileURL, fileURLToPath } from "node:url";
import type { RuntimeBundleBuilderV1, RuntimeBundlePluginV1 } from "../../📋️contract/🟦️.ts";

type ResolveRowV1 = { filter: RegExp; namespace: string | undefined; callback: (arguments_: { path: string; importer: string; namespace: string }) => any };
type LoadRowV1 = { filter: RegExp; namespace: string | undefined; callback: (arguments_: { path: string; namespace: string; loader: string }) => any };
type ModuleHooksV1 = { registerHooks(hooks: { resolve?: (specifier: string, context: { parentURL?: string }, next: (specifier: string, context?: unknown) => any) => any; load?: (url: string, context: { format?: string }, next: (url: string, context?: unknown) => any) => any }): unknown };

const OBJECTS = "__semioRuntimePluginObjects";

function namespaceOf(url: string | undefined): string {
  if (url === undefined) return "file";
  const scheme = /^([a-z][a-z0-9+.-]*):/iu.exec(url)?.[1] ?? "file";
  return scheme === "file" ? "file" : scheme;
}

function pathOf(url: string | undefined): string {
  if (url === undefined) return "";
  return url.startsWith("file:") ? fileURLToPath(url) : url.replace(/^[a-z][a-z0-9+.-]*:/iu, "");
}

function objectModuleV1(exports: Record<string, unknown>): string {
  const registry = ((globalThis as Record<string, unknown>)[OBJECTS] ??= new Map<string, Record<string, unknown>>()) as Map<string, Record<string, unknown>>;
  const id = `${registry.size}:${Math.random().toString(36).slice(2)}`;
  registry.set(id, exports);
  const names = Object.keys(exports).filter(name => /^[A-Za-z_$][\w$]*$/u.test(name) && name !== "default");
  return `const o=globalThis[${JSON.stringify(OBJECTS)}].get(${JSON.stringify(id)});export default o.default??o;${names.map(name => `export const ${name}=o[${JSON.stringify(name)}];`).join("")}`;
}

/** 🔌️ Registers a plugin as a synchronous module-resolution and loading hook of this process; callbacks keep the bundler-plugin shape and virtual namespaces become URL schemes. Like the Bun runtime plugin, resolution applies to relative and absolute specifiers only. */
export function registerLoaderPluginV1(plugin: RuntimeBundlePluginV1): void {
  const resolvers: ResolveRowV1[] = [], loaders: LoadRowV1[] = [];
  const builder: RuntimeBundleBuilderV1 = {
    onResolve: (constraints, callback) => { resolvers.push({ filter: constraints.filter, namespace: constraints.namespace, callback: callback as ResolveRowV1["callback"] }); },
    onLoad: (constraints, callback) => { loaders.push({ filter: constraints.filter, namespace: constraints.namespace, callback: callback as LoadRowV1["callback"] }); },
  };
  const outcome = plugin.setup(builder);
  if (outcome !== undefined) throw new Error("A loader plugin must set up synchronously");
  const module = process.getBuiltinModule("node:module") as unknown as ModuleHooksV1;
  module.registerHooks({
    resolve(specifier, context, next) {
      if (!/^(?:\.{1,2}[\\/]|[\\/]|[A-Za-z]:[\\/])/u.test(specifier)) return next(specifier, context);
      const importerNamespace = namespaceOf(context.parentURL);
      for (const row of resolvers) {
        if (row.namespace !== undefined && row.namespace !== importerNamespace) continue;
        row.filter.lastIndex = 0;
        if (!row.filter.test(specifier)) continue;
        const outcome = row.callback({ path: specifier, importer: pathOf(context.parentURL), namespace: importerNamespace });
        if (outcome === undefined || outcome === null) continue;
        if (outcome.external === true) return next(outcome.path ?? specifier, context);
        const namespace = outcome.namespace ?? "file";
        return { url: namespace === "file" ? pathToFileURL(outcome.path).href : `${namespace}:${outcome.path}`, shortCircuit: true };
      }
      return next(specifier, context);
    },
    load(url, context, next) {
      const namespace = namespaceOf(url), path = pathOf(url);
      for (const row of loaders) {
        if (row.namespace !== undefined && row.namespace !== namespace) continue;
        row.filter.lastIndex = 0;
        if (!row.filter.test(path)) continue;
        const outcome = row.callback({ path, namespace, loader: "js" });
        if (outcome === undefined || outcome === null) continue;
        if (outcome.exports !== undefined) return { format: "module", source: objectModuleV1(outcome.exports), shortCircuit: true };
        const source = typeof outcome.contents === "string" ? outcome.contents : Buffer.from(outcome.contents).toString("utf8");
        return { format: "module", source, shortCircuit: true };
      }
      return next(url, context);
    },
  });
}
