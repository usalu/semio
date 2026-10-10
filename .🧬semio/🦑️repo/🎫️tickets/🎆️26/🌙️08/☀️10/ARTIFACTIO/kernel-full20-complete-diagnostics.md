# Complete Kernel20 Native Receiving Diagnostics

The unchanged registered full Kernel receiving gate completed with terminal1. Six genuine compiler errors prevented native runtime; no test acceptance is inferred. The original all-targets roster, mutation-testing feature, no default filter, build600000ms, assertion15000/300000/900000/1800000ms policies and one assertion thread were preserved. The durable terminal receipt is `🗑️generated/fd/kernel-full20-receipt.json`; raw log and actual canonical launch snapshot are adjacent.

Four errors belong to the original public SQLite native admission fixture observer namespace; Draw owns these receiver ports. Two errors are the new Store original peer authority law, proving the intentionally missing grant-bearing publication API and progress result. VCS owns those exact canonical producers. Production Kernel compilation passed these prior producer floors; no Plugin or Flow/Infinite acceptance follows from this run.

All six primary diagnostics follow, including their complete compiler contexts.

## 1. 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs:208

Raw log line34845.

```text
error[E0433]: cannot find `test_allocation` in `crate`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/../🦀️.rs:208:49
    |
208 |     let (refusal, allocated, released) = crate::test_allocation::observe_backing(|| bind_original_buffers(&mut record, &mut output,...
    |                                                 ^^^^^^^^^^^^^^^ could not find `test_allocation` in the crate root
```

## 2. 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs:213

Raw log line34851.

```text
error[E0433]: cannot find `test_allocation` in `crate`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/../🦀️.rs:213:48
    |
213 |     let (result, allocated, released) = crate::test_allocation::observe_backing(|| bind_original_buffers(&mut record, &mut output, ...
    |                                                ^^^^^^^^^^^^^^^ could not find `test_allocation` in the crate root
```

## 3. 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs:226

Raw log line34857.

```text
error[E0433]: cannot find `test_allocation` in `crate`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/../🦀️.rs:226:50
    |
226 |         let (step, allocated, released) = crate::test_allocation::observe_backing(|| retirement.step(policy).unwrap());
    |                                                  ^^^^^^^^^^^^^^^ could not find `test_allocation` in the crate root
```

## 4. 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs:231

Raw log line34863.

```text
error[E0433]: cannot find `test_allocation` in `crate`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/../🦀️.rs:231:43
    |
231 |     let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(retirement));
    |                                           ^^^^^^^^^^^^^^^ could not find `test_allocation` in the crate root
```

## 5. 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:270

Raw log line38536.

```text
error[E0061]: this method takes 0 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:270:101
     |
 270 |         let (begun, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store.begin_peer_publication(grant));
     |                                                                                                     ^^^^^^^^^^^^^^^^^^^^^^ ----- unexpected argument of type `semio_framework_value::RetainedCloneGrant`
     |
note: method defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:6007:12
     |
6007 |     pub fn begin_peer_publication(&self) -> Result<PresencePeersPublication<P>, String>
     |            ^^^^^^^^^^^^^^^^^^^^^^
help: remove the extra argument
     |
 270 -         let (begun, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store.begin_peer_publication(grant));
 270 +         let (begun, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store.begin_peer_publication());
     |
```

## 6. 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:271

Raw log line38553.

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:271:13
    |
271 |         let (mut publication, progress) = begun.unwrap();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^   -------------- this expression has type `os_store::component::PresencePeersPublication<presence_retirement::tests::Value>`
    |             |
    |             expected `PresencePeersPublication<Value>`, found `(_, _)`
    |
    = note: expected struct `os_store::component::PresencePeersPublication<presence_retirement::tests::Value>`
                found tuple `(_, _)`
```
