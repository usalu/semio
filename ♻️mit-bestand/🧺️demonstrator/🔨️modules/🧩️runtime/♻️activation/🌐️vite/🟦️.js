"use strict";
// #region 🧲️Header
/** @emoji ♻️ Keeps the Demonstrator's merged activation receipt current while its dev server runs.
 * Node-only on purpose: it is imported by `🏗️builder/🌐️vite/🟦️.ts`, never by the browser-shared
 * `🔨️modules/🧩️runtime/🟦️.ts`. */
// #endregion 🧲️Header
Object.defineProperty(exports, "__esModule", { value: true });
exports.demonstratorUnionReceiptVitePlugin = demonstratorUnionReceiptVitePlugin;
var ____ts_1 = require("../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83D\uDCBB\uFE0Fos/\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDD1\u200D\uD83D\uDCBBdev/\u267B\uFE0Factivation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../\uD83D\uDFE6\uFE0F.ts");
/** @emoji 👀️ Republishes the union receipt whenever ANY lane completes a new activation.
 *
 * `semioActivationVitePlugin` watches exactly one receipt directory, and no framework lane can ever
 * produce the Demonstrator's cross-app union — so this plugin owns the fan-in: it subscribes to every
 * lane and folds them back into the one directory that plugin is pointed at. It must therefore be
 * registered BEFORE `semioActivationVitePlugin`, so the union is fresh before that plugin takes its
 * first snapshot.
 *
 * A failed merge is logged, never thrown: a lane receipt is rewritten atomically but a lane can be
 * mid-rebuild and legitimately disagree with its peers for a moment, and killing the dev server over a
 * transient disagreement would cost a full cold plugin boot. */
function demonstratorUnionReceiptVitePlugin(options) {
    var observers = [];
    var report = function (error) { return console.error("[demonstrator] union activation receipt refused: ".concat(error instanceof Error ? error.message : String(error))); };
    var dispose = function () { while (observers.length > 0) {
        try {
            observers.pop().close();
        }
        catch ( /* a watcher already torn down by its own error */_a) { /* a watcher already torn down by its own error */ }
    } };
    return {
        name: "demonstrator-union-activation-receipt",
        apply: "serve",
        configureServer: function (server) {
            var _a;
            var republish = function () { try {
                (0, ____ts_2.publishDemonstratorUnionReceipt)(options.workspace);
            }
            catch (error) {
                report(error);
            } };
            for (var _i = 0, _b = (0, ____ts_2.demonstratorActivationLaneReceiptDirectories)(options.workspace); _i < _b.length; _i++) {
                var directory = _b[_i];
                try {
                    observers.push((0, ____ts_1.observeActivationReceipts)(directory, republish, report));
                }
                catch (error) {
                    report(error);
                }
            }
            (_a = server.httpServer) === null || _a === void 0 ? void 0 : _a.once("close", dispose);
        },
        closeBundle: dispose,
    };
}
