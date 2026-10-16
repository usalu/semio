"""🟦️ Moves the tool run, introduction and tutorial blocks of the manifest TS twin into their owners' TS twins."""
import pathlib, re
R = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules")
m = R / "🛂️manifest/🟦️.ts"
s = m.read_text()

def cut(start, end_inclusive):
    global s
    assert s.count(start) == 1 and s.count(end_inclusive) == 1, (start[:50], end_inclusive[:50])
    i = s.index(start)
    j = s.index(end_inclusive, i) + len(end_inclusive)
    block = s[i:j]
    k = j
    while s[k:k + 1] == "\n":
        k += 1
    s = s[:i] + s[k:]
    return block

tool = cut("/** ⏯️ Generated from Rust `ToolRunDefinition` (`⏯️tool-run/🦀️.rs`", "  return app.tools.some((tool) => tool.run !== undefined) || app.utilities.some((utility) => utility.run !== undefined);\n}\n")
intro_id = cut("/** 🎓️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette)\n * to (re)start an app's introduction", 'export const START_INTRODUCTION_ACTION_ID = "startIntroduction";\n')
intro = cut("/** 🎓️ Generated from Rust `Introduction*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */", "export type IntroductionDemonstration = GeneratedIntroductionDemonstration;\n")
tutorial = cut("//#region 🎬️Tutorial\n", "//#endregion 🎬️Tutorial\n")

tool = tool.replace('} from "../⏯️tool-run/🟦️.ts";', '} from "../🟦️.ts";')
assert '} from "../🟦️.ts";' in tool

generated_line_start = s.index('import type { ActionDescriptor as GeneratedActionDescriptor')
generated_line_end = s.index("\n", generated_line_start)
line = s[generated_line_start:generated_line_end]
moved_tool = ["JobKindId", "ToolRunCounterDefinition", "ToolRunDefinition", "ToolRunReasonDefinition", "ToolRunRebasePolicy", "ToolRunReconfigurePolicy", "ToolRunSettingsReads", "ToolRunStageDefinition", "ToolRunTraceKind", "ToolRunVerdict"]
moved_onboarding = sorted(set(re.findall(r"\b((?:Introduction|Tutorial)[A-Za-z]+) as Generated\1\b", line)))
rest = s[:generated_line_start] + s[generated_line_end:]
def still_used(name):
    return re.search(r"\bGenerated" + name + r"\b", rest) is not None
for name in moved_tool + moved_onboarding:
    assert not still_used(name), name
    entry = f"{name} as Generated{name}, "
    assert line.count(entry) == 1, name
    line = line.replace(entry, "")
s = s[:generated_line_start] + line + s[generated_line_end:]

onboarding_types = "IntroductionDefinition, TutorialDefinition"
anchor = 'import type { AppRole, AppRef } from "./🧬️schema/🟦️.ts";\n'
assert s.count(anchor) == 1
s = s.replace(anchor, anchor + f'import type {{ {onboarding_types} }} from "../🎓️onboarding/🟦️.ts";\n')
m.write_text(s)

gen_path_tool = "../../🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"
tool_imports = ", ".join(f"{n} as Generated{n}" for n in moved_tool)
(R / "⏯️tool-run/🕹️actions/🟦️.ts").write_text(
    "/** ⏯️ TypeScript twin of `semio_framework::tool_run` (`🦀️.rs` beside this file): the run vocabulary a tool or utility\n"
    " * declares and the predicate that reserves the run actions. Exported through `@semio-tech/framework`. */\n"
    f'import type {{ {tool_imports} }} from "{gen_path_tool}";\n\n' + tool)

onboarding_imports = ", ".join(f"{n} as Generated{n}" for n in moved_onboarding)
intro_id = intro_id.replace("/** 🎓️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette)\n * to (re)start", "/** 🎓️ The reserved action id apps dispatch (or the shell auto-injects into the command palette)\n * to (re)start")
(R / "🎓️onboarding/🟦️.ts").write_text(
    "/** 🎓️ TypeScript twin of `semio_framework::onboarding` (`🦀️.rs` beside this file): the introduction and tutorial\n"
    " * declarations and the reserved action ids that start or record them. Exported through `@semio-tech/framework`. */\n"
    f'import type {{ {onboarding_imports} }} from "../🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts";\n'
    'import type { WindowLayout } from "../🛂️manifest/🟦️.ts";\n\n' + intro_id + "\n" + intro + "\n" + tutorial)

li = R / "🎓️onboarding/🏠️local-interaction/🟦️.ts"
t = li.read_text()
assert t.count('from "../../../📡️replication/📡️wire/🏠️local-interaction/🟦️.ts"') == 1
li.write_text(t.replace('from "../../../📡️replication/📡️wire/🏠️local-interaction/🟦️.ts"', 'from "../../📡️replication/📡️wire/🏠️local-interaction/🟦️.ts"'))
print(moved_onboarding)
