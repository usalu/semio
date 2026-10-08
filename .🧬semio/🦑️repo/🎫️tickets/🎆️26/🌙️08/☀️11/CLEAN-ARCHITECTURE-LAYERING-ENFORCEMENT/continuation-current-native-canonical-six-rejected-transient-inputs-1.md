# Rejected Intermediate Transient Capacity Authority

The authored adjacent generic service passed one current allocator-accounting law, but its outer RetireOwned transfer discarded admission redemption metadata. It is removed from the defining runtime API. The feature is implemented by the completed JSON frame inline authority, tested by its own unchanged-budget neutral allocator and Serde law. All180 original Value laws remain; this intermediate newly authored law is preserved here as evidence rather than retained as an unused runtime feature.

## Intermediate service source

```rust
//! ♻️ Typed temporary retirement capacity belongs to the same decoder operation as its source.
use super::{NativeDecodeControl, ValueError, ValueRefusalKind};
use crate::{retirement::{RetireOwned, RetirementCursor, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneStep}};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_OPERATION: AtomicUsize = AtomicUsize::new(1);

/// 🎟️ Retains original typed ownership and its exact born scaffold admission until terminal close.
pub struct NativeDecodeRetirement<T: RetireOwned> { owner: ControlledRetirement<T>, capacity: usize, operation_id: usize }

impl<T: RetireOwned> NativeDecodeRetirement<T> {
    pub fn new(value: T) -> Result<Self, (ValueError, T)> { ControlledRetirement::new(value).map(|owner| Self { owner, capacity: 0, operation_id: 0 }) }
    pub fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
    pub fn next_copy_byte_demand(&self) -> usize { self.owner.next_copy_byte_demand() }
    pub fn next_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(maximum_copy_bytes) }
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    pub fn step(&mut self, grant: RetainedCloneGrant, control: &mut NativeDecodeControl<'_>) -> Result<RetainedCloneStep, ValueError> {
        if self.operation_id != 0 && self.operation_id != control.operation_id { return Err(refusal("transient retirement belongs to a different decode operation")); }
        let capacity = if grant.maximum_items == 0 || grant.maximum_depth < self.next_depth_demand()? { 0 } else { grant.maximum_capacity_bytes.min(self.next_capacity_byte_demand(grant.maximum_copy_bytes)?) };
        let total = control.owned_bytes().checked_add(capacity).filter(|total| *total <= control.maximum_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "transient retirement exceeds the decoder ownership limit"))?;
        if capacity != 0 {
            control.checkpoint()?;
            if control.operation_id == 0 { control.operation_id = NEXT_OPERATION.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1)).map_err(|_| refusal("transient decoder operation identity exhausted"))?; }
            self.operation_id = control.operation_id;
            control.transient_bytes = total - control.owned_bytes;
        }
        let step = match self.owner.step(grant) { Ok(step) => step, Err(error) => { control.transient_bytes -= capacity; return Err(error); } };
        let born = step.progress().retained_capacity_bytes;
        if born > capacity { return Err(refusal("transient retirement exceeded its exact admitted constructor demand")); }
        control.transient_bytes -= capacity - born;
        self.capacity = self.capacity.checked_add(born).ok_or_else(|| refusal("transient retirement capacity overflow"))?;
        if self.terminal_is_empty() {
            control.transient_bytes = control.transient_bytes.checked_sub(self.capacity).ok_or_else(|| refusal("terminal retirement lost its born scaffold admission"))?;
            self.capacity = 0;
        }
        Ok(step)
    }
}

impl<T: RetireOwned> RetireOwned for NativeDecodeRetirement<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { self.owner.retirement() }
    fn retirement_birth_bytes(&self) -> Option<usize> { self.owner.retirement_birth_bytes() }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

fn refusal(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }

```

## Intermediate authored law

```rust
#[test]
fn native_decode_transient_retirement_preserves_original_admission_and_terminal_refund(){
    use crate::{NativeDecodeRetirement,retained_clone::RetainedCloneGrant};
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    let values:Vec<String>=fixture["values"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect();let source=values.capacity()*std::mem::size_of::<String>()+values.iter().map(String::capacity).sum::<usize>();
    let observed=Cell::new(0);let mut callback=|progress:NativeDecodeProgress|{observed.set(progress.owned_bytes);true};let mut control=NativeDecodeControl::new(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut callback);control.charge(source).unwrap();let mut owner=NativeDecodeRetirement::new(values).unwrap_or_else(|(_,value)|{std::mem::forget(value);panic!("original String vector has a typed authority")});let mut events=Vec::new();let mut born=0;let mut released=0;let mut resumed=false;
    for _ in 0..65536{
        if owner.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:owner.next_capacity_byte_demand(3).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let before=control.owned_bytes();let mut denied=grant;denied.maximum_items=0;assert_eq!(owner.step(denied,&mut control).unwrap().progress().retained_capacity_bytes,0);assert_eq!(control.owned_bytes(),before);
        let progress=owner.step(grant,&mut control).unwrap().progress();born+=progress.retained_capacity_bytes;released+=progress.released_bytes;events.push(serde_json::json!({"born":progress.retained_capacity_bytes,"terminal":owner.terminal_is_empty(),"held":control.transient_owned_bytes()}));control.checkpoint().unwrap();assert_eq!(observed.get(),control.owned_bytes());assert_eq!(control.owned_bytes(),source+control.transient_owned_bytes());
        if control.transient_owned_bytes()!=0&&!resumed{let mut foreign_callback=|_|true;let mut foreign=NativeDecodeControl::new(65536,&mut foreign_callback);assert!(matches!(owner.step(grant,&mut foreign).unwrap_err().kind,ValueRefusalKind::InvariantViolated));assert_eq!(foreign.owned_bytes(),0);let receipt=control.pause().unwrap();control=NativeDecodeControl::resume(receipt,&mut callback).unwrap();resumed=true;}
    }
    assert!(owner.terminal_is_empty()&&resumed);assert!(born>0);assert_eq!(released,source+born);assert_eq!(control.owned_bytes(),source);assert_eq!(control.transient_owned_bytes(),0);
    let script="import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=new Database(':memory:');db.run('CREATE TABLE held(bytes INTEGER)');const output=x.events.map(e=>{db.run('INSERT INTO held VALUES(?)',e.born);const held=e.terminal?0:db.query('SELECT COALESCE(SUM(bytes),0) AS bytes FROM held').get().bytes;if(e.terminal)db.run('DELETE FROM held');return held});console.log(JSON.stringify(output));";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"events":events}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success());assert_eq!(events.iter().map(|event|event["held"].as_u64().unwrap()).collect::<Vec<_>>(),serde_json::from_slice::<Vec<u64>>(&output.stdout).unwrap());
    eprintln!("[DEBUG] native transient admission original={source} born={born} physical={released} semantic admission survives terminal; SQLite verifies every held scaffold receipt and pause preserves operation identity");
}

```
