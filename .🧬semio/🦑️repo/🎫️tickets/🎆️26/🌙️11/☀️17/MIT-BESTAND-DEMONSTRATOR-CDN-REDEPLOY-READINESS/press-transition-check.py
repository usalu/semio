from pathlib import Path
import json, subprocess
root=Path.cwd(); ticket=Path(__file__).resolve().parent
source=root/'🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs'
s=source.read_text(); start=s.index('        if matches!(source.original_tool_tag,Some(ToolTag::Scrub(_))){'); end=s.index('        if source.original_press_selected&&source.original_press_result.is_none(){',start)
block=s[start:end]; assert 'source.original_press_selected=true;' in block
assert 'finish_host(if source.original_press_selected{source.original_press_result}else{Some(false)},metadata,cx)' in s
gate=s[end:].split('{',1)[0].strip().removeprefix('if ')
cases=json.loads((ticket/'🔬️press-transition-cases.json').read_text()); literal=lambda x:'true' if x else 'false'
tests='\n'.join(f'check("{c["name"]}",{literal(c["tag"])},{c["fuel"]},{literal(c["fits"])},{literal(c["cancelled"])},{literal(c["yields"])},{literal(c["selected"])},{literal(c["moved"])});' for c in cases)
code='''#![allow(dead_code)]
mod semio_plugin_tool_machine { pub type ScrubTag = u64; }
#[derive(Clone,Copy)] enum ToolTag { Scrub(u64) }
#[derive(Default,Clone,Copy)] struct RetainedCloneProgress { copied_items:usize,copied_bytes:usize }
impl RetainedCloneProgress { fn fits(self,grant:bool)->bool{grant} }
struct Source { original_tool_tag:Option<ToolTag>,original_press_tag:Option<u64>,original_press_selected:bool,original_press_result:Option<bool> }
struct Cx { cancelled:bool, yields:bool, fuel:u64, fits:bool, charged:usize }
impl Cx { fn is_cancelled(&self)->bool{self.cancelled} fn should_yield(&self)->bool{self.yields} fn fuel_remaining(&self)->u64{self.fuel} fn retained_grant(&self)->bool{self.fits} fn consume_retained(&mut self,p:RetainedCloneProgress)->Result<(),()>{self.charged=p.copied_bytes;Ok(())} fn consume_fuel(&mut self,n:u64){self.fuel-=n;} }
fn transfer(source:&mut Source,cx:&mut Cx)->Result<Option<()>,()>{
''' + block + '''
 Ok(Some(())) }
fn check(name:&str,tag:bool,fuel:u64,fits:bool,cancelled:bool,yields:bool,selected:bool,moved:bool){let mut source=Source{original_tool_tag:tag.then_some(ToolTag::Scrub(7)),original_press_tag:None,original_press_selected:false,original_press_result:None};let mut cx=Cx{cancelled,yields,fuel,fits,charged:0};let answer=transfer(&mut source,&mut cx).unwrap();assert_eq!(source.original_press_selected,selected,"{} selection",name);assert_eq!(source.original_press_tag==Some(7),moved,"{} transfer",name);assert_eq!(source.original_tool_tag.is_some(),tag&&!moved,"{} custody",name);assert_eq!(cx.charged>0,moved,"{} funding",name);assert_eq!(answer.is_none(),tag,"{} yield",name);}
fn needs_settlement(source:&Source)->bool{ ''' + gate + ''' }
fn main(){
let mut source=Source{original_tool_tag:None,original_press_tag:None,original_press_selected:false,original_press_result:None};assert!(!needs_settlement(&source));source.original_press_selected=true;assert!(needs_settlement(&source));source.original_press_result=Some(true);assert!(!needs_settlement(&source));
''' + tests + '\nprintln!("6 transition cases and 3 settlement gates passed");}\n'
g=ticket/'🗑️generated'; g.mkdir(exist_ok=True); src=g/'press-check.rs';exe=g/'press-check';src.write_text(code)
rustc=root/'.cache/b/t/m/r/9171f2f44c79a380/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/rustc'
subprocess.run([str(rustc),str(src),'-o',str(exe)],check=True);subprocess.run([str(exe)],check=True)
