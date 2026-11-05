import {readFileSync,appendFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
const graph=JSON.parse(readFileSync(new URL("./🗑️generated/serve-graph.json",import.meta.url),"utf8"));
const root="@semio-tech/framework-os-dev",target="serve-shooting-react-prototype",edges=new Map();
for(const edge of graph.dependencies)(edges.get(edge.source)??(edges.set(edge.source,[]),edges.get(edge.source))).push(edge.target);
const paths=new Map([[root,[root]]]),queue=[root];
for(let i=0;i<queue.length;i++)for(const name of edges.get(queue[i])??[])if(graph.projects[name]&&!paths.has(name)){paths.set(name,[...paths.get(queue[i]),name]);queue.push(name);}
const out=["\n## Sealed Graph Dependency Evidence","", "- Projects: "+Object.keys(graph.projects).length,"- Root present: "+Boolean(graph.projects[root]),"- Serve target present: "+Boolean(graph.projects[root]?.targets[target]),"- Transitive project-edge closure: "+paths.size,"- Direct root project edges: "+JSON.stringify((edges.get(root)??[]).filter(name=>graph.projects[name]))];
for(const [name,path] of paths)if(/repo-lib|playbook-extension-procedural|shooting/.test(name))out.push("- Path to "+name+": "+path.join(" → "));
appendFileSync(new URL("./serve-control.md",import.meta.url),out.join("\n")+"\n");
console.log("Sealed graph inspection appended to "+fileURLToPath(new URL("./serve-control.md",import.meta.url)));
