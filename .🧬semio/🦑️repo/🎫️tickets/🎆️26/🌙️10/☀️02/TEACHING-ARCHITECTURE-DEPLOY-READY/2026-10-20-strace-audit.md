# Strace Sidecar Feasibility Audit

This is a read-only documentation audit. No Docker operation, guest process, attachment or signal was performed. Exact prepared implementation and same-Rosetta neutral proof remain pending.

The [strace manual](https://man7.org/linux/man-pages/man1/strace.1.html) documents that attach mode interrupted with SIGINT detaches and leaves the tracee running. Summary-only -c suppresses regular syscall output and reports per-syscall aggregate counts/time/errors; -C also prints regular output and is unsuitable for this scope. Fatal-signal interruption behavior is controlled by -I; explicit interruptible behavior should be retained in the actual tool command. --kill-on-exit kills tracees and is incompatible with attach; it must not enter this proposal. Tracing slows the target, so counts do not measure unperturbed performance.

A three-second count-only observation can be feasible only after the actual same-Rosetta own neutral fixture demonstrates attach support, aggregate framing, SIGINT-to-tracer detach, tracee survival and TracerPid reset. ARM64 tracer support for that translated task is not established by documentation alone. A failed neutral attach must stop the route rather than expand privileges or attach the real target.

The actual proposal must pin the retained builder identity and target PID/starttime before attachment, isolate the sidecar to that PID namespace without host Source/socket mounts, bound output and30-second settlement, and signal only the owned tracer. Sidecar termination or tracer exit alone is not proof of successful detach; target identity/survival and no residual tracer must be checked. Failure to establish detach is an explicit diagnostic failure, not an empty useful sample. No JS-site or useful-progress inference follows from syscall aggregate counts.


### Version6.1 Interrupt Correction

Independent [upstream strace v6.1 Source](https://github.com/strace/strace/blob/v6.1/src/strace.c#L2779) confirms that INTR_ANYWHERE bypasses installation of interrupt handlers. Therefore the earlier preliminary -I1 suggestion is superseded: use handled -I2, with SIGINT delivered only to the tracer. The cleanup loop detaches attached tasks; its fatal-signal forwarding branch applies to the command spawned by strace, not an arbitrary -p attachment. This supports the revised design but does not prove the actual packaged tool/Rosetta neutral behavior.

A foreground timeout targeting the tracer avoids intentionally signaling the tracee process group. Force-kill settlement must remain a failed detach proof, not an accepted sample. The same-Rosetta own fixture must still show a summary, continued target identity/liveness and TracerPid zero before any real target attach is considered.


### Prepared Pure Cohort Review

The helper matches762a83633fd4f20d618fb3114be4a254a429739ad50d375f7a13b39f57f48d11/13,238bytes. It has no live image/build/metadata/Docker adapter: CLI only permits --check, and its26 cases are neutral port proofs. The proposed args use count-only/-qq/-I2/no tips, foreground2second timeout with1second kill settlement and a single -p; no follow-forks, raw buffers, output files or exitkill. Strict ASCII summary parsing rejects raw syscall lines and validates aggregate totals.

The protocol reauthenticates builder and target, requires pre/post TracerPid zero plus PID/start/comm/basename/live-state equality, verifies isolated sidecar identity before removal, and refuses force-killed status. The recipe requires a supplied official ARM64 digest and pinned strace package, but the digest is unresolved and no image or same-Rosetta proof exists yet. The30second check occurs between awaited ports; it does not interrupt a stuck port, so a future live adapter must independently enforce real deadlines, bounded transport and awaited owned-child cleanup. Likewise final removal must confirm actual removal/detach, rather than rely on a neutral remove promise. These are explicitly unimplemented requirements, not evidence of a live unsafe operation. This cohort is ready as preparation-only logic; it is not ready for actual target attachment.


The retained registry metadata independently matches3,169bytes/SHA25646c39ee2ca498912397a1dfe9b24971260aa4cdc3fb589eed2ccfb7117143722 and contains exactly one Linux/ARM64/v8 descriptor:1d15be465c9ea7b37b464055c5de38319523de9d3f070fcc76c0d0230ebe9669, OCI manifest1,041bytes. The selection receipt agrees. This authenticates descriptor selection only; no parent layer pull, prepared image, installed strace or attach readiness is established.


### Live Adapter Candidate Review

Candidate2b722813…36,534bytes now wires bounded child transport, pinned identity primitives, separate preparation/neutral/actual modes and actual target metadata. The intended -I2/count-only/single-PID arguments and neutral-SHA gate remain. A concrete cleanup gap prevents unconditional live clearance: safe cleanup calls still share remaining() and maximumBytes-total with the primary action. Exhausting either resource causes final sidecar inspect/remove to refuse before launching Docker. The8second admission reserve checks only non-safe operations and does not independently bound/finance subsequent safe target/inspect calls. Thus a deadline/transport failure can leave an owned sidecar unresolved despite final cleanup code. Runtime was notified to provide independently bounded cleanup settlement and explicit unresolved-cleanup evidence.

The actual adapter also limits stderr to4KiB although its pure aggregate parser accepts64KiB; this is a bounded refusal limitation, not a disclosure or custody bypass. No image/neutral/actual action was executed by this audit.


### Final Bounded Transport Refinement

Final candidate6588ed379c46e994ec9dcaab3dfb7fe0a00eb95e0341879910eb829a1bd9eb9e/41,914bytes addresses the prior resource/settlement gaps. Primary work reserves10seconds and16KiB for finally cleanup within the existing totals. The owned Docker-client wrapper sends TERM, escalates only that ChildProcess to KILL after250ms unless exit is already observed, destroys its inherited stdio after500ms, and explicitly rejects unconfirmed closure after750ms. Raw close clears timers; an independently set deadline flag prevents a late transport-zero result being accepted. Per-operation admission reserves1second for this settlement. Unresolved child/sidecar and cleanup failure remain evidence, never success.

The49 retained neutral cases exercise actual pinned transport plus fake-child ignored TERM, cancellation/overflow, early close, inherited pipes and absent close. These controls establish bounded refusal behavior, not guaranteed OS-level kill or detach. No new target/group/builder signal is introduced. Image preparation and same-Rosetta neutral proof are now review-ready; actual managed attachment remains separately gated by a successful exact neutral receipt and root approval. No action was executed by this audit.


The actual prepared image receipt independently matches477bytes/SHA2562920283c53c21519811b73737224207ed593bd6ad70519df3df9801fec7ead52. It records native ARM64/Linux immutable image2dd3c451…49,821,751bytes, the selected1d15 parent, recipeb16e6d… and strace6.1-0.1 runtime-version verification. This is source-free tool image preparation evidence only; neutral attach/detach and actual managed target authorization remain separate. No handle polling or private action was performed.


### Actual First Neutral Preparation Refusal

The actual neutral receipt is405bytes/SHA2569722eb12ff9b90c93cbad98997c4fdc9329ede65082391647b957c42f1639493 and accepted=false, with unresolvedSidecar=true/unresolvedChild=false. The retained projected sidecar metadata is797bytes/SHA256d063479c72394207dc9809df235fe46b75533d40e46f3712d025acce6f235492. It authenticates the expected image/token/name/PID namespace and shows state=created,Pid0,zero StartedAt: the sidecar never started. Docker returns CAP_SYS_PTRACE while the candidate expected SYS_PTRACE, explaining the strict configuration comparison refusal before start. This is a representation bridge defect, not evidence of Rosetta ptrace support or an attach failure. Root owns cleanup of that exact created ID; this auditor performed no Docker action. Strict capability canonicalization and identity-only ownership cleanup can be reviewed separately without weakening accepted runtime configuration.


The focused bridge/cleanup revision matches0080f10077763168f8cd96bd995bf149d4c76f014010ee814d0aa9ed53ff35d1/45,399bytes. Its capability bridge accepts only one exact SYS_PTRACE or CAP_SYS_PTRACE element, rejecting extras/duplicates. Full isolation remains required before start; final cleanup instead authenticates exact ID/image/name/token/ticket/kind ownership, permitting cleanup even after an isolation refusal. Stopped owned sidecars use plain rm; force is confined to authenticated running owned sidecars. Failure receipts retain nonce/name/known ID. Closed optional second selects a fresh receipt and actual gate reads the corresponding exact successful neutral SHA; the original failed receipt is retained. Meaningful retained actual-projection RED and55 neutral cases cover this bridge. No concrete blocker found for a second own neutral attempt; no actual managed attach readiness is inferred. No Docker action performed by this audit.


### Actual Same-Rosetta Neutral2 Proof

The settled actual neutral2 receipt independently matches2,509bytes/SHA256a5941b305974b43386f0f721921c15a58a24c64980f34b5f208a38a5380a2df1. It records accepted=true/sameRosetta=true, stable own fixture PID386/start6685091/commMainThread/executable rosetta with TracerPid0 before/after,16,624 aggregate calls including15,779statx, then successful fixture completion60,100iterations/status0. The owned sidecar is removed; cleanup has seven operations/3,500bytes with no unresolved child/sidecar or failure. This establishes actual attach/count-only/detach/survival for that own neutral translated task and tool image. It does not authorize or establish attachment to a managed process, identify JS code, or measure unperturbed performance. Old PID568 is invalid after resume; a fresh target identity and separate root approval remain required. No guest action was repeated by this audit.
