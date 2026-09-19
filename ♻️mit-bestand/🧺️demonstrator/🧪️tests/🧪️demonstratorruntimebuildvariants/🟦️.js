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
exports.registerTests1 = registerTests1;
function registerTests1(vitest, dependencies, source) {
    return __awaiter(this, void 0, void 0, function () {
        var demonstratorRuntimeBuildVariants, join, describe, expect, it;
        var _this = this;
        return __generator(this, function (_a) {
            demonstratorRuntimeBuildVariants = dependencies.demonstratorRuntimeBuildVariants, join = dependencies.join;
            describe = vitest.describe, expect = vitest.expect, it = vitest.it;
            //#region 🧪️DemonstratorPluginBuildTests
            describe("demonstratorRuntimeBuildVariants", function () {
                it("builds three additional artifacts for eight pane runtime variants", function () {
                    expect(demonstratorRuntimeBuildVariants("generator")).toEqual(["generation3d", "energy", "fem3d"]);
                });
                it("validates the authored runtime catalog against its owner schema module", function () { return __awaiter(_this, void 0, void 0, function () {
                    var readFileSync, createRequire, dirname, fileURLToPath, modulePath, schema, ajv, validate, validatePipeline;
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                            case 1:
                                readFileSync = (_a.sent()).readFileSync;
                                return [4 /*yield*/, Promise.resolve().then(function () { return require("node:module"); })];
                            case 2:
                                createRequire = (_a.sent()).createRequire;
                                return [4 /*yield*/, Promise.resolve().then(function () { return require("node:path"); })];
                            case 3:
                                dirname = (_a.sent()).dirname;
                                return [4 /*yield*/, Promise.resolve().then(function () { return require("node:url"); })];
                            case 4:
                                fileURLToPath = (_a.sent()).fileURLToPath;
                                modulePath = join(dirname(fileURLToPath(source.url)), "🔨️modules/🧩️runtime");
                                schema = JSON.parse(readFileSync(join(modulePath, "🧬️schema/🔣️.json"), "utf8"));
                                expect(schema.$schema).toBe("http://json-schema.org/draft-07/schema#");
                                expect(schema.$id).toBe("https://json.schemas.assets.semio-tech.com/mit-bestand/demonstrator/runtime/schema.json");
                                ajv = new (createRequire(source.url)("ajv").default)({ strict: false });
                                validate = ajv.compile(schema);
                                expect(validate(JSON.parse(readFileSync(join(modulePath, "🔣️.json"), "utf8")))).toBe(true);
                                expect(validate({ schemaVersion: 1, host: "", assetsDirectory: "a", panes: [] })).toBe(false);
                                validatePipeline = ajv.compile(__assign(__assign({}, schema), { $id: "".concat(schema.$id, "#pipeline"), $ref: "#/$defs/DemonstratorPipelineContract" }));
                                expect(validatePipeline(JSON.parse(readFileSync(join(modulePath, "🧫️pipeline.json"), "utf8")))).toBe(true);
                                expect(validatePipeline({ schemaVersion: 2 })).toBe(false);
                                return [2 /*return*/];
                        }
                    });
                }); });
            });
            return [2 /*return*/];
        });
    });
}
