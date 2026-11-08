import sys
sys.path.insert(0,'.')
from rep import apply
D="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛫️encode/"
apply(D+"🛂️allocation/🧪️tests/🔒️receipt/🦀️.rs",[
 ("let mut denied_port=|event:crate::native_encoding::NativeAllocationEvent|","let denied_port=|event:crate::native_encoding::NativeAllocationEvent|",1),
 ("let mut denied_port_loan_issuer=crate::native_encoding::NativeEncodeAllocationIssuer::new();let mut denied_port_loan=denied_port_loan_issuer.loan(&mut denied_port);let mut invalid=crate::native_encoding::NativeForwardedEncodeContinuation::new(ceiling,&mut denied_port_loan);invalid.receipt.as_mut().unwrap().owned_bytes=ceiling+1;",
  "let mut authority=crate::NativeAllocationAuthority::new(crate::eff::Port(denied_port));authority.with_send_loan(&mut (),|denied_port_loan|{let mut invalid=crate::native_encoding::NativeForwardedEncodeContinuation::new(ceiling,denied_port_loan);invalid.receipt.as_mut().unwrap().ledger=Some(crate::native_receiving::NativeAllocationLedger::new(ceiling+1));",1),
 ("assert_eq!(invalid.receipt.as_ref().unwrap().owned_bytes,ceiling+1);let(error","assert_eq!(invalid.receipt.as_ref().unwrap().owned_bytes(),ceiling+1);let(error",1),
 ("assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(invalid.receipt.as_ref().unwrap().owned_bytes,ceiling+1);","assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(invalid.receipt.as_ref().unwrap().owned_bytes(),ceiling+1);",1),
 ("drop(invalid);\n}","drop(invalid);}).unwrap();assert!(authority.into_port().is_ok());\n}",1),
])
print("ok")
