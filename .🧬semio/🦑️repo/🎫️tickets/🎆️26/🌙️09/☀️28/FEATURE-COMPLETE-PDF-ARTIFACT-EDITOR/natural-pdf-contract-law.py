"""Extract the paid natural source authority and decoder progress method for the neutral law."""
from pathlib import Path
from runpy import run_path
import re

ticket = Path(__file__).parent
stage = run_path(str(ticket / "natural-pdf-publication-stage.py"))
changes, output, plugin, instance, decl = (stage[key] for key in ("changes", "output", "plugin", "instance", "decl"))
draft = lambda path: changes[path][1]
contract = draft(decl / "👷️job/🧵️reserved-tool-job/🦀️.rs")
worker = draft(decl / "📥️natural-file/🦀️.rs")
builder = draft(decl / "🪟️surface/🖊️editor-app/🦀️.rs")
factory = draft(stage["factory"])
ingress = draft(stage["ingress"])
render = draft(instance / "🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs")
mount = draft(instance / "📨️dispatch/📥️staging/🎟️producers/🦀️.rs")

assert re.search(r'if A::natural_file_paged_maximum_bytes\(\) != 0 \{\s*registry.register\(FrameworkImportNaturalSourceJobFactory', factory)
assert re.search(r'FrameworkImportNaturalSourceJobFactory, "import-natural-source", 22,\s*NATURAL_SOURCE_TOOL_CONTRACT.max_raw_wire_bytes, NATURAL_SOURCE_TOOL_CONTRACT.max_decoded_items,\s*NATURAL_SOURCE_TOOL_CONTRACT.max_work_units_per_step, NATURAL_SOURCE_TOOL_CONTRACT.max_output_bytes,\s*lanes: \[Artifact, Child\]', factory)
assert 'concat!("framework.reserved.", $route, ".v1")' in factory
assert '::resumable($raw, $items, $work, $output, 7_500, 1, 1)' in factory
assert re.search(r'if verb == NATURAL_SOURCE_TOOL_ID \{\s*return self.framework_registration\(verb\).*?row.contract == NATURAL_SOURCE_TOOL_CONTRACT.*?TypeId::of::<semio_plugin_job::reserved_jobs::FrameworkImportNaturalSourceJobFactory<A>>\(\)', ingress, re.S)
assert 'prepare_reserved_header(NATURAL_SOURCE_TOOL_ID, meta, cx)' in render
assert 'assemble_reserved_request(NATURAL_SOURCE_TOOL_ID, meta' in render
assert 'copy_metadata(6,[action,""],cx)' in render and 'copy_metadata(8,[action,""],cx)' in render
assert 'proof=self.tools.qualified_tool_proof(&*self.stores,action)?;let contract=proof.contract();' in render
assert 'payload_schema_id,contract,app_instance_id:meta.instance_id' in render
assert 'advance_original_reserved_mount(semio_plugin_decl_app::tool_contract::NATURAL_SOURCE_TOOL_ID' in mount
assert 'let max_output_bytes=proof.contract().max_output_bytes;' in mount
assert 'publication_lanes:proof.publication_lanes()' in mount
assert 'admits_paged_natural_request(' in builder
assert 'request.tool_id == semio_plugin_decl_app::tool_contract::NATURAL_SOURCE_TOOL_ID' in builder
assert worker.index('::admits_paged_natural_request(') < worker.index('E::admit_paged_natural_file_decode_owner(')
assert 'pending_port: None, contract: request.contract' in worker
assert 'OriginalSnapshotPackExport::admit(source()?, epoch, self.contract.max_output_bytes' in worker

def cut(s, start, end):
    first = s.index(start)
    return s[first:s.index(end, first)]

action_bus = Path("🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs").read_text(encoding="utf-8")
execution = cut(action_bus, '/// 🔒️ Exact, reviewable admission bounds', 'pub struct ErasedToolJob')
consts = cut(contract, 'pub const NATURAL_SOURCE_TOOL_ID', '/// 🧭️ Owned input')
gate = cut(worker, 'fn admits_paged_input', 'pub(crate) struct NaturalFileImportWork')
progress_start = worker.index('    fn admit_controlled_progress')
progress_end = worker.index('\n    }', progress_start) + len('\n    }')
progress = worker[progress_start:progress_end]
ordinary = re.search(r'FrameworkImportMediaJobFactory, "import-media", 10, ([0-9_, ]+), lanes:', factory)[1]
output.joinpath("contract-extracted.rs").write_text(
    f'{execution}\n{consts}\n{gate}\nimpl Worker {{\n{progress}\n}}\n'
    f'const ORDINARY: ToolExecutionContract = ToolExecutionContract::resumable({ordinary}, 7_500, 1, 1);\n',
    encoding="utf-8",
)
