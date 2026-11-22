# Independent Telemetry for the Silent Origins

At approximately 18:34 UTC on November 21, a bounded read of the coordinator's configured replica store still found no fresh independent telemetry for the three silent origins.

| Origin | Latest independent pool record | Latest Ready record | Latest attestation |
| --- | --- | --- | --- |
| g1-m04 | November 20 23:53:12 UTC | November 20 23:53:10 UTC | November 20 23:43:11 UTC |
| g1-m06 | November 20 18:25:11 UTC | November 20 18:25:11 UTC | November 20 18:25:12 UTC |
| g1-m09 | Absent | Absent | Absent |

The coordinator's configured origin folders also held historical journal heads for g1-m04 and g1-m06. There was no head or non-authentication origin file for g1-m09. Nearby g1-m03 published pool, Ready and attestation files at 18:34:30–32 UTC, showing that the local mirror was receiving current origin records. Local scheduler and pool directories had no directly origin-named telemetry for the three requested origins. This bounded sample supplies no evidence of current reachability for the three machines. It accessed local metadata only and did not attempt any remote operation.

[Earlier independent sample](audit-silent-independent-1515.md)
