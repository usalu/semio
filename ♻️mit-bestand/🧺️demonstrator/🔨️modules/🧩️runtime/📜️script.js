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
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var node_url_1 = require("node:url");
var ____ts_1 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83C\uDFC3\uFE0Fprocess/\uD83E\uDDED\uFE0Frouting/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("./\uD83D\uDFE6\uFE0F.ts");
var ____ts_3 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\u26A1\uFE0Fcaching/\uD83C\uDF10\uFE0Fvite/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("./\u267B\uFE0Factivation/\uD83D\uDFE6\uFE0F.ts");
var ____ts_5 = require("../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\u26A1\uFE0Fcaching/\uD83C\uDF10\uFE0Fvite/\uD83E\uDDFE\uFE0Fsession/\uD83D\uDFE6\uFE0F.ts");
var ____ts_6 = require("./\uD83E\uDDEA\uFE0Fe2e/\uD83D\uDFE6\uFE0F.ts");
/** 🎪️ Verifies the runtime union completed by the outer Nx preparation graph. */
var PreparationScript = /** @class */ (function (_super) {
    __extends(PreparationScript, _super);
    function PreparationScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    PreparationScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var profile, plugin, moduleRoot, _a, pluginModuleDirNames, extensionModuleDirNames, _loop_1, _i, _b, name_1, _c, DEMONSTRATOR_RUNTIME_TARGETS_1, target, file, session;
            return __generator(this, function (_d) {
                switch (_d.label) {
                    case 0:
                        profile = args[0];
                        if (args.length !== 1 || !["dev", "release"].includes(profile))
                            throw new Error("prepare <dev|release>");
                        plugin = (0, node_path_1.join)(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
                        moduleRoot = (0, node_path_1.join)(plugin, "📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules");
                        _a = (0, ____ts_2.demonstratorRuntimeModuleLayout)(____ts_2.DEMONSTRATOR_RUNTIME_TARGETS.map(function (row) { return row.pluginId; })), pluginModuleDirNames = _a.pluginModuleDirNames, extensionModuleDirNames = _a.extensionModuleDirNames;
                        _loop_1 = function (name_1) {
                            var directory = (0, node_path_1.join)(moduleRoot, name_1), marker = JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.join)(directory, ".nx-artifact.json"), "utf8"));
                            if (!Array.isArray(marker.files) || !marker.files.includes("🌉️bridge.js") || !marker.files.includes("🔣️.json") || marker.files.some(function (file) { return !(0, node_fs_1.existsSync)((0, node_path_1.join)(directory, file)); }))
                                throw new Error("Incomplete Demonstrator runtime component: ".concat(name_1));
                        };
                        for (_i = 0, _b = __spreadArray(__spreadArray([], pluginModuleDirNames.slice(2), true), extensionModuleDirNames, true); _i < _b.length; _i++) {
                            name_1 = _b[_i];
                            _loop_1(name_1);
                        }
                        _c = 0, DEMONSTRATOR_RUNTIME_TARGETS_1 = ____ts_2.DEMONSTRATOR_RUNTIME_TARGETS;
                        _d.label = 1;
                    case 1:
                        if (!(_c < DEMONSTRATOR_RUNTIME_TARGETS_1.length)) return [3 /*break*/, 4];
                        target = DEMONSTRATOR_RUNTIME_TARGETS_1[_c];
                        file = (0, node_path_1.join)(plugin, "📇️registry/dist/sessions", target.variant, "🎮️playground-session", "🟦️.ts");
                        return [4 /*yield*/, Promise.resolve("".concat((0, node_url_1.pathToFileURL)(file).href)).then(function (s) { return require(s); })];
                    case 2:
                        session = (_d.sent()).PLAYGROUND_SESSION;
                        if (session.variant !== target.variant || session.registryPluginId !== target.pluginId)
                            throw new Error("Mismatched Demonstrator session: ".concat(target.variant));
                        _d.label = 3;
                    case 3:
                        _c++;
                        return [3 /*break*/, 1];
                    case 4:
                        console.log("Prepared Demonstrator ".concat(profile, ": ").concat(____ts_2.DEMONSTRATOR_RUNTIME_TARGETS.length, " runtime variants"));
                        return [2 /*return*/];
                }
            });
        });
    };
    return PreparationScript;
}(____ts_1.BundleScript));
/** 📡️ Validates activation after all outer preparation prerequisites have completed. */
var ActivationScript = /** @class */ (function (_super) {
    __extends(ActivationScript, _super);
    function ActivationScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    ActivationScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var _a, receipt, laneReceiptDirectories;
            return __generator(this, function (_b) {
                if (args.length)
                    throw new Error("Demonstrator activation accepts no arguments");
                _a = (0, ____ts_4.readDemonstratorActivation)(this.repoRoot), receipt = _a.receipt, laneReceiptDirectories = _a.laneReceiptDirectories;
                console.log("Activated Demonstrator dev: ".concat(receipt.plugins.length, " completed components from ").concat(laneReceiptDirectories.length, " lanes"));
                return [2 /*return*/];
            });
        });
    };
    return ActivationScript;
}(____ts_1.BundleScript));
/** 🖥️ Serves completed runtime artifacts for the lifetime owned by Nx. */
var ServeScript = /** @class */ (function (_super) {
    __extends(ServeScript, _super);
    function ServeScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    ServeScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var root, controller, interrupt, terminate;
            var _a;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        if (args.length)
                            throw new Error("Demonstrator serving accepts no arguments");
                        (0, ____ts_4.readDemonstratorActivation)(this.repoRoot);
                        root = (0, node_path_1.resolve)(this.root, "../.."), controller = new AbortController();
                        interrupt = function () { process.exitCode = 130; controller.abort(); }, terminate = function () { process.exitCode = 143; controller.abort(); };
                        process.once("SIGINT", interrupt);
                        process.once("SIGTERM", terminate);
                        _b.label = 1;
                    case 1:
                        _b.trys.push([1, , 3, 4]);
                        return [4 /*yield*/, (0, ____ts_3.serveVite)({ root: root, config: (0, node_path_1.join)(root, "🏗️builder/🌐️vite/🟦️.ts"), host: process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1", port: Number((_a = process.env.MIT_BESTAND_DEMONSTRATOR_PORT) !== null && _a !== void 0 ? _a : 6029), signal: controller.signal, ready: function (url) { return console.log("Demonstrator ready: ".concat(url)); } })];
                    case 2:
                        _b.sent();
                        return [3 /*break*/, 4];
                    case 3:
                        process.removeListener("SIGINT", interrupt);
                        process.removeListener("SIGTERM", terminate);
                        return [7 /*endfinally*/];
                    case 4: return [2 /*return*/];
                }
            });
        });
    };
    return ServeScript;
}(____ts_1.BundleScript));
/** 🆕️ Prepares an invocation-specific E2E generation after its runtime and browser prerequisites. */
var PrepareTestScript = /** @class */ (function (_super) {
    __extends(PrepareTestScript, _super);
    function PrepareTestScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    PrepareTestScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var session;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (args.length)
                            throw new Error("Demonstrator E2E preparation accepts no arguments");
                        (0, ____ts_4.readDemonstratorActivation)(this.repoRoot);
                        return [4 /*yield*/, (0, ____ts_5.openServiceSession)((0, ____ts_6.demonstratorE2eSessionRoot)(this.repoRoot), ____ts_6.DEMONSTRATOR_E2E_OWNER, (0, ____ts_6.demonstratorE2eInvocationPid)(process.env))];
                    case 1:
                        session = _a.sent();
                        console.log("Prepared Demonstrator E2E service ".concat(session.id));
                        return [2 /*return*/];
                }
            });
        });
    };
    return PrepareTestScript;
}(____ts_1.BundleScript));
/** 🧪️ Owns the isolated E2E listener and announces its prepared generation over HTTP. */
var ServeTestScript = /** @class */ (function (_super) {
    __extends(ServeTestScript, _super);
    function ServeTestScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    ServeTestScript.prototype.run = function (args) {
        return __awaiter(this, void 0, void 0, function () {
            var sessionRoot, session, root, controller, interrupt, terminate;
            var _this = this;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (args.length)
                            throw new Error("Demonstrator E2E serving accepts no arguments");
                        sessionRoot = (0, ____ts_6.demonstratorE2eSessionRoot)(this.repoRoot), session = (0, ____ts_5.readServiceSession)(sessionRoot, ____ts_6.DEMONSTRATOR_E2E_OWNER, (0, ____ts_6.demonstratorE2eInvocationPid)(process.env));
                        root = (0, node_path_1.resolve)(this.root, "../.."), controller = new AbortController();
                        interrupt = function () { process.exitCode = 130; controller.abort(); }, terminate = function () { process.exitCode = 143; controller.abort(); };
                        process.once("SIGINT", interrupt);
                        process.once("SIGTERM", terminate);
                        _a.label = 1;
                    case 1:
                        _a.trys.push([1, , 3, 5]);
                        (0, ____ts_4.readDemonstratorActivation)(this.repoRoot);
                        return [4 /*yield*/, (0, ____ts_3.serveVite)({ root: root, config: (0, node_path_1.join)(root, "🏗️builder/🌐️vite/🟦️.ts"), host: "127.0.0.1", port: 0, signal: controller.signal, session: session, ready: function (url) { return __awaiter(_this, void 0, void 0, function () { return __generator(this, function (_a) {
                                    switch (_a.label) {
                                        case 0: return [4 /*yield*/, (0, ____ts_5.publishServiceReady)(sessionRoot, session, url, controller.signal)];
                                        case 1:
                                            _a.sent();
                                            console.log("Demonstrator E2E ready: ".concat(url));
                                            return [2 /*return*/];
                                    }
                                }); }); } })];
                    case 2:
                        _a.sent();
                        return [3 /*break*/, 5];
                    case 3:
                        process.removeListener("SIGINT", interrupt);
                        process.removeListener("SIGTERM", terminate);
                        return [4 /*yield*/, (0, ____ts_5.closeServiceSession)(sessionRoot, session)];
                    case 4:
                        _a.sent();
                        return [7 /*endfinally*/];
                    case 5: return [2 /*return*/];
                }
            });
        });
    };
    return ServeTestScript;
}(____ts_1.BundleScript));
var router = new ____ts_1.ScriptRouter(import.meta.dir).register("prepare", PreparationScript).register("activate", ActivationScript).register("serve", ServeScript).register("prepare-test", PrepareTestScript).register("serve-test", ServeTestScript);
if (import.meta.main)
    await router.run(process.argv.slice(2));
