import sys, os, shutil
L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
IMP = "🏗️build/🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs"
PARSERS_OLD = '''import { createRequire } from "node:module";

const own = createRequire(import.meta.url);

/** 📦️ Loads the TypeScript compiler API from the parser package: the node resolution starts in the parser package directory and, when its install is absent, continues in the checkout root install. */
export const parserTypescriptV1 = () => own("typescript");
'''
PARSERS_NEW = '''import { typescriptCompilerV1 } from "../🔷️typescript-compiler/🟨️.mjs";

/** 📦️ Loads the TypeScript compiler API of the tooling through its one interface: the exact version the parser package declares, from the install slot of the parser package or the direct dependency of the checkout root, never by the bare name; a wrong or incomplete install is the typed refusal `TypescriptCompilerRefusalV1`, retry never.
 * @see ../🔷️typescript-compiler/🟨️.mjs */
export const parserTypescriptV1 = () => typescriptCompilerV1();
'''
SHIM = lambda rel, names: ('import type * as TsTypes from "typescript";\nimport { typescriptCompilerV1 } from "%s";\n' % (rel + IMP))
EDITS = [
 ("🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs", PARSERS_OLD, PARSERS_NEW),
 ("🕸️dependencies/🟦️typescript/🟨️.mjs", 'import { observedFs,', 'import { typescriptCompilerFileV1, typescriptCompilerV1 } from "../../' + IMP + '";\nimport { observedFs,'),
 ("🕸️dependencies/🟦️typescript/🟨️.mjs", 'readRegistrationBytes(require.resolve("typescript"))', 'readRegistrationBytes(typescriptCompilerFileV1())'),
 ("🕸️dependencies/🟦️typescript/🟨️.mjs", 'const compiler = require("typescript"), text =', 'const compiler = typescriptCompilerV1(), text ='),
 ("🕸️dependencies/🟦️typescript/⚙️configured/🟦️.ts", 'compiler=require("typescript"),facts=', 'compiler=require("../../../' + IMP + '").typescriptCompilerV1(),facts='),
 ("🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts", 'import type ts from "typescript";\n', 'import type ts from "typescript";\nimport { typescriptCompilerFileV1, typescriptCompilerV1 } from "../../../' + IMP + '";\n'),
 ("🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts", 'const typescript: typeof ts = (await import("typescript")).default;', 'const typescript = typescriptCompilerV1() as unknown as typeof ts;'),
 ("🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts", 'createRequire(import.meta.url).resolve("typescript");', 'typescriptCompilerFileV1();'),
 ("⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🧪️diff-only-law/🟦️.ts", 'import { createRequire } from "node:module";\n', 'import { createRequire } from "node:module";\nimport { typescriptCompilerV1 } from "../../../../' + IMP + '";\n'),
 ("⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🧪️diff-only-law/🟦️.ts", 'createRequire(import.meta.url)("typescript") as typeof import("typescript")', 'typescriptCompilerV1() as unknown as typeof import("typescript")'),
 ("⚖️laws/🔬️interactivity-audit/🧭️all-app/🟦️.ts", 'import { dirname } from "node:path";\n', 'import { dirname } from "node:path";\nimport { typescriptCompilerV1 } from "../../../' + IMP + '";\n'),
 ("⚖️laws/🔬️interactivity-audit/🧭️all-app/🟦️.ts", 'const typescript = await import("typescript");', 'const typescript = typescriptCompilerV1();'),
 ("⚖️laws/🧫️test-store-isolation/🟦️.ts", 'import ts from "typescript";\n', 'import type * as TsTypes from "typescript";\nimport {typescriptCompilerV1} from "../../' + IMP + '";\n'),
 ("⚖️laws/🧫️test-store-isolation/🟦️.ts", 'import rulesSchema from "./🧬️schema/🔣️.json" with {type:"json"};\n',
  'import rulesSchema from "./🧬️schema/🔣️.json" with {type:"json"};\nconst ts=typescriptCompilerV1() as unknown as typeof TsTypes;\ndeclare namespace ts{type CallExpression=TsTypes.CallExpression;type Expression=TsTypes.Expression;type Identifier=TsTypes.Identifier;type Node=TsTypes.Node;type ScriptKind=TsTypes.ScriptKind;type ScriptTarget=TsTypes.ScriptTarget;type SyntaxKind=TsTypes.SyntaxKind;type VariableDeclaration=TsTypes.VariableDeclaration;type VariableDeclarationList=TsTypes.VariableDeclarationList}\n'),
 ("🧹️normalization/🧪️support/🏗️source-services/🟦️.ts", 'import ts from "typescript";\n', 'import { typescriptCompilerV1 } from "../../../' + IMP + '";\n\nconst ts = typescriptCompilerV1() as unknown as typeof import("typescript");\n'),
]
def main(mode, base, backup):
    files = sorted({e[0] for e in EDITS})
    if mode == "apply":
        os.makedirs(backup, exist_ok=True)
        texts = {}
        for f in files:
            p = os.path.join(base, f); texts[f] = open(p, encoding="utf8").read()
        for f, old, new in EDITS:
            if texts[f].count(old) != 1: print("ANCHOR", f, texts[f].count(old), old[:60]); return 3
            texts[f] = texts[f].replace(old, new)
        for f in files:
            p = os.path.join(base, f); b = os.path.join(backup, f)
            os.makedirs(os.path.dirname(b), exist_ok=True); shutil.copy2(p, b)
        for f in files:
            open(os.path.join(base, f), "w", encoding="utf8").write(texts[f])
        print("applied", len(EDITS), "edits in", len(files), "files"); return 0
    if mode == "restore":
        texts = {f: open(os.path.join(base, f), encoding="utf8").read() for f in files}
        for f, old, new in reversed(EDITS):
            if texts[f].count(new) != 1: print("INVERSE", f, texts[f].count(new)); return 4
            texts[f] = texts[f].replace(new, old)
        for f in files: open(os.path.join(base, f), "w", encoding="utf8").write(texts[f])
        print("restored", len(files), "files"); return 0
sys.exit(main(sys.argv[1], sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else ""))
