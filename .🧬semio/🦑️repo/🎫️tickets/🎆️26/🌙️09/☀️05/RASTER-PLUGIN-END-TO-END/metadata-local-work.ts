import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/📚️metadata/🦀️.rs";
let text=readFileSync(path,"utf8");
for(const [before,after] of [
 ["*self.selected_redo=Some(semio_os_vcs::HistoryPageStack::empty());return Ok(receipt(d))", "*self.selected_redo=Some(semio_os_vcs::HistoryPageStack::empty());work=1;return Ok(receipt(d))"],
 ["self.redo_ready=true;return Ok(receipt(d))", "self.redo_ready=true;work=1;return Ok(receipt(d))"],
 ["target.try_push(source.clone()).map_err", "work=1;target.try_push(source.clone()).map_err"],
 ["*self.source=Some(source);self.ordinal=ordinal;return Ok(p)", "*self.source=Some(source);self.ordinal=ordinal;work=1;return Ok(p)"]
]){if(!text.includes(before))throw new Error(`missing metadata local action: ${before}`);text=text.replace(before,after)}writeFileSync(path,text);
