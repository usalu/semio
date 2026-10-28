import type{LayoutMutation}from'../../🟦️.ts';
import type{LayoutSnapshot}from'../../../📸️snapshot/🟦️.ts';
import{QuestionDictionaryEqualityCursor,type QuestionEqualityGrant,type QuestionEqualityDemand}from'../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧾️dictionary/🟰️equal/🟦️.ts';
/** 🎚️ Actual optional dictionary fields bind their authored and baseline source paths before comparison. */
export class DataFieldsDecision{private readonly equality=new QuestionDictionaryEqualityCursor();private readonly authoredPath={};private readonly baselinePath={};
 private sources(m:LayoutMutation,b:LayoutSnapshot){if(!('ChangeDataFields'in m))throw Error('typed data-field command required');return[{value:m.ChangeDataFields.newFields??undefined,identity:m,projection:this.authoredPath},{value:b.dataFields,identity:b,projection:this.baselinePath}]as const;}
 demand(m:LayoutMutation,b:LayoutSnapshot,body:number):QuestionEqualityDemand{const[left,right]=this.sources(m,b);return this.equality.demand(left,right,body);}
 advance(m:LayoutMutation,b:LayoutSnapshot,g:QuestionEqualityGrant){const[left,right]=this.sources(m,b),step=this.equality.advance(left,right,g);return{progress:step.progress,done:step.equal!==undefined,unchanged:step.equal===true,error:undefined};}
 beginClose(){this.equality.beginClose();}
 closeDemand(body:number){return this.equality.closeDemand(body);}
 closeStep(g:QuestionEqualityGrant){return this.equality.closeStep(g);}
 terminalIsEmpty(){return this.equality.terminalIsEmpty();}
}
