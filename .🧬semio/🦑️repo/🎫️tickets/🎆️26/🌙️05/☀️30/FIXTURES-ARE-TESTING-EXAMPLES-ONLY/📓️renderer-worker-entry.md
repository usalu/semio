# Renderer Worker Test Entry

Read-only current Source observations; no execution or authorship credit.

The helper `engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` exports `registerTests1` at line 34. It is not a standalone Vitest suite. The genuine entry is `engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`: lines 4575–4577 check `import.meta.vitest`, dynamically import that helper, and call it with the real defining dependency set and original URL.

React config lines 215–219 include this defining PluginRuntime source in `longInSourceSuites`; exhaustive inherits it at 221–224. Line 259 chooses these only for long/exhaustive (unless the explicit backbone/agent switches replace them). Fundamental/quick have no such includeSource. The exact source filter must therefore name `🧱️elements/🔌️PluginRuntime/🟦️.tsx`, rather than the helper under tests. From the package root its relative path is `../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx`.

The original package target `test-long` calls `bun ./📜️script.ts test long`; its TestScript at 37–38 resolves the level and forwards the remaining arguments to the original config. Use this existing target with the defining-source file filter and literal test-name selection. Four preserved worker laws are declared in the helper: `binds the actual actor document port and retires it before guest disposal` (774), `awaits actual actor handle disposal until its exact document retirement settles` (813), `awaits actual actor pending creation without opening an orphan after disposal` (851), and `retries actual actor retirement with the original witness after final acknowledgement failure` (2264). A NoTestsFound result from selecting the helper is not a RED or behavioral receipt.

Current hashes are retained in the accompanying input. Config metadata is selection evidence only; actual four named outcomes remain to be acquired by Root.
