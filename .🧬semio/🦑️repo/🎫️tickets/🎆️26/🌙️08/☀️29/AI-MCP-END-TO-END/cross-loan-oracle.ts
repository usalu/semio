import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const root=import.meta.dirname,law=JSON.parse(readFileSync(`${root}/cross-loan-cases.json`,"utf8"));
const validate=new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(`${root}/cross-loan-cases.schema.json`,"utf8")));if(!validate(law))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:"),check=db.query("select ?1-?2-?3 as during, ?2+?4 as origin_spent, ?5 as next_spent, ?1-?2-?4-?8 as after, (?4+?6+?8=?3) as closes, (?5<=?7) as fresh_fits");
for(const row of law.cases)for(let i=0;i<3;i++){const got=check.get(row.origin[i],row.originActual[i],row.held[i],row.heldActual[i],row.nextFresh[i],row.unusedRelease[i],row.nextGrant[i],row.forfeited[i]) as {during:number,origin_spent:number,next_spent:number,after:number,closes:number,fresh_fits:number};if(got.during!==row.originAvailableWhileHeld[i]||got.origin_spent!==row.originObservedAfter[i]||got.next_spent!==row.nextObservedAfter[i]||got.after!==row.originAvailableAfter[i]||!got.closes||!got.fresh_fits)throw Error(JSON.stringify({row,axis:law.axes[i],got}));}
db.run("create table claims(producer integer, source integer, generation integer, purpose text, loan integer, claim integer, held integer, redeemed integer default 0, primary key(producer,source,generation,purpose,loan,claim))");
const id=law.identity,params=[id.producer,id.source,id.sourceGeneration,id.originPurpose,id.loanSerial,id.claimSerial];db.query("insert into claims values(?1,?2,?3,?4,?5,?6,48,0)").run(...params);
const other=law.concurrentIdentity,otherParams=[other.producer,other.source,other.sourceGeneration,other.originPurpose,other.loanSerial,other.claimSerial];db.query("insert into claims values(?1,?2,?3,?4,?5,?6,32,0)").run(...otherParams);
const redeem=db.query("update claims set redeemed=?7 where producer=?1 and source=?2 and generation=?3 and purpose=?4 and loan=?5 and claim=?6 and redeemed=0 and ?7<=held");
for(const foreign of [[8,11,4,'semantic',13,19],[7,12,4,'semantic',13,19],[7,11,5,'semantic',13,19],[7,11,4,'infrastructure',13,19],[7,11,4,'semantic',14,19],[7,11,4,'semantic',13,20]])if(redeem.run(...foreign,48).changes!==0)throw Error('foreign claim accepted');
if(redeem.run(...params,49).changes!==0||redeem.run(...params,48).changes!==1||redeem.run(...params,48).changes!==0)throw Error('claim amount or duplicate rejected incorrectly');
const prior=db.query("select redeemed from claims where producer=?1 and source=?2 and generation=?3 and purpose=?4 and loan=?5 and claim=?6").get(...params) as {redeemed:number};const replay=(amount:number)=>amount===prior.redeemed;if(!replay(48)||replay(47))throw Error('identical replay or changed replay misclassified');
if(redeem.run(...otherParams,32).changes!==1)throw Error('concurrent claim lost');const first=db.query("select redeemed from claims where producer=?1 and source=?2 and generation=?3 and purpose=?4 and loan=?5 and claim=?6").get(...params) as {redeemed:number};if(first.redeemed!==48)throw Error('concurrent claim changed original');
if(JSON.stringify(law.failure.retained)!==JSON.stringify([law.failure.primary,law.failure.receiving]))throw Error('failure causes lost');
console.log(`SQLite cross-loan conservation passed ${law.cases.length} cases and ${law.axes.length} axes`);
