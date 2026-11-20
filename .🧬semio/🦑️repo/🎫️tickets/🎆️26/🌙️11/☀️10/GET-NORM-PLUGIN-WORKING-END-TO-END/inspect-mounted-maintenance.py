"""🩺 Replay the exact command fixture with a passive census of its live maintenance owners."""
from pathlib import Path
import os
import subprocess

ticket = Path(__file__).resolve().parent
work = ticket / '🗑️generated'
baseline = work / 'archive_context_runtime_commands.rs'
census = work / 'archive_census.rs'
case = work / 'archive-census.wasm'
source = baseline.read_text()
marker = '            if self.admitted && !isolate_command {'
assert source.count(marker) == 1
source = source.replace(marker, '''            if self.commands.is_some() && turn % 50 == 0 {
                let instances = self.runtime.instances.borrow();
                let cell = instances.get(1).unwrap();
                let instance = cell.background_instance().unwrap();
                let app = &instance.app;
                let ladder = app.ladder_ref();
                let tools = &app.tools;
                let owners = (0..32).filter_map(|slot| tools.tool_operations.entry(slot)).map(|(id, owner)|
                    (id, owner.stage, owner.producer_build_pending(), owner.result_page_presented)).collect::<Vec<_>>();
                let units = (0..41).filter_map(|position| ladder.maintenance_unit_at(position, app.maintenance.maintenance_stage))
                    .filter_map(|unit| ladder.maintenance_unit_demand(unit, 32768, cx.fuel_remaining()).unwrap().map(|quote| (unit, quote)))
                    .collect::<Vec<_>>();
                let custody = tools.tool_cancellations.handle().map(|handle| (std::sync::Arc::strong_count(&handle.state), std::sync::Arc::weak_count(&handle.state)));
                println!("[DEBUG] maintenance census {turn}: terminal {}, units {units:?}, tool owners {owners:?}, cancellations {} custody {custody:?}, emission {}/{}/{}, agent preview {}, direct ingress {}, action ingress {}, execution owner empty {:?}",
                    app.maintenance_terminal_is_empty(), tools.tool_cancellations.active_operation_count(),
                    tools.original_emission_admission.is_some(), tools.original_emission_receipt.is_some(), tools.original_emission_construction.is_some(),
                    tools.agent_lane_preview.is_some(), app.stores.window_config_store.direct_ingress_quiescent(), app.ingress.action_ingress_sources_idle(),
                    tools.instance_operation_owner.terminal_is_empty());
            }
''' + marker)
census.write_text(source)
baseline_args = (work / 'archive-context-runtime-commands.args').read_text().splitlines()
args = [str(census) if argument == str(baseline) else str(case) if argument == str(work / 'archive-context-runtime-commands.wasm') else argument for argument in baseline_args]
flags = work / 'archive-census.args'
flags.write_text('\n'.join(args) + '\n')
if subprocess.run([os.environ['RUSTC'], '@' + str(flags)]).returncode:
    raise SystemExit('Mounted maintenance observation fixture failed to compile')
subprocess.run(['node_modules/.bin/jco', 'run', '--jco-dir', work / 'archive-census-jco', case, 'wire', 'replacement', 'runtime-cleanup', 'browser-commands'], cwd=ticket.parents[6], check=True)
