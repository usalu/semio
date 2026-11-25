# setContributions stalls: status from current source (2026-11-25)

Method: static reading of current source and the ticket record. No cargo run: `repo capacity status` showed 3.4 to 4.2 GiB available (needs > 4.5 GiB), so nothing here is executed.

## Defect map (guest dates are older than most fixes)
| Defect | Seen in | In current source | Proven |
|---|---|---|---|
| Lease livelock: fresh grant released by idle publication, command answers `command-pending` forever, lane held, then 4096-unit stall fault | cad, process (Nov 21), sourcing (Nov 22), also Aussuchen/Bearbeiten | fixed: `Lease.armed` in `instance/dispatch/staging/host-event-routing` (file Nov 23 19:21), plus pending-holder retention | Rust-vs-Python oracle only; never in a browser |
| Silent retry of deterministic refusals until the 4096 fault | all | fixed: `deterministic_preparation_refusal`, `fault_refused_typed_operation_publication` | law fixture; not run now |
| Ledger blames ownership instead of work | demonstrator | fixed: `work_conserved` split (return-escrow) | unit test green on Nov 24 |
| Quote ignores held Source fuel (`native-return-refused` work-limit) | demonstrator Nov 23 | applied: `with_original_effect_at` now `let fuel=self.fuel_remaining()` (Nov 25 03:41) | NOT proven: see below |
| `one-item publication blocked` x4094 in Koordinator/Aussuchen | demonstrator | NOT root-caused (0200 note item 3); no source change targets it | open |

cad, process and sourcing guests all predate the lease fix, so their symptoms (hang in Set Active Example, 4096-unit stall) are the lease/stall class and are presumably fixed in source, but this is unverified until those guests are rebuilt. The cad and process guests were not rebuilt (they stay withheld via `withheldExtensions`). Sourcing's 4096 message is the same stall fault string, so it is the same class; the Blocked cause behind it remains unidentified.

## The with_original_effect_at change
Reading: the change is consistent. `fuel_remaining()` is the same clamp (`min(field, source.remaining().fuel)`) that `charge_work_units`, `charge_work_unit` and `charge_item` already use; it equals the field when no Source is bound. Risk: when the ledger already carries a sticky refusal, `remaining()` errs, the quote becomes 0 and the effect returns `None`, which callers read as "retry later". That turns an overrun into a silent no-progress turn, which is the same shape as the unexplained Blocked/stall.

Regression test written, NOT run: `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/held-fuel-quote.rs` (module registered in that `🦀️.rs`). It opens a receiving scope (held half share), asserts `held_fuel > 0` and `fuel_remaining() <= turn_fuel - held_fuel`, then asserts the plain effect's ceiling `maximum_work_units()` equals `fuel_remaining()` and that performing exactly that much is admitted. I did not perform a deliberate overshoot because an over-ceiling `performed` still lands in the ledger and cannot discriminate. Open points: the test assumes the receiving claim is still held between `with_native_receiving` and `consume_receiving_retained` (its `held>0` assertion will say if not), and it was not compiled.

Run when capacity allows: `repo cargo --store=shared --configuration=prototype -- test -j 2 -p semio-framework-job-context --test original-native-progress held_fuel`. If it fails on `held>0`, the construction is wrong; if on the ceiling, the change is wrong.

## Next
Probe line `[DEBUG] return ledger first work refusal` (see work-limit-root-cause) in the next demonstrator build decides between this quote defect and the Blocked stall.
