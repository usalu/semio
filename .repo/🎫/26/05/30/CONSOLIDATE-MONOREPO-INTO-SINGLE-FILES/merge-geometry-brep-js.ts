import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const dir = join(root, "geometry/brep/js");

function stripHeader(src: string): string {
	const lines = src.split(/\r?\n/);
	let i = 0;
	if (lines[i]?.includes("#region 🧲Header")) {
		i++;
		while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
		i++;
	}
	while (i < lines.length && lines[i]?.trim() === "") i++;
	return lines.slice(i).join("\n");
}

function stripContractsImport(src: string): string {
	return src
		.replace(
			/^import type \{ MeshTransfer \} from "\.\/contracts\.ts";\r?\n/m,
			"",
		)
		.replace(
			/^import \{[\s\S]*?\} from "\.\/contracts\.ts";\r?\n/m,
			"",
		);
}

const indexPath = join(dir, "index.ts");
const testsMatch = readFileSync(indexPath, "utf8").match(
	/\/\/ #region 🧪Tests[\s\S]*$/,
);
if (!testsMatch) throw new Error("tests region missing");

const contracts = stripHeader(readFileSync(join(dir, "contracts.ts"), "utf8"));
const kernel = stripContractsImport(stripHeader(readFileSync(join(dir, "kernel.ts"), "utf8")));
const mesh = stripContractsImport(stripHeader(readFileSync(join(dir, "mesh.ts"), "utf8")));

const merged = `// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 \`@semio-tech/geometry-brep-js\` — cad-free brepjs + OpenCascade kernel and contracts. */
// #endregion 🧲Header

// #region 📐Contracts
${contracts}// #endregion 📐Contracts

// #region 🧭Kernel
${kernel}// #endregion 🧭Kernel

// #region 🖼️Mesh
${mesh}// #endregion 🖼️Mesh

${testsMatch[0].replace(
	/const \{ BrepjsGeometryKernel, ensureBrepWasmLoaded \} = await import\("\.\/kernel\.ts"\);\r?\n\tconst \{ isRenderableMeshTransfer \} = await import\("\.\/mesh\.ts"\);/,
	"const { BrepjsGeometryKernel, ensureBrepWasmLoaded, isRenderableMeshTransfer } = await import(\"./index.ts\");",
)}`;

writeFileSync(indexPath, merged);
for (const f of ["contracts.ts", "kernel.ts", "mesh.ts"]) {
	unlinkSync(join(dir, f));
}
console.log("merged geometry/brep/js into index.ts");
