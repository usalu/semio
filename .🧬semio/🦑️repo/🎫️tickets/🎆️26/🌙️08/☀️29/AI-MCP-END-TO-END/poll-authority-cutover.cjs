const fs = require('node:fs');
const path = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
let after = before;
function replace(from, to) { if (after.split(from).length !== 2) throw Error(`expected one ${from}`); after = after.replace(from, to); }
replace('runtime:&semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,input:semio_plugin_abi::owned_abi::PollInput)->u64{if !actor_receipt_available()', 'runtime:&semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,input:semio_plugin_abi::owned_abi::PollInput,authority:kernel::OriginalJobBudget)->u64{if !actor_receipt_available()');
replace(' let retained=input.budget.retained;ORIGINAL_CLOCK_BINDING.with(|slot|slot.set(retained.work.clock_binding));', ' let retained=authority.input;let budget=input.limits.narrow(authority);ORIGINAL_CLOCK_BINDING.with(|slot|slot.set(retained.work.clock_binding));');
replace('let step=step_budget(retained,input.budget.retained_policy,input.budget.work_fuel,input.budget.deadline_us)?;', 'let step=step_budget(retained,budget.retained_policy,budget.work_fuel,budget.deadline_us)?;');
replace('input.command_page,input.cold_pair_page,input.budget,identity,cx)', 'input.command_page,input.cold_pair_page,budget,identity,cx)');
const temporary = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/poll-authority.tmp';
fs.writeFileSync(temporary, after);
if (fs.readFileSync(path, 'utf8') !== before) throw Error('shared actor exports changed');
fs.renameSync(temporary, path);
