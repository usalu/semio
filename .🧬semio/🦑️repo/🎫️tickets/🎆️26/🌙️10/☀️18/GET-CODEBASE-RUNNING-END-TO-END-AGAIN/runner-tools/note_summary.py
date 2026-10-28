"""📝 Puts the ordered apply list and the verdict count at the top of the FOR coordinator section (one-shot ticket helper; rerunnable: replaces its own block)."""
import os, sys
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
head = "## FOR coordinator\n"
start = t.index(head) + len(head)
marker = "FOR coordinator: SUMMARY"
if t[start:].startswith(marker):
    end = t.index("\nFOR coordinator: tsdebt", start) + 1
    t = t[:start] + t[end:]
summary = (f"{marker} ({sys.argv[1]}): 30 official runs, every listed product has a verdict (hub: first official run ever, RED). GREEN: pets/course-react, ui-styling(+tokens), check-generated, course-rs/pets-rs build, proctor, presentation, projektetage, bericht, repo Rust bins r2+r3 (16 crates), framework-server-rs, and (after G46) course/framework-rs/pets-rs tests. "
           "RED with a staged fix: framework/ui-react/course typecheck + print:check (tsdebt), cargo custody fixtures (tsdebt part 2), ui-react Diagram test (forceclock), Go cli test (innerclock), os-dev contribution (nodecast), compiler starvation by dev listeners (devslot). "
           "RED needing your decision or another owner: repo-cli-rs (~/.local/bin/repo owner), workspace:verify (build-script crates refused), lint aggregates (dependsOn project globs), os-dev config bounds, plugin-registry, os-mcp consent test, custody queued-wait, tui keystroke law, hub (3 causes).\n"
           "FOR coordinator: APPLY ORDER (all `stage.py check` = base on live at the summary time; each is independent; verify after each): 1 `python3 -I C/runner-tools/staged/tsdebt/stage.py apply` ; 2 `…/nodecast/stage.py apply` ; 3 `…/innerclock/stage.py apply` ; 4 `…/forceclock/stage.py apply` ; 5 `…/devslot/stage.py apply` — "
           "then one product chain: `G pm-c3 '@semio-tech/framework' 'typecheck,test'`, `G pm-c6 '@semio-tech/ui-react' 'typecheck,lint,test'`, `G pm-osdev '@semio-tech/framework-os-dev' 'test'`, `G pm-x1 '@semio-tech/framework-process' 'test-cargo-build-lease,test-cargo-build-custody,test-resource-leases'`, `G pm-go <the 7 Go projects> 'test'`. schemaidx needs nothing (superseded, live drifted past it).\n")
t = t[:start] + summary + t[start:]
open(p, "w", encoding="utf8").write(t)
