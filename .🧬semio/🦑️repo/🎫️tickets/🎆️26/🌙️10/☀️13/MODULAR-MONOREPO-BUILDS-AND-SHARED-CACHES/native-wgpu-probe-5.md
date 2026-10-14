# Current WGPU Probe 5

Actual prototype Nx WASM producer reached the current Rust compiler using the shared existing profile. It exited 1 with 20 semio-framework production type errors, before World/EngineCanvas compile. No full workspace or green leaf claim.

```text
error[E0277]: the trait bound `CapabilityToken: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:3666:9
     |
3662 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
3666 |     pub token: CapabilityToken,
     |         ^^^^^ unsatisfied trait bound
```

```text
error[E0277]: the trait bound `CapabilityToken: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:3666:16
     |
3666 |     pub token: CapabilityToken,
     |                ^^^^^^^^^^^^^^^ unsatisfied trait bound
     |
help: the trait `RetireOwned` is not implemented for `CapabilityToken`
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:68:1
```

```text
error[E0277]: the trait bound `ActorInstanceOpenRequest: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2593:9
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
2593 |         request: ActorInstanceOpenRequest,
     |         ^^^^^^^ the trait `RetireOwned` is not implemented for `ActorInstanceOpenRequest`
```

```text
error[E0277]: the trait bound `AppInstanceId: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2594:9
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
2594 |         app_id: AppInstanceId,
     |         ^^^^^^ unsatisfied trait bound
```

```text
error[E0277]: the trait bound `ActorInstanceCloseRequest: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2586:10
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorInstanceCloseRequest`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `ActorInstanceLifecycleAck: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2586:10
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorInstanceLifecycleAck`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `CommandPageCursor: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2615:9
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
2615 |         cursor: CommandPageCursor,
     |         ^^^^^^ the trait `RetireOwned` is not implemented for `CommandPageCursor`
```

```text
error[E0277]: the trait bound `FixedCommandPage: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2616:9
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
2616 |         bytes: FixedCommandPage,
     |         ^^^^^ the trait `RetireOwned` is not implemented for `FixedCommandPage`
```

```text
error[E0277]: the trait bound `ColdDocumentPairPage: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2586:10
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
     |
help: the trait `RetireOwned` is not implemented for `ColdDocumentPairPage`
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📥️cold-pair/🦀️.rs:63:1
```

```text
error[E0277]: the trait bound `ActorUiPatchReceipt: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2644:9
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
2644 |         receipt: ActorUiPatchReceipt,
     |         ^^^^^^^ the trait `RetireOwned` is not implemented for `ActorUiPatchReceipt`
```

```text
error[E0277]: the trait bound `ActorUiPatchReceipt: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2651:9
     |
2586 | #[derive(semio_framework_value::RetireOwned)]
     |          ---------------------------------- required by a bound introduced by this call
...
2651 |         receipt: ActorUiPatchReceipt,
     |         ^^^^^^^ the trait `RetireOwned` is not implemented for `ActorUiPatchReceipt`
```

```text
error[E0277]: the trait bound `ActorInstanceOpenRequest: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2593:18
     |
2593 |         request: ActorInstanceOpenRequest,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorInstanceOpenRequest`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `AppInstanceId: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2594:17
     |
2594 |         app_id: AppInstanceId,
     |                 ^^^^^^^^^^^^^ unsatisfied trait bound
     |
help: the trait `RetireOwned` is not implemented for `AppInstanceId`
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:124:1
```

```text
error[E0277]: the trait bound `ActorInstanceCloseRequest: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2601:19
     |
2601 |     InstanceClose(ActorInstanceCloseRequest),
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorInstanceCloseRequest`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `ActorInstanceLifecycleAck: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2602:26
     |
2602 |     InstanceLifecycleAck(ActorInstanceLifecycleAck),
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorInstanceLifecycleAck`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `CommandPageCursor: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2615:17
     |
2615 |         cursor: CommandPageCursor,
     |                 ^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `CommandPageCursor`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `FixedCommandPage: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2616:16
     |
2616 |         bytes: FixedCommandPage,
     |                ^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `FixedCommandPage`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `ColdDocumentPairPage: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2619:26
     |
2619 |     ColdDocumentPairPage(ColdDocumentPairPage),
     |                          ^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
     |
help: the trait `RetireOwned` is not implemented for `ColdDocumentPairPage`
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📥️cold-pair/🦀️.rs:63:1
```

```text
error[E0277]: the trait bound `ActorUiPatchReceipt: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2644:18
     |
2644 |         receipt: ActorUiPatchReceipt,
     |                  ^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorUiPatchReceipt`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

```text
error[E0277]: the trait bound `ActorUiPatchReceipt: RetireOwned` is not satisfied
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️.rs:2651:18
     |
2651 |         receipt: ActorUiPatchReceipt,
     |                  ^^^^^^^^^^^^^^^^^^^ the trait `RetireOwned` is not implemented for `ActorUiPatchReceipt`
     |
     = help: the following other types implement trait `RetireOwned`:
               &'static str
```

