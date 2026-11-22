"""Check the production WGPU effect queue with a manually driven archive loader."""
from pathlib import Path
import json
import subprocess

root = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu")
route = next(root.rglob("🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs")).read_text()
types = next(root.rglob("🫧️surfaces/🧬️component/🦀️.rs")).read_text()
pump = next(root.rglob("🖐️pointer/⌨️keyboard-and-palette/🔁️deferred-menus/🦀️.rs")).read_text()
assert "if self.as_window_actions().poll_deferred_document_effects()" in pump
head = route.index("    pub fn queue_host_effects")
end = route.index("                semio_framework::kernel::Effect::SetActiveUtility", head)
load_start = route.index("                semio_framework::kernel::Effect::LoadDocument", end)
load_end = route.index("                semio_framework::kernel::Effect::RequestMediaFrames", load_start)
definitions = types[types.index("pub struct DeferredHostEffects"):types.index("/// 🫀️ `SettleState`")]
portable_route = route[head:end] + route[load_start:load_end] + "} } } }"
portable_route = portable_route.replace('target_arch = "wasm32"', 'feature = "browser"')
portable_route = portable_route.replace(
    "semio_renderer_wgpu_host_base::spawn_app_task(load);", "self.settle.spawn_app_task(load);"
)
prelude = r"""
extern crate self as semio_framework;
extern crate self as semio_renderer_wgpu_program_bridge;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;

pub type ProgramFault = &'static str;
pub mod kernel {
    pub enum Effect {
        LoadDocument { pack: Vec<u8>, spr: Vec<u8> },
        DispatchAction { action: String, args: Option<()> },
    }
}
use kernel::Effect;
pub struct UiDirtyScope;
impl UiDirtyScope {const Full: Self=Self;}
struct ActionDescriptor { controller_id:String, action:String, args:Option<()> }
enum ShellIoCompletion{Finished}
struct ActiveSession { plugin_id:String,instance_id:u32,window_id:String }
#[derive(Clone)]struct Plugin {plugin_id:String}
impl Plugin {
    async fn load_app_document_pack(&self,_instance:u32,pack:&[u8],_spr:&[u8])->Result<(),ProgramFault> {
        if pack==b"fail" {Err("load failed")} else {Ok(())}
    }
}
struct Plugins{plugins:Vec<Plugin>}
struct Docs {session:Option<ActiveSession>}
struct Prefs;
impl Prefs {fn note_refused_open(&self,error:ProgramFault,_chrome:&mut ())->String{error.into()}}
#[derive(Default)]
struct Settle {
    deferred_document_load:Option<DeferredDocumentLoad>,
    held_host_effects:VecDeque<DeferredHostEffects>,
    deferred_actions:Vec<ActionDescriptor>,
    jobs:VecDeque<Pin<Box<dyn Future<Output=()>>>>,
    refreshes:usize,
}
impl Settle {
    fn submit_shell_io_future(&mut self, f:impl Future<Output=ShellIoCompletion> + 'static) {
        self.spawn_app_task(async move { f.await; });
    }
    fn spawn_app_task(&mut self,f:impl Future<Output=()>+'static){self.jobs.push_back(Box::pin(f));}
    fn complete(&mut self) {
        let mut job=self.jobs.pop_front().expect("the load has started");
        assert!(job.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())).is_ready());
    }
    fn owe_refresh(&mut self,_scope:UiDirtyScope){self.refreshes+=1;}
}
struct Context {docs:Docs,plugins:Plugins,settle:Settle,prefs:Prefs,chrome:()}
impl Context {
    fn new()->Self{
        Self{docs:Docs{session:Some(ActiveSession{plugin_id:"app".into(),instance_id:1,window_id:"view".into()})},
            plugins:Plugins{plugins:vec![Plugin{plugin_id:"app".into()}]},settle:Settle::default(),prefs:Prefs,chrome:()}
    }
    fn drain(&mut self)->Vec<String>{
        std::mem::take(&mut self.settle.deferred_actions).into_iter().map(|action|{
            assert_eq!(action.controller_id,"controller");
            assert!(action.args.is_none());
            action.action
        }).collect()
    }
"""
ticket = Path(__file__).parent
fixture = json.loads((ticket / "document-load-order.json").read_text())
body = ""
for index, row in enumerate(fixture["cases"]):
    body += f'#[test]\nfn case_{index}() {{ let mut c=Context::new();\n'
    for step in row["steps"]:
        op, *args = step
        if op == "queue":
            effects = []
            for kind, value in args[0]:
                text = json.dumps(value)
                if kind == "load":
                    effects.append(f"Effect::LoadDocument{{pack:b{text}.to_vec(),spr:vec![]}}")
                else:
                    effects.append(f"Effect::DispatchAction{{action:{text}.into(),args:None}}")
            body += 'c.queue_host_effects("controller",vec![' + ",".join(effects) + ']);\n'
        elif op == "drain":
            values = "vec!" + json.dumps(args[0]) if args[0] else "Vec::<&str>::new()"
            body += f'assert_eq!(c.drain(),{values},"{row["id"]}");\n'
        elif op == "poll":
            body += f'assert_eq!(c.poll_deferred_document_effects(),{str(args[0]).lower()},"{row["id"]}");\n'
        elif op == "complete":
            body += "c.settle.complete();\n"
        elif op == "focus":
            body += f'c.docs.session.as_mut().unwrap().window_id={json.dumps(args[0])}.into();\n'
        elif op == "session":
            body += f'c.docs.session.as_mut().unwrap().instance_id={args[0]};\n'
    body += "}\n"
source = definitions + prelude + portable_route + body
output = ticket / "🗑️generated" / "document-load-order-proof"
try:
    for kind in ("native", "browser"):
        options = ["--cfg", 'feature="browser"'] if kind == "browser" else []
        subprocess.run(
            ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
             "rustc", "--edition=2024", "--test", *options, "-A", "dead_code", "-A", "non_upper_case_globals",
             "-o", str(output), "-"],
            input=source, text=True, check=True,
        )
        subprocess.run([str(output)], check=True)
finally:
    output.unlink(missing_ok=True)
