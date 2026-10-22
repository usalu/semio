import json, os
import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🪞unmirror/🏃run"
V = "️"
E = lambda c: c + V
BASE = "✏" + V + "s/" + "\U0001F50C" + V + "plugins/p/"
def art(kind, name):
    return BASE + ("\U0001F5FF" + V + "artifacts" if kind == "artifact" else "\U0001F4C7" + V + "registry") + "/" + name
PK = "/" + "\U0001F4E6" + V + "packages/" + "\U0001F980" + V + "rust"
def crate(name, dir):
    return {"name": name, "ident": name.replace("-", "_"), "dir": dir, "root": dir + "/lib.rs"}
foo = crate("semio-s-artifact-foo", art("artifact", "foo") + PK)
foo_alpha = crate("semio-s-artifact-foo-mut-alpha", art("artifact", "foo") + "/schema/alpha" + PK)
reg = crate("semio-s-artifact-stdio", art("registry", "stdio") + PK)
reg_carved = crate("semio-s-artifact-stdio-core", art("registry", "stdio") + "/core" + PK)
bar = crate("semio-s-artifact-bar", art("artifact", "bar") + PK)
bar_lookalike = crate("semio-s-artifact-barrel", art("artifact", "bar") + "/x" + PK)
bar_elsewhere = crate("semio-s-artifact-bar-x", art("artifact", "other") + "/x" + PK)
fw = crate("semio-framework", "\U0001F9F0" + V + "framework/x")
fw_carved = crate("semio-framework-core", "\U0001F9F0" + V + "framework/x/core")
facade = [
 {"name": "an artifact crate with a carved crate below it is a facade", "crates": [foo, foo_alpha], "expected": [foo["ident"]]},
 {"name": "a registry crate with a carved crate below it is a facade", "crates": [reg, reg_carved], "expected": [reg["ident"]]},
 {"name": "an artifact crate without carved crates is not", "crates": [foo], "expected": []},
 {"name": "a name prefix without the dash is not a carved crate", "crates": [bar, bar_lookalike], "expected": []},
 {"name": "a carved crate in another directory does not count", "crates": [bar, bar_elsewhere], "expected": []},
 {"name": "crates outside the plugin tree are never facades", "crates": [fw, fw_carved], "expected": []},
 {"name": "several facades", "crates": [foo, foo_alpha, reg, reg_carved, bar, bar_lookalike, fw], "expected": [foo["ident"], reg["ident"]]},
 {"name": "no crates", "crates": [], "expected": []},
]
for f in facade:
    f["expected"] = sorted(f["expected"])
