/** 🌐️ Loads the hub admin Vite owner through vite's own native loader, the way
 * `🎚️tool-configuration-ownership`'s "loads all five Vite owners" case does, and prints its effective
 * root, base and plugin names. The suite itself cannot reach this owner today: it iterates the five
 * owners in fixture order and dies on `demonstrator-vite`, whose config imports a module that does not
 * exist in the tree. */
import { resolve } from "node:path";
import { loadConfigFromFile } from "vite";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const ownerPath = "🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts";
const effectiveRoot = "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript";

process.env.SEMIO_PLUGIN = "s";
process.env.SEMIO_RENDERER = "react";
const loaded = await loadConfigFromFile({ command: "serve", mode: "development", isSsrBuild: false, isPreview: false }, resolve(repoRoot, ownerPath), undefined, "silent", undefined, "native");
if (!loaded) throw new Error("hub admin vite owner did not load");
const config = loaded.config as { root?: string; base?: string; plugins?: unknown[] };
const actualRoot = resolve(config.root ?? repoRoot);
if (actualRoot !== resolve(repoRoot, effectiveRoot)) throw new Error(`hub admin vite effective root drifted: ${actualRoot}`);
const names = (config.plugins ?? []).flat(Infinity).filter(Boolean).map((plugin) => (plugin as { name?: string }).name ?? "<anonymous>");
console.log(`hub-admin-vite: root=${effectiveRoot} base=${config.base} plugins=${names.length}`);
console.log(`hub-admin-vite plugins: ${names.join(", ")}`);
