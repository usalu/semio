"use strict";
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
exports.DEMONSTRATOR_RUNTIME_TARGETS = exports.DEMONSTRATOR_RUNTIME_PANES = exports.DEMONSTRATOR_ASSETS_DIR = exports.DEMONSTRATOR_HOST = void 0;
exports.demonstratorPaneRuntimeVariant = demonstratorPaneRuntimeVariant;
exports.demonstratorRuntimePluginId = demonstratorRuntimePluginId;
exports.demonstratorRuntimeBuildVariants = demonstratorRuntimeBuildVariants;
exports.demonstratorRuntimeModuleLayout = demonstratorRuntimeModuleLayout;
var ____json_1 = require("./\uD83D\uDD23\uFE0F.json");
var ____mjs_1 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDD78\uFE0Fdependencies/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDFE8\uFE0F.mjs");
var ____ts_1 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83E\uDD16\uFE0Fgenerated/\uD83C\uDFAE\uFE0Fplaygrounds/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83E\uDD16\uFE0Fgenerated/\uD83E\uDDE9\uFE0Fplugins/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83D\uDCE6\uFE0Fdeployment/\uD83D\uDFE6\uFE0F.ts");
exports.DEMONSTRATOR_HOST = ____json_1.default.host;
exports.DEMONSTRATOR_ASSETS_DIR = ____json_1.default.assetsDirectory;
exports.DEMONSTRATOR_RUNTIME_PANES = ____json_1.default.panes;
/** 🎛️ Selects a pane's authored runtime variant. */
function demonstratorPaneRuntimeVariant(variant) {
    var pane = ____json_1.default.panes.find(function (row) { return row.variant === variant; });
    if (!pane)
        throw new Error("Unknown demonstrator pane variant: ".concat(variant));
    return pane.runtimeVariant;
}
/** 🪪️ Resolves a runtime variant to its generated component identity. */
function demonstratorRuntimePluginId(variant) {
    return runtimePluginId(variant);
}
/** 🪪️ Resolves a runtime variant to its generated component identity. */
function runtimePluginId(variant) {
    var target = ____ts_1.PLAYGROUND_BUILD_TARGETS.find(function (row) { return row.variant === variant; });
    if (!target)
        throw new Error("Unknown demonstrator runtime variant: ".concat(variant));
    return target.pluginId;
}
/** 🧮️ Selects one representative variant per additional runtime component. */
function demonstratorRuntimeBuildVariants(primaryVariant) {
    var selected = new Set([runtimePluginId(primaryVariant)]), result = [];
    for (var _i = 0, _a = ____json_1.default.panes; _i < _a.length; _i++) {
        var pane = _a[_i];
        var id = runtimePluginId(pane.runtimeVariant);
        if (!selected.has(id)) {
            selected.add(id);
            result.push(pane.runtimeVariant);
        }
    }
    return result;
}
/** 🛣️ Maps the complete runtime closure to its authored public deployment directories. */
function demonstratorRuntimeModuleLayout(rootPluginIds) {
    var components = __spreadArray(__spreadArray([], ____ts_2.PLUGIN_BUILD_TARGETS, true), ____ts_2.EXTENSION_TARGETS, true), byId = new Map(components.map(function (row) { return [row.pluginId, row]; }));
    var ids = (0, ____mjs_1.runtimeComponentClosure)(components, rootPluginIds);
    return {
        pluginModuleDirNames: __spreadArray([____ts_3.MODULE_VENDOR_DIRECTORY, ____ts_3.MODULE_SHARD_DIRECTORY], ids.filter(function (id) { return byId.get(id).role === "plugin"; }).map(____ts_3.moduleDirectoryName), true),
        extensionModuleDirNames: ids.filter(function (id) { return byId.get(id).role === "extension"; }).map(____ts_3.moduleDirectoryName),
    };
}
var variants = new Set(____json_1.default.panes.flatMap(function (row) { return [row.variant, row.runtimeVariant]; }));
exports.DEMONSTRATOR_RUNTIME_TARGETS = ____ts_1.PLAYGROUND_BUILD_TARGETS.filter(function (row) { return variants.has(row.variant); });
for (var _i = 0, variants_1 = variants; _i < variants_1.length; _i++) {
    var variant = variants_1[_i];
    runtimePluginId(variant);
}
