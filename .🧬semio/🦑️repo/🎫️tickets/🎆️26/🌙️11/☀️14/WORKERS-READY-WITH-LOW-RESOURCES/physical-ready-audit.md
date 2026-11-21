# Physical Worker Readiness Audit

At 08:35–08:37 UTC on 2026-11-21, origin ready.json records for g1-m05, g1-m11, and g1-m12 reported backoff, ready=false, workspace stage 5/5, exit 1, and null readyMs. Attempts were 12, 19, and 25 respectively. Their most recent workspace logs record an authorizer receipt refusal with reason output-read, followed by an authenticated derivative material reference error. Source-index progress and authorizer release activity are progress, not successful workspace completion.

The m01, m02, m03, m07, m08, and m10 records had fresh heartbeats and ready=true, but their readyMs dates from November 18: this is an earlier readiness result for their reported digest, not evidence that the changed workspace path was rerun. m04 and m06 records were stale since November 20 and reported running/ready=false. All physical workers therefore cannot be confirmed Ready.

The m05 attestation reported sync=true alongside an incoming queue of 2737, a stale view, and approximately 1.36 million blocked sharing violations. Its current readiness digest differed from the macOS workers. Sync=true alone does not establish convergence.

After fixing the authorizer failure, observe fresh per-origin readiness records, current expected digest, successful workspace completion, ready=true, and fresh heartbeat. Independently investigate stale m04/m06 records and m05 replication blockage. These observations are read-only; no workers were restarted.
