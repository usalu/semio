# Fleet Recheck

Read-only check 2026-11-20 14:55 PST. `/Volumes/semio-store` is mounted over NFS from `192.168.178.70:/Users/Shared/semio-store`; `df` reports 926 GiB total, 888 GiB used, 38 GiB available. This establishes mounted shared storage, not a healthy cache API, execution scheduler, worker capacity, exact artifact availability, or a safe way to bypass the protocols.

`repo remote-cache status --lang=en` still resolves fleet facts generation 9 to `http://192.168.178.70:6094`, instance `semio`, with credential present. Its three read-only probes all report unreachable, unauthorized, null status and no server/store statistics. `repo remote-execution status --lang=en` reports `configured: false`, null contract, scheduler, gRPC and server, and no worker token. Local `.cache/f/health.json` reports group store `ok`, build-agent running and build-scheduler/build-cache sidecars off. The recent local offload decline remains `infrastructure: cache endpoint unreachable`, and the latest shared unit event journal includes `remote-skip` with `breaker: down`. These observations are consistent with the earlier ticket audit, not evidence of a recovered fleet service. A mounted NFS path is not an execution worker or an exact-key cache hit.

`native-queue.sh` limits concurrent local component jobs with `JOBS`; its driver uses `native-lanes.sh` release mode and the normal `components describe ... --facet browser` path. The per-unit wrapper may attempt normal offload only under its policy and available admission, but there is no configured execution contract and the cache endpoint is unreachable. Increasing `JOBS` or changing queue ordering cannot make these services healthy. A future service recovery should be verified with normal status and actual exact-key hits/admission before anticipating benefit. No remote work, remote writes, publishing, global settings change or service start was performed.

The 17:03 operator recheck still reports an unconfigured execution contract and scheduler. All three authenticated cache status probes returned unreachable. Local module compilation continued.

At 17:34, remote execution was still unconfigured and the three configured cache probes were unreachable. The two module queues continued locally.

At 18:42 Pacific all three configured remote cache probes again reported unreachable and the execution scheduler remained unconfigured. Native compilation continued.

November 21, 07:54 PST. Current official cache status returned three unreachable read probes with no HTTP status. Remote execution status returned configured=false. The two admitted release roots continued on native lanes.

November 21, 10:40 PST. Remote execution still had no configured scheduler. All three cache probes reported unreachable. Both native slots remained in use.

November 21, 11:44 PST. The execution contract remained unconfigured and all three remote cache probes were unreachable. The local queue continued running both requested module roots.

November 25 at 17:57 Pacific. Fleet facts generation 13 supplied the Darwin ARM64 execution contract and scheduler. The live gRPC capability probe failed with ERR_HTTP2_STREAM_CANCEL. All three remote cache protocol probes also remained unreachable. Native compilation continued.

November 25, 18:32 Pacific. The configured generation-13 runner still returned ERR_HTTP2_STREAM_CANCEL to the gRPC capability probe. All cache services remained unreachable and status provided no server. The local compiler continued. The separately detached fleet cache uploader still held an active spool of 8,341 queued files, so it was left to its owner.
