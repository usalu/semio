import {readFileSync,writeFileSync} from "node:fs";
const root=process.argv[2]!;
function change(relative:string,edits:[string,string][]){const path=`${root}/${relative}`;let source=readFileSync(path,"utf8");for(const[before,after]of edits){if(!source.includes(before))throw new Error(`Missing input in ${relative}: ${before}`);source=source.replaceAll(before,after);}writeFileSync(path,source);}
change('🧪️testing/🦀️.rs',[
 ['let mut bound=work::Work::new();for','let mut bound=work::Work::new();let mut allocation_actual=None;for'],
 ['admission.advance(g)','admission.advance(g,&mut allocation_actual)'],
 ['let step=debit(&mut remaining,&mut cpu,g,turn,heap);','let step=debit(&mut remaining,&mut cpu,g,turn,heap);if let Some(actual)=allocation_actual.take(){assert_eq!(actual,heap.0)}']
]);
change('🧪️tests/🔐️query/🦀️.rs',[
 ['for _ in 0..cut.as_u64().unwrap(){let g=bank.grant(owner.next_demand().unwrap());bank.success(g,||owner.advance(g),|s|s.progress());}', 'let mut allocation_actual=None;for _ in 0..cut.as_u64().unwrap(){let g=bank.grant(owner.next_demand().unwrap());let(_,heap)=bank.success(g,||owner.advance(g,&mut allocation_actual),|s|s.progress());if let Some(actual)=allocation_actual.take(){assert_eq!(actual,heap.0)}}']
]);
change('🎟️work/🧪️tests/🦀️.rs',[
 ['let mut cpu=law["maximumCpuWork"].as_u64().unwrap();','let mut cpu=law["maximumCpuWork"].as_u64().unwrap();let mut allocation_actual=None;'],
 ['admission.advance(denied)','admission.advance(denied,&mut allocation_actual)'],
 ['assert_eq!(heap,(0,0));assert_eq!(admission.original_authority()', 'assert_eq!(heap,(0,0));assert!(allocation_actual.is_none());assert_eq!(admission.original_authority()'],
 ['admission.advance(g)','admission.advance(g,&mut allocation_actual)'],
 ['birth_work+=turn.work;','birth_work+=turn.work;if let Some(actual)=allocation_actual.take(){assert_eq!(actual,heap.0)}']
]);
change('🧪️law-bank/🦀️.rs',[
 ['let mut total=P::default();let mut output=None;','let mut total=P::default();let mut output=None;let mut allocation_actual=None;'],
 ['let(result,_)=self.run(g,||if admission.phase()', 'let(result,heap)=self.run(g,||if admission.phase()'],
 ['admission.advance(g)','admission.advance(g,&mut allocation_actual)'],
 ['|step|step.progress());let p=result.unwrap().progress();','|step|step.progress());if let Some(actual)=allocation_actual.take(){assert_eq!(actual,heap.0)}let p=result.unwrap().progress();'],
 ['let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||admission.advance(short));','let mut allocation_actual=None;let(turn,heap)=crate::allocation_testing::observe_retirement_allocations(||admission.advance(short,&mut allocation_actual));assert!(allocation_actual.is_none());']
]);
