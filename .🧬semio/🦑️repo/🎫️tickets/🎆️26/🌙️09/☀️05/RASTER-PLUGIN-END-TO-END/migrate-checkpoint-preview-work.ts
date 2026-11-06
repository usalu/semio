import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/📸️checkpoint-read/🦀️.rs";
const before="operation_adopted:false,preparation_receipt:Default::default(),plan:None";
const after="operation_adopted:false,preparation_receipt:Default::default(),preparation_work:None,plan:None";
const text=readFileSync(path,"utf8");if(!text.includes(before))throw new Error("missing checkpoint preview constructor");writeFileSync(path,text.replace(before,after));
