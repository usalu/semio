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
exports.DEMONSTRATOR_UNION_RECEIPT_DIRECTORY = exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE = void 0;
exports.demonstratorActivationLanes = demonstratorActivationLanes;
exports.demonstratorActivationLaneReceiptDirectory = demonstratorActivationLaneReceiptDirectory;
exports.demonstratorActivationLaneReceiptDirectories = demonstratorActivationLaneReceiptDirectories;
exports.mergeDemonstratorActivationReceipts = mergeDemonstratorActivationReceipts;
exports.publishDemonstratorUnionReceipt = publishDemonstratorUnionReceipt;
exports.readDemonstratorActivation = readDemonstratorActivation;
exports.demonstratorActivationComponents = demonstratorActivationComponents;
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var ____ts_1 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDD1\u200D\uD83D\uDCBBdev/\u267B\uFE0Factivation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../\uD83D\uDCE6\uFE0Fassets/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../\uD83D\uDFE6\uFE0F.ts");
var ____mjs_1 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDD78\uFE0Fdependencies/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDFE8\uFE0F.mjs");
var ____ts_4 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83E\uDD16\uFE0Fgenerated/\uD83E\uDDE9\uFE0Fplugins/\uD83D\uDFE6\uFE0F.ts");
var ____ts_5 = require("../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDCC7\uFE0Fregistry/\uD83D\uDCE6\uFE0Fdeployment/\uD83D\uDFE6\uFE0F.ts");
//#region 🔖️DemonstratorActivationLanes
/** @emoji 🛣️ The pane whose own activation lane carries the Demonstrator's OWN component closure — the
 * `demonstrator` plugin and everything it depends on. Every other lane exists only to add a component
 * that closure does not already reach. */
exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE = "generator";
/** @emoji 📦️ Repository-relative root of the `@semio-tech/framework-os-dev` package every lane's
 * `activate-<lane>-react-dev` target writes its receipt below. */
var OS_DEV_PACKAGE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
/** @emoji 🧾️ The DEMONSTRATOR-OWNED directory holding the merged receipt this product's Vite server
 * consumes. It is deliberately not one of the framework's per-variant lane directories: no single
 * `activate-<variant>-react-dev` target can ever produce the Demonstrator's cross-app union, because
 * each of them publishes exactly one playground session's plugin list. */
exports.DEMONSTRATOR_UNION_RECEIPT_DIRECTORY = "♻️mit-bestand/🧺️demonstrator/dist/♻️activation/dev";
/** @emoji 🛣️ The activation lanes whose receipts together cover the Demonstrator's runtime union.
 *
 * The primary lane already carries its own component closure, so a pane's runtime variant earns a lane
 * of its own only when its plugin is NOT inside an already-covered closure. That is what keeps
 * `generation3d` out: `procedural` is reached through `demonstrator`, so the `generation3d` lane would
 * contribute no component while adding a second, independently cached receipt for eleven shared plugins
 * — and two lanes disagreeing about one plugin's `artifactSha256` is precisely the staleness this merge
 * refuses (the two receipts on disk disagree about all eleven today). `energy` and `fem` are genuinely
 * unreachable from `demonstrator`, so those two lanes are required. */
