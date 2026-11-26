# Draw Final Placement and CAS Refusal

The former final waited in the offload wrapper from 02:40:59 through 02:54:17 CET on November 26. Its action key was 9123085640758267c74ccf8e5680ef28a97a637ca596810da28bedd1addc7aff. The component driver reported 868 compiled and 41 reused, zero local actions, and one pending fleet action. HTTP 507 affected multiple input digests before scheduler submission. The process emitted offload-local-fallback for each infrastructure attempt, while the outer beyond-host policy prevented local compilation. The driver terminated on SIGTERM.

The active sealed wrapper 866f3e63bb8cffab classified the final using both local and hypothetical fleet estimates. The unit cache held 2,720 MiB at one backend thread. Fleet placement uses four threads and charges 512 MiB per extra thread, giving 4,256 MiB. The old check used the larger value and put the final beyond a 16,384-MiB host's 4,096-MiB final ceiling. The local scheduler runs managed cdylibs with a one-thread front end and the no-parallel-backend rustc flag. The live invocation carried the default frontend threads=0 for the scheduler to rewrite. Its 16 codegen units described work partitions. The native wrapper now declares its serial local final mode and uses only measured one-thread evidence for this host classification. Fleet placement keeps its independent forecast.

All eight prior successful prototype WASI records for the exact final key had peaks between 1,290.6 and 3,293.4 MiB. The newest was 1,290.6 MiB. The R-7 p95 is 3,290.77 MiB, for a 3,784.39-MiB forecast after the broker's 15% headroom multiplier when these records are eligible. The observed 1,685–1,711 MiB of parent overhead puts the total forecast at about 5,469–5,496 MiB. The paid 6,144-MiB single-job restart has room for this forecast. The separate live physical guard on a 16,384-MiB Mac preserves 2,457.6 MiB, so the compiler forecast needs at least 6,242 MiB reported available at start. The new exact input evidence can change the forecast.

Managed component finals with a verified parent now wait for observed physical fit. Unknown availability and insufficient physical room after the former 180-second idle fallback stay held. The warden and swapping checks remain in force. Predictions above the parent reservation fail early with parent-memory. Rolling noncompiler overhead can temporarily hold a within-reservation unit. The 4,096-MiB runtime ceiling is unchanged.

## CAS Recovery

Fleet facts generation 13 named one CAS at http://192.168.178.70:6094 and scheduler at http://192.168.178.70:8980. The active process inherited that CAS. Read-only status showed successful authorization and reachability, with 17,628 CAS rejections and zero CAS puts in that snapshot. The ~73 MiB of entries and ~2,457.5 MiB maximum do not capture all transient leased content or prove partition headroom. The server sends 507 for capacity refusal and can include its effective cap, partition free space, reserve, and top holders in the full response or owner diagnostics. The wrapper truncates the body in its per-attempt log.

Beyond-host waits nominally for 60 minutes, retries infrastructure and no-idle-machine after 5 seconds, and memoizes storage-full reports for 30 seconds. The 507 upload attempts took 38–50 seconds including individual requests and retry delay. The outer deadline is checked between attempts and supplies the remaining time to the scheduler as the queue budget. An ongoing attempt can outlast the nominal deadline. Other permanent refusals return immediately. On terminal beyond-host refusal, the wrapper returns status 1 without starting rustc. The retired invocation's nominal cutoff was about 03:41 CET.

Restoring CAS writes in the configured service was the lowest-risk way to recover the active remote action. Repo supports explicitly configured matched endpoints, and live attempts reread published fleet facts. The machine had no alternate published pair. Selecting another worker behind the same CAS leaves the shared input upload failure in place.

## Normal Entry and Checks

The parent resumed through the established registered component driver and the warmed prototype lane, with the normal paid capacity, broker and warden. The active command uses SEMIO_OFFLOAD=never and SEMIO_COMPILER_PARALLELISM=1.

```sh
repo capacity run --memory=6144 --jobs=1 -- repo tools exec --light bun -- \
  "$T/draw-native-dev.ts" "$T/🗑️generated" component-carved26 \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" \
  component --manifest "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml"
```

The focused offload owner passed 13 tests. Its Python oracle generated a language-neutral fixture spanning the exact 2,720/4,256 case, one-thread-only, remote-only, mixed and above-ceiling observations. The broker parent, growth and replay owners passed 18 tests. Independent SQLite arithmetic and live socket tests covered the parent, guard, unreadable availability and physical timeout cases. Before the physical fix, the same 5,996-MiB host sample started the final after the idle wait bound. Both JSON contracts remained within the existing owner scope. The role carve's nine-file patch was reapplied against the member-decoder lock and owner registry and passed independent diff, TOML and Ajv checks. The resumed component build supplies actual fresh WASI peak measurements and publication evidence.

Owned tracked files are the Cargo unit-cache script, the remote offload barrel and admission, the offload unit-admission Bun, Python and JSON fixtures, native admission, and the native parent fixture, parent broker law and replay chain law.


## Authenticated retained-input replay

After the status schema correction, read-only status returned reachable and authorised with serverFault null, roughly 50,931 entries and 2,457.5 MiB metadata-cache maximum. This reports metadata store usage and is not the host CAS partition reserve.

The earlier final action 91230856… recorded a 507 for CAS digest d98cae06dde281ccdd338118cbcdc5cee3ea5680b78cc131437dafb19dff4165. The retained local CAS object was 14,394 bytes and independently rehashed to exactly that SHA-256. Using the real fleet resolution, signed transport and HTTP content owner, an authenticated HEAD said absent and one normal CAS PUT of those exact bytes returned HTTP 507. The server response stated 54.22 GiB held versus 25.25 GiB effective host-store cap, 45.31 GiB partition free and 74.11 GiB hard reserve. A subsequent read-only authenticated HEAD/GET still reported absent. No successful upload or readback occurred; no action was submitted. The values are the server’s reported observations at that request, not a separate filesystem inspection.

No GC, mutable cache binding, configuration, deployment, remote machine operation or broad upload was performed. Ticket helper check-retained-cas-26.ts records the reproducible scoped client route; --read-only cannot upload. No credentials are printed.


## Hosted store cleanup boundary

The normal authenticated cache handler exposes status, evidence, claims, slots and content routes, but no remote host-CAS collection or dry-run endpoint. The 507 capacity diagnostic reports usage, effective cap, partition free/reserve and up to three largest holders from its last census; it does not report how many bytes are actually reclaimable. The client truncates the diagnostic to 200 characters, and this audit did not issue a second write to obtain a longer body. There is no basis to calculate reclaimable space from the reported figures.

On the actual CAS-owning machine, repo cache cas gc --unreferenced --dry-run --lang=en is the supported read-only protected host-CAS plan: it excludes held, pinned, leased, linked and young objects. repo remote-cache gc --dry-run --max-bytes <bytes> --lang=en previews the local remote-cache metadata LRU and is not an estimate of host-CAS unreferenced bytes. Both commands act on the machine on which they run. No authenticated remote administrative ingress or owner-side preview result has been established for .70; running either here would inspect this machine, not .70. No cleanup, reserve change or machine access was attempted.
