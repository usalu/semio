import{afterAll}from"bun:test";let live=true;export const active=()=>live;console.log("[DEBUG] lifetime-born");afterAll(()=>{live=false;console.log("[DEBUG] lifetime-ended");});
