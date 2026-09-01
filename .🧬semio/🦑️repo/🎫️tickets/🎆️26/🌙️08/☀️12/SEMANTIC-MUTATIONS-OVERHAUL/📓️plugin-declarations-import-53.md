# Plugin Declaration Import Join

Runtime's actual const-mode RED compile reported E0252 for duplicate declarations imports in Plugin testkit, separately from its four intended const-argument errors. No tests ran.

The fixture executor removed only the later redundant import and its attached cfg(test) attribute. The first unconditional import and all declaration helper bodies remain. Root inspected the current region and independently confirmed main SHA256 b115fb7e44e311352da1222292712817b17cb84e8422ddf61031c7da257f0d3e. The executor recorded pre-edit SHA256 7f7b17b8beabde935b839e5c512bd6ffe91ac9862b52ebde3ee3d96b693181f8.

The executor placed its initial report in the runtime ticket at 📓️plugin-declarations-import-repair.md. It remains preserved there; this record establishes mutation-lane source ownership. No Cargo or publication was run here. Native verification remains runtime-owned.
