"""Compile the production report token and scalar cursor and verify its JSON text."""
import json
from pathlib import Path

source = Path("✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🩺️validation-report/🦀️.rs").read_text()
fixture = json.loads(Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️original-validation-report.json").read_text())
print(r"""
#![allow(dead_code)]
extern crate self as semio_framework_3d_brep_representation;
extern crate self as semio_framework_value;
#[macro_export]macro_rules! artifact_retire_struct{($($t:tt)*)=>{}}
#[macro_export]macro_rules! artifact_retire_leaf{($($t:tt)*)=>{}}
mod topology{pub mod history{
 pub struct PersistentLabel(pub u64);
 impl PersistentLabel{
  pub fn decimal(self,buf:&mut [u8;20])->&str{
   let mut i=20;let mut n=self.0;
   loop{i-=1;buf[i]=b'0'+(n%10)as u8;n/=10;if n==0{break;}}
   std::str::from_utf8(&buf[i..]).unwrap()
  }
 }
}}
#[derive(Debug)]struct ValueError;
impl ValueError{fn invariant(_: &'static str)->Self{Self}}
struct Issue{entity:String,code:&'static str,message:String}
struct BodyValidationJob(Vec<Issue>);
impl BodyValidationJob{
 fn issue_count(&self)->usize {self.0.len()}
 fn issue(&self,i:usize)->Option<&Issue>{self.0.get(i)}
 fn issue_depth(&self,_:usize)->Result<usize,ValueError>{Ok(1)}
}
""")
print(source[source.index("enum ReportStage"):source.index("/// 📋️ Retains the selected shape")])
print(r"""
fn render(body:&BodyValidationJob)->String{
 let mut cursor=ReportText::default();
 let mut length=0;
 for writing in [false,true] {
  let mut out=String::with_capacity(if writing{length}else{0});
  loop {
   if let Some(value)=cursor.scalar(body).unwrap(){
    cursor.offset+=value.source_width;
    if writing {out.push_str(std::str::from_utf8(&value.bytes[..value.width]).unwrap());}
    else {length+=value.width;}
   }else{
    cursor.next(body.issue_count());
    if matches!(cursor.token,Token::Done){break;}
   }
  }
  if writing {assert_eq!(out.len(),length);return out;}
  cursor=Default::default();
 }
 unreachable!()
}
#[test]fn normal_and_escaped_report(){
""")

def literal(text):
    return 'r###"' + text + '"###'

expected = fixture["expected"]
finding = fixture["finding"]
print("assert_eq!(render(&BodyValidationJob(vec![]))," + literal(json.dumps(expected, separators=(",", ":"))) + ");")
issues = ",".join(
    "Issue{entity:" + literal(item["entity"]) + ".into(),code:" + literal(item["code"]) + ",message:" + literal(item["message"]) + ".into()}"
    for item in [finding, finding]
)
data = {"ok": False, "issueCount": 2, "issues": [finding, finding]}
text = json.dumps(data, separators=(",", ":"), ensure_ascii=False).replace(chr(92) + "n", chr(92) + "u000a")
print(f"assert_eq!(render(&BodyValidationJob(vec![{issues}])),{literal(text)});}}")
