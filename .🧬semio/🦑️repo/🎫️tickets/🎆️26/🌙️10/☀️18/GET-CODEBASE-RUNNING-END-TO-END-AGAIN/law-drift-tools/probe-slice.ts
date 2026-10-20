import {pluginFamilyText} from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
const s=pluginFamilyText();
const [from,to,...needles]=JSON.parse(process.argv[2]!) as string[];
const a=s.indexOf(from!);console.log("from at",a);
const b=to===""?s.length:s.indexOf(to!,a);console.log("to at",b);
const body=s.slice(a,b);console.log("len",body.length);
for(const n of needles)console.log(body.includes(n)?"HAS ":"MISS",n.slice(0,150));
if(process.argv[3])console.log(body.slice(0,Number(process.argv[3])));
