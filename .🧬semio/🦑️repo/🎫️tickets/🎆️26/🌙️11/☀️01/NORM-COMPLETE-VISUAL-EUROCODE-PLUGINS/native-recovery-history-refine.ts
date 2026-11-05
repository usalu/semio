import fs from 'node:fs';
const b='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/';
function replace(s:string,a:string,c:string){if(!s.includes(a))throw Error('Missing source: '+a.slice(0,120));return s.replace(a,c);}
{
 const p=b+'🧾️metadata/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=replace(s,'if let Some(status)=self.auxiliary_step(control,g)?{return Ok(status)}let child=','if let Some(status)=self.auxiliary_step(control,g)?{return Ok(status)}self.last_work=1;let child=');
 s=replace(s,'if self.partial.is_some(){return Ok(RetainedHistoryOpMetaStep::Blocked)}','if self.partial.is_some(){self.last_work=0;return Ok(RetainedHistoryOpMetaStep::Blocked)}');
 s=replace(s,'let Some((issued,p))=receipt else{return Ok(RetainedHistoryOpMetaStep::Blocked)}','let Some((issued,p))=receipt else{self.last_work=0;return Ok(RetainedHistoryOpMetaStep::Blocked)}');
 s=s.replace(/else\{return Ok\(RetainedHistoryOpMetaStep::Blocked\)\}/g,'else{self.last_work=0;return Ok(RetainedHistoryOpMetaStep::Blocked)}');
 fs.writeFileSync(p,s);
}
{
 const p=b+'🗂️file/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=replace(s,'let Some((issued,p))=receipt else{return Ok(RetainedHistorySourceStep::Blocked)}','let Some((issued,p))=receipt else{self.last_work=0;return Ok(RetainedHistorySourceStep::Blocked)}');
 fs.writeFileSync(p,s);
}
