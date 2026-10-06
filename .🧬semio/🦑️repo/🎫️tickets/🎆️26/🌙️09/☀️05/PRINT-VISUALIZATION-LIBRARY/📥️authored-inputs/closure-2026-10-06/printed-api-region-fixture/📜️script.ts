import{readFileSync,writeFileSync}from"node:fs";const p="🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json",f=JSON.parse(readFileSync(p,"utf8"));
f.printed.regions=[
{id:"exclude-undelared-comment",regions:[{name:"Keys",scope:"neutral / theme",keys:[{name:"stroke",type:"string",declared:true},{name:"font",type:"string",declared:false}]}],expected:[{scope:"neutral / theme",keys:["stroke"]}]},
{id:"same-region-distinct-owners",regions:[{name:"Keys",scope:"neutral / set",keys:[{name:"fit",type:"boolean",declared:true}]},{name:"Keys",scope:"neutral / regression",keys:[{name:"fit",type:"string",declared:true}]}],expected:[{scope:"neutral / set",keys:["fit"]},{scope:"neutral / regression",keys:["fit"]}]}
];writeFileSync(p,JSON.stringify(f,null,2)+"\n");