function demonstratorActivationLanes() {
    var components = __spreadArray(__spreadArray([], ____ts_4.PLUGIN_BUILD_TARGETS, true), ____ts_4.EXTENSION_TARGETS, true);
    var covered = new Set((0, ____mjs_1.runtimeComponentClosure)(components, [(0, ____ts_3.demonstratorRuntimePluginId)(exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE)]));
    var lanes = [exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE];
    for (var _i = 0, _a = (0, ____ts_3.demonstratorRuntimeBuildVariants)(exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE); _i < _a.length; _i++) {
        var variant = _a[_i];
        var pluginId = (0, ____ts_3.demonstratorRuntimePluginId)(variant);
        if (covered.has(pluginId))
            continue;
        for (var _b = 0, _c = (0, ____mjs_1.runtimeComponentClosure)(components, [pluginId]); _b < _c.length; _b++) {
            var id = _c[_b];
            covered.add(id);
        }
        lanes.push(variant);
    }
    return lanes;
}
/** 🗂️ One lane's framework-owned receipt directory. */
function demonstratorActivationLaneReceiptDirectory(workspace, lane) {
    return (0, node_path_1.join)((0, ____ts_1.developmentRuntimeRoot)((0, node_path_1.join)(workspace, OS_DEV_PACKAGE_ROOT), lane, "dev", "react"), "activation");
}
/** 🗂️ Every lane's receipt directory, in lane order — what the union watcher subscribes to. */
function demonstratorActivationLaneReceiptDirectories(workspace) {
    return demonstratorActivationLanes().map(function (lane) { return demonstratorActivationLaneReceiptDirectory(workspace, lane); });
}
/** @emoji 🧾️ Merges every lane's completion receipt into the ONE receipt describing the Demonstrator's
 * exact runtime union — pure, so the union rule is testable without a staged workspace.
 *
 * Every lane stages into the SAME `🔌️plugin-modules` root, so one plugin's `artifactSha256` cannot
 * legitimately differ between two lanes: a disagreement means one lane's `activate-…` ran against
 * different bytes and its receipt is stale. Overlapping rows otherwise keep the EARLIEST `rebuiltAt`,
 * because the hot-swap watcher treats a bumped timestamp as "this component changed" and a lane that
 * merely re-activated unchanged bytes must not fake a change. */
