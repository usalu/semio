# Native Current Compiler Integration

Actual `workspace-prototype-actual4.log` observations are grouped below. Concurrent domain changes mean every repair must first compare current source; the old UI step signature diagnostic already differs from the current job trait. No legacy variants or protocol aliases will be introduced.

## 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️transition/🛂️payload/🦀️.rs

- E0603 at 2: struct `RetainedWireUtf8Validation` is private

## 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs

- E0053 at 3579: method `step` has an incompatible type for trait
- E0061 at 2697: this method takes 1 argument but 0 arguments were supplied
- E0600 at 2697: cannot apply unary operator `!` to type `Result<bool, ValueError>`

## 🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs

- E0277 at 40: the trait bound `original_job_source::OriginalJobCallFailure: Copy` is not satisfied

## 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs

- E0053 at 1545: method `close_step` has an incompatible type for trait
- E0599 at 1436: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 1437: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 1438: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 1443: no variant named `Refused` found for enum `InteractiveJobCloseStep`

## 🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/🤝️provider/🦀️.rs

- E0277 at 39: the trait bound `retained_turn::RetainedTurnInput: RetireOwned` is not satisfied
- E0277 at 44: the trait bound `retained_turn::RetainedTurnInput: RetireOwned` is not satisfied

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🦀️.rs

- E0053 at 3480: method `close_step` has an incompatible type for trait
- E0308 at 3270: mismatched types
- E0599 at 3271: no method named `progress` found for enum `std::result::Result<T, E>` in the current scope
- E0599 at 3277: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3488: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3494: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3506: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3521: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3526: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3533: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3537: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3552: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3556: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3564: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3569: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3591: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3596: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 3597: no variant named `Refused` found for enum `InteractiveJobCloseStep`

## 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs

- E0599 at 920: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 921: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 924: no variant named `Refused` found for enum `InteractiveJobCloseStep`

## 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs

- E0308 at 1621: mismatched types
- E0308 at 2432: mismatched types
- E0308 at 508: mismatched types
- E0308 at 510: mismatched types
- E0308 at 518: mismatched types
- E0308 at 520: mismatched types
- E0308 at 521: mismatched types
- E0308 at 522: mismatched types
- E0599 at 1621: no method named `admit` found for enum `Result<T, E>` in the current scope
- E0599 at 1622: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 2432: no method named `admit` found for enum `Result<T, E>` in the current scope
- E0599 at 2439: no variant named `Refused` found for enum `InteractiveJobCloseStep`
- E0599 at 511: no variant named `Refused` found for enum `WorkerJobCloseStep`
- E0599 at 512: no method named `progress` found for enum `Result<T, E>` in the current scope
- E0599 at 523: no variant named `Refused` found for enum `WorkerJobCloseStep`
- E0599 at 524: no method named `progress` found for enum `Result<T, E>` in the current scope

## Current Repair Scope

UI flex and mounted layout retain grant-based private helpers with `Result<CloseStep, ValueError>`. The mounted layout actual job provider receives its original `StepContext`, verifies the terminal witness, debits retained effects once, and returns `OriginalTurnFailure` for primary and receipt refusals. OS owned schema still requires its corresponding direct original context repair. Existing domain neutral close/receipt tests and actual compiled current owner are the validation boundary.
