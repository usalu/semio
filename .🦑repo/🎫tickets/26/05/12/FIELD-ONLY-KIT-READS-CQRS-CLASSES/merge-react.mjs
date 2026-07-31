import fs from "node:fs";

const idx = fs.readFileSync("compose/react/index.tsx", "utf8");
const sp = fs.readFileSync("compose/react/sketchpad-runtime.tsx", "utf8");

const spStart = sp.indexOf("const {\n  applyKitClientSnapshotToLocalStore");
if (spStart < 0) throw new Error("sketchpad const block not found");
const spBody = sp.slice(spStart);

const idxOut = idx.replace(/\/\/#region 📎SketchpadRuntime\nexport \* from "\.\/sketchpad-runtime\.js";\n\/\/#endregion 📎SketchpadRuntime\n/, `//#region 🪁SketchpadHost\n${spBody}\n//#endregion 🪁SketchpadHost\n`);

fs.writeFileSync("compose/react/index.tsx", idxOut, "utf8");
console.log("merged react bytes", idxOut.length);
