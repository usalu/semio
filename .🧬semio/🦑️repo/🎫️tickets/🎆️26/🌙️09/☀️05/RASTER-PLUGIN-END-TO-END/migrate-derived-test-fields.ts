import {readFileSync,writeFileSync} from "node:fs";
const paths=["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🧪️tests/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🧪️tests/🦀️.rs"];
for(const path of paths){let text=readFileSync(path,"utf8");const before="preparation_receipt:Default::default(),";if(!text.includes(before))throw new Error(`missing derived fixture in ${path}`);text=text.replaceAll(before,"preparation_receipt:Default::default(),preparation_work:None,");writeFileSync(path,text);}
