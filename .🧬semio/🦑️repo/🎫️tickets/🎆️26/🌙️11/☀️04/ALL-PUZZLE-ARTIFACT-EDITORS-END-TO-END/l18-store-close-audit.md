# L18 Audit: Dead 2D Store-Close Targets

Package: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust`

## Definitions
- `📋️project.json:22-30` holds `test-store-close-source`, which runs `repo ./📜️script.ts test-store-close source`.
- `📋️project.json:31-39` holds `test-store-close-native`.
- `📋️project.json:15` is the nativeSources glob `🧰️owned/🧪️tests/🧫️fixtures/♻️store-close/**/*`.
- `📜️script.ts:22-29` defines `StoreCloseScript`. It is registered at `:39`, and its import `runArtifactRustTests` at `:5` is used only at `:27`.

## Liveness
- Source branch: the test file `🔨️modules/🏠️host/🧰️owned/🧪️tests/♻️store-close/🟦️.ts` is missing, so the target fails with ENOENT.
- Native branch: the law `standalone_history_close_resumes_past_the_original_ledger_page` does not exist in the puzzle plugin.
- The fixture `🧪️tests/🧫️fixtures/♻️store-close/🔣️.json` is read only by the glob.
- `🧬️schema/♻️store-close/🔣️.json` is separate. Leave it.
- Result: both targets are cacheable and always fail. No other config references them.

## Removal edit list (pending user decision)
1. `📋️project.json`: delete lines 22-39 and line 15, then fix the commas.
2. `📜️script.ts`: delete lines 22-29, the `.register("test-store-close", …)` call at line 39, and `runArtifactRustTests` from the line-5 import.
3. Delete the fixture dir `🧪️tests/🧫️fixtures/♻️store-close/`.

The alternative is to restore the deleted test file and the rust law. Under the greenfield rules, removal is the clean option.

## Resolution (2026-11-23)
Done by another lane before L27: `📋️project.json` no longer has `test-store-close-*` targets or the `♻️store-close` nativeSources glob, `📜️script.ts` no longer has `StoreCloseScript` or its registration, and the fixture dir `🧪️tests/🧫️fixtures/♻️store-close/` is gone. Removal was chosen; nothing remains to decide.
