"""🧷️ AV2 14b: resolves the 14 `git merge-file` conflicts left after re-basing the AV2 overlay edits (snapshot
`s14b-av2-snapshot`) onto the tree the Codex peer edited overnight (icon-render-export wire twin, document-transfer task lane,
plugin rustfmt). Every conflict is a union of both sides; this script states each one explicitly and refuses anything else.

Usage: python3 av2-rebase-resolve.py [--root <overlay>]
"""
import os, re, sys

ROOT = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay"
ELEMENTS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
CONFLICT = re.compile(r"<<<<<<< av2\n(.*?)=======\n(.*?)>>>>>>> live\n", re.S)


def import_union(av2: str, live: str) -> str:
    head, rest = live.split("{", 1)
    names, tail = rest.split("}", 1)
    wanted = [n.strip() for n in names.split(",") if n.strip()]
    key = lambda n: (n.startswith("type "), n.removeprefix("type ").split(" as ")[0].lower())
    for name in (n.strip() for n in av2.split("{", 1)[1].split("}", 1)[0].split(",")):
        if name and name not in wanted:
            at = next((i for i, existing in enumerate(wanted) if key(existing) > key(name)), len(wanted))
            wanted.insert(at, name)
    return f"{head}{{ {', '.join(wanted)} }}{tail}"


def plugin_downloaded(av2: str, live: str) -> str:
    return live.replace("| semio_framework::kernel::Effect::IconRenderExport { .. }));", "| semio_framework::kernel::Effect::IconRenderExport { .. } | semio_framework::kernel::Effect::VideoRenderExport { .. }));")


def live_then_av2(av2: str, live: str) -> str:
    return live + av2


def av2_fn_then_live(av2: str, live: str) -> str:
    return av2 + "}\n\n" + live


def lane_type(av2: str, live: str) -> str:
    return (" * is executing, a program's tool run (Fill, Generate, Reconstruct…), a document archive the shell is importing, or a\n"
            " * host media export (a rendered video). */\n"
            'export type TaskManagerTaskLaneV1 = "job" | "activation" | "toolCall" | "toolRun" | "documentTransfer" | "export";\n')


def shellhost_imports(av2: str, live: str) -> str:
    first_av2, second_av2 = av2.split("\n", 1)
    return import_union(first_av2 + "\n", live) + second_av2


def shellhost_cache_ref(av2: str, live: str) -> str:
    return live.replace("readonly shell: readonly TaskManagerTaskV1[]; readonly transfers", "readonly exports: readonly VideoRenderJobRow[]; readonly shell: readonly TaskManagerTaskV1[]; readonly transfers", 1)


def shellhost_tasks(av2: str, live: str) -> str:
    return ("      if (cached && cached.jobs === jobs && cached.exports === exports && cached.shell === shell && cached.transfers === transfers) return cached.tasks;\n"
            "      const tasks = [...spawnedJobTasksV1(jobs), ...videoRenderExportTasksV1(exports, uiLocaleRef.current), ...shell, ...transfers];\n"
            "      taskManagerTasksCacheRef.current = { jobs, exports, shell, transfers, tasks };\n")


PLAN = {
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs": [plugin_downloaded],
    "🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts": [lambda a, l: l + "}\n\n" + a, live_then_av2],
    f"{ELEMENTS}/🔌️PluginRuntime/🟦️.tsx": [lambda a, l: import_union(a, l), live_then_av2],
    f"{ELEMENTS}/🧵️TaskManager/🟦️.tsx": [live_then_av2, live_then_av2, lane_type, av2_fn_then_live, live_then_av2],
    f"{ELEMENTS}/🏛️ShellHost/🟦️.tsx": [shellhost_imports, shellhost_cache_ref, live_then_av2, shellhost_tasks],
}


def main() -> int:
    for rel, steps in PLAN.items():
        path = os.path.join(ROOT, rel)
        text = open(path, encoding="utf-8").read()
        found = CONFLICT.findall(text)
        if len(found) != len(steps):
            print(f"[av2-resolve] REFUSED {rel}: {len(found)} conflicts, plan has {len(steps)}")
            return 1
        index = iter(range(len(steps)))
        text = CONFLICT.sub(lambda m: steps[next(index)](m.group(1), m.group(2)), text)
        open(path, "w", encoding="utf-8").write(text)
        print(f"[av2-resolve] {rel}: {len(steps)} resolved")
    return 0


if __name__ == "__main__":
    sys.exit(main())
