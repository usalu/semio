# Remote cache unreachable fails a successful run

Root cause: `SEMIO_REMOTE_CACHE_ENDPOINT` (192.168.178.70:6090) declares a contract; `nxChildEnvironment` (`🔌️backend/🟦️.ts:254`) renders it into `NX_SELF_HOSTED_REMOTE_CACHE_SERVER`, and `nxRemoteCacheEnvironmentV1` (`🌐️remote-cache/🧶️nx/🟦️.ts`, called at `🔌️backend/🟦️.ts:454`) returned `{}` for a declared contract without ever probing. Nx's Rust client then failed its upload and the process admission wrapper reported "first failure".

Fix: a declared contract is probed once per process and endpoint (GET `/v1/cache/semio-probe`, 1500 ms bound, 200/404 = ok). Otherwise the Nx routing variables are removed and one bilingual warning (`warning.unreachable`) is printed.

Test: `🌐️remote-cache/🧪️tests/🟦️.ts` (closed port, hanging server, 401 server, reachable server). Result: remote-cache 20 pass/2 skip/0 fail; cas 35 pass.

Not covered: a server that dies after the probe mid-run.
