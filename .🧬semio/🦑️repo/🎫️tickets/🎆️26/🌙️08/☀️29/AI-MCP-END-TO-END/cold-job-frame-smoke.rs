//! 🧪️ Runs the authored receiver with a finite simulated StepContext.
extern crate self as semio_framework_actor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValueRefusalKind { InvalidValue, OwnershipLimit, AllocationFailed }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueError(ValueRefusalKind);
impl ValueError {
    fn literal(kind:ValueRefusalKind,_:&'static str)->Self { Self(kind) }
}
type OriginalTurnFailure=ValueError;
fn invalid()->ValueError { ValueError(ValueRefusalKind::InvalidValue) }

#[derive(Clone,Copy)]
struct Grant { maximum_depth:usize, maximum_copy_bytes:usize, maximum_capacity_bytes:usize, maximum_items:usize }
#[derive(Default)]
struct RetainedCloneProgress { copied_items:usize, copied_bytes:usize, retained_capacity_bytes:usize }
struct StepContext<'a> { grant:Grant, fuel:u64, _owner:std::marker::PhantomData<&'a ()> }
impl StepContext<'_> {
    fn retained_grant(&self)->Grant{self.grant}
    fn fuel_remaining(&self)->u64{self.fuel}
    fn should_yield(&self)->bool{self.fuel==0}
    fn is_cancelled(&self)->bool{false}
    fn consume_fuel(&mut self,units:u64){self.fuel=self.fuel.checked_sub(units).unwrap();}
    fn consume_retained(&mut self,spent:RetainedCloneProgress)->Result<(),ValueError>{
        self.grant.maximum_items=self.grant.maximum_items.checked_sub(spent.copied_items).unwrap();
        self.grant.maximum_copy_bytes=self.grant.maximum_copy_bytes.checked_sub(spent.copied_bytes).unwrap();
        self.grant.maximum_capacity_bytes=self.grant.maximum_capacity_bytes.checked_sub(spent.retained_capacity_bytes).unwrap();
        Ok(())
    }
}
fn charge(cx:&mut StepContext<'_>,_:bool,bytes:usize)->Result<bool,ValueError>{
    if cx.grant.maximum_items==0||cx.grant.maximum_copy_bytes<bytes||cx.should_yield(){return Ok(false)}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
    cx.consume_fuel(1);
    Ok(true)
}

#[derive(Debug)]
pub struct OriginalJobSource { job:u64, kind:String, input:Option<Vec<u8>>, checkpoint:Option<Vec<u8>> }
enum ActorInput { Job(OriginalJobSource), Poll }
#[path="🗑️generated/cold-job-frame-wire.rs"]
mod frame;
pub use frame::OriginalJobSourceWireHeader;
#[path="cold-job-frame-receiver.rs"]
mod receiving;

fn main(){
    let mut sample=String::new();
    std::io::Read::read_to_string(&mut std::io::stdin().lock(),&mut sample).expect("frame");
    let wire:Vec<_>=(0..sample.len()).step_by(2).map(|index|u8::from_str_radix(&sample[index..index+2],16).unwrap()).collect();
    let mut receiver=receiving::JobFrame::select(&wire,100_000,100_000).unwrap();
    let mut output=Some(ActorInput::Poll);
    for _ in 0..128 {
        let grant=Grant{maximum_items:3,maximum_copy_bytes:1500,maximum_capacity_bytes:100_000,maximum_depth:3};
        let mut cx=StepContext{grant,fuel:16_384,_owner:std::marker::PhantomData};
        match receiver.advance(&wire,&mut output,&mut cx).unwrap(){
            Ok(true)=>{
                let Some(ActorInput::Job(job))=output else{panic!("receiver left no typed Job")};
                fn encoded(bytes:Option<&[u8]>)->String{
                    bytes.map(|bytes|bytes.iter().map(|byte|format!("{byte:02x}")).collect()).unwrap_or("-".into())
                }
                println!("{} {} {} {}",job.job,encoded(Some(job.kind.as_bytes())),encoded(job.input.as_deref()),encoded(job.checkpoint.as_deref()));
                return
            }
            Ok(false)=>{},
            Err(error)=>{println!("{:?}",error.0);return}
        }
    }
    panic!("receiver failed to finish within the bounded turns");
}
