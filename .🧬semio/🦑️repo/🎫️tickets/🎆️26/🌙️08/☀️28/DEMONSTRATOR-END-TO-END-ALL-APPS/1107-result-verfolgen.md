STATUS: UNVERIFIED (no gis smoke has run; last compile gate W-pane-gis-142 rc1). Blocked on semio-framework-dsl-record-model (55 errors left, all in dsl/🧬️schema/🫴️receiving) and replication-wire-core (port-a).

## Done this wave (all cited in 1105b-claims.md)
- retained-command dep key drift (semio-plugin-result-page) FIXED; job-payload, value-native-receiver-protocol (slot.rs), value-native-receiving-contract FIXED (verfolgen-103 rc 0).
- value/🧪️testing/🧾️wire admit_owned_source on EffectBank + value testkit dep (value + job-context + pack-json etc. then compiled past it, W-pane-gis-139).
- dsl/🧬️schema/🛫️encode/🧮️expression ported to the effect protocol (advance/close take RetainedAllocationEffect, RetainedWorkOutcome; RetireOwned seed). verfolgen-105: 0 errors in that file. NOTE: external consumer pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor calls the old grant signatures.

## dsl-record-model 📥️owned (option C), analysis for the next owner
55 errors, files: owned/{🦀️,🧬️contract,📦️storage,🔢️scalar,📋️vector,📋️persistent,📝️text,🔢️array,🎁️wrapping,🔄️mapping,🌳️intrinsic}, receiving/🦀️.rs (NativeSnapshotEncodeEffectQuote import), binding/{👯️pair,📋️persistent} (E0210 orphan impls), model/🦀️.rs.
Removed APIs they use: NativeDecodeControl::receive_retained_work, NativeOwnedStorage/NativeRecordStorage::{reserve,take_box} (new: reserve_demand / release_work_demand, effect-based), crate semio_framework_value_native_receiving_original (now NativeReceivingOriginal Poll in value/decode/owned), RetainedCloneSourceAdmission::{advance,take_output,return_inputs} take effect, ControlledRetirement::step(effect)->OriginalRetirementPoll, PagedList::{reserve_one_funded,release_empty_page}(effect).
Template: pack/🔤️json/📥️decode/🪪️native-input (resident NativeReceivingChild + NativeLocalAuthority; `action.poll(control,demand,grant,quote,max_work,authority,|control,effect,authority,action| authority.drive(&mut NativeLocalOperation::Poll(&mut |effect| op(control,effect)),effect,action))`), tool-run tick/receive/native (`pure(effect)` helper), my expression port (pure/reserve/release patterns).
Callers that must move with the trait: dsl/🧬️schema/✨️derive/📥️owned (macro output), 🪞️transparent, 🫴️receiving/🦀️.rs, os dsl/🫴️receiving, store codec native-snapshot receiving, space collection body-receiving, tests.
Size: ~87 KB dense code in 10 modules plus derive; estimate many compile iterations. Recommend a fresh lane with its own context; claim 08:50 is released by this note.
