const root=process.argv[2];
const{fleetReadStaleGenerationV1}=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🤝️prefetch/🟦️.ts");
const lines=[],write=process.stderr.write.bind(process.stderr);process.stderr.write=c=>{lines.push(String(c));return true;};
const dirty=[...Array.from({length:515},(_,i)=>"@semio-tech/p"+i),"@semio-tech/repo-lib","workspace"];
try{await fleetReadStaleGenerationV1({root:"/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/p14/repro-root",request:{generation:3,dirty,roots:{workspace:"."},structural:false,implementation:"a".repeat(64)},invocation:{control:{signal:new AbortController().signal}},language:"en",environment:{PATH:process.env.PATH,HOME:"/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/p14/repro-root",SEMIO_GRAPH_MEMORY_MB:process.argv[3]}});}
catch(e){}
process.stderr.write=write;console.log(lines.join("").slice(0,900));
