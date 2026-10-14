"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// extension-host-build-7a5OFB/entry.ts
var entry_exports = {};
__export(entry_exports, {
  result: () => result
});
module.exports = __toCommonJS(entry_exports);
var import_node_path = require("node:path");

// extension-host-build-7a5OFB/library.ts
var answer = 42;
if (void 0) {
}

// extension-host-build-7a5OFB/entry.ts
var result = { answer, name: (0, import_node_path.basename)("/fixture/input") };
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  result
});
