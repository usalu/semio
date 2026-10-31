import{test,expect}from"bun:test";import corpus from"./rust-surface-vectors.json";const{rustSurfaceRowsV1}=require("./rust-surface-oracle.cjs");
for(const row of corpus)test("independent Rust surface "+row.id,async()=>{expect((await rustSurfaceRowsV1(row.source)).map(({name,visibility}:any)=>({name,visibility}))).toEqual(row.expected)});
