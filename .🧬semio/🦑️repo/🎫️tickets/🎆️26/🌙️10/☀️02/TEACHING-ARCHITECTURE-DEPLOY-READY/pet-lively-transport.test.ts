import {test,expect} from "bun:test";
import _ from "lodash";
import ts from "typescript";
import {readFileSync} from "node:fs";
import fixture from "./pet-lively-transport.json";
const path="🎓️teaching/🏛️architecture/📚️course/🧪️tests/🐕️pet-walk/🧰️pet-kit/🟦️.ts",source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="carry");
if(!declaration)throw Error("Actual carry helper required");
const code=ts.transpileModule(declaration.getText(source),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText.replace("export ",""),middleOf=(body:typeof fixture.body)=>({x:_.mean([body.left,body.right]),y:_.mean([body.top,body.bottom])}),framesPass=()=>{};
for(const row of fixture.cases)test("actual carry transport "+row.mode,async()=>{
 const moves:{x:number;y:number;steps?:number}[]=[],frames:number[]=[],buttons:string[]=[],page={mouse:{async move(x:number,y:number,options?:{steps:number}){moves.push({x,y,...options});},async down(){buttons.push("down");}},async evaluate(fn:unknown,count:number){expect(fn).toBe(framesPass);frames.push(count);},locator(){return{async getAttribute(){return"grab";}};}},carry=new Function("eventually","bodies","middleOf","CURSOR","ANSWER_MS","framesPass",code+";return carry;")(async(_page:unknown,probe:()=>Promise<unknown>,done:(value:unknown)=>boolean)=>{if(!done(await probe()))throw Error("Genuine model pickup precondition");},async()=>[fixture.body],middleOf,"data-pet-cursor",6000,framesPass);
 if(row.mode==="default")await carry(page,fixture.body,fixture.target);else await carry(page,fixture.body,fixture.target,row.mode);
 expect(buttons).toEqual(["down"]);expect(moves.length).toBe(row.moves);expect(frames.length).toBe(row.frameCalls);expect(_.sum(frames)).toBe(row.frames);
 const from=middleOf(fixture.body);expect(moves[0]).toEqual(from);expect(moves.at(-1)).toEqual(fixture.target);
 if(row.mode!=="continuous")expect(moves.slice(1,25)).toEqual(_.range(1,25).map(step=>({x:from.x+(fixture.target.x-from.x)*step/24,y:from.y+(fixture.target.y-from.y)*step/24})));
 else{expect(moves[1]).toEqual({...fixture.target,steps:24});expect(frames).toEqual([10]);}
});
