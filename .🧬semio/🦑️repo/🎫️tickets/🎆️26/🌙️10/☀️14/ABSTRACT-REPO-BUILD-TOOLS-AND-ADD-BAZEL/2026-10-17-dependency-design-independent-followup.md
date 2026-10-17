# Dependencies Maintenance Design Follow-up

Read-only bounded review of the inactive-generation retirement and protected producer/rebind design reports. Neither proposal is implemented or physically accepted by this review; no protected input was read.

One concrete accounting clarification is required in the retirement design: do not derive a total logical-file-extent ceiling from physical allocation capacity. Multiple hardlink entries can contribute repeated logical lengths while consuming the same physical extents. Keep the existing independently defined logical/work/metadata limits explicit, preserve actual hardlink transition checks and retain the complete Native charge. An inventory that exceeds an actual logical contract must refuse; physical capacity alone does not establish that logical contract. This finding was sent to Interfaces.

Receipt-last deletion and the separate fresh-current empty-generation intent are a coherent failure boundary, provided the empty intent proves the exact current role/roster under actual parent/Core/global bindings/ALL owner custody and never invents the removed full generation identity or historical funding. Cancellation or barriers after visible deletion must remain unsuccessful with whole charge retained.

The producer proposal correctly keeps readonly context non-authoritative, requires fixed first-party writes under actual parent/Core/global EX/ALL owner custody, and separates unqualified symlink leaf replacement from adopting or deleting its former target. The implementation must reject unknown physical directories and must bind one canonical recipe across ordinary inspection/production/consumption and protected production. All-scope readiness before the first binding switch reduces partial transitions but cannot make multi-link switches atomic; every failed suffix remains an honestly observed partial transition.

A linker/configuration proposal supplies no path guarantee. The actual current root is already hoisted. Any isolated recipe must be a new canonical identity and must pass the unchanged complete installed-path guard before receipt or binding publication. No layout change, new logical bound, retirement authority or runtime qualification is accepted here.

Interfaces acknowledged and corrected the proposal before implementation: no allocation-derived logical ceiling; separate bounded read-work and safe logical counters, alias totals may exceed physical capacity, no freed-physical claim and whole charge retained. This is an attributed design correction, not a runtime acceptance.
