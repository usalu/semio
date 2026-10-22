import lib
from lib import S
UNIT = "🏗authoring/📦crate-tree/🪞unmirror/🔁cycle-memo"
def closure(edges):
    nodes = set(edges) | {t for ts in edges.values() for t in ts}
    out = {}
    for n in nodes:
        seen, stack = set(), [n]
        while stack:
            x = stack.pop()
            if x in seen:
                continue
            seen.add(x)
            stack.extend(edges.get(x, []))
        out[n] = sorted(seen)
    return out
graphs = {
 "a chain": {"a": ["b"], "b": ["c"], "c": []}, "a two-node cycle": {"a": ["b"], "b": ["a"]}, "a diamond": {"a": ["b", "c"], "b": ["d"], "c": ["d"], "d": []}, "a cycle with a tail": {"t": ["a"], "a": ["b"], "b": ["c"], "c": ["a", "z"], "z": []},
 "a self loop": {"a": ["a", "b"], "b": []}, "two cycles sharing a node": {"a": ["b"], "b": ["c", "d"], "c": ["a"], "d": ["a"]}, "an isolated node": {"a": []}, "a larger knot": {"1": ["2"], "2": ["3", "5"], "3": ["4"], "4": ["2", "6"], "5": ["6"], "6": ["1"]},
}
corpus = {"version": 1, "graphs": [{"name": n, "edges": g, "reach": closure(g), "orders": [sorted(closure(g)), sorted(closure(g), reverse=True)]} for n, g in graphs.items()], "property": {"runs": 300, "seed": 20261021}}
schema = S.obj({"version": S.const(1), "graphs": S.arr(S.obj({"name": S.str, "edges": S.map(S.strs()), "reach": S.map(S.strs()), "orders": S.arr(S.strs())}), 1), "property": S.obj({"runs": S.int, "seed": S.int})})
body = '''const reach=(memo:CycleMemo,edges:Record<string,string[]>,counts?:Record<string,number>)=>{
 const visit=(node:string):Set<string>=>memo.compute<Set<string>>(node,()=>new Set(),()=>{if(counts)counts[node]=(counts[node]??0)+1;const out=new Set([node]);for(const next of edges[node]??[])for(const name of visit(next))out.add(name);return out;});
 return visit;
};
const brute=(edges:Record<string,string[]>,start:string):string[]=>{const seen=new Set<string>(),stack=[start];while(stack.length){const node=stack.pop()!;if(seen.has(node))continue;seen.add(node);stack.push(...(edges[node]??[]));}return[...seen].sort();};
for(const row of corpus.graphs)test("reachability through the memo: "+row.name,()=>{
 for(const order of row.orders){const memo=new CycleMemo(),visit=reach(memo,row.edges);for(const node of order)expect([...visit(node)].sort(),row.name+" "+node+" in order "+order.join()).toEqual(row.reach[node]!);}
 for(const node of Object.keys(row.reach)){const visit=reach(new CycleMemo(),row.edges);expect([...visit(node)].sort(),row.name+" fresh "+node).toEqual(row.reach[node]!);}
 expect(Object.fromEntries(Object.keys(row.reach).map(node=>[node,brute(row.edges,node)]))).toEqual(row.reach);
});
test("an acyclic computation runs each node once and answers repeats from the memo",()=>{
 const edges=corpus.graphs.find(row=>row.name==="a diamond")!.edges,counts:Record<string,number>={},visit=reach(new CycleMemo(),edges,counts);visit("a");visit("a");visit("b");expect(counts).toEqual({a:1,b:1,c:1,d:1});
});
test("a node inside a cycle is final once its cycle head has finished only when it never saw a cut",()=>{
 const edges=corpus.graphs.find(row=>row.name==="a cycle with a tail")!.edges,counts:Record<string,number>={},visit=reach(new CycleMemo(),edges,counts);visit("t");expect(counts["z"]).toBe(1);expect(counts["t"]).toBe(1);visit("t");expect(counts["t"]).toBe(1);
});
test("a failing computation does not poison the memo",()=>{
 const memo=new CycleMemo();let attempts=0;
 expect(()=>memo.compute("k",()=>"cut",()=>{attempts+=1;throw new Error("boom");})).toThrow("boom");expect(memo.compute("k",()=>"cut",()=>{attempts+=1;return "ok";})).toBe("ok");expect(attempts).toBe(2);expect(memo.compute("k",()=>"cut",()=>"again")).toBe("ok");
});
test("random graphs: every query order answers the true reachable set (fast-check)",()=>{
 const names=["a","b","c","d","e","f"],edgeList=fc.array(fc.tuple(fc.constantFrom(...names),fc.constantFrom(...names)),{maxLength:14});
 fc.assert(fc.property(edgeList,fc.shuffledSubarray(names,{minLength:1,maxLength:6}),fc.shuffledSubarray(names,{minLength:1,maxLength:6}),(pairs,first,second)=>{
  const edges:Record<string,string[]>={};for(const[from,to]of pairs)(edges[from]??=[]).push(to);
  const memo=new CycleMemo(),visit=reach(memo,edges);
  for(const node of[...first,...second])expect([...visit(node)].sort(),node).toEqual(brute(edges,node));
  for(const node of names)expect([...reach(new CycleMemo(),edges)(node)].sort(),"fresh "+node).toEqual(brute(edges,node));
 }),{numRuns:corpus.property.runs,seed:corpus.property.seed});
});'''
ts = lib.ts_wrap(UNIT, "🔁️ Contract: the cycle memo answers a lookup that reaches a computation still in progress with the fallback, keeps results that never saw such a cut final and reuses the others only while the computation they cut against runs, so every query order over a cyclic graph yields the true reachable set; the corpus is language-agnostic and fast-check (seeded random graphs and query orders) plus a brute-force DFS are the independent oracle.",
 'import fc from "fast-check";\nimport {CycleMemo} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts)
