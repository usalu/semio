# Native Work View

Snapshot selection must borrow the actual remaining scoped CPU budget before choosing variable copy work. Retained items or byte limits cannot provide that authority. The recipient view will preserve the same observed-since-scope semantics as each existing admission predicate; actual work still goes through its original prepare/commit protocol.

## Actual Recipient Census

### 🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/🎟️native/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.fuel;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=self.fuel))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/♻️retirement/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{if self.allow{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}else{Ok(0)}}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(self.allow&&additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🎟️work/🧪️tests/view.rs

```rust
impl NativeDecodeWorkRecipient for Stock{
 fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.remaining.checked_sub(observed).ok_or_else(||ValueError::invariant("view fixture exceeded its original scoped work"))}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.calls+=1;Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining))}
}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🎟️work/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.remaining;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.calls+=1;Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🎟️work/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Interrupt{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{if additional>self.available_work(observed)?{return Ok(false)}self.calls+=1;panic!("original quote interrupted")}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️backing/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for WorkLoan<'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=(*self.0).min(1);limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=(*self.0).min(1)))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for ConstructionWork{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for Interrupt{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{if additional>self.available_work(observed)?{return Ok(false)}self.calls+=1;panic!("original work quote interrupted")}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Fuel {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=1;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}
        fn admits(&mut self, observed:u64, additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1))}
    }
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔐️shared-slots/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Fuel{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=8))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🎟️work/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for OriginalFuel{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|sum|sum<=self.0))}}
```

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🔁️continuation/🧪️tests/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for Fuel{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=1;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=1))}}
```

### 🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/🦀️.rs

```rust
impl crate::native_decoding::NativeDecodeWorkRecipient for WorkLoan<'_>{fn available_work(&mut self,observed:u64)->Result<u64,crate::ValueError>{let limit:u64=(*self.remaining).min(self.maximum);limit.checked_sub(observed).ok_or_else(||crate::ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,crate::ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=(*self.remaining).min(self.maximum)))}}
```

### 🧰️framework/🔨️modules/🎒️pack/🌱️value/📖️record-decode/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for RecordWorkStock{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.units;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}
 fn admits(&mut self,observed_since_scope:u64,additional:u64)->Result<bool,ValueError>{Ok(observed_since_scope.checked_add(additional).is_some_and(|work|work<=self.units))}
}
```

### 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🏗️birth/🎟️original/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.0.checked_sub(observed).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original finite work was already exceeded"))}fn admits(&mut self,observed:u64,extra:u64)->Result<bool,ValueError>{Ok(observed.checked_add(extra).is_some_and(|n|n<=self.0))}}
```

### 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🛬️receive/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work<'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=(*self.0).min(1);limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,seen:u64,next:u64)->Result<bool,ValueError>{Ok(seen.checked_add(next).is_some_and(|n|n<=(*self.0).min(1)))}}
```

### 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔤️text/🛬️receive/🧪️tests/🦀️.rs

```rust
impl semio_framework_value::native_decoding::NativeDecodeWorkRecipient for OriginalTextWork{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🛬️receive/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work<'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=(*self.0).min(1);limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,seen:u64,next:u64)->Result<bool,ValueError>{Ok(seen.checked_add(next).is_some_and(|n|n<=(*self.0).min(1)))}}
```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🏭️producer/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️decoding/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{if self.0{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}else{Ok(0)}}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(self.0&&additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{if self.0{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}else{Ok(0)}}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(self.0&&additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{Ok(0)}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(false)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=65536;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=65536))}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧾️receipt/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{if additional>self.available_work(observed)?{return Ok(false)}panic!("denied backing cannot reserve work")}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=1000000;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1000000))}}
```

### 🧰️framework/🔨️modules/🚪️io/⏱️control/🧾️failure/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.remaining;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧵️paged-text/🧪️.rs

```rust
impl NativeDecodeWorkRecipient for DLoan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,more:u64)->Result<bool,ValueError>{Ok(observed.checked_add(more).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for ActualWork<'_,'_,'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.turn.with(|cx,_|if !paused(cx,self.closing)&&(self.closing||!cx.is_cancelled()){cx.fuel_remaining().checked_sub(observed).unwrap_or(0)}else{0})}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{
  self.turn.with(|cx,_|!paused(cx,self.closing)&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))
 }
}
```

### 🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeCurrentWork<'_,'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{Ok(if !self.cancel.is_cancelled_now()&&(self.now)().is_some_and(|now|now<*self.deadline){self.fuel.checked_sub(observed).unwrap_or(0)}else{0})}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{
  Ok(!self.cancel.is_cancelled_now()&&(self.now)().is_some_and(|now|now<*self.deadline)&&observed.checked_add(additional).is_some_and(|work|work<=*self.fuel))
 }
}
```

