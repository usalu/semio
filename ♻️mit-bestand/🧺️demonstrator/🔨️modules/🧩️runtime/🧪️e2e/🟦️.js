"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DEMONSTRATOR_E2E_OWNER = void 0;
exports.demonstratorE2eSessionRoot = demonstratorE2eSessionRoot;
exports.demonstratorE2eInvocationPid = demonstratorE2eInvocationPid;
var node_path_1 = require("node:path");
exports.DEMONSTRATOR_E2E_OWNER = "@semio-tech/mit-bestand-demonstrator:e2e";
/** 🗂️ Locates ephemeral E2E service records outside cacheable application outputs. */
function demonstratorE2eSessionRoot(workspace) {
    return (0, node_path_1.join)(workspace, "♻️mit-bestand/🧺️demonstrator/dist/services/e2e");
}
/** 🧭️ Requires Nx's shared invocation identity before preparing a fresh service generation. */
function demonstratorE2eInvocationPid(environment) {
    var value = environment.NX_INVOCATION_ROOT_PID;
    if (!value || !/^[1-9][0-9]*$/.test(value) || !Number.isSafeInteger(Number(value)))
        throw new Error("Demonstrator E2E must run through Nx");
    return Number(value);
}
