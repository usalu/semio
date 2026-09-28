# Generic Preparation Capacity Admission Review

Root reviewed the new generic Store preparation source while native6 was pending. This is source reasoning, not an observed native test result.

`RetainedClonePreparation::clone_grant` uses the Store turn’s `maximum_bytes` as both copy-byte and allocation-capacity grants. `StringCursor` and `VecCursor` phase zero reserve their complete final backing allocation and return zero progress when that allocation exceeds `maximum_capacity_bytes`. The new lifecycle fixture uses a 4,096-byte Store turn but only a scalar `DemoSnapshot`. It therefore does not establish that a real artifact with an 8 KiB text field can leave clone phase zero under that same grant.

The generic factory’s admitted retained footprint also uses the existing one-item maximum of 1,048,576 bytes. This is a separate operation admission limit, not proof of the requested larger-document acceptance capacity. Counting every reserve against that ceiling is honest but leaves the wider migration unfinished.

The execution owner was asked to add a neutral real Store large-text/collection law proving resumable progress or explicit bounded admission refusal and to document the exact distinction between operation retained capacity and per-turn copy work. No artifact migration is accepted on the scalar-only lifecycle proof. Simply raising the turn grant or accumulating credit without a bounded allocation design is not sufficient evidence of responsive large-document operation.
