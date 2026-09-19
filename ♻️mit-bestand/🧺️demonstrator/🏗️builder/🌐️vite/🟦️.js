"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
var node_path_1 = require("node:path");
var node_url_1 = require("node:url");
var vite_1 = require("@tailwindcss/vite");
var plugin_react_1 = require("@vitejs/plugin-react");
var vite_2 = require("vite");
var ____ts_1 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDDB1\uFE0Fui/\uD83C\uDFA8\uFE0Fstyling/\uD83C\uDFD7\uFE0Fbuilder/\uD83C\uDF10\uFE0Fvite/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCE6\uFE0Fsite/\uD83D\uDDFA\uFE0Ftile-serve-mode/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83D\uDCE6\uFE0Fdeployment/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDD1\u200D\uD83D\uDCBBdev/\uD83D\uDD0C\uFE0Fvite-plugins/\uD83D\uDFE6\uFE0F.ts");
var ____ts_5 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83C\uDFEA\uFE0Fstore/\uD83D\uDCE5\uFE0Finstallation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_6 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83C\uDF10\uFE0Fbrowser-bundle/\uD83D\uDCE6\uFE0Fdistribution/\u26A1\uFE0Fvite/\uD83D\uDFE6\uFE0F.ts");
var ____ts_7 = require("../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDCE6\uFE0Fassets/\uD83D\uDFE6\uFE0F.ts");
var ____ts_8 = require("../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDE9\uFE0Fruntime/\u267B\uFE0Factivation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_9 = require("../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDE9\uFE0Fruntime/\u267B\uFE0Factivation/\uD83C\uDF10\uFE0Fvite/\uD83D\uDFE6\uFE0F.ts");
var ____ts_10 = require("../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDFE6\uFE0F.ts");
var ____ts_11 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\u26A1\uFE0Fcaching/\uD83D\uDFE6\uFE0F.ts");
var playDir = node_path_1.default.resolve(node_path_1.default.dirname((0, node_url_1.fileURLToPath)(import.meta.url)), "../..");
/** @emoji 🚫️ Keep wasm-pack engine packages out of Vite's dep optimizer — their `pkg/` entries are produced by `buildEngineWasm`. */
var FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE = [
    "@semio-tech/framework-surface-rs",
    "@semio-tech/framework-editor-rs",
    "@semio-tech/framework-surface-node-graph-rs",
    "@semio-tech/framework-surface-board-2d-rs",
    "@semio-tech/flow-core",
];
var repoRoot = node_path_1.default.resolve(playDir, "../..");
//#region 🔖️DemonstratorUnionAssets
/** @emoji 🎪️ Registry rows for exactly this demonstrator's eight panes — the union this page needs to
 * actually mount, not every playground variant in the monorepo (mirrors `os/dev`'s own `resolvedPlaygroundAssets`,
 * scoped down from its "studio serves everything" fallback since a demonstrator pane list is fixed). */
var resolvedPlaygroundAssets = ____ts_10.DEMONSTRATOR_RUNTIME_TARGETS.flatMap(function (target) { return target.assets; });
/** @emoji 🔌️ Transitive runtime assets for every pane, split by the exact public roots encoded in the generated catalog. */
var _a = (0, ____ts_10.demonstratorRuntimeModuleLayout)(__spreadArray([], new Set(____ts_10.DEMONSTRATOR_RUNTIME_TARGETS.map(function (target) { return target.pluginId; })), true)), pluginModuleDirNames = _a.pluginModuleDirNames, extensionModuleDirNames = _a.extensionModuleDirNames;
//#endregion 🔖️DemonstratorUnionAssets
exports.default = (0, vite_2.defineConfig)(function (_a) {
    var _b, _c;
    var command = _a.command;
    var profile = command === "build" ? "release" : "dev";
    var development = command === "serve" ? (0, ____ts_8.readDemonstratorActivation)(repoRoot) : undefined;
    var pluginModulesDir = node_path_1.default.join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules");
    var installedExtensionsDir = (_b = development === null || development === void 0 ? void 0 : development.extensionsDirectory) !== null && _b !== void 0 ? _b : pluginModulesDir;
    return {
        root: playDir,
        base: "./",
        cacheDir: (0, ____ts_11.repoCacheDirectory)(repoRoot, "vite", "mit-bestand-demonstrator"),
        publicDir: node_path_1.default.join(playDir, "public"),
        assetsInclude: ["**/*.wasm"],
        worker: { format: "es" },
        define: { "import.meta.vitest": "undefined" },
        resolve: {
            alias: __spreadArray(__spreadArray([], (0, ____ts_1.playgroundSceneHostResolveAliases)(repoRoot), true), [
                { find: "@semio-tech/ui-react/test", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts") },
                { find: "@semio-tech/ui-react/runtime", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎠️runtime/🟦️.ts") },
                { find: "@semio-tech/ui-react", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
                { find: "@semio-tech/assets", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
                { find: "@semio-tech/ui-styling", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
                { find: "@semio-tech/infinite-canvas-react-renderer", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx") },
                { find: "@semio-tech/infinite-world-r3f", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx") },
                { find: "@semio-tech/framework-renderer-react", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
                { find: "@semio-tech/framework", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
                { find: "@semio-tech/framework-os", replacement: node_path_1.default.resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") },
                { find: ____ts_3.MODULE_PLUGIN_ROUTE, replacement: pluginModulesDir },
                { find: ____ts_3.MODULE_EXTENSION_ROUTE, replacement: installedExtensionsDir },
            ], false),
            dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
        },
        server: {
            port: Number((_c = process.env.MIT_BESTAND_DEMONSTRATOR_PORT) !== null && _c !== void 0 ? _c : 6029),
            strictPort: true,
            fs: { allow: [repoRoot, pluginModulesDir, installedExtensionsDir] },
            watch: {
                // Generated registry/session rewrites must not bounce Vite.
                ignored: ["**/📇️registry/🤖️generated/**", "**/🤖️generated/**", "**/.vscode/launch.json"],
            },
        },
        plugins: __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray([], (0, ____ts_1.semioHostHtmlVitePlugin)(repoRoot, {
            title: "Entwerfen mit Bestand · Demonstrator",
            entry: "./🟦️.tsx",
            bodyClass: "h-screen w-screen overflow-hidden bg-background text-foreground",
            cnameHost: ____ts_10.DEMONSTRATOR_HOST,
        }), true), [
            (0, ____ts_1.semioEmojiIndexHtmlVitePlugin)(playDir),
            (0, ____ts_1.playgroundFlowWasmDevStubPlugin)(repoRoot),
            (0, ____ts_4.semioBackboneVitePlugin)(),
            (0, ____ts_4.semioBlobVitePlugin)(),
            development && (0, ____ts_9.demonstratorUnionReceiptVitePlugin)({ workspace: repoRoot }),
            development && (0, ____ts_4.semioActivationVitePlugin)({ receiptDirectory: development.receiptDirectory, moduleRoot: pluginModulesDir, installRoot: installedExtensionsDir, components: (0, ____ts_8.demonstratorActivationComponents)(repoRoot) }),
            command === "serve" && (0, ____ts_5.semioExtensionStoreVitePlugin)({ installRoot: installedExtensionsDir, repoRoot: repoRoot })
        ], false), (0, ____ts_1.semioAssetsVitePlugin)(repoRoot), true), (command === "build" ? [(0, ____ts_6.browserArtifactVitePlugin)((0, ____ts_7.demonstratorRuntimeAssetSources)(repoRoot, "release"))] : __spreadArray(__spreadArray(__spreadArray([], pluginModuleDirNames.flatMap(function (name) { return (0, ____ts_1.staticDirVitePlugin)(repoRoot, { kind: "static-dir", route: "".concat(____ts_3.MODULE_PLUGIN_ROUTE, "/").concat(name), root: node_path_1.default.relative(repoRoot, node_path_1.default.join(pluginModulesDir, name)) }); }), true), (0, ____ts_1.staticDirVitePlugin)(repoRoot, { kind: "static-dir", route: "".concat(____ts_3.MODULE_PLUGIN_ROUTE, "/\uD83E\uDE9E\uFE0Fvendor"), root: node_path_1.default.relative(repoRoot, (0, ____ts_7.demonstratorRuntimeAssetSources)(repoRoot, "dev").find(function (row) { return row.owner === "infinite:fonts"; }).root) }), true), extensionModuleDirNames.flatMap(function (name) { return (0, ____ts_1.staticDirVitePlugin)(repoRoot, { kind: "static-dir", route: "".concat(____ts_3.MODULE_EXTENSION_ROUTE, "/").concat(name), root: node_path_1.default.relative(repoRoot, node_path_1.default.join(installedExtensionsDir, name)) }); }), true)), true), [
            (0, ____ts_1.staticDirVitePlugin)(repoRoot, { kind: "static-dir", route: "/".concat(____ts_10.DEMONSTRATOR_ASSETS_DIR), root: ____ts_10.DEMONSTRATOR_ASSETS_DIR })
        ], false), (0, ____ts_1.playgroundAssetVitePlugins)(repoRoot, resolvedPlaygroundAssets, (0, ____ts_2.demonstratorGisMapTileServeMode)(command === "build" ? "build" : "serve")), true), [
            (0, plugin_react_1.default)(),
            (0, vite_1.default)(),
        ], false),
        optimizeDeps: __assign({ entries: [node_path_1.default.join(playDir, "🌐️.html")] }, (0, ____ts_1.playgroundSceneHostOptimizeDeps)({
            exclude: __spreadArray(["playwright", "playwright-core", "chromium-bidi", "fsevents"], FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE, true),
        })),
        build: (0, ____ts_1.semioViteProductionBuild)(),
    };
});
