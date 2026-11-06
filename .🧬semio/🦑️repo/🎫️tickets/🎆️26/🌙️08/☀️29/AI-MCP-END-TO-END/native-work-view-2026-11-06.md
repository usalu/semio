# Native Work View

Snapshot selection must borrow the actual remaining scoped CPU budget before choosing variable copy work. Retained items or byte limits cannot provide that authority. The recipient view will preserve the same observed-since-scope semantics as each existing admission predicate; actual work still goes through its original prepare/commit protocol.

## Actual Recipient Census

### 🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/🎟️native/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=self.fuel))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/♻️retirement/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(self.allow)}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🎟️work/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.calls+=1;Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🎟️work/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Interrupt{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{self.calls+=1;panic!("original quote interrupted")}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️backing/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for WorkLoan<'_>{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=(*self.0).min(1)))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for ConstructionWork{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for Interrupt{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{self.calls+=1;panic!("original work quote interrupted")}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Fuel {
        fn admits(&mut self, observed:u64, additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1))}
    }
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔐️shared-slots/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🎟️work/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for OriginalFuel{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|sum|sum<=self.0))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🔁️continuation/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for Fuel{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=1))}}
```

### 🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for WorkLoan<'_>{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,crate::ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=(*self.remaining).min(self.maximum)))}}
```

### 🧰️framework/🔨️modules/🎒️pack/🌱️value/📖️record-decode/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for RecordWorkStock{
 fn admits(&mut self,observed_since_scope:u64,additional:u64)->Result<bool,ValueError>{Ok(observed_since_scope.checked_add(additional).is_some_and(|work|work<=self.units))}
}
```

### 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🏗️birth/🎟️original/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.0.checked_sub(observed).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original finite work was already exceeded"))}fn admits(&mut self,observed:u64,extra:u64)->Result<bool,ValueError>{Ok(observed.checked_add(extra).is_some_and(|n|n<=self.0))}}
```

### 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🛬️receive/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work<'_>{fn admits(&mut self,seen:u64,next:u64)->Result<bool,ValueError>{Ok(seen.checked_add(next).is_some_and(|n|n<=(*self.0).min(1)))}}
```

### 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔤️text/🛬️receive/🧪️tests/🦀️.rs

```rust
impl semio_framework_value::native_decoding::NativeDecodeWorkRecipient for OriginalTextWork{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🛬️receive/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work<'_>{fn admits(&mut self,seen:u64,next:u64)->Result<bool,ValueError>{Ok(seen.checked_add(next).is_some_and(|n|n<=(*self.0).min(1)))}}
```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🏭️producer/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️decoding/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(self.0)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(self.0)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(false)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=65536))}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{panic!("denied backing cannot reserve work")}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1000000))}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧵️paged-text/🧪️.rs

```rust
impl NativeDecodeWorkRecipient for DLoan{fn admits(&mut self,observed:u64,more:u64)->Result<bool,ValueError>{Ok(observed.checked_add(more).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,_:u64)->Result<bool>{Ok(true)}}
```

### 🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for ActualWork<'_,'_,'_>{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{
  self.turn.with(|cx,_|!paused(cx,self.closing)&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))
 }
}
```

### 🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeCurrentWork<'_,'_>{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{
  Ok(!self.cancel.is_cancelled_now()&&(self.now)().is_some_and(|now|now<*self.deadline)&&observed.checked_add(additional).is_some_and(|work|work<=*self.fuel))
 }
}
```

### 🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeRetirementWork<'_>{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok((*self.retirement||(self.now)().is_some_and(|now|now<*self.deadline))&&observed.checked_add(additional).is_some_and(|work|work<=*self.fuel))}}
```

### 🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧪️tests/🔬️custody/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.remaining)}fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.remaining=self.remaining.checked_sub(actual).ok_or_else(||ValueError::invariant("fixture work overdraw"))?;self.committed+=actual;Ok(())}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔬️probe-document/📥️receiving/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.0.checked_sub(observed).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original finite work was already exceeded"))}fn admits(&mut self,observed:u64,extra:u64)->Result<bool,ValueError>{Ok(observed.checked_add(extra).is_some_and(|value|value<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🧪️tests/🚪️control/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for EnabledWork<'_>{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{if self.0.load(std::sync::atomic::Ordering::Relaxed){Ok(true)}else{Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode work is cancelled"))}}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for UnlimitedWork{fn admits(&mut self,_:u64,_:u64)->Result<bool,ValueError>{Ok(true)}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=1))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OriginalMemberWork<'_,'_,'_>{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.turn.with(|cx,_|!paused(cx,self.closing)&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))}
}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for DriverWork{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.quantum.min(self.remaining)))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️testing/♻️retirement/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeWorkWallet{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining.min(self.quantum)))}
}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/🧩️composition-and-retained-decode/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for TurnWork{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.0.min(4096)))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-child-member-registry/🦀️.rs

