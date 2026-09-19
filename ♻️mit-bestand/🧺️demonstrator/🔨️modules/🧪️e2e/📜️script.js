#!/usr/bin/env bun
"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
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
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
var node_path_1 = require("node:path");
var ____ts_1 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83C\uDFC3\uFE0Fprocess/\uD83E\uDDED\uFE0Frouting/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\u26A1\uFE0Fcaching/\uD83C\uDF10\uFE0Fvite/\uD83E\uDDFE\uFE0Fsession/\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\u26A1\uFE0Fcaching/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../\uD83E\uDDE9\uFE0Fruntime/\uD83E\uDDEA\uFE0Fe2e/\uD83D\uDFE6\uFE0F.ts");
/** 🎭️ Runs acceptance tests only against the server generation prepared and owned by Nx. */
var TestScript = /** @class */ (function (_super) {
    __extends(TestScript, _super);
    function TestScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    TestScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var root, sessionRoot, controller, session, interrupt, terminate, baseURL, output, child_1, cancel, status_1;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (args.length)
                            throw new Error("Demonstrator E2E accepts no compiler or server arguments");
                        root = (0, node_path_1.resolve)(this.root, "../.."), sessionRoot = (0, ____ts_4.demonstratorE2eSessionRoot)(this.repoRoot), controller = new AbortController();
                        session = (0, ____ts_2.readServiceSession)(sessionRoot, ____ts_4.DEMONSTRATOR_E2E_OWNER, (0, ____ts_4.demonstratorE2eInvocationPid)(process.env));
                        interrupt = function () { process.exitCode = 130; controller.abort(); }, terminate = function () { process.exitCode = 143; controller.abort(); };
                        process.once("SIGINT", interrupt);
                        process.once("SIGTERM", terminate);
                        _a.label = 1;
                    case 1:
                        _a.trys.push([1, , 7, 8]);
                        console.log("Waiting for Demonstrator E2E service ".concat(session.id));
                        return [4 /*yield*/, (0, ____ts_2.waitForServiceReady)(sessionRoot, session, controller.signal)];
                    case 2:
                        baseURL = _a.sent();
                        output = process.env.SEMIO_TICKET_DIR ? (0, node_path_1.join)(process.env.SEMIO_TICKET_DIR, "🗑️generated/demonstrator-e2e", session.id) : (0, node_path_1.join)(root, "dist/reports/e2e", session.id);
                        console.log("Testing Demonstrator at ".concat(baseURL));
                        child_1 = Bun.spawn([process.execPath, (0, node_path_1.join)(this.repoRoot, "node_modules/playwright/cli.js"), "test", "--config", (0, node_path_1.join)(root, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts"), "--output", output], { cwd: this.repoRoot, env: __assign(__assign({}, process.env), { PLAYWRIGHT_BASE_URL: baseURL, PLAYWRIGHT_BROWSERS_PATH: (0, ____ts_3.repoCacheDirectory)(this.repoRoot, "tools", "ms-playwright") }), stdout: "inherit", stderr: "inherit", stdin: "ignore" });
                        cancel = function () { child_1.kill("SIGTERM"); };
                        controller.signal.addEventListener("abort", cancel, { once: true });
                        if (controller.signal.aborted)
                            cancel();
                        _a.label = 3;
                    case 3:
                        _a.trys.push([3, , 5, 6]);
                        return [4 /*yield*/, child_1.exited];
                    case 4:
                        status_1 = _a.sent();
                        return [3 /*break*/, 6];
                    case 5:
                        controller.signal.removeEventListener("abort", cancel);
                        return [7 /*endfinally*/];
                    case 6:
                        if (status_1 !== 0 && !controller.signal.aborted)
                            throw new Error("Demonstrator acceptance tests failed (".concat(status_1, ")"));
                        return [3 /*break*/, 8];
                    case 7:
                        process.removeListener("SIGINT", interrupt);
                        process.removeListener("SIGTERM", terminate);
                        return [7 /*endfinally*/];
                    case 8: return [2 /*return*/];
                }
            });
        });
    };
    return TestScript;
}(____ts_1.BundleScript));
var router = new ____ts_1.ScriptRouter(import.meta.dir).register("test", TestScript);
if (import.meta.main)
    await router.run(process.argv.slice(2));
