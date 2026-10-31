# Capacity Lock Acquisition Repair

A normal native filling run failed before Python with EPERM while atomically creating the host-capacity lock directory. The gate now retries only EPERM/EBUSY creation failures using the existing bounded concurrent-edit schedule (10,40,160,640,2560ms; six attempts). EEXIST continues through the existing exact-owner/staleness fence. EACCES and unrelated errors fail directly. No admission limit, host-memory floor, grant policy, ACL or unrelated process changes were made.

Owner publication uses exclusive file creation. A peer owner appearing after directory creation is preserved and the earlier taker returns false. Existing release and growth fencing remain intact.

Schema-first portable fixtures cover two transient sharing denials, a transient busy directory, exhausted sharing retries, permanent access denial, and a peer taking during retry. Independent Ajv validation and RxJS retry traces reproduce the expected results. The owner-publication race is a separate filesystem test. Before implementation, the combined cohort had9 passing and6 failing tests. After implementation, all15 tests/40 assertions passed in398ms, including existing release, stale-holder, growth and progress fencing. The run used normal capacity admission.

Changed source: host-capacity gate and co-located acquisition JSON/schema/TypeScript test. Ticket wrapper and artifact preload keep test scratch paths under generated.
