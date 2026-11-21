import {readFileSync} from 'node:fs';
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts';
const {asset}=JSON.parse(readFileSync(process.argv[2], 'utf8'));
const args=process.argv.slice(3);
if(args[0]==="--all"){
 args.shift();
 for(const [d,r] of Object.entries(asset.representations) as [string,{status:string,validRange:{minDenominator:number,maxDenominator:number}}][]){
  if(r.status==="authored")for(let k=r.validRange.minDenominator;k<=r.validRange.maxDenominator;k++)args.push(d+"/"+k);
 }
}
if(!args.length)for(const d of [50,100,200,500]){
 const {status,validRange}=asset.representations[d];if(status!=='authored')continue;
 for(let i=0;i<=12;i++)args.push(d+'/'+Math.round(validRange.minDenominator+(validRange.maxDenominator-validRange.minDenominator)*i/12));
}
let measured = 0;
for (const arg of args) {
 const [d,k]=arg.split('/').map(Number);
 const {suitable,violations}=measureSuitability(asset,asset.representations[d],k,DEFAULT_PRINT_PROFILE);
 if(!suitable)process.exitCode=1;
 if(++measured % 5 === 0) Bun.gc(true);
 console.log(JSON.stringify({d,k,suitable,violations:violations.slice(0,8)}));
}
