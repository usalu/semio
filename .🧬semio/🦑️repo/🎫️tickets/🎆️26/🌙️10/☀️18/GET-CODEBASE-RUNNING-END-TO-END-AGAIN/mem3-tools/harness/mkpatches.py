#!/usr/bin/env python3
"""[mem2] Writes the image patch list for the instrumented harness (not a library file): farmid's two graph-free patches (scratch fact store, inventory references hold) plus phase marks inside the production parent loop emojiProjectJsonNodes; marks are optional calls, so farm workers (unmarked loader) run them as no-ops. usage: mkpatches.py <out.json> [extra.json ...] (extra lists are appended)."""
import json, pathlib, sys

NODES = "📦️project-nodes/🟨️.mjs"
MARKS = [
    ("const catalog=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);", "catalog"),
    ("nativeClosures:new Map()}:await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),texts=new Map();", "discovery"),
    ("const contracts = projectGeneratorContractsV1(workspaceRoot);", "contracts"),
    ('if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","complete");', "command-inputs"),
    ("const declaredRoots = new Set([...rootsByName.values()]);let cargoIndex=0;", "project-configs"),
    ("const projects = results.flatMap(([, result]) => Object.values(result.projects));", "cargo-targets"),
    ('await phases?.begin("inference",jobs.length);', "pre-inference"),
]


def main() -> None:
    here = pathlib.Path(__file__).parent
    patches = json.loads((here / "base-patches.json").read_text(encoding="utf-8"))
    for anchor, label in MARKS:
        patches.append({"suffix": NODES, "find": anchor, "replace": anchor + f"globalThis.__m2mark?.({json.dumps(label)});"})
    osdev = 'json.targets = { ...json.targets, ...await playgroundPreparationTargets(configFiles, workspaceRoot, root, contracts,catalogContributions,context) };'
    patches.append({"suffix": NODES, "find": osdev, "replace": '{globalThis.__m2mark?.("os-dev-playground-start");' + osdev + 'globalThis.__m2mark?.("os-dev-playground");}'})
    first = 'await yieldInferenceV1(context,"build:projects",++projectIndex);'
    patches.append({"suffix": NODES, "find": first, "replace": first + 'if(projectIndex===1)globalThis.__m2mark?.("first-project");'})
    done = "options?.onOwnerObservations?.(name,rows,derived);"
    patches.append({"suffix": NODES, "find": done, "replace": done + 'globalThis.__m2mark?.("done:"+name+":"+rows.length);'})
    for extra in sys.argv[2:]:
        patches.extend(json.loads(pathlib.Path(extra).read_text(encoding="utf-8")))
    pathlib.Path(sys.argv[1]).write_text(json.dumps(patches, ensure_ascii=False, indent=1), encoding="utf-8")


if __name__ == "__main__":
    main()
