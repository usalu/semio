"""🗃 Anchored, idempotent edits of batch `testmemo` (lane rustedit): repository cargo TESTS (print-rs:test and every `runRepositoryCargoTests` caller) admit their sources through the same foreign-digest book and per-unit admission memo as `bun repo cargo`; rustfast wired them only into the launcher.
usage: python3 -I edit.py <root> check|apply   (exit 2 = an anchor is missing)"""
import os, sys
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/"
LAUNCH, ADMIT, TEST, ALAW = L + "🚪️launcher/🟦️.ts", L + "🎟️admission/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts", L + "🎟️admission/🧪️tests/🟦️.ts"
FUNCS = ('/** 🗂️ The planning port of a repository compile: foreign (registry) crate sources are inventoried through the digest book kept under the store\'s unit cache, so an unchanged crate directory is stamped, not re-hashed. Shared by `bun repo cargo` and repository tests. */\n'
 'export function repositoryCargoPlanningPortV1(store:Readonly<{root:string}>){return actualCargoCompilerPlanningPortV1({digests:foreignDigestMemoV1(unitCacheRootV1(store.root))});}\n'
 '/** 📖️ The source admission memo of a repository compile, keyed by the store\'s toolchain digest and kept under its unit cache, so the second admission of an unchanged unit reads no source byte. Shared by `bun repo cargo` and repository tests. */\n'
 'export function repositoryCargoSourceMemoV1(store:Readonly<{root:string;toolchain:Readonly<{digest:string}>}>){return unitAdmissionMemoV1(unitCacheRootV1(store.root),store.toolchain.digest);}\n\n')
E = {
 LAUNCH: [
  ('/** 🧵️ One Cargo command\'s lane (see {@link RepositoryCargoAgentLaneV1}).', FUNCS + '/** 🧵️ One Cargo command\'s lane (see {@link RepositoryCargoAgentLaneV1}).'),
  ('control,actualCargoCompilerPlanningPortV1({digests:foreignDigestMemoV1(unitCacheRootV1(store.root))}))', 'control,repositoryCargoPlanningPortV1(store))'),
  (',unitAdmissionMemoV1(unitCacheRootV1(store.root),store.toolchain.digest))', ',repositoryCargoSourceMemoV1(store))'),
 ],
 ADMIT: [
  ('import {launchRepositoryCargoV1,readRepositoryCargoLaunchV1} from "../🚪️launcher/🟦️.ts";', 'import {launchRepositoryCargoV1,readRepositoryCargoLaunchV1,repositoryCargoPlanningPortV1,repositoryCargoSourceMemoV1} from "../🚪️launcher/🟦️.ts";'),
  ('workspaceRoot:workspace,control:actualPhase,storage:store.storage,sourceBoundary:store.sourceBoundary,', 'workspaceRoot:workspace,control:actualPhase,storage:store.storage,sourceBoundary:store.sourceBoundary,planningPort:repositoryCargoPlanningPortV1(store),sourceMemo:repositoryCargoSourceMemoV1(store),'),
 ],
 ALAW: [("repositoryCargoNetworkV1:()=>({offline:true}),", "repositoryCargoNetworkV1:()=>({offline:true}),repositoryCargoPlanningPortV1:()=>undefined,repositoryCargoSourceMemoV1:()=>undefined,")],
 TEST: [
  ('import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1,type CargoSourceBoundaryPolicyV1,type CargoSourceAdmissionPortV1} from', 'import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1,type CargoSourceBoundaryPolicyV1,type CargoSourceAdmissionPortV1,type CargoSourceAdmissionMemoV1} from'),
  ('readonly sourcePort?:CargoSourceAdmissionPortV1;readonly withCargoInputs?:CargoCompilerInputsPortV1}', 'readonly sourcePort?:CargoSourceAdmissionPortV1;readonly sourceMemo?:CargoSourceAdmissionMemoV1;readonly withCargoInputs?:CargoCompilerInputsPortV1}'),
  ('admission.sourcePort??actualCargoSourceAdmissionPortV1(env,request.cwd,storage))', 'admission.sourcePort??actualCargoSourceAdmissionPortV1(env,request.cwd,storage),admission.sourceMemo)'),
 ],
}
root, verb = sys.argv[1], sys.argv[2]
texts, bad = {}, False
for rel, edits in E.items():
    text = open(os.path.join(root, rel), encoding="utf8").read()
    for old, new in edits:
        if text.count(new) == 1 and text.count(old) == (1 if old in new else 0): continue
        if text.count(old) != 1: print("ANCHOR", text.count(old), rel[-40:], old[:70]); bad = True; continue
        text = text.replace(old, new)
    texts[rel] = text
if bad: sys.exit(2)
if verb == "apply":
    for rel, text in texts.items(): open(os.path.join(root, rel), "w", encoding="utf8").write(text)
print(verb, "ok")
