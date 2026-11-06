import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/🧪️tests/🦀️.rs";
const text=readFileSync(path,"utf8");const before="issuance:ledger.read_issuance,";if(!text.includes(before))throw new Error("missing public issuance use");writeFileSync(path,text.replaceAll(before,"issuance:ledger.retained_issuance(),"));
