"""📝️ Advance current ticket summaries while retaining prior verification context."""
from pathlib import Path

ticket = Path(__file__).parent
status = 'Expanded122 registration completes3047pass/99225assertions/12existing gated skips/0fail/3059tests in294.00seconds (88IFC4x3+26details+8shared query/ring/Boolean/circle), terminal0 and0selected test-source hash changes. Comparison retains all119 test observations and adds61: basis11/world-circle9/nodal41. Circle10 and nodal7 independently close their actual public routes/physical facts; nativeLIST49 source audit closes without native execution proof. See checkpoint122.md and exact audit scopes. Native original-owner query10 is frozen for independent review; loaded-FEM/SELECT next work is schema-first. Root next polygonal4tests are intentionally1pass/3fail and unregistered with no polygonal production; exact schema/geometry identity facts are being established. Full IFC4x3/all-MVD/editing/interchange goal/ticket remain active/incomplete; native compiler/type/runtime/heap/receipt/Store/transport/capability/publication and whole-view conformance remain unproved. Older paragraphs are historical.'
for name in ['current-state.md', 'registered-test-scope.md', 'exchange-profile-current-state.md']:
    path = ticket / name
    title, separator, body = path.read_text(encoding='utf-8').partition('\n\n')
    if not body.startswith(status):
        path.write_text(title + separator + status + '\n\n' + body, encoding='utf-8', newline='\n')
