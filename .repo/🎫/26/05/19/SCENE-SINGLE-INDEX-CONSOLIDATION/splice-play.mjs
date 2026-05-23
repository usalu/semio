import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const sceneDir = path.resolve(__dirname, "../../../../../../elements/client/lib/scene");
const mainPath = path.join(sceneDir, "react/index.tsx");
const fragPath = path.join(__dirname, "fragment.txt");

let main = fs.readFileSync(mainPath, "utf8");
const frag = fs.readFileSync(fragPath, "utf8");
const inject = `//#region 🖥️PlayHarness
${frag}

if (!import.meta.vitest && typeof document !== "undefined") {
	const rootEl = document.getElementById("root");
	if (rootEl) {
		createRoot(rootEl).render(<ScenePlayApp />);
	}
}
//#endregion 🖥️PlayHarness
`;
const marker = /\r?\n\/\/#endregion 🎬Scene\r?\n\r?\nif \(import\.meta\.vitest\)/;
if (!marker.test(main)) throw new Error("marker missing");
main = main.replace(
	marker,
	`

//#endregion 🎬Scene

${inject}

if (import.meta.vitest)`,
);
const tailMarker = `\tdescribe("resolveSceneWireKindForVortex", () => {\r\n\t\tit("falls back to default wire id", () => {\r\n\t\t\texpect(resolveSceneWireKindForVortex("any", undefined)).toBe("board.wire.link");\r\n\t\t});\r\n\t});\r\n}`;
const tailAdd = `\tdescribe("resolveSceneWireKindForVortex", () => {\r\n\t\tit("falls back to default wire id", () => {\r\n\t\t\texpect(resolveSceneWireKindForVortex("any", undefined)).toBe("board.wire.link");\r\n\t\t});\r\n\t});\r\n\tdescribe("scene play fixture hook", () => {\r\n\t\tit("parses nakagin fixture", () => {\r\n\t\t\tconst f = parseSceneFixtureV1(sceneFixtureJson as unknown);\r\n\t\t\texpect(f?.ties.length).toBeGreaterThan(0);\r\n\t\t\texpect(f?.objects.length).toBeGreaterThan(0);\r\n\t\t});\r\n\t});\r\n}`;
if (!main.includes(tailMarker)) throw new Error("tail marker missing");
main = main.replace(tailMarker, tailAdd);
fs.writeFileSync(mainPath, main);
console.log("splice ok");
