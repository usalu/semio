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
exports.demonstratorRuntimeComponentIds = demonstratorRuntimeComponentIds;
exports.demonstratorRuntimeAssetSources = demonstratorRuntimeAssetSources;
var node_path_1 = require("node:path");
var ____ts_1 = require("../\uD83D\uDFE6\uFE0F.ts");
var ____mjs_1 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDD78\uFE0Fdependencies/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDFE8\uFE0F.mjs");
var ____ts_2 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83E\uDD16\uFE0Fgenerated/\uD83E\uDDE9\uFE0Fplugins/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83D\uDCE6\uFE0Fdeployment/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83C\uDF10\uFE0Fbrowser-bundle/\uD83D\uDD78\uFE0Fimports/\uD83D\uDFE6\uFE0F.ts");
/** 🧩️ Resolves the complete component union shared by distribution and development activation. */
function demonstratorRuntimeComponentIds() {
    return (0, ____mjs_1.runtimeComponentClosure)(__spreadArray(__spreadArray([], ____ts_2.PLUGIN_BUILD_TARGETS, true), ____ts_2.EXTENSION_TARGETS, true), ____ts_1.DEMONSTRATOR_RUNTIME_TARGETS.map(function (row) { return row.pluginId; }));
}
/** 🎪️ Selects only immutable component, browser-support and font outputs for the Demonstrator. */
function demonstratorRuntimeAssetSources(workspace, profile) {
    if (!["dev", "release"].includes(profile))
        throw new Error("Unknown Demonstrator profile: ".concat(profile));
    var moduleRoot = (0, node_path_1.join)(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules");
    var components = __spreadArray(__spreadArray([], ____ts_2.PLUGIN_BUILD_TARGETS, true), ____ts_2.EXTENSION_TARGETS, true), catalog = new Map(components.map(function (row) { return [row.pluginId, row]; }));
    var ids = demonstratorRuntimeComponentIds(), pluginRoute = ____ts_3.MODULE_PLUGIN_ROUTE.slice(1);
    return __spreadArray(__spreadArray([], ids.map(function (id) {
        var row = catalog.get(id), name = (0, ____ts_3.moduleDirectoryName)(id), extension = row.role === "extension";
        return __assign({ root: (0, node_path_1.join)(moduleRoot, name), destination: "".concat((extension ? ____ts_3.MODULE_EXTENSION_ROUTE : ____ts_3.MODULE_PLUGIN_ROUTE).slice(1), "/").concat(name), owner: "".concat(row.cratePath, "/Cargo.toml:browser:").concat(profile) }, (extension ? { shimDirectory: "".concat(pluginRoute, "/").concat(____ts_4.PREVIEW2_VENDOR_RELATIVE) } : {}));
    }), true), [
        { root: (0, node_path_1.join)(moduleRoot, ____ts_4.PREVIEW2_VENDOR_RELATIVE), destination: "".concat(pluginRoute, "/").concat(____ts_4.PREVIEW2_VENDOR_RELATIVE), owner: "browser-support:".concat(profile, ":preview2") },
        { root: (0, node_path_1.join)(moduleRoot, ____ts_3.MODULE_SHARD_DIRECTORY), destination: "".concat(pluginRoute, "/").concat(____ts_3.MODULE_SHARD_DIRECTORY), owner: "browser-support:".concat(profile, ":shard") },
        { root: (0, node_path_1.join)(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts"), destination: "".concat(pluginRoute, "/").concat(____ts_3.MODULE_VENDOR_DIRECTORY), owner: "infinite:fonts" },
    ], false);
}
