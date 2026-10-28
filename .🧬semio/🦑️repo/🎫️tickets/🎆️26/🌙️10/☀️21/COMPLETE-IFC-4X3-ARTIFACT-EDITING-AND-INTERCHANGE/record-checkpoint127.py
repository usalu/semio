"""📝️ Record the current genuine broad checkpoint and keep historical evidence visible."""
from pathlib import Path

ticket = Path(__file__).parent
status = 'Expanded127 completes3216pass/105154assertions/12existing gated skips/0fail/3228tests in321.00seconds (92IFC4x3+27details+8shared), terminal0 and0selected test-source hash changes. Actual comparison retains all122 observations and adds169 (SELECT15/PFS55/CSG42/Surface57). See checkpoint127.md and exact independent audit scopes. The failed125 stale fixture checkpoint and corrected SET budget fixture remain historical evidence. Entity-only SELECT16 is separately independently green5630assertions/TS5+native64source-closed and registered only after127terminal, making128current registered files; it was not selected in127. Full IFC4x3/all-MVD goal/ticket remain active/incomplete. TFS32 owner-query/closure admission is next; normal exchange is still explicitly omitted. Native canonicalFEM9/export14/unit15/SELECT58/reference64 are source-only audits, with genuine Cargo101 before rustc and no native type/runtime/heap/receipt/Store/transport/capability/publication proof. Older paragraphs are historical.'
for name in ['current-state.md', 'registered-test-scope.md', 'exchange-profile-current-state.md']:
    path = ticket / name
    title, separator, body = path.read_text(encoding='utf-8').partition('\n\n')
    if not body.startswith(status):
        path.write_text(title + separator + status + '\n\n' + body, encoding='utf-8', newline='\n')
