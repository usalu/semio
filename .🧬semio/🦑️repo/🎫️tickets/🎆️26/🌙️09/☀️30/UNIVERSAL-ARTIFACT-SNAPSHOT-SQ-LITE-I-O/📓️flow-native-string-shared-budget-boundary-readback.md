# Flow Native String Shared Budget Boundary Readback

The shared decoder intentionally preserves the native allocation budget independently of SQLite semantic value bytes. Actual Store `snapshot-capability/native-decoding` supplies `allocation_stage`'s remaining allowance to `NativeDecodeControl`, decodes the physical record and calls the typed constructor with that same control. It does not apply `max_value_bytes` to every native string. Root owns the Flow provider's domain semantic check; this audit changes no Flow production or test region.

Actual `NativeDecodeControl::copy_text` charges the complete literal byte length before `try_reserve_exact`, then copies with cancellation boundaries every 64 KiB. `DslField for String::from_value_controlled` calls that method directly. Core controlled contiguous UTF-8 materialization checks the complete length against both its own maximum and native remaining allowance before borrowed validation and paid copy. The paged UTF-8 path performs those same full-length checks, validates the immutable source, then allocates through its admitted octet buffer and transfers the identical buffer. No path inspected here charges only String metadata for an owned string.

The shared Store allocation laws explicitly decode successfully with a smaller `max_value_bytes`, while separately pinning full literal allocation settlement and refusal at exhausted native backing. This is the existing shared contract, not evidence that Flow's domain semantic cap is satisfied. A Flow provider must apply its semantic value extent in addition to shared allocation accounting. The reported Flow refusal gap therefore does not justify a primitive ByteSpan or NativeDecodeControl repair from this readback.

Inspected sources: `store/codec/snapshot-capability/native-decoding` lines 14–47; native value decoding lines 54–81; DSL String binding lines 170–190; Core controlled materialization lines 2050–2069; shared Store allocation tests lines 27–56. This is static readback. Root's separate actual Flow gate and forthcoming domain repair retain their own runtime authority.

## Completed Independent Owning Receipts

After real prepaid Core sink and operation measurement providers, the actual Core package replay completed 133/133 passed, zero skipped, 6.601 seconds assertions and 19.0 seconds Nx. Log: `🗑️generated/operation-pages-core-package-regressions-after-real-prepaid-sink.log`.

The selected Replication byte-owner replay completed 7/7 passed, 266 filtered, 0.139 seconds assertions and 10.5 seconds Nx. Log: `🗑️generated/operation-pages-measurement-native-after-provider.log`. These receipts cover the current byte owner, reader, comparison, limiter, prepaid backing, empty exact owner and nonallocating measurement laws. They do not promote the still-unadopted OpBinary/ChildEmit or global immutable frame family.
