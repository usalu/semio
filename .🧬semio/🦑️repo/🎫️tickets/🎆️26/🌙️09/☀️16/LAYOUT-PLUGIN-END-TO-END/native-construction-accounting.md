# Native Construction Accounting

The original construction helper preflights its physical demand, enters the genuine existing Job receiver, executes the owner constructor, reconciles its reported physical receipt, and records reported allocation exactly once with NativeDecodeControl.record_constructed_allocation.

NativeDecodeControl.charge also increments owned_bytes and forwards allocation. Calling it inside the same helper callback for the same allocation double-debits the decoder. New host births, the shared tag registry backing allocation, and frozen-fault construction now allocate under their admitted demand and report actual capacity once; they do not also call charge. The helper still validates the physical quote and forwards actual reported allocation.

Parser operations differ: an incremental native parser already charges individual candidate/string allocation. Wrapping its repeated physical capacity in another constructor record also double-debits. The owning host-configuration agent is repairing only that actual parser hop using the genuine native receiving and reconciliation interface. Retirement charge APIs remain distinct and were not blanket removed.

The host native failure also exposed receiver bootstrap ordering: reading receiving_identity before the genuine native construction scope exists refused the authentic empty decoder. Birth now captures ledger/recipient identity inside the actual construction control. No identity is fabricated and the subsequent strict source/decoder/ledger fences remain.

The original registry fixture incorrectly expected128 slots; the existing declared ARTIFACT_LIVE_OUTPUT_SLOTS is64. The neutral fixture and independent model now use64, assert that exact declared contract, and drive genuine continuation progress without a four-attempt assumption. No runtime funding or slot bound changed. Actual host rerun is pending.
