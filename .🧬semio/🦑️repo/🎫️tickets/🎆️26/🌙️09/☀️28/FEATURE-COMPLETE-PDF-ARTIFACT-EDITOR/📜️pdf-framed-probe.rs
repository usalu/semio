use semio_framework_replication_codec::OpBinary;
mod dsl {
    pub use semio_framework_replication_codec::ByteReader;
    pub mod original_receiving {
        pub use semio_framework_replication_codec::{OpBinaryReceiving, OpBinaryReceivingCursor};
        pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
    }
}
mod store { pub mod pack_rt { pub const OP_BINARY_FORMAT: u8 = 1; } }
const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[
    ("SetLanguage", "setLanguage", 53), ("SetOutputIntents", "setOutputIntents", 46)
];
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, PartialEq, semio_framework_value::RetireOwned)]
struct PdfMutation(DslValue);
impl FromValue for PdfMutation {
    fn from_value(value: DslValue) -> Result<Self, ValueError> { if value["language"].as_str().is_none() { return Err(receiving_invalid("projected PDF field is not text")); } Ok(Self(value)) }
}
impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> { unreachable!() }
    fn decode_op(_: &[u8]) -> Result<Self, semio_framework_replication_error::ProtocolError> { unreachable!() }
}

#[cfg(test)]
mod tests {
 use super::*;
 use semio_framework_value_native_owned::OriginalSourceTurnIssuer;
 use std::cell::Cell;

 #[test]
 fn original_framed_json_grants_refusal_cancellation_and_retirement() {
  let fixture: serde_json::Value = serde_json::from_str(include_str!("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR/📐️pdf-framed-wires.json")).unwrap();
  let body = fixture["bodyBytes"].as_u64().unwrap() as usize;
  let quantum = RetainedCloneGrant { maximum_items:64, maximum_copy_bytes:body, maximum_capacity_bytes:1<<20, maximum_release_bytes:1<<20, maximum_depth:128 };
  let stock = RetainedCloneGrant { maximum_items:1<<20, maximum_copy_bytes:1<<24, maximum_capacity_bytes:1<<24, maximum_release_bytes:1<<24, ..quantum };
  for (frame, cancel) in fixture["frames"].as_array().unwrap().iter().map(|f|(f,None))
      .chain(fixture["cancelBeforeSteps"].as_array().unwrap().iter().map(|n|(&fixture["frames"][0],Some(n.as_u64().unwrap() as usize)))) {
   let name=frame["name"].as_str().unwrap();
   let wire:Vec<u8>=serde_json::from_value(frame["bytes"].clone()).unwrap();
   let source=[b"prefix".as_slice(),&wire,b"suffix"].concat();
   let issuer=OriginalSourceTurnIssuer::new();
   let dispatch=issuer.issue().unwrap();
   let mut bank=bank::OperationBank::new(quantum,stock,1<<20,dispatch.identity());
   let canceled=Cell::new(false);
   let mut checkpoint=|_| !canceled.get();
   let mut native=NativeDecodeControl::new(stock.maximum_capacity_bytes,&mut checkpoint);
   native.with_source_witness(Some(dispatch.witness()),|native| {
    let range=(6,source.len()-6);
    let copied=size_of::<PdfMutationReceiving>();
    let under=PdfMutation::admit_receiving(&source,range,RetainedCloneGrant{maximum_copy_bytes:copied-1,..quantum});
    assert_eq!(under.work,0);
    assert!(under.result.unwrap().is_none());
    let admission=PdfMutation::admit_receiving(&source,range,RetainedCloneGrant{maximum_copy_bytes:copied,..quantum});
    let (mut cursor,progress)=admission.result.unwrap().unwrap();
    assert_eq!(progress.copied_bytes,copied);
    bank.debit(progress,admission.work);
    assert!(cursor.demands(&source.clone(),body).is_err());
    let mut refusal=None;
    for turn in 0..4096 {
     if cursor.is_ready(){break}
     let quote=match cursor.demands(&source,body){Ok(q)=>q,Err(e)=>{refusal=Some(format!("{e:?}"));break}};
     if turn==0 {
      let grant=RetainedCloneGrant{maximum_copy_bytes:quote.copy_bytes-1,..bank.grant()};
      let denied=bank.native(native,grant,1+semio_framework_value::ORIGINAL_INVOCATION_WORK,|native,grant|cursor.step(&source,native,body,grant));
      assert_eq!((denied.result.unwrap(),denied.work),(Default::default(),0));
      assert_eq!(cursor.demands(&source,body).unwrap(),quote);
     }
     if cancel==Some(turn){canceled.set(true)}
     let advanced=bank.normal(native,1,|native,grant|cursor.step(&source,native,body,grant));
     if let Err(error)=advanced.result{
      if cancel==Some(turn){ assert!(format!("{error:?}").contains("Canceled")); assert_eq!(advanced.work,0); }
      refusal=Some(format!("{error:?}")); break;
     }
     assert!(advanced.work>0,"{name}: turn {turn} stalled at {quote:?}");
    }
    let allowed=cancel.is_none()&&frame["accept"]==true;
    assert_eq!(cursor.is_ready(),allowed,"{name}, cancel {cancel:?}: {refusal:?}");
    assert_eq!(refusal.is_none(),allowed,"{name}, cancel {cancel:?}: {refusal:?}");
    let mut output=None;
    let under=cursor.take_into(&mut output,RetainedCloneGrant{maximum_copy_bytes:OUTPUT_HANDOFF-1,..quantum});
    assert_eq!((under.result.unwrap(),under.work),(Default::default(),0));
    assert!(output.is_none());
    if allowed{
     let taken=cursor.take_into(&mut output,RetainedCloneGrant{maximum_copy_bytes:OUTPUT_HANDOFF,..quantum});
     assert_eq!(taken.work,1);
     bank.debit(taken.result.unwrap(),taken.work);
     assert_eq!(output.unwrap().0["language"].as_str(),Some("de-CH"));
    }
    for _ in 0..4096 {
     if cursor.terminal_is_empty(){break}
     bank.close_native(native,|effect|cursor.close_step(effect)).result.unwrap();
    }
    assert!(cursor.terminal_is_empty(),"{name} retained custody");
   });
  }
 }
}
