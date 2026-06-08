import { readFileSync, writeFileSync, unlinkSync, rmSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const dir = join(root, "repo/client/vscode");
const codegenDir = join(dir, "codegen");

function stripHeader(src: string): string {
	const lines = src.split(/\r?\n/);
	let i = 0;
	if (lines[i]?.includes("#region") && lines[i]?.includes("Header")) {
		i++;
		while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
		i++;
	}
	while (i < lines.length && lines[i]?.trim() === "") i++;
	const out: string[] = [];
	for (; i < lines.length; i++) {
		const line = lines[i]!;
		if (/^import \* as types from ['"]\.\/graphql['"];?\s*$/.test(line.trim())) continue;
		if (/^import \{ Incremental \} from ['"]\.\/graphql['"];?\s*$/.test(line.trim())) continue;
		out.push(line);
	}
	return out.join("\n");
}

const extensionPath = join(dir, "extension.ts");
let extension = readFileSync(extensionPath, "utf8");

extension = extension.replace(
	/^\/\/ #region ⌛Queries\r?\nimport \{ graphql \} from "\.\/codegen\/gql";\r?\n/m,
	"// #region ⌛Queries\n",
);

const graphql = stripHeader(readFileSync(join(codegenDir, "graphql.ts"), "utf8"));
const gql = stripHeader(readFileSync(join(codegenDir, "gql.ts"), "utf8"));

const block = `// #region 🧬CodegenGraphql
${graphql}
// #endregion 🧬CodegenGraphql

// #region 🧬CodegenGql
${gql}
// #endregion 🧬CodegenGql

`;

extension = extension.replace(
	/^(\/\/ #endregion 🔌Adapters\r?\n)/m,
	`$1\n${block}`,
);

writeFileSync(extensionPath, extension);
rmSync(codegenDir, { recursive: true, force: true });
console.log("merged vscode codegen into extension.ts");
