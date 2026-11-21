# Remote Constrained Lab Route Audit

No authored structured authenticated build/test target or scheduler action for the ticket’s frozen all-stage constrained lab on another enrolled Darwin host was found in the inspected execution, scheduler, lab and build-owner declarations. This is a source audit, not a claim about another host’s live capability or free disk. No job was submitted.

The lab entry `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/📜️script.ts` lines 6 and 12 declares local run options including memory, cores, stages, frozen, work and output. The ticket’s `constrained.ts` wrapper owns the physical admission and receipt export described in `audit-constrained-lab-wrapper.md`; the lab entry alone does not establish those exact physical receipts on a remote host.

The authenticated remote-execution service accepts REAPI execution submissions (`🏗️build/🌐️remote-execution/🖥️server/🟦️.ts` lines 69–89, 198–214), but the authored offload builds a rustc unit (`🚀️offload/🟦️.ts` lines 40–44 and `🚀️offload/📜️action/🟦️.ts` lines 20–22, 35–36). Worker policy defaults to rustc and requires confinement for arbitrary actions (`👷️worker/🟦️.ts` lines 305–312); the rustc validator admits only its declared command (`🧱️actions/🟦️.ts` lines 4, 51–57). A general REAPI facility does not itself declare the constrained lab action or its physical resource and disk receipt contract.

The worker agent admits actions against host capacity and reports disk availability (`🛰️agent/🟦️.ts` lines 26, 43–45); that is not the lab wrapper’s full-volume physical admission and exported receipts. No SSH, process operation, remote message, or new command envelope was used.
