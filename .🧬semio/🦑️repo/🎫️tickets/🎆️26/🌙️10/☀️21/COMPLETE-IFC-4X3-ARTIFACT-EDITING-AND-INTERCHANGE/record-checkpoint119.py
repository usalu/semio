"""📝️ Keep current ticket summaries explicit while retaining historical checkpoints."""
from pathlib import Path

ticket = Path(__file__).parent
status = 'Expanded119 registration completes2986pass/98931assertions/12existing gated skips/0fail/2998tests in292.71seconds (86IFC4x3+26details+7shared query/ring/Boolean), terminal0 and0selected test-source hash changes. All previous104 printed test observations remain and430new are added. Newly source/audit-closed faceted/mapped16 renders both unchanged official BReps, all five supported mapped leaf kinds and full declared-shell topology; finite LIST TS5, native RMB39 source-only, canonical geometry index, exchange20/header5/solid4 and exact BIM phase8 are included. See checkpoint119.md and each independent audit for physical/proof scope. Subsequent world-circle9tests2pass7fail and FEM nodal36tests7pass29fail are intentionally unregistered while production fixes begin after this completed reader. Native LIST work continues separately. Full IFC4x3/all-MVD/editing/interchange goal and ticket remain active/incomplete; native runtime/compiler/heap/receipt/Store/transport/capability/publication and whole-view conformance remain unproved. Older paragraphs are historical.'
for name in ['current-state.md', 'registered-test-scope.md', 'exchange-profile-current-state.md']:
    path = ticket / name
    title, separator, body = path.read_text(encoding='utf-8').partition('\n\n')
    if not body.startswith(status):
        path.write_text(title + separator + status + '\n\n' + body, encoding='utf-8', newline='\n')
