"""🧾 [audit7] Anchored edits of batch `contracts`: a graph farm worker infers with the generator contracts its parent keyed the job with (carried in the session), never with a later read of the taxonomy. usage: edit_core.py <root>  (exit 1 = an anchor is not found exactly once)"""
import os, sys
root = sys.argv[1]
I = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/"
F = I + "🏊️project-farm/"
EDITS = {
 I + "🟨️.mjs": [
  ("/** 🧱️ Builds, from plain data, what the inference of one project shares across the projects of a graph construction; a worker process opens the session its parent describes. */\nexport function createProjectInferenceEnvironmentV1(session) {\n  const { workspaceRoot, registry, projectsByRoot, componentRoots } = session,",
   "/** 📜️ The generator contracts of the workspace taxonomy, read through the observed source analysis in the scope of the caller: a graph construction reads them once and keys every project record with their digest. */\nexport function projectGeneratorContractsV1(workspaceRoot) {\n  const contractPath = join(workspaceRoot, \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json\");\n  return existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};\n}\n\n/** 🧱️ Builds, from plain data, what the inference of one project shares across the projects of a graph construction; a worker process opens the session its parent describes, the generator contracts included: they are no fact of a project record but part of its key, so a worker that read them again after a peer edit would write content of other contracts under the key of its parent. */\nexport function createProjectInferenceEnvironmentV1(session) {\n  const { workspaceRoot, registry, projectsByRoot, componentRoots, contracts } = session,"),
  ("  const contractPath = join(workspaceRoot, \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json\");\n  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};\n  scripts.generatedOutput = path => readBuildGeneratedCompileOutputV1(path, workspaceRoot, contracts, registry);",
   "  if (contracts === null || typeof contracts !== \"object\" || Array.isArray(contracts)) throw Error(\"Generator contracts of the parent inference required\");\n  scripts.generatedOutput = path => readBuildGeneratedCompileOutputV1(path, workspaceRoot, contracts, registry);"),
  ("createProjectInferenceEnvironmentV1({ workspaceRoot, registry, projectsByRoot: [], componentRoots: [], commandInputs: null, lateClosures: false })",
   "createProjectInferenceEnvironmentV1({ workspaceRoot, registry, contracts: projectGeneratorContractsV1(workspaceRoot), projectsByRoot: [], componentRoots: [], commandInputs: null, lateClosures: false })"),
  ("  const contractPath = join(workspaceRoot, \"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json\");\n  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};\n  scripts.generatedOutput=path=>",
   "  const contracts = projectGeneratorContractsV1(workspaceRoot);\n  scripts.generatedOutput=path=>"),
  ("openProjectFarmV1({workspaceRoot,context,projects:jobs.length,", "openProjectFarmV1({workspaceRoot,context,contracts,projects:jobs.length,"),
 ],
 F + "🟨️.mjs": [
  ("export function openProjectFarmV1({ workspaceRoot, context, projects,", "export function openProjectFarmV1({ workspaceRoot, context, contracts, projects,"),
  ("session: { version: 2, workspaceRoot, registry: context.registry ?? null,", "session: { version: 3, workspaceRoot, registry: context.registry ?? null, contracts,"),
  (" * In a process that runs from a captured snapshot", " * The session hands every worker what the parent keyed its projects with (the registry, the command inputs and the generator contracts it read once), so a peer edit between the parent's read and a worker's start can never put content of other contracts under the parent's key.\n * In a process that runs from a captured snapshot"),
 ],
 F + "🟨️.d.mts": [
  ("openProjectFarmV1(options:{workspaceRoot:string;context:{readonly registry?:unknown};", "openProjectFarmV1(options:{workspaceRoot:string;context:{readonly registry?:unknown};contracts:Readonly<Record<string,unknown>>;"),
 ],
 F + "🧪️tests/🟦️.ts": [
  ("openProjectFarmV1({workspaceRoot:root,context:{registry:null},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment,", "openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment,"),
  ("openProjectFarmV1({workspaceRoot:root,context:{registry:null},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:\"en_US.UTF-8\"},", "openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:\"en_US.UTF-8\"},"),
 ],
 F + "🧪️tests/🔒️farm-close/🟦️.ts": [
  ("openProjectFarmV1({workspaceRoot:root,context:{registry:null},launch:null,", "openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,"),
 ],
 F + "🧪️tests/🪆️captured/🟦️.ts": [
  ("openProjectFarmV1({workspaceRoot:area(),context:{registry:null},projects:566,", "openProjectFarmV1({workspaceRoot:area(),context:{registry:null},contracts:{},projects:566,"),
  ("session={version:2,workspaceRoot:base,registry:null,commandInputs:null,", "session={version:3,workspaceRoot:base,registry:null,contracts:{},commandInputs:null,"),
 ],
}
def main():
    bad = 0
    for rel, edits in EDITS.items():
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        for old, new in edits:
            n = text.count(old)
            if n != 1:
                if text.count(new) == 1: continue
                print(f"[audit7] anchor found {n} times in {rel}: {old[:90]!r}"); bad = 1; continue
            text = text.replace(old, new)
        mode = os.stat(path).st_mode & 0o7777
        with open(path, "w", encoding="utf8") as handle: handle.write(text)
        os.chmod(path, mode)
    sys.exit(bad)
main()
