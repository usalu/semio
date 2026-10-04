# Native Control Current Boundary Audit

Source inspection only; no Cargo invocation, runtime validation, production edit, or test mounting. The reported Root 80/80 result is not independent evidence from this audit. No six-test producer/schema result is claimed; both producer test source files remain outside this audit's coverage.

Paths below are relative to the repository. Framework schema prefix: `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/`. OS DSL prefix: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/`.

## Current Boundaries

- Framework and OS `🧬️schema/🏭️producer/🦀️.rs:7–49` retain `ValueError` through the control trait, producer function pointers, and depth/stage admission. Framework binding `🪆️binding/🦀️.rs:15,214,258,303,367,444` and OS `🦀️.rs:49,248,292,337,401` retain typed shape/variant metadata errors. No premature message projection was found in those calls.
- Framework physical emitter `🛫️encoding/🦀️.rs:13,17` projects at the `TextError` return boundary; its recursive record/shape/intrinsic/expression calls remain typed. OS emitter has the same body; its diff is imports only. Framework decoding `🛬️decoding/🦀️.rs:19` projects schema production at the existing textual parser boundary.
- Framework derive `✨️derive/🦀️.rs:801,1000` and OS derive `✨️derive/🦀️.rs:1563,1660,1933` correctly put the inner textual result inside `Ok::<_,ValueError>(...)`, then map the outer depth refusal, then unwrap the outer result. Thus inner `TextError`/`String` retains its existing type and depth refusal is projected once. No `String::into_message` call was found here.
- Framework binding `🪆️binding/🦀️.rs:414` uses Display for its TextError projection. `🧰️framework/🔨️modules/🌱️value/⚠️refusal/🦀️.rs:20,22` confirms Display and `into_message` both expose exactly the message. No error-kind prefix or prose reconstruction occurs.
- Ticket `📋️json-controlled-flat-output-typed-draft.rs:6,13,62` keeps projection/frontier helpers typed and moves the message only when adapting to the existing record constructor's `TextError` callback. No concrete current API inconsistency was found in that adapter.

## Actionable Draft Inconsistency

The unmounted SVG shared XML drafts still use bare native control operations inside `Result<_,String>` functions. With the current typed native control API these need explicit terminal message projections, or typed internal helpers with one outer projection; they must not rely on an implicit `From<ValueError> for String`.

- `📋️svg-shared-xml-native-decoding-draft.rs:9`: `advance(...) ?`; `:22`: `borrow_text(...) ?` and directly returned `copy_text(...)`; `:31`: `charge(...) ?`; `:71`: `allocate_vec(...) ?`; `:104`: `begin_stage(...) ?`.
- `📋️svg-shared-xml-native-encoding-draft.rs:9`: `advance(...) ?`; `:78`: `begin_stage(...) ?`; `:80`: `begin_stage(...) ?` and `allocate_vec(...) ?`; `:81`: `checkpoint(...) ?`.

These are draft source findings, not compiled failures or mounted owner coverage. No draft was mounted or modified.
