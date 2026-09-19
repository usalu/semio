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
Object.defineProperty(exports, "__esModule", { value: true });
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <test> [args…]`. */
var node_path_1 = require("node:path");
var ____ts_1 = require("../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDCE6\uFE0Fpackages/\uD83D\uDFE6\uFE0Ftypescript/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("./\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDE9\uFE0Fruntime/\uD83D\uDFE6\uFE0F.ts");
var TestScript = /** @class */ (function (_super) {
    __extends(TestScript, _super);
    function TestScript() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    TestScript.prototype.run = function (segments) {
        return __awaiter(this, void 0, void 0, function () {
            var rest;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        rest = (0, ____ts_1.resolveTestLevel)(segments).rest;
                        return [4 /*yield*/, (0, ____ts_1.runVitest)(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts")];
                    case 1:
                        _a.sent();
                        return [2 /*return*/];
                }
            });
        });
    };
    return TestScript;
}(____ts_1.BundleScript));
var router = new ____ts_1.ScriptRouter(import.meta.dir).register("test", TestScript);
if (import.meta.main)
    await (0, ____ts_1.runBundleScriptMain)(router, import.meta.url);
if (import.meta.vitest) {
    var registerTests1 = (await Promise.resolve().then(function () { return require("./🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts"); })).registerTests1;
    await registerTests1(import.meta.vitest, { demonstratorRuntimeBuildVariants: ____ts_2.demonstratorRuntimeBuildVariants, join: node_path_1.join }, { directory: import.meta.dir, url: import.meta.url });
    var _a = await Promise.resolve().then(function () { return require("./🪧️brand.ts"); }), DEMONSTRATOR_PANES = _a.DEMONSTRATOR_PANES, demonstratorPaneDescriptionParagraphs = _a.demonstratorPaneDescriptionParagraphs;
    var registerDescriptionTests = (await Promise.resolve().then(function () { return require("./🧪️tests/🧪️demonstratorpanedescription/🟦️.ts"); })).registerTests1;
    await registerDescriptionTests(import.meta.vitest, { demonstratorPaneDescriptionParagraphs: demonstratorPaneDescriptionParagraphs, DEMONSTRATOR_PANES: DEMONSTRATOR_PANES }, { directory: import.meta.dir, url: import.meta.url });
    var _b = await Promise.resolve().then(function () { return require("./🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts"); }), demonstratorGisMapTileServeMode = _b.demonstratorGisMapTileServeMode, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER = _b.DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR = _b.DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR;
    var registerMapTileTests = (await Promise.resolve().then(function () { return require("./🧪️tests/🧪️demonstratormaptiles/🟦️.ts"); })).registerTests1;
    await registerMapTileTests(import.meta.vitest, { demonstratorGisMapTileServeMode: demonstratorGisMapTileServeMode, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER: DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_RASTER, DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR: DEMONSTRATOR_STATIC_MAP_TILE_Z_MAX_VECTOR });
    var registerCompileClosureTests = (await Promise.resolve().then(function () { return require("./🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts"); })).registerTests1;
    await registerCompileClosureTests(import.meta.vitest);
}
