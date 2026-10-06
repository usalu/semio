import { readFileSync } from "node:fs";

test("Node readline independently decodes the native control leader vectors", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⌨️controls/🔣️.json",import.meta.url),"utf8"));
  const source="const {PassThrough}=require('node:stream'),{emitKeypressEvents}=require('node:readline'),fixture=JSON.parse(process.argv[1]),input=new PassThrough(),events=[];emitKeypressEvents(input);input.on('keypress',(_,key)=>events.push({key:key.name==='space'?' ':key.name,ctrl:Boolean(key.ctrl)}));input.end(Buffer.from(fixture.bytes));setImmediate(()=>console.log(JSON.stringify(events)));";
  const child=Bun.spawn(["node","-e",source,JSON.stringify(fixture)],{stdout:"pipe",stderr:"pipe"});
  const output=await new Response(child.stdout).text();expect(await child.exited,await new Response(child.stderr).text()).toBe(0);expect(JSON.parse(output)).toEqual(fixture.events);
});
