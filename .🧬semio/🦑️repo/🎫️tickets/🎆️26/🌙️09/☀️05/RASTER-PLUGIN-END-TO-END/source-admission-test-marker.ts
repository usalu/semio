import {readFileSync,writeFileSync} from "node:fs";
const path=process.argv[2]!;let source=readFileSync(path,"utf8");
function edit(before:string,after:string){if(!source.includes(before))throw new Error(`Missing anchored input: ${before}`);source=source.replaceAll(before,after);}
edit('struct Bank{left:G,fuel:u64}','struct Bank{left:G,fuel:u64,actual:Option<usize>}');
edit('fuel:s["fuel"].as_u64().unwrap()','actual:None,fuel:s["fuel"].as_u64().unwrap()');
edit('->Result<S,E>{assert!(turn.work<=self.fuel);','->Result<S,E>{if let Some(actual)=self.actual.take(){assert_eq!(actual,heap.0)}assert!(turn.work<=self.fuel);');
edit('owner.advance(g)','owner.advance(g,&mut bank.actual)');
edit('assert_eq!(owner.phase(),before);','assert_eq!(owner.phase(),before);assert!(bank.actual.is_none());');
writeFileSync(path,source);
