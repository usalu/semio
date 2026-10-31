# Live Maintenance Test Cache Policy

The existing neutral MaintenanceVerification contract requires cache:false, and the actual test target runs genuine shared graph writer and Native image/wait custody checks. Cached source-only success cannot prove current physical host state. The actual filtered regression failed0pass/1fail at cache actualtrue/expectedfalse (schema-maintenance-cache-red.log). Corrected only target.cache to false, preserving the same runtimeSourceEntries, source freshness, inputs, outputs, command, Native tests and received Original. No fixture or guard changed. Actual filtered current verification is pending.

Actual normal-admitted current37173 passes the existing filtered contract:1test41assertions0,19other cases filtered. This proves registration/neutral owner policy and Source dispatch, not the full Native maintenance suite. No cache or custody guard was relaxed.