```rust
impl semio_plugin_document_media::NativeDecodeWorkRecipient for Work<'_, '_>{
            fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{let cx=self.0.try_lock().map_err(|_|semio_framework_value::ValueError::invariant("original archive test caller is already borrowed"))?;Ok(!cx.retirement_should_yield()&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))}
        }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for StepWork<'_,'_,'_>{
    fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{
        let cx=self.0.actual.try_lock().map_err(|_|ValueError::invariant("original archive caller is already borrowed"))?;
        let paused=if self.0.closing{cx.retirement_should_yield()}else{cx.should_yield()};
        Ok(!paused&&(self.0.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))
    }
}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OperationWorkLoan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.fuel))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/identity-index-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=10_000_000))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/loads/receiver-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.limit))}
}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/identity-index-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=10_000_000))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/loads/receiver-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {
    fn admits(&mut self, observed:u64, additional:u64) -> Result<bool, ValueError> {
        Ok(observed.checked_add(additional).is_some_and(|total|total<=self.limit))
    }
}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### ✏️s/🔌️plugins/🏗️fem/🧩️sqlite/🧮️projection/🧪️tests/🦀️.rs

```rust
impl semio_framework_value::native_decoding::NativeDecodeWorkRecipient for DecodeLoan{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|value|value<=self.0))}}
```

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for StepWork{
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.quantum))}
}
```

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/🧪️tests/🧰️helpers/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OriginalWork{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1<<40))}}
```

## Live Caller View

The required view now borrows the actual native decoder loan. Current and cleanup job turns, original history/member turns, and the archive maintenance caller preserve their actual cancellation, yield, deadline and remaining-fuel gates. Observed scoped work is deducted from that same caller budget; no byte-derived CPU ceiling or unlimited fallback was introduced. This is authored source, pending native qualification.

## Canonical Test Source Admission

Capture now requires an explicit actual remaining-work callback. The staged allocator marker survives the child action, is validated against actual observed heap capacity, and clears on its own funded turn. The helper checks actual Source work demand before effects, rather than assuming one CPU unit per retained item. Wallet post verifies the actual finite CPU stock; controlled close checks its original next_computational_work quote first. Caller census edits are saved in generated/admission-work-callers.json; native law qualification remains pending.

## Canonical Test Source Admission

Capture now requires an explicit actual remaining-work callback. The staged allocator marker survives the child action, is validated against actual observed heap capacity, and clears on its own funded turn. The helper checks actual Source work demand before effects, rather than assuming one CPU unit per retained item. Wallet post verifies the actual finite CPU stock; controlled close checks its original next_computational_work quote first. Caller census edits are saved in generated/admission-work-callers.json; native law qualification remains pending.

Native source marker law is registered as original-source-marker in the canonical testing-admission owning manifest; it compares all actual CPU actions and heap births with the same neutral corpus as independent SQLite. New source total is12work/9turns/3physicalbirths, with the three actual marker clears separatelyfunded; owning NativeOwned and customFactory fixture expectations were updated together. The Factory test keeps its zero-copy child constructor grant and a separate explicit finite parent marker CPU/item/copy stock. Projection immutable copyCPU now2 (=originalroot1+parentcapture1), with matching strictschema and SQL oracle. Latest changed-fixture batch3tests39assertions GREEN; no native test claim.

One caller edit briefly used an adjacent atomic stagingfile instead of ticket-generated staging; it was renamed immediately and no residualfile remains. All subsequent authoring scripts stage inside the ticket generated directory.

## Shared Construction Slots Owner

The actual generic NativeSharedSlots implementation moved from NativeOwned into its own domain owner under native decoding construction/slots. Its dependencies are the same canonical control, shared issuance, retirement, clone, and native-decoding types; NativeOwned explicitly reexports the same type. The old module path is removed, not retained as a compatibility facade. The no-pending receiving error now has one canonical NativeDecode helper. This releases production source space for real work quote ports and preserves all existing native laws. New source path/move targets were resolved and checked within the workspace before mutation.

SharedAdmission now quotes its actual sealed allocator birth or one-step publication and Arc allocation/publication. It rejects unresolved or wrong-kind frames. SharedSlots validates its actual input/output custody before delegating; these queries neither advance nor create a work loan. Native compilation remains pending.

Root marker native run is terminal: mcp-65764.json, actual exit 1, duration 1562446 ms. It stopped before compilation because Framework Cargo manifest changed from the captured 131745-byte extent to 132076 bytes. No marker native assertion ran. Wrapper exit is not interpreted as native success. Guest projection and Server pair-slot runs ended on the same manifest extent change. No replacements are started while required canonical marker signatures are being completed.

The actual IntrinsicSharedField implementation moved into native construction/field, semio-framework-value-native-shared-field. NativeOwned reexports the same canonical type; its pre-effect receiving frames, original binding, clone cursor, shared slots and cleanup owners remain together. This is a domain owner with direct canonical dependencies, not a duplicated type, source exclusion or cap increase. New paths are checked inside the workspace and under 256 characters.

The actual retirement work/conservation suite now runs through its own explicit native integration target original-retirement-work. Its law body, fixtures and assertions stay in their existing locations and import canonical public retirement/factory/allocator owners through a small owning entry. Removing only the former inline module mount releases the monolithic cfg(test) Source closure without skipping the suite or raising caps. Compilation and the full marker migration remain pending.

## Receiving And Retirement Foundation Work

Root authored bounded max-work and explicit advance/take/close quotes for IntrinsicSharedField, NativeReceivingOriginal, VariantSelector and Result receiving. Their own real effects use stable pre-effect Value/Retirement receiving frames. Result selects its payload on a separately funded turn and retains the actual original child projection; later quotes borrow that alias without repeating source getters. Canonical Ref.try_project_owned checks alias admission before one fallible accessor; native fixture laws cover denied access, pointer/path/source identity, absent child and paid closure. Getter work remains the explicit caller charge.

A deeper cleanup audit found infallible Box allocations in RetireOwned::retirement, including generated Vec/deferred/Box sequences. Server introduced a concrete associated Cursor and allocator-before-take birth helper, with six neutral cases validated by AJV and SQLite (19 assertions); native actual allocator assertions are authored and unrun. Root is migrating primitive/std inline constructors, tuples, arrays and the struct macro. The struct cursor holds MaybeUninit original storage and accesses only the selected initialized field, then retains each born child before a separate funded transfer. Standard collection child births allocate before pop. Child quote/work propagation and the complete controlled closure chain are still in progress; these edits are not a compile or native acceptance claim.

The Root parser gate for eight receiving/owner entries reports no parser errors, with formatting-only rustfmt exit 1. Latest NativeOwned cfg(test) Source census before ongoing leaf edits: 27 files /260230 bytes /1163 active lines. SharedSlots 1/9127/60 and IntrinsicSharedField 1/12085/78. Launch confirmed normal MCP dependency membership469 packages /3 workspaces /0issues. No cap increase, source exclusion or removed native assertion is used.

## Full Log Correction

Inspection of complete structured stderr corrects the earlier terminal-tail diagnosis: all three runs reached dependency compilation before the later Framework manifest extent refusal. Guest projection mcp-36724 compiled ControlContract/AllocationReturn and failed PagedList on a captured missing reservation/release source. Root marker mcp-65764 reports E0631 in TestingAdmission; Server source-pair mcp-30264 reports E0061 in retained-work. The two latter stderr values are marked truncated. No requested target law ran. These are actual compiler diagnostics as well as stale-source extent refusals; they are not classified as purely precompiler runs. The prior report paragraph claiming precompiler status is superseded by this evidence.

Child projection neutral cases now explicitly include zero items, zero depth, successful row retention and an absent row. The native law consumes the same rows and expected accessor counts; the portable strict-schema/SQLite batch passes 2 tests /10 assertions. Root fixed the captured TestingAdmission E0631 by preserving OriginalRetirementFailure in the close receipt mapper. Latest expanded Root parser gate has no syntax errors; retirement cfg(test) is29 files/251882bytes/2340lines after real work-law factoring. These are authoring/parser/portable checks; no native test pass is claimed.

A native integration law original-inline-retirement now compares six concrete cursor families (scalar, UTF8, vector, tuple, array, struct) against real standard-drop allocator releases. It asserts constructor allocation zero, finite actual work and physical budgets, marker/heap equality on each child turn, funded child cursor release and net original allocation conservation. The shared neutral fixture has an independent SQLite input oracle. This law is authored; no native run is claimed.

## Actual Leaf And Candidate Birth Continuation

The generator audit disproved the earlier assumption that every SnapshotProjection::begin_projection is allocation free: derived candidate storage performs a raw allocation. Its public trait and Root field/owned/scalar wrappers now require the actual allocator marker; Server reports the derived allocator records Some(0) before the attempt and the real extent after success. External propagation and stable generic construction gating remain active Launch work, with no posthoc progress-derived marker accepted.

Root migrated Mutex, Result, OriginalVector, unique Box, std Arc, refusal-tree and sealed original issuer constructors to concrete inline cursor ownership and actual marker forwarding. In-place shared retirement now exposes full physical demand plus independent work quote instead of a cap-only method. The sealed producer allocates before taking the original slot. Std Arc frame admission uses the original source slot, an empty preborn destination and actual marker, preserving the source on denial or allocation failure. Paged retirement uses Guest's real paid pop-source primitive and retains release selection before its paid release/clear phases. FixedArray selection now uses a stored remaining index instead of scanning previously consumed fields.

Still unqualified: external/manual RetireOwned implementations and ordinary marker call chains, direct leaf laws, generic construction same-control/loan composition, and std ordered collection node release accounting. No native run is active and no law has been skipped to obtain a pass. The original full error mapper compiler diagnostic E0631 was fixed at TestingAdmission; Server reports real marker propagation for the retained-work E0061. These changes await native compilation after API coherence.

## Source Release Currency, 2026-11-06

The existing nominal Source budget and escrow table now carry held release bytes. Snapshot conservation checks spent release plus held release; available grants subtract the hold. The Source terminal and renewal checks retain live release holds. Existing return claims explicitly reserve zero release bytes.

Escrow amounts now include release bytes, with checked addition and per-row limits. Staged rows retain and validate that currency. The publisher and actor reader use the same 160-byte row (20 words), including held, redeemed and unused release. Native snapshot, table denial/conservation, and reader over-redemption/tombstone tests were authored; they have not run natively.

The new language-neutral release fixture passes independent SQLite arithmetic and strict AJV validation: 1 test, 12 assertions. The guard fixture initially failed its strict schema after its actual amount-write price changed; the schema was updated with the fixture. The prior Source guard and actor codec portable suites are being rechecked. Scoped Source censuses work normally; earlier broad censuses were expensive reads, not evidence of repository bootstrap failure.

This is accounting and wire plumbing, not a completed construction release hold. Launch owns real same-row opaque claim reservation/redemption, durable work holds, and constructor output lifecycle integration. A live output must retain its Source claim through admission reset and transfer to cleanup. No numeric spare check, inferred capacity authority, or data-only permission qualifies that lifecycle.

Current cfg-test Source closures: JobPayload 15 files /129527 bytes /1158 active lines; ActorReceipt 4 files /59438 bytes /316 active lines before the small codec test addition. Limits remain unchanged. No native target was started, replaced or stopped by this work.

## Source-Slot Shared Admission and Default Cursors, 2026-11-06

Sealed shared admission now accepts the actual original and recipient slots. The lower private shared Allocation is genuinely allocated before the original slot is taken; denial retains the original. Sealed issuance, intermediate-slot clear and final recipient placement compose with that allocator. Pure demand/work ports select exact typed copy and seven work units. The native slot law was authored before production changes and remains unrun. Its strict-schema/independent SQLite event-order oracle passes 1 test /5 assertions. Server owns the SourceAdmission caller and follow-up shared prepare/publication/duplicate ports, with genuine lower owner factoring to keep the fixed source boundary.

Default production was independently audited. Scalar defaults and cursor constructors now quote actual Option placements. Public take remains the canonical zero-copy ownership handoff; internal Slot capture quotes its own Option<T> write and one additional work unit. Closure forwards the retained actual allocation marker. A direct child close intent is selected and paid before entering that child, rather than recursively marking the whole tree in one operation.

The paired-default cursor now has finite work quotes, maxWork-aware demands, preborn value and retirement effects, complete recovery delegation, paid parent capture/phase/publication, and marker-last close. Partial fields stay in their original resident slots until funded publication or retirement. Independent Rust parsing passed for core, Slot and Pair; one introduced extra delimiter in the Slot quote was found and corrected before that pass. This is syntax verification, not native/type qualification.

Guest authored the Default placement neutral corpus and native laws before the pricing change: denied copy preserves the output, real full placement creates an empty String without allocating, and zero-work denies the empty-array publication. Guest owns the array production rewrite and Default test migrations.

Latest measured cfg-test source closures: NativeDefault 11 files /74398 bytes /293 active lines; JobPayload 15 /129610 /1158. The fixed 32-file, 262144-byte and 4000-line limits are unchanged. No new native target has been requested.

## Full Physical Scope Hold and Host Conservation, 2026-11-06

The release-only row described earlier is superseded by one full physical-currency row. Pending native allocation/receipt work can retain capacity as well as CPU, items, copy and release; an unheld numeric capacity grant would allow the Source to spend the same stock elsewhere. Launch is integrating one Source-owned scope loan in the existing nominal escrow table, with actual native work subdivisions, actual/pending reconciliation, future outcome metadata held before callback, and same-row refund of unused authority.

Root added held_capacity_bytes beside held_release_bytes in the actual Source budget and snapshot. Both are checked by conservation, subtracted from available stock, and block renewal/terminal closure while live. The same checked escrow Amounts carries all five currencies. The single wire row now has 23 words /184 bytes, with a maximum 32-row extent of5936 bytes; this is the schema's actual storage size, not a source-boundary or artifact-policy increase. Currency order is items, copy, work, release, capacity.

The actor reader, publisher, staged table, reactor fixture/schema and host escrow/history book now use that one layout. Host validation, monotonic cumulative rows, origin deltas, purpose aggregation, fresh-receipt subtraction and ledger reconciliation cover all five currencies. Source identity/producer/epoch checks and incremental one-axis paid turns remain. Native tests were updated for actual row offsets; the actor codec law also rejects over/under-settled release and excess capacity. None have run natively.

Independent validation: full release/capacity fixture1test/22 assertions; reactor wire +guard fixtures4tests/34 assertions; actor codec portable suite3tests/21 assertions. Launch subsequently widened actual held_fuel metadata prices and updated the guard's exact258-byte observer fixture; its rerun passes1test/14 assertions. Selected Source budget/escrow/actor reader/host book/history/reactor/publication Rust syntax checks report no parser errors. These are not native/type passes.

A scoped external Source census identified all additional SRE1 consumers; the interpreter's empty48-byte snapshot needs no row change. Broad filesystem censuses completed slowly; they were read-only and did not restart/kill any native target. Existing Source positive laws still need migration to the new scope-claim setup and full Source metadata accounting, including legitimate setup progress before the native callback. Constructor output cleanup claim lifecycle and native metadata capture remain required.

Root also migrated Intrinsic/Variant bind work to the canonical Ref.binding_work_demand; binding cleanup forwards actual markers. Result child projection quotes compose getter work +seven-work by-value constructor +one actual Option recipient placement, with narrowed child grant and complete resident recovery. Selected parsing passed. The Result public output/take protocol still merits a native law audit; no blanket receiver qualification is claimed.

## AllocationReturn Native Admission, 2026-11-06

Server reported the smallest AllocationReturn --lib source/API closure coherent (10 files /49442 bytes /369 active lines), without claiming compilation. Actual OS census and repo MCP all-store lease census completed before admission. All seven enumerated stores show no prior Root marker/projection/clone target holders; the old isolated store has no holders or waiters. Unrelated shared PagedList and paged-refusal-source work plus an existing show-projects reader remain owned by other work and were left untouched. No process was killed or replaced. AllocationReturn and direct ControlContract source/data are held stable by the work fleet for this native run; Source Job/host work continues outside that closure. Sole request: isolated, prototype, parallelism1, semio-framework-value-allocation-return --lib. A submitted/running request is not a native pass.

## First Small Native Execution Result, 2026-11-06

The isolated AllocationReturn library request23348 is terminal. The actual untruncated result is generated/mcp-70940.json: operation exit1, duration1131886ms. Rust compiled ControlContract, AllocationReturn and test dependencies, then executed all11 native tests. Ten passed; one failed; none were ignored or filtered. This is genuine compilation and execution, not a parser or admission-only result.

The sole failure is shared_admission_uses_the_real_layout_for_aligned_and_zero_sized_payloads at shared/tests line42: actual successful direct value-form work2 versus a stale expected1. The real allocator/slot/original publication/concurrent alias/field release/null-failure laws passed. Server owns the narrow test correction, preserving aligned/ZST allocator/layout assertions. Production AllocationReturn and ControlContract remain stable for a clean-baseline retry; Root will perform another actual OS/all-store lease census first. This run has no later Source-extent refusal. It is not an all-green native qualification or semio MCP readiness.

## Record Storage and Generator Continuation

Root authored updated language-neutral record storage fixtures/schema and native tests before changing production primitives. The independent SQL/AJV oracle passes1test/16 assertions. Initialization now prices only its actual ready bool. Field placement/removal include the original/output Option, field value and live-frontier metadata, with selected work3. By-value and original-Box handoff quote their real metadata and work4 before custody changes. Release composes real raw backing release with its ready-flag clear. Reserve retains its real actual allocator marker.

Native laws preserve null allocation, missing-axis denial, original pointer identity, complete/partial frontier, genuine value and complete cleanup assertions. They now carry a persistent marker, separately paid clear and finite Wallet, plus remove/reinsert and underfunded boxed handoff checks. Selected production and test Rust parsing passed; native record laws remain unrun. Latest NativeOwned source closure27files255303bytes1146active lines remains within the unchanged fixed boundary.

Guest found the entire NativeReceive derive emitter still producing old calls and missing work/recovery/marker semantics. That canonical rewrite is now prioritized; Root took SnapshotProjection marker tests and RecordStorage ports. NativeReceiver close starts are being made direct-child paid operations (mandatory is_closing and canonical begin_close demand/work/step), rather than recursively closing a whole receiver tree in one step. Root/Guest/Server own their respective families. Broad Owned/Default/Note/UI native qualification remains blocked on this actual production generator and Source setup/hold integration.

## AllocationReturn Native Retry Admission

The native aligned/ZST success-work test now reads the already-established neutral successWork2; allocator/layout/marker checks remain. Fresh OS census reports no selected prior native target processes. Fresh all-store result mcp-63736.json enumerates7stores; the previous AllocationReturn request has no holders/waiters, and isolatedstoresempty. Unrelated reader/refusal-source leases remain untouched. Root admits one exact isolated --lib retry, with AllocationReturn production and ControlContract still stable. The first result remains preserved in mcp-70940.json.

## AllocationReturn Clean Native Baseline

The sole retry94772 is terminal and genuinely green. Actual result generated/mcp-42968.json reports exit0/oktrue,695584ms, untruncated stdout/stderr. Rust compiled the selected native library test artifact and all11 native tests passed; zero failed, ignored, measured or filtered. Source handback also completed without an extent refusal. The previous failure was only the now-corrected aligned/ZST work expectation; allocator/layout/actual-marker checks remain.

This qualifies exactly the captured AllocationReturn/ControlContract baseline. The freeze is lifted for necessary new lower take/election/capture ports; new changes require new qualification. Guest owns the next sole actual derive-expansion AST native request after fresh process/all-store census (session68020/MCP48612); no Owned/Default/Source/MCP readiness is claimed.

## Selector, Result and Close Corrections

Root found a meaningful error in an earlier receiver patch: Result cleanup methods had been placed inside VariantSelector, referencing fields that Selector does not own. Independent syntax parsing cannot detect that semantic error. The misplaced block is removed, and the canonical pure VariantSelector now lives beside Matcher in the existing NativeSchema owner; Owned reexports its unique types. No package/member/cap increase. NativeSchema's missing direct retirement dependency was added.

The Selector retains cached record length, funds actual discriminator/key reads only in callbacks, and charges actual typed scan/tag/matcher/selection/phase writes. Matcher close intent, matcher cleanup, binding return and final phase are independently paid; complete recovery and actual markers are forwarded. All close/settlement intent ports are available to generated enum receivers. Result now retains an actual Option<Result<T,E>> before publication, separates child-take/parent placement from public zero-copy take, and closes the output, child receivers, payload projection and Selector through actual marker-last paths. This remains uncompiled.

Original and Intrinsic begin-close now mark only their own flag. Original delegates direct child intent/close to the canonical backing helper; Intrinsic pays the retained field cursor's own start before its close. Typed original-slot clear, own flag and binding cleanup are quoted. A delimiter error introduced while changing the typed original-slot receipt was caught and corrected; selected parses pass. Root still has Intrinsic normal capture/phase prices, Result public law migration and SnapshotProjection marker tests to finish.

Root has not implemented release-claim output transfer yet. Aggregate allocated-minus-released is explicitly rejected as ownership proof: retiring an unrelated old input can mask a live new output. The integration must attach opaque cleanup custody to the actual original-native recipient/slot/lease and require its Source-qualified terminal witness before refund. Launch's actual pooled scope and metadata gates remain unqualified until that lifetime protocol and compile/cap checks finish.

## Cached Capture Selection and Fixed Shared Cleanup Frame

The derive baseline retry is genuinely green: generated/mcp-63380.json reports operation exit0, oktrue,642780ms, untruncated stdout/stderr;14 native AST tests pass with zero failed/ignored/filtered. This is the frozen generator baseline before subsequent capture/mapping emitter changes, not emitted-code type or runtime qualification.

Root mounted mandatory NativeDefaultCursor capture_plan plus Value/Slot/Pair implementations, and cached selected-plan getters on Original, Result, Selector and Intrinsic. Pending own/retirement/child frames take precedence; active normal stages select the same authentic frame used by their callback. Queries use cached state and do not invoke input getters. The opaque selection identifies the real frame; it does not itself qualify Source metadata authority. Server/Launch still own exact field permissions and Source preloan enforcement.

SharedSlots capture/take/reclaim now price both actual Option writes and work2. Static capture_work_demand/reclaim_work_demand/take_work_demand return2. Intrinsic composes these ports, its typed raw output/phase placement, close-intent bool/phase and terminal phase write. Original adds internal child-take/parent-Option placement while public take remains copy0. Its ready observer now waits for all pending recovery. Intrinsic exposes its own paid close-intent ports for the manual record law.

Result's existing serde oracle passes1test/20 assertions after preserving all9 standard shape cases and migrating its native integration law to selected work, resident output, finite Wallet and persistent marker/paid clear. The native law remains unrun. All9 changed production/test Rust sources pass independent syntax parsing; no native type pass is inferred.

Sealed Cursor now has fixed pointer-sized NativeOwnedStorage<T> fallback instead of inline Option<T>. Distinct native/shared issuer function pointers keep the hot in-place shared branch out of whole-payload transfer code. Whole-value handoff and native cursor birth are isolated behind cold non-inlined functions. Election composes lower exact retire_step with actual outer source/original placements. Unsupported in-place fallback first admits real typed backing, then uses lower take_value_step under its exact grant, then admits the actual native cursor before taking the stored value; empty backing is separately released. No source take happens before fallback backing is available. New native layout/denial laws were authored before this implementation. The independent SQL fallback event-order oracle passes1test/5 assertions; syntax parses. The Sealed native source closure is10files75662bytes551active lines under unchanged limits. Existing older Sealed private laws still require marker/associated-type migration before a native library run.

## Cleanup Claim Custody Audit

OriginalConstructionAdmission currently publishes an arbitrary generic O into a bare caller Option<O>. This gives no proof that downstream owners have actually retired; an empty donor slot alone cannot release its stock. Native root terminal witnesses are genuine for Send owners. Actor-local nonSend owners use OriginalActorReturnSlot<T>, which pins an actual home/donor and qualifies OriginalCallerReturnPlace before writes, but lives in Actor above Job. Importing Actor into Job would introduce a cycle, and imposing Send/RetireOwned would discard valid actor-local outputs. A shared lower return-custody interface or a managed original output custody transition is required; no aggregate allocated-minus-released, address-only qualifier or manually asserted bool will be used as proof. Root is continuing this actual lifetime integration with Launch's same-row stock transfer hook.
