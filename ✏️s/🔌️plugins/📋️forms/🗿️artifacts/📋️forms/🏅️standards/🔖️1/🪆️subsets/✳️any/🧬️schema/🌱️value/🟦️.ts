/** 🌱️ Forms consumes the complete canonical intrinsic value domain. */
import type{DslValue}from"../🧬️mutations/🟦️.ts";
import{binary64Value}from"../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export{parseIntrinsicValue as parseFormsValue}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
/** 🔢️ Match the native intrinsic numeric projection without changing its stored identity. */
export function formsValueNumber(value:DslValue):number|undefined{switch(value.kind){case"unsigned":case"signed":return Number(value.value);case"float":return binary64Value(value.value);default:return undefined;}}
/** 🧮️ Compare intrinsic variants iteratively using native numeric and ordered-member semantics. */
export function formsValueEqual(left:DslValue,right:DslValue):boolean{
 const pending:[DslValue,DslValue][]=[[left,right]];while(pending.length){const[a,b]=pending.pop()!;if(a.kind!==b.kind){if((a.kind==="unsigned"&&b.kind==="signed"||a.kind==="signed"&&b.kind==="unsigned")&&a.value===b.value&&a.value>=0n&&a.value<=9223372036854775807n)continue;return false;}switch(a.kind){case"null":break;case"boolean":case"unsigned":case"signed":case"text":if(!("value"in b)||a.value!==b.value)return false;break;case"float":if(b.kind!=="float"||binary64Value(a.value)!==binary64Value(b.value))return false;break;case"bytes":if(b.kind!=="bytes"||a.value.length!==b.value.length)return false;for(let i=0;i<a.value.length;i++)if(a.value[i]!==b.value[i])return false;break;case"array":if(b.kind!=="array"||a.items.length!==b.items.length)return false;for(let i=a.items.length-1;i>=0;i--)pending.push([a.items[i]!,b.items[i]!]);break;case"object":if(b.kind!=="object"||a.members.length!==b.members.length)return false;for(let i=a.members.length-1;i>=0;i--){if(a.members[i]!.name!==b.members[i]!.name)return false;pending.push([a.members[i]!.value,b.members[i]!.value]);}break;}}return true;
}

