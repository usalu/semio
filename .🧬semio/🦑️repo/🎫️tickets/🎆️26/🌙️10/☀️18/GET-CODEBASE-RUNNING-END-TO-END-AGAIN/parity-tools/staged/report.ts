import help from "../🔣️help.json" with {type:"json"};
import type {ParityReportV1,ParityRulesV1} from "../🟦️.ts";

/** 🌐️ The languages the verifier speaks, in preference order. */
export const parityLanguagesV1:readonly string[]=help.languages;

type Text=Readonly<Record<string,string>>;
const cell=(value:string|undefined):string=>(value??"").replaceAll("|","\\|").replaceAll("\n"," ");
const table=(header:readonly string[],rows:readonly (readonly string[])[]):string[]=>[`| ${header.join(" | ")} |`,`| ${header.map(()=>"---").join(" | ")} |`,...rows.map(row=>`| ${row.map(cell).join(" | ")} |`)];
const at=(text:Text,language:string):string=>text[language]??text[parityLanguagesV1[0]!]!;

/** 🏷️ Reads one label of the report in one language. */
export function parityLabelV1(name:keyof typeof help.labels,language:string):string{return at(help.labels[name],language);}

/** 📖️ Renders the usage of `verify build-parity` in one language. */
export function renderParityHelpV1(language:string):string{
 const command=help.command,width=Math.max(...command.flags.map(flag=>flag.name.length+("valueName" in flag?flag.valueName!.length+3:0)));
 const lines=[`repo ${command.name} - ${at(command.summary,language)}`,"",`${at(help.labels.usage,language)}:`,`  repo ${command.synopsis}`,"",`${at(help.labels.options,language)}:`];
 for(const flag of command.flags){const left=flag.name+("valueName" in flag?`=<${flag.valueName}>`:"");lines.push(`  ${left.padEnd(width+3)}  ${at(flag.description,language)}`);}
 lines.push("",`${at(help.labels.examples,language)}:`,...command.examples.flatMap(row=>[`  repo ${row.command.replace(/^repo /u,"")}`,`      ${at(row.description,language)}`]),"",...command.notes.map(note=>at(note,language)));
 return lines.join("\n")+"\n";
}

/** 📝️ Renders the report as Markdown tables in one language: the sample overview, every divergence and every documented normalization with how often it applied. */
export function renderParityReportV1(report:ParityReportV1,rules:ParityRulesV1,language:string,evidence?:string):string{
 const label=(name:keyof typeof help.labels)=>at(help.labels[name],language),code=(name:string)=>at((help.codes as Record<string,Text>)[name]??{[language]:name},language),status=(name:string)=>at((help.status as Record<string,Text>)[name]!,language),check=(name:string)=>at((help.checks as Record<string,Text>)[name]!,language);
 const lines=[`# ${label("title")}`,"",...table([label("samples"),status("equal"),status("diverged"),status("error"),label("divergences")],[[report.totals.samples,report.totals.equal,report.totals.diverged,report.totals.error,report.totals.divergences].map(String)]),""];
 lines.push(report.totals.diverged+report.totals.error===0?label("verdict-equal"):label("verdict-diverged"),"");
 if(evidence)lines.push(`${label("evidence")}: \`${evidence}\``,"");
 lines.push(`## ${label("sample")}`,"",...table([label("sample"),label("area"),label("target"),label("status")],report.samples.map(row=>[row.sample,row.area,row.target,status(row.status)])),"");
 const rows=report.samples.flatMap(sample=>sample.checks.flatMap(item=>item.divergences.map(row=>[sample.sample,check(row.check),item.phase,code(row.code),row.subject,row.nx??"",row.bazel??""])));
 lines.push(`## ${label("divergences")}`,"");
 if(rows.length)lines.push(...table([label("sample"),label("check"),label("phase"),label("code"),label("subject"),label("nx"),label("bazel")],rows),"");else lines.push(label("none"),"");
 const byId=new Map(rules.rules.map(rule=>[rule.id,rule]));
 lines.push(`## ${label("rules")}`,"",...table([label("rule"),label("applied"),label("justification")],report.rules.map(row=>[row.id,String(row.applied),at(byId.get(row.id)!.justification as Text,language)])),"");
 return lines.join("\n");
}
