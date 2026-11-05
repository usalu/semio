# History Source Work Port Map

Read-only current source; no edit or native job.

Owner: framework OS modules/📡️spr/📜️history/🛬️decode/🗂️file/🦀️.rs.

step :37 takes original RetainedWireByteSource, mutable NativeDecodeControl, RetainedCloneGrant and returns Result<RetainedHistorySourceStep,RetainedHistoryOpMetaFault>. close_step :82 uses dyn NativeDecodeCloseControl and same Result. Its receipt/take_receipt APIs carry only (grant,physical progress); no actual work counter exists in this owner.

| Selected path | Actual local or child action |
| --- | --- |
| Entry gates | source/control validation, blocked for pending receipt/closed/closing; denied demands before selected action |
| Auxiliary | prior owned retirement and displaced identity custody |
| Frames :44/:60 | verified source frame step; separately funded frame transfer; decoder admission |
| Dictionary | bounded scalar prefix and one dictionary entry frontier |
| Child :61–66 | Edit/Transition/Conflict/Identity step, take_fault, take_ready; same child physical receipt accepted |
| Validation :42 | transition payload validation step, take_validated, close, final receipt identity check |
| Identity | install_identity transfers selected authored identity and retires old identity |
| Closing :82 | actual child close; validation close; clear validated receipt; dictionary/records/partial retirement |

The Child enum at :12 currently delegates take_receipt and cancellation, not work. Nested RetainedHistorySourceFrames also returns Result plus physical receipt, so adding a getter to the parent cannot recover genuine child attempts without porting their actual selected action counters. Preserve already completed child work before any later Result error.

Production consumers include original history wrapper 📜️history/🦀️.rs (decoder field :81, admission :306, close :421), Store native-snapshot receiving/📜️pair phase5, and Plugin document maintenance ladder history path. Window pair loader is downstream through pair. Existing History wrapper must forward the real work, not infer copied_items.

Existing permanent fixture/test seam: file/🧪️tests/🧫️fixtures/🔣️.json and file/🧪️tests/🦀️.rs; frame companion file/📚️frames/🧪️tests/🦀️.rs independently verifies committed header/frame hashes with blake3, source/grant/recipient mismatch refusal, denied currencies, cancellation, and actual backing observations. Metadata child tests already inspect last_turn_work on their own native owner; reuse those existing honest counters where present.

Source caps were not re-inferred in this read-only audit. Do not lift package quotas; keep a separate narrow test target if new fixture scope would exceed its current source closure.
