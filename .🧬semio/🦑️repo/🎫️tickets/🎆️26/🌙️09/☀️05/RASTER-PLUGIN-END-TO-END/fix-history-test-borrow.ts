import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/🧪️tests/🦀️.rs";
const before="let turn=ledger.advance_retained_read(key,&factory,grant(ledger.retained_read_demands(key).unwrap()));";
const after="let demand=ledger.retained_read_demands(key).unwrap();let turn=ledger.advance_retained_read(key,&factory,grant(demand));";
const text=readFileSync(path,"utf8");if(!text.includes(before))throw new Error("missing history test borrow");writeFileSync(path,text.replace(before,after));
