"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var node_url_1 = require("node:url");
var node_path_1 = require("node:path");
//#region 🔌️Adapters
var config_1 = require("vitest/config");
var testRoot = (0, node_path_1.resolve)((0, node_path_1.dirname)((0, node_url_1.fileURLToPath)(import.meta.url)), "../..");
//#endregion 🔌️Adapters
/** 🧪️ Tests for the demonstrator task router. */
exports.default = (0, config_1.defineConfig)({
    root: testRoot,
    test: {
        root: testRoot,
        name: "@semio-tech/mit-bestand-demonstrator",
        environment: "node",
        include: [],
        includeSource: [
            "./📜️script.ts",
            "./🪧️brand.ts",
            "./🔨️modules/🧩️runtime/♻️activation/🟦️.ts",
            "./🧪️tests/🧪️demonstratorpanebranding/🟦️.ts",
        ],
        passWithNoTests: false,
    },
});
