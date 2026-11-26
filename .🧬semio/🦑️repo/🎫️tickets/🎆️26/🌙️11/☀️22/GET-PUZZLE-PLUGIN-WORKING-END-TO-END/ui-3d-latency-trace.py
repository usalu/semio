import os
from pathlib import Path

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
probe = (ticket / "current-crossing-grant-trial.rs").read_text()
marker = "        let census=semio_plugin::app::typed_operation_unit_census()-before;"
probe = probe.replace(marker, marker + '''
        for (_, unit) in (0..64).filter_map(|index| raw.tools.tool_operations.entry(index)) {
            let owner = unit.page_publication.owner.as_ref();
            let page = owner.map(|page| (page.plan().map(|plan| plan.intent), page.backing_ready(), page.is_framed(), page.written_bytes(), page.phase_witness()));
            let pending = unit.pending_artifact_publication.as_ref().map(|store| (store.is_closing(), store.holds_store_custody(), store.accepted_receipt().is_some()));
            eprintln!("[DEBUG] turn={turn} page={page:?} result={:?} retiringResult={} pending={pending:?} pageWitness={}", unit.result_page.as_ref().map(|page| page.lane), unit.result_page_retirement_pending(), unit.page_publication.phase_witness());
        }
''')
name = "ui-3d-latency-trace.rs"
(generated / name).write_text(probe)
os.environ.update(PUZZLE_3D_PROBE=name, PUZZLE_OUTPUT="ui-3d-latency-trace", PUZZLE_PROBE_WIDTH="64")
outer = ticket / "current-3d-live-graph.py"
source = outer.read_text().replace(
    "s=base.read_text()",
    's=base.read_text().replace("probe = (ticket/probe_name).read_text()", "probe = (gen/probe_name).read_text()")',
)
exec(compile(source, str(outer), "exec"), {"__file__": str(outer)})
