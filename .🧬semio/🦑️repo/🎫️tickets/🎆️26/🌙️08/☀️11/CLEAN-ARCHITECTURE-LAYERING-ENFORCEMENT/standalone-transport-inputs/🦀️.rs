//! 🎛️ Standalone product callers supply finite transport credit and a cancellation deadline.
use semio_framework_pack_error::{PackTransportContext,PackTransportPolicy,PackTransportProgress,TransportAdmission,TransportContextRefusal};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_async::CancelToken;
use std::time::{Duration,Instant};

/// 🚪️ Global policy arguments precede the untouched product command arguments.
pub enum Invocation<'a>{Help,Run{arguments:&'a[String],options:TransportOptions}}
/// 🛑️ Missing and malformed policies refuse execution before any transport ownership.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum InvocationRefusal{MissingCredit,MissingInput,MissingTimeout,InvalidCredit,InvalidInput,InvalidTimeout,DuplicateCredit,DuplicateInput,DuplicateTimeout,MissingCommand}
impl std::fmt::Display for InvocationRefusal{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{self:?}")}}
impl std::error::Error for InvocationRefusal{}
/// 📏️ Policy values are portable on every supported native pointer width.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct TransportOptions{credit:u32,input_credit:u32,timeout_ms:u32}
impl TransportOptions{
    pub const fn input_credit(self)->u64{self.input_credit as u64}
    pub fn create_context(self,cancellation:CancelToken)->Result<PackTransportContext,TransportContextRefusal<TransportDeadline>>{PackTransportContext::try_new(TransportAdmission::new(self.credit as usize,0),TransportDeadline{started:Instant::now(),timeout:Duration::from_millis(u64::from(self.timeout_ms)),cancellation})}
}
/// ⏱️ The process owns its deadline and reports actual transport ledger progress.
pub struct TransportDeadline{started:Instant,timeout:Duration,cancellation:CancelToken}
impl TransportDeadline{
    fn check_elapsed(&self,elapsed:Duration)->Result<(),ValueError>{if self.cancellation.is_cancelled_now(){Err(ValueError::new(ValueRefusalKind::Canceled,"standalone transport caller canceled"))}else if elapsed>=self.timeout{Err(ValueError::new(ValueRefusalKind::Canceled,"standalone transport deadline elapsed"))}else{Ok(())}}
}
impl PackTransportPolicy for TransportDeadline{
    fn checkpoint(&self,_:PackTransportProgress)->Result<(),ValueError>{self.check_elapsed(self.started.elapsed())}
    fn progress(&self,progress:PackTransportProgress){eprintln!("transport {:?}: bytes={} committed={}",progress.phase,progress.bytes,progress.committed_bytes)}
}
fn decimal(value:&str)->Option<u32>{if value.is_empty()||!value.bytes().all(|byte|byte.is_ascii_digit())||(value.len()>1&&value.starts_with('0')){None}else{value.parse().ok()}}
/// 🧭️ Required global policy has one canonical spelling and no implicit defaults.
pub fn parse(args:&[String])->Result<Invocation<'_>,InvocationRefusal>{
    if args.is_empty()||(args.len()==1&&matches!(args[0].as_str(),"help"|"--help"|"-h")){return Ok(Invocation::Help)}
    let(mut credit,mut input_credit,mut timeout_ms,mut offset)=(None,None,None,0);
    while let Some(flag)=args.get(offset){
        match flag.as_str(){
            "--transport-credit"=>{if credit.is_some(){return Err(InvocationRefusal::DuplicateCredit)}credit=Some(args.get(offset+1).and_then(|value|decimal(value)).ok_or(InvocationRefusal::InvalidCredit)?);},
            "--input-credit"=>{if input_credit.is_some(){return Err(InvocationRefusal::DuplicateInput)}input_credit=Some(args.get(offset+1).and_then(|value|decimal(value)).ok_or(InvocationRefusal::InvalidInput)?);},
            "--timeout-ms"=>{if timeout_ms.is_some(){return Err(InvocationRefusal::DuplicateTimeout)}timeout_ms=Some(args.get(offset+1).and_then(|value|decimal(value)).filter(|value|*value>0).ok_or(InvocationRefusal::InvalidTimeout)?);},
            _=>break,
        }
        offset+=2;
    }
    let options=TransportOptions{credit:credit.ok_or(InvocationRefusal::MissingCredit)?,input_credit:input_credit.ok_or(InvocationRefusal::MissingInput)?,timeout_ms:timeout_ms.ok_or(InvocationRefusal::MissingTimeout)?};
    let arguments=&args[offset..];if arguments.is_empty(){return Err(InvocationRefusal::MissingCommand)}
    Ok(Invocation::Run{arguments,options})
}
/// 📖️ Help explains the required product policy without constructing a transport context.
pub fn print_help(program:&str){println!("{program} --transport-credit <bytes> --input-credit <bytes> --timeout-ms <milliseconds> <command> [arguments]\nGlobal policy options precede the command. Both credits are 0..4294967295 bytes; timeout is 1..4294967295 milliseconds. Add --cancel-stdin after the command to enable cancellation input. Transport progress is written to stderr.")}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
