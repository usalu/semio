# Current Full Store Native Receipt

Actual uncached registered kernel store:: native run: 546 run, 543 pass, 3 fail, 681 outside filter; runtime231.666s, Nx21m48s exit1. Log `🗑️generated/store-full-common-group-prepared-read-current.log`. This includes current public common-group/prepared-read and prior clone/history/reprojection repairs, which passed. No entire Store green claimed.

### edit_message_clamp_settlement_cancels_at_every_owned_stage


@semio-tech/framework-os-kernel: thread 'os_store::component::edit_message_clamp::settlement::tests::edit_message_clamp_settlement_cancels_at_every_owned_stage' (2266854) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📨️messages/✂️clamp/🔁️settlement/🦀️.rs:130:9:
@semio-tech/framework-os-kernel: settlement cleanup did not terminate
@semio-tech/framework-os-kernel: note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
@semio-tech/framework-os-kernel: test os_store::component::edit_message_clamp::settlement::tests::edit_message_clamp_settlement_cancels_at_every_owned_stage ... FAILED
@semio-tech/framework-os-kernel: failures:

### retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open


@semio-tech/framework-os-kernel: thread 'os_store::component::presence_retirement::tests::retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open' (2267067) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:159:5:
@semio-tech/framework-os-kernel: assertion `left == right` failed
@semio-tech/framework-os-kernel:   left: false
@semio-tech/framework-os-kernel:  right: true
@semio-tech/framework-os-kernel: note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

### member_open_partial_parse_and_initialization_owners_retire_exactly


@semio-tech/framework-os-kernel: thread 'os_store::component::tests::member_open_partial_parse_and_initialization_owners_retire_exactly' (2274178) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:2032:9:
@semio-tech/framework-os-kernel: retained owner must reach terminal empty across interruptions
@semio-tech/framework-os-kernel: note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
@semio-tech/framework-os-kernel: thread 'os_store::component::tests::member_open_partial_parse_and_initialization_owners_retire_exactly' (2274178) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:558:9:
@semio-tech/framework-os-kernel: member-open reached Drop before exact adoption or bounded rejection retirement

## Clamp Exact Authority Repair

The cancellation case uses copy1/7 as physical release authority for a partial MessageCopyCursor, whose genuine whole allocation is larger. Canonical settlement now carries separate explicit copy/release arguments, forwarding release to cleanup and copy-cursor close while keeping original copy limits. Existing neutral settlement corpus declares release4096 and independent TextEncoder/JSON Patch copy-release oracle. The wrapper forwards exact current owner close demand. All cancellation stages, original worst/status and completed_work assertions remain. Production caller explicitly passes4096 for each axis. Current native66796/source94972 are pending in `🗑️generated/store-clamp-settlement-copy-release-current.log` and `🗑️generated/store-clamp-settlement-copy-release-source.log`; no green claimed.
