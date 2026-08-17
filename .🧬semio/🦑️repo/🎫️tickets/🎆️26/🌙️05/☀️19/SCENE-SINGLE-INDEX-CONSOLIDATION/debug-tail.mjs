import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sceneDir = path.resolve(__dirname, "../../../../../../elements/client/lib/scene");
const mainPath = path.join(sceneDir, "react/index.tsx");
const main = fs.readFileSync(mainPath, "utf8");
const i = main.lastIndexOf("resolveSceneWireKindForVortex");
console.log(JSON.stringify(main.slice(i - 30)));
