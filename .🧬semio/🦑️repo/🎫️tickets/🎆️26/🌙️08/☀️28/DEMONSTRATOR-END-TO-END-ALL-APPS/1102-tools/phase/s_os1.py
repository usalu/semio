from ph import *
E("semio-framework-os-store-initialization-replay","🧬️native/🦀️.rs","self.input.as_mut().unwrap().close_step(grant)",'self.input.as_mut().ok_or_else(||ValueError::invariant("initialization input owner is absent"))?.close_step(grant)')
E("semio-framework-os-store-initialization-replay","🧬️native/🦀️.rs","self.issuer_close.as_mut().unwrap().step(grant)",'self.issuer_close.as_mut().ok_or_else(||ValueError::invariant("initialization issuer closure is absent"))?.step(grant)')
E("semio-framework-os-flow-wasm","🧵️dag-input/🦀️.rs","match admit_owned_retirement(original.take().unwrap(),grant).result{","let Some(value)=original.take()else{return Ok(true)};match admit_owned_retirement(value,grant).result{")
fe="semio-framework-os-flow-extensions"
X="🎫️evaluate-request-admission-and-drive/🦀️.rs"
E(fe,X,'retained.units.checked_sub(before_units).expect("original evaluation unit progress changes only at explicit restart")','retained.units.saturating_sub(before_units)')
E(fe,X,'physical=physical.checked_add(receipt).expect("original evaluation cumulative physical receipt");','physical=physical.saturating_add(receipt);')
if __name__=="__main__":build()
