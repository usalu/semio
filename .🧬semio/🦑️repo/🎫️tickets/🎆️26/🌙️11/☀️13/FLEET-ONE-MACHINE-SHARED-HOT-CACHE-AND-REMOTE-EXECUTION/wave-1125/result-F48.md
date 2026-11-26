# Result F48: host-name state in `repo fleet status`, F47 tests run (22:59)

## 1. Host-name state in fleet status (schema-first, tested)
- Schema (`🚢️fleet-status/🧬️schema/🔣️.json`): optional `hostName {state: ok|duplicate|named-as|unknown, label|null, others[]}` on `WorkerRow` and `WorkerReportRow`; texts `workers.host.*` and findings `worker-host-name-duplicate|named-as` (warn), en + de, each with the hint to run `repo fleet hostname apply` on that machine.
- Reader `readFleetHostNamesV1` (`📥️readers/🟦️.ts`): reads only the `hostLabel` of every `S/v1/origins/*/addresses.json` and applies the same `fleetAdvertiseGateV1` the agents run. Sets of agents that do not run the new code yet (no `namedAs`) still count. Without a label: `unknown (nothing published yet)`.
- Human output: `, host name ok (<label>)` / `, host name <label> is also on <others>: run \`repo fleet hostname apply\` on this machine` / `... belongs to <origin>: ...`. `--json`: `workers.rows[].hostName`.
- Test `fleet status shows the host name state per origin ...` (fleet-status tests 29/0): reader on a scratch store (ok, duplicate with `-2` suffix, an old-format file, named-as, null, missing), schema validation of facts and report, en/de texts, and findings equal to the Python oracle (`🐍️.py` got the finding and a seeded `hostName`, so the "every finding text is produced" law still holds).

## 2. Live (22:56, `repo fleet status`)
```
g1-m01 ok (ueli-workstation-mac-mini-m6-16gb-256gb-01-lan)   g1-m09 unknown (nothing published yet)
g1-m02 ok (...-m4-...-01-wlan)                                g1-m10 duplicate uelis-mac-mini, also on g1-m08, g1-m13, g1-m14
g1-m03 ok (...-03-lan)                                        g1-m11 ok (...-08-lan)
g1-m04 unknown (nothing published yet)                        g1-m12 ok (...-07-lan)
g1-m05 ok (desktop-6nsleuo)                                   g1-m13 duplicate uelis-mac-mini, also on g1-m08, g1-m10, g1-m14
g1-m06 ok (...-04-lan)   g1-m07 ok (...-02-lan)               g1-m14 duplicate uelis-mac-mini, also on g1-m08, g1-m10, g1-m13
g1-m08 duplicate uelis-mac-mini, also on g1-m10, g1-m13, g1-m14   g1-m15 ok (mac)
g1-server ok (ueli-mac)
```
Four findings (m08, m10, m13, m14), all with the bilingual hint. `g1-server` shows `ok (ueli-mac)` because its published `hostLabel` is still the old name: its agent has not republished since the LocalHostName became `semio-g1-m08`; once it does, the same gate reports `named-as g1-m08`. (The coordinator's current OS name was only confirmed by the brief, not by fleet code.)

## 3. F47 tests run one file at a time
- Pass: sealed `🧪️tests/🟦️.ts` 7/0, sealed `🖥️group.ts` 2/0, provenance (evidence) 6/0, fleet identity 9/0, remote-cache `🧪️tests/🟦️.ts` 27/0, graph exchange evidence 1/0, fleet-status 29/0.
- signed-fleet cluster test: three stale-harness defects fixed in `🔏️signed-fleet/🟦️.ts`: (a) the scheduler could not start (`no fleet facts for the scheduler yet`): facts carry no address and a loopback test store publishes none, so the harness seeds `.cache/f/last-good.json` (cache for every machine, execution for agents and requester); (b) it read `cache.endpoints[0]` from the facts, now `cache.url`; (c) agents joined without a worker (macOS partition guard, same as the sidecar test): the harness attaches a 96 MB APFS image per agent worktree and detaches in `afterAll`.
- STILL FAILING (not fixed): the offload step. Run 2: `both agents report headroom` timeout (90 s); run 3: both workers registered with the scheduler, but 0 assignments, the unit waited 120 s and fell back (`offload-declined queue-timeout`). Not proven to be a product defect or load: scheduler and agent logs show no error; assignments never started. Next step: print the submitted action's platform properties against the worker's `offering rustc for 8 target(s)`, and test with the F44-era tree for an A/B.
