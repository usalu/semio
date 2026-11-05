import { readFileSync } from "node:fs";
const fixture=JSON.parse(readFileSync(process.argv[2],"utf8"));
const bitsBuffer=new ArrayBuffer(8),bitsView=new DataView(bitsBuffer);
const bits=x=>{bitsView.setFloat64(0,x,false);return bitsView.getBigUint64(0,false).toString(16).padStart(16,"0")};
function reduce(word){
 const match=/^([+-]?)(?:(\d+)(?:\.(\d*))?|\.(\d+))(?:[eE]([+-]?\d+))?$/.exec(word);
 if(!match)return null;
 const fraction=match[3]??match[4]??"",digits=((match[2]??"")+fraction).replace(/^0+/,""),negative=match[1]==="-";
 if(digits.length===0)return negative?"-0":"0";
 const count=Math.min(fixture.maximumSignificantDigits,digits.length);
 let coefficient=digits.slice(0,count),power=BigInt(match[5]??"0")-BigInt(fraction.length)+BigInt(digits.length-count);
 const scientific=power+BigInt(count)-1n;
 if(scientific>308n)return negative?"-Infinity":"Infinity";
 if(scientific< -324n)return negative?"-0":"0";
 if(/[1-9]/.test(digits.slice(count))){coefficient+="5";power-=1n}
 const normalized=(negative?"-":"")+coefficient+"e"+power;
 if(normalized.length>fixture.maximumNormalizedBytes)throw Error("normalized word exceeded proved bound");
 return normalized;
}
let cases=0,longest=0;
function check(word){
 const normalized=reduce(word);
 if(normalized===null)throw Error("proof word was rejected");
 const original=Number(word),reduced=Number(normalized);
 if(bits(original)!==bits(reduced))throw Error(JSON.stringify({length:word.length,normalized,original:bits(original),reduced:bits(reduced)}));
 cases++;longest=Math.max(longest,word.length);
}
function family(n,d,tail){
 const coefficient=d>=0?n*5n**BigInt(d):n*2n**BigInt(-d),power=d>=0?-d:0,text=coefficient.toString();
 check(text+"e"+power);
 check(text+"0".repeat(tail)+"5e"+(power-tail-1));
 check((coefficient-1n).toString()+"9".repeat(tail+1)+"e"+(power-tail-1));
 check("0".repeat(tail)+text+"e"+power);
 check("-"+text+"0".repeat(tail)+"e"+(power-tail));
}
for(const item of fixture.midpoints)for(const tail of fixture.tailZeroes)family(BigInt(item.numerator),item.denominatorPower,tail);
for(const word of fixture.serdeFiniteWords)check(word);
let seed=0x2a5f8123d9c4b7e1n;
function next(){seed^=seed<<13n;seed^=seed>>7n;seed^=seed<<17n;seed&=(1n<<64n)-1n;return seed}
for(let index=0;index<2000;index++)family((next()&((1n<<54n)-1n))|1n,Number(next()%2047n)-971,Number(next()%1025n));
process.stdout.write("[DEBUG] "+JSON.stringify({cases,longestOriginalBytes:longest,maximumNormalizedBytes:fixture.maximumNormalizedBytes,oracle:"V8 Number on exact BigInt midpoint families",result:"pass"})+"\n");
