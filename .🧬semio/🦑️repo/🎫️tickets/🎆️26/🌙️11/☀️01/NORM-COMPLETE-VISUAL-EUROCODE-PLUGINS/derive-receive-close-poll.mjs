const paths=["🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs","🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🔀️enum/🦀️.rs"];
function matching(s,start){let level=0,quoted=false,escape=false;for(let i=start;i<s.length;i++){const c=s[i];if(quoted){if(escape)escape=false;else if(c==="\\")escape=true;else if(c==='"')quoted=false;continue}if(c==='"'){quoted=true;continue}if(c==="{")level++;else if(c==="}"&&--level===0)return i}throw Error("unclosed generated function")}
for(const p of paths){let s=await Bun.file(p).text();for(const name of["close_step","close_original_birth_step","close_original_step"]){const at=s.indexOf("fn "+name+"(");if(at<0)throw Error("missing "+name);const open=s.indexOf("{",at),end=matching(s,open);let signature=s.slice(at,open).replace("->#value::OriginalRetirementOutcome","->#value::OriginalRetirementPoll");let body=s.slice(open+1,end);
if(name==="close_step"){
body=body.replace("let result:Result<#value::RetainedCloneStep,#value::OriginalRetirementFailure>=","let result:Result<#value::RetainedCloneStep,#value::RetirementPollExit>=");
body=body.replace(/let turn=([^;]+);work=turn\.work/g,(all,expr)=>/\.close_step\(|\.step_granted\(|::close_one\(/.test(expr)?"let turn=#value::RetirementPollExit::settled("+expr+")?;work=turn.work":all);
body=body.replace(/\.map\(\|s\|#value::RetainedCloneStep::Progress\(s\.progress\(\)\)\)(?!\.map_err)/g,".map(|s|#value::RetainedCloneStep::Progress(s.progress())).map_err(::std::convert::Into::into)");
body=body.replace(/\.map\(\|s\|s\.unwrap_or\(#value::RetainedCloneStep::Progress\(Default::default\(\)\)\)\)(?!\.map_err)/g,".map(|s|s.unwrap_or(#value::RetainedCloneStep::Progress(Default::default()))).map_err(::std::convert::Into::into)");
body=body.replace("#value::RetainedWorkOutcome{result,work}","#value::RetirementPollExit::finish(result,work)");
body=body.replace("self.partial.release(grant)","self.partial.release(effect)");
}else{let cursor=0;while((cursor=body.indexOf("#value::RetainedWorkOutcome{",cursor))>=0){const brace=body.indexOf("{",cursor),last=matching(body,brace);body=body.slice(0,cursor)+"#value::RetainedWorkPoll::Complete("+body.slice(cursor,last+1)+")"+body.slice(last+1);cursor=last+35}}
s=s.slice(0,at)+signature+"{"+body+"}"+s.slice(end+1)}
s=s.replace(/let turn=([^;]+);work=turn\.work;return turn\.result/g,(all,expr)=>/\.close_step\(&mut effect\.reborrow/.test(expr)?"let turn=#value::RetirementPollExit::settled("+expr+")?;work=turn.work;return turn.result":all);
await Bun.write(p,s)}
const facade="🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs";let s=await Bun.file(facade).text();const a="pub use semio_framework_value_retirement::retirement;";if(!s.includes(a))throw Error("facade retirement");s=s.replace(a,a+"\npub use semio_framework_value_retirement::retirement_contract::RetirementPollExit;");await Bun.write(facade,s);
