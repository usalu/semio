import{readFileSync,writeFileSync,renameSync}from"node:fs";
const n="🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned",t=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END";
function end(s,start){let depth=0,quote=false,escape=false;for(let i=start;i<s.length;i++){let c=s[i];if(quote){if(escape)escape=false;else if(c==="\\")escape=true;else if(c==='"')quote=false;continue}if(c==='"'){quote=true;continue}if(c==="(")depth++;if(c===")"&&--depth===0)return i}throw Error("call")}
function arity(s){let a=1,d=0,q=false,e=false;for(const c of s){if(q){if(e)e=false;else if(c==="\\")e=true;else if(c==='"')q=false;continue}if(c==='"')q=true;else if("([{".includes(c))d++;else if(")]}".includes(c))d--;else if(c===","&&d===0)a++}return a}
const rows=[
 ["🧺️set/🧪️tests/🦀️.rs","bank.work_remaining()"],
 ["🧺️set/🎟️work/🧪️tests/🦀️.rs","fuel"],
 ["🧳️original/🪞️projection/🧪️tests/🦀️.rs","bank.fuel"],
 ["🧳️original/🧪️tests/🦀️.rs","bank.fuel"],
 ["🧩️record/🧪️tests/🎮️receiving/🦀️.rs","bank.fuel"],
 ["🗂️map/🎟️work/🧪️tests/🦀️.rs","fuel"],
 ["🔗️sealed-shared-receiver/🧪️tests/📖️public/🦀️.rs","bank.work_remaining()"],
 ["🔄️map/🧪️tests/🦀️.rs","bank.fuel"],
 ["🔄️map/📦️box/🧪️tests/🦀️.rs","fuel"],
 ["📦️storage/🧪️tests/📖️initializers/🦀️.rs","*fuel"],
 ["📥️default/🎟️work/🧪️tests/🦀️.rs","bank.fuel"],
 ["🎟️work/🧪️tests/🦀️.rs","fuel"],
 ["🌳️value/🧱️bulk/🎟️work/🧪️tests/🦀️.rs","bank.fuel"],
 ["🌳️value/🔢️array/🎟️work/🧪️tests/🦀️.rs","fuel"],
 ["🌳️value/🔀️variant/🎟️work/🧪️tests/🦀️.rs","fuel"],
 ["🌳️value/📖️source/🧪️tests/🦀️.rs","bank.work_remaining()"],
 ["♻️header/🎟️work/🧪️tests/public.rs","bank.work"]
];
for(const[f,work]of rows){const p=n+"/"+f,o=readFileSync(p,"utf8");let s=o;let count=0,offset=0;
s=s.replace(/(\b(?:receiver|owner|matcher|cursor|r)\.begin_close\(\)(?:\.unwrap\(\))?;)/g,"$1let mut actual=None;");
const re=/\b(receiver|owner|matcher|cursor|r|source)\.close_step\(/g;
for(;;){re.lastIndex=offset;const m=re.exec(s);if(!m)break;const object=m[1],a=m.index,paren=s.indexOf("(",a),b=end(s,paren),args=s.slice(paren+1,b);if(arity(args)!==1){offset=b+1;continue}
const prefix=s.slice(Math.max(0,a-300),a);if(/fn close_step[^{}]*\{[^{}]*$/.test(prefix)){offset=b+1;continue}
const demand=object==="source"?"next_close_demand":"close_demands",quote=object==="source"?"next_close_work_demand":"close_work_demand";const body="4096";const next="crate::source_fixture::close_step(&mut actual,"+args+","+work+","+object+"."+demand+"("+body+"),"+object+"."+quote+"("+body+"),|g,a|"+object+".close_step(g,a))";s=s.slice(0,a)+next+s.slice(b+1);offset=a+next.length;count++}
for(const object of["receiver","owner","matcher","cursor","r","source"]){s=s.replaceAll("if "+object+".terminal_is_empty(){break}","if "+object+".terminal_is_empty()&&actual.is_none(){break}").replaceAll("if "+object+".terminal_is_empty(){return}","if "+object+".terminal_is_empty()&&actual.is_none(){return}").replaceAll("while !"+object+".terminal_is_empty(){","while !"+object+".terminal_is_empty()||actual.is_some(){");}
if(!count)continue;
if(!s.includes("let mut actual=None;"))throw Error("no retained actual slot "+f);
if(readFileSync(p,"utf8")!==o)throw Error("changed");const tmp=t+"/🗑️generated/test-markers-"+Math.random()+".tmp";writeFileSync(tmp,s);renameSync(tmp,p);console.log(JSON.stringify({file:f,calls:count}));
}
