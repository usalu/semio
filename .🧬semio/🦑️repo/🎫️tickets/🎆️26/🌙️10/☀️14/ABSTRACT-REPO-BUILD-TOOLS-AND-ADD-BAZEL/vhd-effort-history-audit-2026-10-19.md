# VHD Effort History Audit — 2026-10-19

Read-only bounded Git history and ticket-document inspection. No Git mutation, source edit, or tests.

## What Can Be Measured

The current fixed-storage source path `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` first appears in this scoped history in commit `7c8af563c74`, authored 2026-10-14 18:47:40 +02:00. Eight commits touch this exact path through `8f91299152c`, authored 2026-10-19 02:09:34 +02:00. The repository cache budget policy at `repo/.../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` appears in that same October 14 commit and has three scoped touching commits: October 14, 18, and 19.

The eight fixed-path commit dates are October 14; twice October 15; twice October 16; October 17; October 18; and October 19. This is evidence of repeated integration over six calendar dates and about four days plus seven hours between the first and latest sampled authorship timestamps. It is **not** evidence of that many hours of active work. Both paths could have predecessors elsewhere; no full rename/predecessor archaeology was attempted.

`git blame -L 148,159` assigns the current retirement entrypoint to October 16, queue acquisition to October 15, and the publication-settled checks plus retained-writer retirement variant to October 17. Thus lifecycle/custody design continued changing after initial introduction; the feature was not a single isolated creation commit.

## Ticket Evidence of Iteration

The covering ticket is dated October 14 and titled **Abstract Repo Build Tools And Add Bazel**. Its description covers tool-neutral Nx/Bazel interfaces, setup, CI, launch, agents, and dashboard. It is broader than VHD work; the ticket's total duration or work cannot be attributed wholly to this storage feature.

Inspected reports show multiple separate workstreams:

- `windows-native-provisioning-follow-up.md` investigates Windows-root discovery, Hyper-V/New-VHD provisioning constraints, and qualification; its dated subsection records a root-namespace candidate at 2026-10-16 12:29 UTC.
- `windows-native-provisioning-plan.md` develops preparation-owned metadata barriers, exact volume identity, cancellation, retained parent custody, and schema/oracle qualification.
- `windows-native-provisioning-inputs.md` describes a 29-row review corpus, distinguishes unexecuted review inputs from runtime proof, and searches for independent VHD-format oracles.
- `windows-sequential-allocation-plan.md` proposes replacing the New-VHD allocation action with a privately owned sequential writer, while retaining footer format, exact physical allocation, cancellation, descriptor closure, and existing action/time limits.
- Current history and the October 19 runtime audit add concrete incomplete-birth/receipt recovery investigation.

A bounded top-level ticket filename filter (`windows|native|fixed|vhd|publication|persistent`) matched 285 Markdown documents. This is only a rough documentation-volume signal: many matches cover other publication/persistence work, repeated reviews, or unrelated portions of the broad ticket. It is not a count of VHD implementation sessions, independent features, completed tasks, or labor hours.

## Honest Effort Conclusion

The evidence supports calling this a substantial, repeatedly refined infrastructure effort spanning multiple days, with platform discovery, allocation/format work, concurrency/lifecycle ownership, persistence protection, qualification, and recovery. There is no reliable actual labor log in the inspected sources. Commit timestamps, number of reports, agent mentions, and elapsed calendar span cannot establish person-hours or compute cost. Any numeric hours estimate would be invented.

This historical investment is relevant to replacement cost and domain knowledge, but does not decide whether mandatory VHDs are the right default. That decision still depends on the hard-capacity requirement and operational evidence, not sunk effort.

