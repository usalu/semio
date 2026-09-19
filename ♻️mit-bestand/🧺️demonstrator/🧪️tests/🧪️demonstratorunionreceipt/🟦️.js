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
function registerTests1(vitest, dependencies, _source) {
    return __awaiter(this, void 0, void 0, function () {
        var demonstratorActivationLanes, demonstratorRuntimeComponentIds, mergeDemonstratorActivationReceipts, describe, expect, it, sha, lane;
        return __generator(this, function (_a) {
            demonstratorActivationLanes = dependencies.demonstratorActivationLanes, demonstratorRuntimeComponentIds = dependencies.demonstratorRuntimeComponentIds, mergeDemonstratorActivationReceipts = dependencies.mergeDemonstratorActivationReceipts;
            describe = vitest.describe, expect = vitest.expect, it = vitest.it;
            sha = function (seed) { return seed.repeat(64).slice(0, 64); };
            lane = function (name, plugins) { return ({
                lane: name,
                receipt: { schema: "semio.dev.activation/v1", variant: name, profile: "dev", plugins: plugins.map(function (_a) {
                        var pluginId = _a[0], seed = _a[1], rebuiltAt = _a[2];
                        return ({ pluginId: pluginId, artifactSha256: sha(seed), rebuiltAt: rebuiltAt });
                    }) },
            }); };
            describe("demonstratorActivationLanes", function () {
                it("adds a lane only for a pane plugin the primary closure cannot reach", function () {
                    expect(demonstratorActivationLanes()).toEqual(["generator", "energy", "fem3d"]);
                });
                it("covers every component of the runtime union across its lanes", function () {
                    var ids = demonstratorRuntimeComponentIds();
                    expect(ids).toContain("energy");
                    expect(ids).toContain("fem");
                    expect(new Set(ids).size).toBe(ids.length);
                });
            });
            describe("mergeDemonstratorActivationReceipts", function () {
                it("unions every lane's rows into one sorted generator receipt", function () {
                    var merged = mergeDemonstratorActivationReceipts([lane("generator", [["procedural", "b", 7], ["demonstrator", "c", 7]]), lane("energy", [["energy", "d", 9]]), lane("fem3d", [["fem", "e", 9]])], ["demonstrator", "energy", "fem", "procedural"]);
                    expect(merged.schema).toBe("semio.dev.activation/v1");
                    expect(merged.variant).toBe("generator");
                    expect(merged.profile).toBe("dev");
                    expect(merged.plugins.map(function (row) { return row.pluginId; })).toEqual(["demonstrator", "energy", "fem", "procedural"]);
                });
                it("keeps the earliest rebuiltAt when two lanes agree about one component", function () {
                    var merged = mergeDemonstratorActivationReceipts([lane("generator", [["procedural", "b", 12]]), lane("energy", [["procedural", "b", 3], ["energy", "d", 4]])], ["energy", "procedural"]);
                    expect(merged.plugins.find(function (row) { return row.pluginId === "procedural"; }).rebuiltAt).toBe(3);
                });
                it("refuses a stale lane that disagrees about one component's artifact", function () {
                    expect(function () { return mergeDemonstratorActivationReceipts([lane("generator", [["procedural", "b", 7]]), lane("energy", [["procedural", "f", 7], ["energy", "d", 7]])], ["energy", "procedural"]); }).toThrow(/Stale Demonstrator activation lane: procedural/);
                });
                it("refuses a union missing one required component", function () {
                    expect(function () { return mergeDemonstratorActivationReceipts([lane("generator", [["demonstrator", "c", 7]])], ["demonstrator", "energy"]); })
                        .toThrow(/does not contain its exact runtime union — missing: energy; extra: \(none\)/);
                });
                it("refuses a union carrying a component the demonstrator never asked for", function () {
                    expect(function () { return mergeDemonstratorActivationReceipts([lane("generator", [["demonstrator", "c", 7], ["puzzle", "a", 7]])], ["demonstrator"]); })
                        .toThrow(/does not contain its exact runtime union — missing: \(none\); extra: puzzle/);
                });
                it("refuses a lane directory holding another variant's receipt", function () {
                    expect(function () { return mergeDemonstratorActivationReceipts([__assign(__assign({}, lane("energy", [["energy", "d", 7]])), { lane: "fem3d" })], ["energy"]); })
                        .toThrow(/Demonstrator activation lane fem3d carries a energy receipt/);
                });
                it("refuses a release receipt in a development lane", function () {
                    var release = lane("energy", [["energy", "d", 7]]);
                    expect(function () { return mergeDemonstratorActivationReceipts([__assign(__assign({}, release), { receipt: __assign(__assign({}, release.receipt), { profile: "release" }) })], ["energy"]); })
                        .toThrow(/Demonstrator activation lane energy is not a dev activation: release/);
                });
            });
            return [2 /*return*/];
        });
    });
}
