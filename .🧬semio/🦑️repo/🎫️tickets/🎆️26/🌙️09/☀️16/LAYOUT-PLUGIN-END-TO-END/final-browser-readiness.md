# Final Browser Readiness

2026-10-29. Source-only review; no server, activation, browser, compiler, graph or test started. Actual browser runs remain zero and the ticket is open.

## Contract Review

Current create-layout-app declares addFrame with the rect/text/image Select and x/y numeric arguments, addPage, exportSvg, exportPng and exportPdf. These content/export actions are Migrated. Current retained factories register addFrame/addPage/patchDocument and the three export contracts. The journey uses their actual names and public controls, verifies the kind portal selection, stages arguments before the history baseline, submits with focused keyboard Enter, and derives the introduced frame ID by exact set difference. Artifact stays mounted during immediate undo/redo.

Inspection still generates layout-play-inspector.global.story.<id>.content with fingerprinted span inputs. document_input_at uses Commit with blur. The journey resolves the story-1 textarea anew and blurs its authored marker. It assumes no patchStory palette action.

Catalog qualification changes internal admission, not UI selectors or native action names. Current typed qualify_action validates original context and retained catalog/action identity, and registration must match the actual app owner and tool ID. Command-route capture qualifies before typing preparation; handle_intent_frame captures both pending/fresh paths before prepare_original_typed_action. The harness sends ordinary UI events without synthetic catalog credit, body or decoder. Its state/download polls must observe successful resumable admission. Detailed identity proof remains in typing-policy-review.md. No new selector mismatch or catalog bypass was found.

## Evidence And Limits

SVG/PNG use independently decoded/rasterized media, baseline differences, expected authored rect color and agreement across a144pixel interior patch. PDF independently checks exact page count/dimensions and authored marker with stock text absent. Boot checks visible canvas size, nonuniform pixels, published host/board status, visible shell faults and console errors. Pixel variance alone does not prove complete visual correctness.

Archive creation precedes deliberate original-only page/story divergence. Import must create one new Blueprint owner absent from the old owner set, restore exact saved frame/page IDs and marker, and exclude both later mutations. This witnesses a separate owner archive round-trip. It makes no browser reload, process restart or multi-user persistence claim.

Actionability, native event delivery, font/media execution, GPU support and live import remain unproved until execution. The existing narrowly ignored empty-pack diagnostic remains an explicit console filter. No new source change justified repeating already-passing syntax/helper checks.

## Managed Browser And Outputs

The journey resolves the ticket via import.meta.url and applies repoToolCacheEnv(workspace) before dynamic Playwright import. The pinned package selects the platform executable; no system-browser fallback or hardcoded macOS executable is present. Prior browser-readiness.md proves current managed executable presence, not successful launch. Existing caller path overrides follow repo policy.

downloadsPath, saved media, screenshots, report and console files resolve under ticket/🗑️generated/<sanitized-run-name>; dot/double-dot names are refused. launch-layout.py resolves its own workspace, pins the specific generated catalog and forwards optional --tool to each child. The same managed Python/Bun interfaces apply on native Windows/macOS/Linux.

## Next Owned Doors

After the existing activation succeeds, from /Users/ueli/Documents/semio start the declared server once:

~~~text
repo --tool=bazel run @semio-tech/framework-os-dev:serve-layout-react-prototype --output=stream
~~~

Wait for ordinary listener readiness on fixed6079, then run one real probe:

~~~text
repo --tool=bazel tools exec python -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/launch-layout.py probe --tool bazel
~~~

Equivalent direct managed entry:

~~~text
repo --tool=bazel tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/browser-journey.mjs
~~~

No --light on the actual browser run. Do not duplicate current activation/native work. Expected URL is http://127.0.0.1:6079/?plugin=layout; HTTP delivery is insufficient. Preserve the continuous server session for owned cancellation. Require complete:true plus all12required step witnesses before claiming the UI goal.
