const fs=require("fs"),path=require("path");
let root=__dirname;
while(!fs.existsSync(path.join(root,"🧰️framework/🛍️products/📓️print")))root=path.dirname(root);
const base=path.join(root,"🧰️framework/🛍️products/📓️print/🖋️latex");
const window=fs.readFileSync(path.join(base,"semio-window.sty"),"utf8");
const core=fs.readFileSync(path.join(base,"semio-core.sty"),"utf8");
const contract=JSON.parse(fs.readFileSync(path.join(__dirname,"silhouette-style.case.json"),"utf8"));
const slash=String.fromCharCode(92);
for(const kind of contract.kinds){
  const define=slash+"semio_window_kind_define:nnnnn { "+kind.counter+" } { "+kind.kind+" } { "+kind.environment+" } { "+kind.tier+" } { "+kind.list+" }";
  if(!window.includes(define))throw Error("Native kind missing: "+kind.environment);
  for(const [locale,label,plural] of [["en",kind.en,kind.pluralEn],["de",kind.de,kind.pluralDe]]){
    if(!core.includes(slash+"semio@kind@register@"+locale+"{"+kind.kind+"}{"+label+"}"))throw Error("Localized label missing");
    if(!core.includes(slash+"semio@kind@register@plural@"+locale+"{"+kind.kind+"}{"+plural+"}"))throw Error("Localized plural missing");
  }
}
if(!window.includes(slash+"bool_set_true:N "+slash+"l_semio_window_title_set_bool"))throw Error("Explicit empty title must retain its authored intent");
if(!window.includes(slash+"bool_set_false:N "+slash+"l_semio_window_title_set_bool"))throw Error("Omitted title must retain existing default behavior");
console.log("PASS native kind registration, localized labels, and three title semantics");
