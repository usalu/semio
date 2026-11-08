const fs=require("fs");const f=process.argv[2];const j=JSON.parse(fs.readFileSync(f,"utf8"));const t=j.texts;
const N={en:"Designing with Existing Stock",de:"Entwerfen mit Bestand"},A={en:"DwES",de:"EmB"};
const ids=["generator","koordinator","aggregator","energie","aussuchen","bearbeiten","verfolgen","statik"];
const out={};
for(const [k,v] of Object.entries(t)){
 if(k==="demonstrator.landing.title"){out["demonstrator.brand.name"]={en:N.en,de:N.de}; out["demonstrator.brand.abbreviation"]={en:A.en,de:A.de};}
 out[k]=v;
}
for(const id of ids){
 const lab=t[`demonstrator.brand.pane.${id}.label`];
 const w={},s={};
 for(const l of ["en","de"]){w[l]=`${N[l]} · ${lab[l]}`;s[l]=`${A[l]} · ${lab[l]}`}
 out[`demonstrator.brand.pane.${id}.windowTitle`]=w;out[`demonstrator.brand.pane.${id}.shortWindowTitle`]=s;
}
j.texts=out;
const lines=Object.entries(out).map(([k,v])=>`    ${JSON.stringify(k)}: ${JSON.stringify(v).replace(/":"/g,'": "').replace(/,"de"/,', "de"').replace(/^\{"/,'{ "').replace(/\}$/,' }')}`);
fs.writeFileSync(f,`{\n  "version": ${j.version},\n  "locales": ["en", "de"],\n  "texts": {\n${lines.join(",\n")}\n  }\n}\n`);
