import sys,os
p=sys.argv[1]
V="️"
s=open(p,encoding="utf8").read()
lines=s.split("\n")
a=next(i for i,l in enumerate(lines) if l=="    if flags.mutated {")
b=len(lines)-1
while lines[b].strip()=="" : b-=1
assert lines[b]=="}", lines[b]
body=lines[a:b]
sig="""/// 🏁️ Pulls the pending effects of a mutating exchange, drives the typed operation drain, folds the ephemeral snapshot and the presence outcome into the reply frames and seals the exchange output.
pub(super) async fn finish_exchange<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx: &mut semio_framework_job::StepContext<'_>,
    mut frames: Vec<protocol::AppFrame>,
    mut effect_bytes: Vec<Vec<u8>>,
    mut event_bytes: Vec<Vec<u8>>,
    retry_command: Option<(u64, PluginCommandIngress)>,
    command_terminal_fault: Option<Vec<u8>>,
    presence_pending: Option<u64>,
    presence_terminal: Option<u64>,
    presence_terminal_fault: Option<Vec<u8>>,
    flags: ExchangeFlags,
) -> Result<PluginExchangeOutput, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {"""
child="//! 🏁️ Exchange finish: the effect pull, the typed operation drain, the ephemeral snapshot and the presence outcome that seal one plugin exchange. See [`super`].\n\nuse super::*;\n\n"+sig+"\n"+"\n".join(body)+"\n}\n"
d=os.path.join(os.path.dirname(p),"🏁️exchange-finish")
os.makedirs(d,exist_ok=False)
open(os.path.join(d,"🦀️.rs"),"w",encoding="utf8").write(child)
call="    finish_exchange(runtime, instance_id, identity, cx, frames, effect_bytes, event_bytes, retry_command, command_terminal_fault, presence_pending, presence_terminal, presence_terminal_fault, flags).await"
lines=lines[:a]+[call]+lines[b:]
# module declaration after the last mod/use pair
mi=max(i for i,l in enumerate(lines[:90]) if l.startswith("use ") and ("_commands::" in l or "exchange_finish" in l or "::{arm_" in l))
lines[mi+1:mi+1]=["","#[path = \"🏁️exchange-finish/🦀️.rs\"]","mod exchange_finish;","use exchange_finish::finish_exchange;"]
open(p,"w",encoding="utf8").write("\n".join(lines))
print("tail extracted",len(body))
