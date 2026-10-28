"""♨ Anchored, idempotent edit of the script-inputs component for batch `warmderive`: edit_si.py <root> check|apply. The TypeScript program question takes the installed reuse store (parsed files by text, the previous program as `oldProgram` while armed)."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
path = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs")
EDITS = [
 ('import { fileURLToPath, pathToFileURL } from "node:url";\n',
  'import { fileURLToPath, pathToFileURL } from "node:url";\nimport { activeTypeScriptProgramReuseV1 } from "./♨️reuse/🟨️.mjs";\n'),
 ("  return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute });",
  "  return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute, reuse: activeTypeScriptProgramReuseV1() });"),
 ("  const { compiler, host, readdir, join, dirname, isAbsolute } = environment;\n  const configs = [...new Set(inputs.flatMap(input => DECLARED_TSCONFIG_V1.test(input)",
  "  const { compiler, host, readdir, join, dirname, isAbsolute, reuse } = environment;\n  const configs = [...new Set(inputs.flatMap(input => DECLARED_TSCONFIG_V1.test(input)"),
 ("    const program = compiler.createProgram({ rootNames: parsed.fileNames, options, projectReferences: parsed.projectReferences, host: compilerHost });\n",
  "    if (reuse) compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile);\n    const oldProgram = reuse?.oldProgram(config);\n    const program = compiler.createProgram({ rootNames: parsed.fileNames, options, projectReferences: parsed.projectReferences, host: compilerHost, ...(oldProgram ? { oldProgram } : {}) });\n    reuse?.keep(config, program);\n"),
 ("/** 🪁️ Runs the compiler's own program over the TypeScript projects a target declares, so the typecheck closure is exactly what `tsc -p` reads:",
  "/** 🪁️ Runs the compiler's own program over the TypeScript projects a target declares (with `environment.reuse`, a long-lived deriver's store of parsed files and previous programs, see `./♨️reuse/🟨️.mjs`), so the typecheck closure is exactly what `tsc -p` reads:"),
]
text = open(path, encoding="utf8").read(); new = text; bad = 0
for old, rep in EDITS:
    if rep and new.count(rep) == 1: pass
    elif new.count(old) == 1: new = new.replace(old, rep)
    else: print("ANCHOR", repr(old[:70])); bad += 1
print("same" if new == text else "edit", "script-inputs")
if verb == "apply" and not bad and new != text:
    mode = os.stat(path).st_mode & 0o777; tmp = path + ".e30"; open(tmp, "w", encoding="utf8").write(new); os.chmod(tmp, mode); os.replace(tmp, path)
sys.exit(1 if bad else 0)
