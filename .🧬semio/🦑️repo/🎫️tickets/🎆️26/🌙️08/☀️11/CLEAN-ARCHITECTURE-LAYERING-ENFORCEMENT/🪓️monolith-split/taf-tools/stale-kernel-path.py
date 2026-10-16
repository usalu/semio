R = "/Users/ueli/Documents/semio/"
K = "🧰️framework/🔨️modules/🎠️kernel/"
OS = "🧰️framework/🛍️products/💻️os/"
def sub(path, pairs):
    p = R + path
    t = open(p, encoding="utf8", newline="").read()
    for old, new, cnt in pairs:
        assert t.count(old) == cnt, (path, old[:80], t.count(old))
        t = t.replace(old, new)
    open(p, "w", encoding="utf8", newline="").write(t)
    print("ok", path.split("/")[-2], len(pairs))
sub(".cursor/plans/realtime_interaction_history_c71a5401.plan.md", [("/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts)", "/🧰️framework/🔨️modules/🎠️kernel/🎯️ui-dirty-scope/🟦️.ts)", 1)])
sub("✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs", [("`buildContributionsJson` (`🎠️kernel/🟦️.ts`)", "`buildContributionsJson` (`🎠️kernel/🧾️contributions-json/🟦️.ts`)", 1)])
sub("✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs", [("`buildContributionsJson` (`🎠️kernel/🟦️.ts`)", "`buildContributionsJson` (`🎠️kernel/🧾️contributions-json/🟦️.ts`)", 1)])
sub(K + "🎞️media-jobs/🦀️.rs", [
  ("`FRAMEWORK_RESERVED_JOB_KIND` in `🎠️kernel/🟦️.ts`", "`FRAMEWORK_RESERVED_JOB_KIND` in `🎠️kernel/🛠️spawned-job/🟦️.ts`", 1),
  ("`spawnedJobCompletion` in `🎠️kernel/🟦️.ts`", "`spawnedJobCompletion` in `🎠️kernel/🛠️spawned-job/🟦️.ts`", 1),
  ("`mediaExportBytes` in `🎠️kernel/🟦️.ts`", "`mediaExportBytes` in `🎠️kernel/🎬️media-export/🟦️.ts`", 1),
  ("`videoRenderProgramProblem` in `🎠️kernel/🟦️.ts`", "`videoRenderProgramProblem` in `🎠️kernel/🎥️video-render-program/🟦️.ts`", 1),
  ("`importPayloadChunks` in `🎠️kernel/🟦️.ts`", "`importPayloadChunks` in `🎠️kernel/🗄️file-open-import/🟦️.ts`", 1),
])
sub(K + "🖼️ui/🦀️.rs", [("as the TypeScript twin in `🎠️kernel/🟦️.ts`", "as the TypeScript twin in `🎠️kernel/🎯️ui-dirty-scope/🟦️.ts`", 1)])
sub(K + "🧪️tests/⬇️media-export-encoding/🦀️.rs", [("that `🎠️kernel/🟦️.ts`'s TypeScript twin", "that `🎠️kernel/🎬️media-export/🟦️.ts`'s TypeScript twin", 1)])
sub(K + "🧪️tests/🐢️ui-dirty-scope/🦀️.rs", [("that `🎠️kernel/🟦️.ts`'s TypeScript twin", "that `🎠️kernel/🎯️ui-dirty-scope/🟦️.ts`'s TypeScript twin", 1)])
sub(K + "🧪️tests/📤️file-open-import/🦀️.rs", [("that `🎠️kernel/🟦️.ts`'s TypeScript twin", "that `🎠️kernel/🗄️file-open-import/🟦️.ts`'s TypeScript twin", 1)])
sub(K + "🧪️tests/🧵️spawned-job-drive/🦀️.rs", [("that `🎠️kernel/🟦️.ts`'s TypeScript twin", "that `🎠️kernel/🛠️spawned-job/🟦️.ts`'s TypeScript twin", 1)])
sub(K + "🧬️schema/🔣️history-patch/🔣️.json", [("TypeScript twin `🎠️kernel/🟦️.ts`", "TypeScript twin `🎠️kernel/🕰️history/🟦️.ts`", 1)])
sub("🧰️framework/🔨️modules/🖱️ui/📜️declaration/📐️layout/🦀️.rs", [("`🎠️kernel/🟦️.ts` — same boundary", "`🎠️kernel/🧭️app-router/🟦️.ts` — same boundary", 1)])
sub(OS + "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🛰️framework-plugin-runtime/🟦️.ts", [
  ("(`🎠️kernel/🟦️.ts`'s\n  // `PluginWasmHandle` header doc)", "(`🎠️kernel/🪝️plugin-wasm-handle/🟦️.ts`'s\n  // `PluginWasmHandle` header doc)", 1),
  ("(`🎠️kernel/🟦️.ts`'s own doc comment names it)", "(`🎠️kernel/🌉️plugin-backbone-routing/🟦️.ts`'s own doc comment names it)", 1),
])
sub(OS + "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🪟️app-mode-layouts/🔣️.json", [("`resolveLayoutForMode` (🎠️kernel/🟦️.ts)", "`resolveLayoutForMode` (🎠️kernel/🧾️contributions-json/🟦️.ts)", 1)])
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
sub(L + "⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture/🟦️.ts", [("`🎠️kernel/🟦️.ts`, whose own docstring", "`🫧️transient/🟦️.ts`, whose own docstring", 1)])
sub(L + "🎛️commands/✅️verify/🟦️.ts", [("law imports `🎠️kernel/🟦️.ts` and would", "law imports `🎠️kernel/🎬️media-export/🟦️.ts` and would", 1)])
