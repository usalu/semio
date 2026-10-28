SC = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts"
SD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔌️sdk/🟦️.ts"
EDITS = [
(SC, '''Readonly<{self:readonly Readonly<{fileset?:string;env?:string}>[];dependencies:readonly string[];projectInputs:number;dependencyFilesets:readonly string[]}>;}''', '''Readonly<{self:readonly Readonly<{fileset?:string;env?:string}>[];dependencies:readonly string[];projectInputs:readonly Readonly<{input:string;projects:readonly string[]}>[];dependencyFilesets:readonly string[]}>;}'''),
(SC, '''  const expanded=planner.inputs(graph,project,target,nx,input);if(expanded.projectInputs)return false;''', '''  const expanded=planner.inputs(graph,project,target,nx,input);
  for(const row of expanded.projectInputs)for(const owner of row.projects)if(!graph.nodes[owner]||!await inputs(owner,target,row.input))return false;'''),
(SD, '''projectInputs:expanded.projectInputs.length,''', '''projectInputs:expanded.projectInputs.map((row:any)=>({input:row.input,projects:[...row.projects]})),'''),
]
