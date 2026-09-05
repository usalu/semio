# Current Window Options Authority

The central semantic directory registry already identifies `☑️options`. Its empty-facet projection contract still omitted the `options` kind and allowed only old `🎚️options`, colliding with sibling `🎚️config`. This was a source-ownership inconsistency, not a reason to preserve a duplicate name.

Added hand-authored neutral `☑️options.json` and independent `📐️options.schema.json` beside the existing empty-facet authority test. Eight vectors cover three current owner forms (artifact window, engine window, extension window) and five rejections (old options spelling, unknown owner, wrong parent, wrong kind, traversal). The test compares actual authority results with independent Ajv classification and with the exact pure authority compiled separately by Bun and TypeScript. It also parses the fixture with independent jsonc-parser.

RED before the central correction returned `unclaimed` for the real artifact-window options owner. After adding only the registered `options` kind/current `☑️options` name to the exact capture and replacing the old entry in `taxonomyLeafParentDirs`, GREEN passes **1 test / 42 assertions** through `@semio-tech/repo-lib:test-artifact-empty-facet-authority --test-name-pattern=current-options`. No old-name alias or blanket ownership exception was added.

To reach this check, the existing test's golden reader was corrected to the fixture's actual nested path. Its pure closure extractor now includes the real `leadingEmojiIdentity` helper and its exact segmenter declaration, which the authority already calls. No production parsing function needed a behavior change.

## Preserved older inconsistencies

The full existing gate currently reports **2 pass / 3 fail**. The older 19-case input is 3155 bytes while its retained assertion expects 3154; its former window-options case expects old `🎚️options` to project; and its registration schema reader repeats the authority directory in its path. The registration data also still names the old flat source path. Those frozen input bytes, assertions, and historical expectations were not weakened or rewritten. The new current-options fixture remains green and explicitly rejects the old spelling. The parent was informed; this is not a full-gate success claim.

Evidence: `🗑️generated/norm/options-red.log`, `options-green.log`, and `empty-facet-full-current.log`.

## First physical consumers

Three individually reviewed EN 1990 windows now have `☑️options`: editor inputs, editor results, viewer report. Their exact parent paths are under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any` with editor `✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs` and `📊️results`, and viewer `👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report`. Existing `📌️.empty.md` payloads remain 71, 71, and 0 bytes respectively; the zero-byte source was preserved, not filled with invented content. No executable reader referred to those old options paths.
