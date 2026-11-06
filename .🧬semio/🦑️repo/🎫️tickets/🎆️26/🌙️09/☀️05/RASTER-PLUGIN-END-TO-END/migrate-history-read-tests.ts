import { readFileSync, writeFileSync } from "node:fs";
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/🧪️tests/🦀️.rs";
let text = readFileSync(path, "utf8");
for (const name of ["advance_retained_read", "retain_read", "return_retained_read"]) text = text.replaceAll(`${name}_work(`, `${name}(`);
text = text.replace("let turn=ledger.advance_retained_read(key,&factory,grant(ledger.retained_read_demands(key).unwrap()));", "let demand=ledger.retained_read_demands(key).unwrap();let turn=ledger.advance_retained_read(key,&factory,grant(demand));");
for (const [before,after] of [
  ["ledger.return_retained_read(key,&mut returned,funded).unwrap()", "witness(ledger.return_retained_read(key,&mut returned,funded)).unwrap()"],
  ["ledger.advance_retained_read(key,&factory,denied).unwrap()", "witness(ledger.advance_retained_read(key,&factory,denied)).unwrap()"],
  ["ledger.advance_retained_read(key,&factory,funded).unwrap()", "witness(ledger.advance_retained_read(key,&factory,funded)).unwrap()"],
  ["ledger.retain_read(key,funded).unwrap()", "witness(ledger.retain_read(key,funded)).unwrap()"],
  ["observed(||ledger.return_retained_read(key,&mut returned,funded));match result", "observed(||witness(ledger.return_retained_read(key,&mut returned,funded)));match result"],
  ["observed(||ledger.advance_retained_read(key,&wrong,funded));assert!(result.is_err())", "observed(||witness(ledger.advance_retained_read(key,&wrong,funded)));assert!(result.is_err())"]
]) text = text.replaceAll(before,after);
const anchor = "fn corpus()->serde_json::Value";
if (!text.includes("fn witness<T>")) text = text.replace(anchor, "fn witness<T>(turn:semio_framework_value::RetainedWorkOutcome<T>)->Result<T,ValueError>{HISTORY_TEST_WORK.fetch_add(turn.work,std::sync::atomic::Ordering::Relaxed);turn.result}\nstatic HISTORY_TEST_WORK:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);\n" + anchor);
writeFileSync(path,text);