graphs = [
 {"name": "a DAG", "edges": {"a": ["b", "c"], "b": ["d"], "c": ["d"], "d": []}, "expected": []},
 {"name": "a self loop", "edges": {"a": ["a"]}, "expected": ["a -> a"]},
 {"name": "two nodes", "edges": {"a": ["b"], "b": ["a"]}, "expected": ["a -> b -> a"]},
 {"name": "a triangle", "edges": {"a": ["b"], "b": ["c"], "c": ["a"]}, "expected": ["a -> b -> c -> a"]},
 {"name": "a cycle entered from a tail reports only the cycle", "edges": {"t": ["a"], "a": ["b"], "b": ["a"]}, "expected": ["a -> b -> a"]},
 {"name": "two disjoint cycles", "edges": {"a": ["b"], "b": ["a"], "c": ["d"], "d": ["c"]}, "expected": ["a -> b -> a", "c -> d -> c"]},
 {"name": "edges to unknown nodes are ignored", "edges": {"a": ["x", "y"]}, "expected": []},
 {"name": "a diamond with a back edge reports the first cycle found", "edges": {"a": ["b", "c"], "b": ["d"], "c": ["d"], "d": ["a"]}, "expected": ["a -> b -> d -> a"]},
 {"name": "an empty graph", "edges": {}, "expected": []},
 {"name": "a long chain closing on itself", "edges": {"n0": ["n1"], "n1": ["n2"], "n2": ["n3"], "n3": ["n4"], "n4": ["n0"]}, "expected": ["n0 -> n1 -> n2 -> n3 -> n4 -> n0"]},
 {"name": "a cycle behind a shared node", "edges": {"r": ["x", "y"], "x": ["z"], "y": ["z"], "z": ["w"], "w": ["z"]}, "expected": ["z -> w -> z"]},
 {"name": "a cycle through a non-first start", "edges": {"p": ["q"], "q": ["r"], "r": ["q"], "s": ["p"]}, "expected": ["q -> r -> q"]},
]
cr = S.obj({"name": S.str, "ident": S.str, "dir": S.str, "root": S.str})
schema = S.obj({"version": S.const(1), "facades": S.arr(S.obj({"name": S.str, "crates": S.arr(cr), "expected": S.strs()}), 1), "graphs": S.arr(S.obj({"name": S.str, "edges": S.map(S.strs()), "expected": S.strs()}), 1)})
corpus = {"version": 1, "facades": facade, "graphs": graphs}
body = '''test("a facade is an artifact or registry crate with a carved crate named and placed below it",()=>{for(const row of corpus.facades)expect([...facadeIdents(row.crates)].sort(),row.name).toEqual(row.expected);});
const edgesOf=(row:{edges:Record<string,string[]>}):Map<string,string[]>=>new Map(Object.entries(row.edges));
for(const row of corpus.graphs)test("dependency cycles: "+row.name,()=>{expect(dependencyCycles(edgesOf(row))).toEqual(row.expected);});
const chain=(text:string):string[]=>text.split(" -> ");
test("graphlib agrees: a graph is acyclic exactly when no cycle is reported, every chain is a real cycle and every strongly connected component with a cycle is covered",()=>{
 for(const row of corpus.graphs){
  const graph=new Graph();for(const[name,next]of Object.entries<string[]>(row.edges)){graph.setNode(name);for(const target of next)graph.setEdge(name,target);}
  const reported=dependencyCycles(edgesOf(row));expect(reported.length===0,row.name).toBe(alg.isAcyclic(graph));
  for(const text of reported){const nodes=chain(text);expect(nodes[0],text).toBe(nodes.at(-1));for(let index=0;index+1<nodes.length;index+=1)expect(graph.hasEdge(nodes[index]!,nodes[index+1]!),text+" edge "+index).toBe(true);}
  for(const component of alg.tarjan(graph).filter(members=>members.length>1||graph.hasEdge(members[0]!,members[0]!)))expect(reported.some(text=>chain(text).every(node=>component.includes(node))),row.name+" "+component.join(",")).toBe(true);
 }
});
test("the reported cycles are deterministic and do not depend on a second call",()=>{for(const row of corpus.graphs)expect(dependencyCycles(edgesOf(row))).toEqual(dependencyCycles(edgesOf(row)));});
test("a graph of many nodes with many cycles terminates",()=>{
 const edges=new Map<string,string[]>();for(let index=0;index<300;index+=1)edges.set("n"+index,["n"+((index+1)%300),"n"+((index*7+3)%300)]);
 const found=dependencyCycles(edges);expect(found.length).toBeGreaterThan(0);const graph=new Graph();for(const[name,next]of edges)for(const target of next)graph.setEdge(name,target);expect(alg.isAcyclic(graph)).toBe(false);
});'''
ts = lib.ts_wrap(UNIT, "🏃️ Contract: the unmirror runner recognizes the facade crates of a plugin artifact or registry tree (those with carved crates below them) and reports the dependency cycles of the carved crates as `a -> b -> a` chains; the corpus is language-agnostic and graphlib (Tarjan strongly connected components) is the independent third-party oracle for acyclicity, chain validity and cycle coverage.",
 'import {Graph,alg} from "graphlib";\nimport {dependencyCycles,facadeIdents} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