function mergeDemonstratorActivationReceipts(lanes, expectedPluginIds, variant) {
    if (variant === void 0) { variant = exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE; }
    var rows = new Map();
    for (var _i = 0, lanes_1 = lanes; _i < lanes_1.length; _i++) {
        var _a = lanes_1[_i], lane = _a.lane, receipt = _a.receipt;
        (0, ____ts_1.parseActivationReceipt)(receipt);
        if (receipt.profile !== "dev")
            throw new Error("Demonstrator activation lane ".concat(lane, " is not a dev activation: ").concat(receipt.profile));
        if (receipt.variant !== lane)
            throw new Error("Demonstrator activation lane ".concat(lane, " carries a ").concat(receipt.variant, " receipt"));
        for (var _b = 0, _c = receipt.plugins; _b < _c.length; _b++) {
            var row = _c[_b];
            var previous = rows.get(row.pluginId);
            if (!previous) {
                rows.set(row.pluginId, { lane: lane, row: row });
                continue;
            }
            if (previous.row.artifactSha256 !== row.artifactSha256)
                throw new Error("Stale Demonstrator activation lane: ".concat(row.pluginId, " is ").concat(previous.row.artifactSha256, " in ").concat(previous.lane, " but ").concat(row.artifactSha256, " in ").concat(lane, " (run bun nx run @semio-tech/framework-os-dev:activate-").concat(lane, "-react-dev)"));
            if (row.rebuiltAt < previous.row.rebuiltAt)
                rows.set(row.pluginId, { lane: lane, row: row });
        }
    }
    var expected = __spreadArray([], expectedPluginIds, true).sort(), present = __spreadArray([], rows.keys(), true).sort();
    var missing = expected.filter(function (id) { return !rows.has(id); }), extra = present.filter(function (id) { return !expected.includes(id); });
    if (missing.length > 0 || extra.length > 0)
        throw new Error("Demonstrator activation does not contain its exact runtime union \u2014 missing: ".concat(missing.join(", ") || "(none)", "; extra: ").concat(extra.join(", ") || "(none)"));
    return (0, ____ts_1.parseActivationReceipt)({ schema: "semio.dev.activation/v1", variant: variant, profile: "dev", plugins: present.map(function (id) { return (__assign({}, rows.get(id).row)); }) });
}
/** 📖️ Reads one lane's receipt, naming the exact target that produces it when it is absent. */
function readDemonstratorActivationLane(workspace, lane) {
    try {
        return { lane: lane, receipt: (0, ____ts_1.readActivationReceipt)(demonstratorActivationLaneReceiptDirectory(workspace, lane)) };
    }
    catch (error) {
        if ((error === null || error === void 0 ? void 0 : error.code) !== "ENOENT")
            throw error;
        throw new Error("Missing Demonstrator activation lane receipt: ".concat(lane, " (run bun nx run @semio-tech/framework-os-dev:activate-").concat(lane, "-react-dev)"));
    }
}
/** 📬️ Publishes the merged union receipt into the Demonstrator's own `dist` and returns it. */
function publishDemonstratorUnionReceipt(workspace) {
    var lanes = demonstratorActivationLanes();
    var receipt = mergeDemonstratorActivationReceipts(lanes.map(function (lane) { return readDemonstratorActivationLane(workspace, lane); }), (0, ____ts_2.demonstratorRuntimeComponentIds)());
    var receiptDirectory = (0, node_path_1.join)(workspace, exports.DEMONSTRATOR_UNION_RECEIPT_DIRECTORY);
    (0, node_fs_1.mkdirSync)(receiptDirectory, { recursive: true });
    (0, ____ts_1.publishActivationReceipt)(receiptDirectory, receipt);
    return { receiptDirectory: receiptDirectory, receipt: receipt, laneReceiptDirectories: lanes.map(function (lane) { return demonstratorActivationLaneReceiptDirectory(workspace, lane); }) };
}
/** 🧾️ Requires the exact completed Demonstrator component union before starting its development host. */
function readDemonstratorActivation(workspace) {
    var _a = publishDemonstratorUnionReceipt(workspace), receiptDirectory = _a.receiptDirectory, receipt = _a.receipt, laneReceiptDirectories = _a.laneReceiptDirectories;
    var primary = (0, ____ts_1.developmentRuntimeRoot)((0, node_path_1.join)(workspace, OS_DEV_PACKAGE_ROOT), exports.DEMONSTRATOR_PRIMARY_ACTIVATION_LANE, "dev", "react");
    return { receiptDirectory: receiptDirectory, extensionsDirectory: (0, node_path_1.join)(primary, "extensions"), receipt: receipt, laneReceiptDirectories: laneReceiptDirectories };
}
//#endregion 🔖️DemonstratorUnionReceipt
/** 🔎️ Describes every closure component for the activation-receipt freshness watcher, keyed by the owner root descriptors are staged from. */
function demonstratorActivationComponents(workspace) {
    var byId = new Map(__spreadArray(__spreadArray([], ____ts_4.PLUGIN_BUILD_TARGETS, true), ____ts_4.EXTENSION_TARGETS, true).map(function (row) { return [row.pluginId, row]; }));
    return (0, ____ts_2.demonstratorRuntimeComponentIds)().map(function (id) {
        var row = byId.get(id);
        return { pluginId: id, directoryName: (0, ____ts_5.moduleDirectoryName)(id), role: row.role === "extension" ? "extension" : "plugin", sourceRoot: (0, node_path_1.resolve)(workspace, row.cratePath, "..", "..") };
    });
}
if (import.meta.vitest) {
    var registerTests1 = (await Promise.resolve().then(function () { return require("../../../🧪️tests/🧪️demonstratorunionreceipt/🟦️.ts"); })).registerTests1;
    await registerTests1(import.meta.vitest, { demonstratorActivationLanes: demonstratorActivationLanes, demonstratorRuntimeComponentIds: ____ts_2.demonstratorRuntimeComponentIds, mergeDemonstratorActivationReceipts: mergeDemonstratorActivationReceipts }, { directory: import.meta.dirname, url: import.meta.url });
}
