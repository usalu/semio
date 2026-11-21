# 🧪 Matrix Runner Follow-up

## Running subsets

Start the runner from the workspace with `repo tools exec --light node -- <ticket>/runner-server.cjs`. Use Tab and Enter on **Start batch**. The iframe is 900 px high and cycles widths 1440, 768 and 375 for English and German.

- All widths and languages for selected apps use `http://127.0.0.1:5700/?apps=play,os,elements`.
- Specific cells use `http://127.0.0.1:5700/?cells=course:en:375,hub:de:1440,demo:en:1440`.
- **Retry failed cells** selects cells with reports, errors or timeouts in the current batch.
- `http://127.0.0.1:5700/progress` reports the last app/story and target the runner reached.
- Reopen the same Elements or OS cell after interruption. Each completed story is saved locally under the generated matrix directory. Its key combines app, language, viewport and a hash of the runner, probe and story fixture. The final result replaces the partial checkpoint.
- Start, restart or stop manual previews with `repo tools exec --light node -- <ticket>/preview-cli.cjs start|restart|stop [app|all]`. The shell wrappers use this command. The supervisor launches through the repository tool registry and releases its recorded child processes when their last owner leaves. A healthy independently launched preview can be reused.
- Regenerate the durable `live-final.md` with `repo tools exec --light python -- <ticket>/matrix-report.py`. Its `--check` option fails unless all 48 latest cells pass. The updated supervisor records start, timeout and cancellation so a superseded result cannot satisfy this check.

## Locale and readiness

The runner waits for mounted content after the HTTP readiness check. The Hub preview waits for the mock-authorized tabs and switches languages through the actual admin Select. The selected control value confirms the Hub locale. Demonstrator, Course and Staffage use rendered locale controls and their declared language. The Play preview passes the locale to its existing landing fixture and verifies the app count is mounted. Play checks onboarding controls within the active overlay, activates Skip, and records a separate landing sweep with inert pane bodies.

Elements and OS run their existing story sweep entry points with a shared UI i18n instance per requested locale. An English sample string in a fixture may remain English where the production component receives caller-authored data. The CAD preview selects the shared provider and passes the locale to the GIS inference panel. Its authored toolbar, command, model and filter strings have both variants.

Story sweeps wait for painted content and shared-border readiness. The deferred OS submenu retains its explicit delay. Each story saves target count, focus checks and findings, or a render error. Standard app sweeps still exercise only the rendered state and the shared probe's visibility rules.

The hover and focus probe waits on the target's nearest compositor. Controls whose compositor has yet to attach use the original document-wide check until ownership is known, and each check retains two painted frames before measuring. The compositor also restores its owned SVG if a text-only React control replaces its children on a locale change.

## Local checks

- Six contract cases for full, app-filtered, cell-filtered and invalid-cell batch selection passed through `matrix-cells.test.cjs`.
- Six JSDOM cases in `matrix-settle.test.cjs` passed for sibling activity, delayed root attachment, new owner activity during paint and the existing wait bound.
- All 51 shared-border compositor tests passed. The localized label regression uses React to replace the button text and verifies the compositor reattaches its original SVG on the next commit.
- The live Hub German cells at 1440 × 900 and 768 × 900 passed with ten targets and ten focus-visible checks each after the reattachment fix.
- The actual Play and Elements matrix proxies delivered HTML with the bridge and served 486 and 364 reachable static JavaScript modules respectively. A third-party module lexer verified named static imports. The Play landing module graph did not reach the old WASM stub.
- Hub Admin HTML and its mock overview endpoint returned HTTP 200 on the isolated preview. OS showcase HTML and its locale-aware parts module returned 200. CAD HTML in both language URLs and its transformed locale fixture module returned 200.
- The server passed checkpoint create, read and delete on a synthetic story. Browser sweep counts and findings are tracked separately in the live results.
- Five preview ownership cases passed. Isolated HTTP checks verified reuse of the external Elements preview and clean shutdown of a newly launched Vite child. The attempt integration test verified stale-result rejection and durable readiness timeout reporting.
- All six Play cells completed two onboarding and 78 landing targets with 80 focus checks and no reports on the staged path.
- The bridge regression checks that a page error raised during the first Staffage target persists in the result. The runner reports Staffage's initial paint, script load, sweep start and first target separately so a timeout identifies the stalled phase. All 13 bridge, cell-selection and settle cases passed.

## Final browser verification

The final English tablet Staffage and mobile OS cells passed. The OS retry used the original showcase through native Vite with isolated dependencies after reads from the mounted repository caches stalled. The chip probe checked snapped outlines at their painted corners. Its regression fixture reproduces the measured closed outline, a missing side and a disconnected side. [Final matrix](live-final.md) · [Launcher diagnosis](launcher-diagnosis.md)