### 🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeRetirementWork<'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{Ok(if *self.retirement||(self.now)().is_some_and(|now|now<*self.deadline){self.fuel.checked_sub(observed).unwrap_or(0)}else{0})}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok((*self.retirement||(self.now)().is_some_and(|now|now<*self.deadline))&&observed.checked_add(additional).is_some_and(|work|work<=*self.fuel))}}
```

### 🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧪️tests/🔬️custody/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.remaining.checked_sub(observed).ok_or_else(||ValueError::invariant("observed fixture work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔬️probe-document/📥️receiving/🧪️tests/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.0.checked_sub(observed).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original finite work was already exceeded"))}fn admits(&mut self,observed:u64,extra:u64)->Result<bool,ValueError>{Ok(observed.checked_add(extra).is_some_and(|value|value<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🧪️tests/🚪️control/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for Loan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->std::result::Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for EnabledWork<'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{if self.0.load(std::sync::atomic::Ordering::Relaxed){1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}else{Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode work is cancelled"))}}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{if self.0.load(std::sync::atomic::Ordering::Relaxed){Ok(additional<=self.available_work(observed)?)}else{Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode work is cancelled"))}}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for FiniteWork{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=1;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=1))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{Ok(1024u64.checked_sub(observed).unwrap_or(0))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1024))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OriginalMemberWork<'_,'_,'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.turn.with(|cx,_|if !paused(cx,self.closing)&&(self.closing||!cx.is_cancelled()){cx.fuel_remaining().checked_sub(observed).unwrap_or(0)}else{0})}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{self.turn.with(|cx,_|!paused(cx,self.closing)&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))}
}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for DriverWork{fn available_work(&mut self,observed:u64)->Result<u64,semio_framework_value::ValueError>{let limit:u64=self.quantum.min(self.remaining);limit.checked_sub(observed).ok_or_else(||semio_framework_value::ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.quantum.min(self.remaining)))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️testing/♻️retirement/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for NativeWorkWallet{fn available_work(&mut self,observed:u64)->Result<u64,semio_framework_value::ValueError>{let limit:u64=self.remaining.min(self.quantum);limit.checked_sub(observed).ok_or_else(||semio_framework_value::ValueError::invariant("native observed work exceeds its actual finite scope"))}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.remaining.min(self.quantum)))}
}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/🧩️composition-and-retained-decode/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for TurnWork{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0.min(4096);limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.0.min(4096)))}}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-child-member-registry/🦀️.rs

```rust
impl semio_plugin_document_media::NativeDecodeWorkRecipient for Work<'_, '_>{fn available_work(&mut self,observed:u64)->Result<u64,semio_framework_value::ValueError>{let cx=self.0.try_lock().map_err(|_|semio_framework_value::ValueError::invariant("original archive test caller is already borrowed"))?;if cx.retirement_should_yield(){return Ok(0)}cx.fuel_remaining().checked_sub(observed).ok_or_else(||semio_framework_value::ValueError::invariant("observed test work exceeds its original caller"))}
            fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{let cx=self.0.try_lock().map_err(|_|semio_framework_value::ValueError::invariant("original archive test caller is already borrowed"))?;Ok(!cx.retirement_should_yield()&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))}
        }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for StepWork<'_,'_,'_>{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let cx=self.0.actual.try_lock().map_err(|_|ValueError::invariant("original archive caller is already borrowed"))?;let paused=if self.0.closing{cx.retirement_should_yield()}else{cx.should_yield()};Ok(if !paused&&(self.0.closing||!cx.is_cancelled()){cx.fuel_remaining().checked_sub(observed).unwrap_or(0)}else{0})}
    fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{
        let cx=self.0.actual.try_lock().map_err(|_|ValueError::invariant("original archive caller is already borrowed"))?;
        let paused=if self.0.closing{cx.retirement_should_yield()}else{cx.should_yield()};
        Ok(!paused&&(self.0.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))
    }
}
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OperationWorkLoan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.fuel;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.fuel))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/identity-index-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=10_000_000;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=10_000_000))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/loads/receiver-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.limit;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.limit))}
}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.remaining;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/identity-index-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=10_000_000;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=10_000_000))}}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/loads/receiver-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.limit;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}
    fn admits(&mut self, observed:u64, additional:u64) -> Result<bool, ValueError> {
        Ok(observed.checked_add(additional).is_some_and(|total|total<=self.limit))
    }
}
```

### ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs

```rust
impl NativeDecodeWorkRecipient for Loan {fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.remaining;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining))}}
```

### ✏️s/🔌️plugins/🏗️fem/🧩️sqlite/🧮️projection/🧪️tests/🦀️.rs

```rust
impl semio_framework_value::native_decoding::NativeDecodeWorkRecipient for DecodeLoan{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.0;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|value|value<=self.0))}}
```

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/🦀️.rs

```rust
impl NativeDecodeWorkRecipient for StepWork{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{let limit:u64=self.quantum;limit.checked_sub(observed).ok_or_else(||ValueError::invariant("native observed work exceeds its actual finite scope"))}
 fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.quantum))}
}
```

### ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/🧪️tests/🧰️helpers/🦀️.rs

```rust
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for OriginalWork{fn available_work(&mut self,observed:u64)->Result<u64,semio_framework_value::ValueError>{let limit:u64=1<<40;limit.checked_sub(observed).ok_or_else(||semio_framework_value::ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,semio_framework_value::ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=1<<40))}}
```
