"use strict";
// #region 🧲️Header
// 💻️ ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts
// Specs: Run Playwright acceptance coverage against a live "Entwerfen mit Bestand" demonstrator dev server.
// Summary: Nx prepares an isolated continuous service; the E2E consumer validates its generation
// before supplying PLAYWRIGHT_BASE_URL. Playwright consumes that server and does not start one.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header
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
// #region 🔌️Adapters
var node_path_1 = require("node:path");
var node_url_1 = require("node:url");
var test_1 = require("@playwright/test");
var repo_lib_1 = require("@semio-tech/repo-lib");
// #endregion 🔌️Adapters
var demonstratorDir = (0, node_path_1.resolve)((0, node_path_1.dirname)((0, node_url_1.fileURLToPath)(import.meta.url)), "../../..");
var playwrightTimeoutMs = (0, repo_lib_1.playwrightTestTimeoutMs)();
function withTrailingSlash(url) {
    return url.endsWith("/") ? url : "".concat(url, "/");
}
if (!process.env.PLAYWRIGHT_BASE_URL)
    throw new Error("Run the Demonstrator test-e2e target through Nx");
var baseURL = withTrailingSlash(process.env.PLAYWRIGHT_BASE_URL);
/** 🖥️ Every one of the demonstrator's eight panes drives a GPU surface, and one of them only boots on a real adapter.
 *
 * `--use-angle=swiftshader` is enough for the r3f/WebGL World3d panes (aussuchen's grid paints its beams
 * under it), but the wasm/wgpu `TiledMapHost` never finishes `attachCanvas` on it: measured 2026-09-16
 * against the same serve, verfolgen's map requested ZERO tiles under swiftshader and 65 (`/osm/0/0/0.png`,
 * `/vt/3/0/1.pbf`, …) under ANGLE-Metal, where it paints continents, labels and the marker. So the
 * default is the machine's real adapter; `DEMONSTRATOR_E2E_GPU=swiftshader` forces the software stack
 * back for a host that has none, knowing the map pane cannot be graded there. */
function browserLaunchArgs() {
    if (process.env.DEMONSTRATOR_E2E_GPU === "swiftshader")
        return ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"];
    return [process.platform === "darwin" ? "--use-angle=metal" : "--use-angle=gl", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"];
}
exports.default = (0, test_1.defineConfig)({
    testDir: (0, node_path_1.resolve)(demonstratorDir, "🧪️tests"),
    testMatch: ["🎭️acceptance/🟦️.ts"],
    fullyParallel: false,
    forbidOnly: !!process.env.CI,
    retries: process.env.CI ? 2 : 0,
    timeout: playwrightTimeoutMs,
    expect: { timeout: Math.min(playwrightTimeoutMs, 120000) },
    workers: 1,
    reporter: [["list"]],
    use: {
        baseURL: baseURL,
        trace: "on-first-retry",
    },
    projects: [
        {
            name: "chromium",
            use: __assign(__assign({}, test_1.devices["Desktop Chrome"]), { launchOptions: {
                    args: __spreadArray([], browserLaunchArgs(), true),
                } }),
        },
    ],
});
