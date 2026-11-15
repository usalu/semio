import{expect,test}from"bun:test";
import{classifyOwnersV1,emptyOwnerCostTableV1,estimateOwnerCostV1,orderByCostV1,ownerCostPolicyV1,ownerGrantV1,recordOwnerCostV1}from"../../🟦️.ts";
import{wantedPolicyV1}from"../../../🟦️.ts";

/** 💰️ Contract: heavy is a measured property of a named owner (its own recorded samples or a declared seed), never a consequence of a small grant or of the cost of its kind: at a grant of 0 MB only the measured heavy owners are refused, and the unknown ones are ordered by their estimate but stay light. */
const ZERO=ownerGrantV1(2048,wantedPolicyV1),SIXTEEN=ownerGrantV1(7924,wantedPolicyV1);
const names=[...Array.from({length:517},(_,at)=>`@semio-tech/p${at}`),"@semio-tech/s-dev-composition","@semio-tech/repo-lib","workspace","@semio-tech/framework-os-dev","@semio-tech/framework-os-kernel","@semio-tech/framework-rs"];

test("at a grant of 0 MB only the declared and the recorded heavy owners are heavy; 517 unknown projects are light",()=>{
 expect(ZERO).toEqual({footprintMegabytes:0,managedMegabytes:0});
 const kinds:Record<string,string>={workspace:"root:language:cargo","@semio-tech/p1":"language:typescript","@semio-tech/p2":"root:language:cargo"};
 const classes=classifyOwnersV1(names,name=>estimateOwnerCostV1(undefined,name,kinds[name]),ZERO);
 expect(classes.heavy.map(row=>row.name)).toEqual(["@semio-tech/s-dev-composition","@semio-tech/repo-lib","@semio-tech/framework-os-dev","@semio-tech/framework-os-kernel","@semio-tech/framework-rs","workspace"]);
 expect(classes.heavy.every(row=>row.source==="seed")).toBe(true);expect(classes.light).toHaveLength(517);
 expect(classes.heavy.map(row=>row.megabytes*ownerCostPolicyV1.footprintFactor).slice(0,2)).toEqual([27952,9652]);
});

test("an owner recorded or stopped as heavy is heavy from then on; the same owner at the grant of a 16 GB machine is heavy only above 1649 MB",()=>{
 const table=recordOwnerCostV1(recordOwnerCostV1(emptyOwnerCostTableV1(),{name:"@semio-tech/measured",kind:"project",megabytes:2000,seconds:9,rows:30,stopped:false,atMs:1}),{name:"@semio-tech/stopped",kind:"project",megabytes:1700,seconds:9,rows:30,stopped:true,atMs:2});
 const light=recordOwnerCostV1(table,{name:"@semio-tech/small",kind:"project",megabytes:1000,seconds:9,rows:30,stopped:false,atMs:3});
 const classes=classifyOwnersV1(["@semio-tech/measured","@semio-tech/stopped","@semio-tech/small","@semio-tech/unknown"],name=>estimateOwnerCostV1(light,name,"project"),SIXTEEN);
 expect(SIXTEEN.managedMegabytes).toBe(1649);
 expect(classes.heavy.map(row=>[row.name,row.source])).toEqual([["@semio-tech/measured","recorded"],["@semio-tech/stopped","recorded"]]);
 expect(classes.light).toEqual(["@semio-tech/small","@semio-tech/unknown"]);
});

test("the estimate of a kind or of the unknown still orders the serving machine's work",()=>{
 const estimate=(name:string)=>estimateOwnerCostV1(undefined,name,name==="a"?"language:typescript":name==="b"?"language:go":"project");
 expect(orderByCostV1(["b","c","a"],estimate)).toEqual(["a","c","b"]);
});
