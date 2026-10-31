# Native Primitive 2 Lock Drift Audit

Read-only Source audit. Neither lock was edited and no Source copy was written. Only the five expected new ActionB dependency entries were candidates for in-memory removal; complete candidate bytes were compared with Root prelaunch hashes.

- `Cargo.lock`: current `4481bf597efb85440d2857f5bd985a74898fe26e42b79ce5b459f38eff28c872` (741775bytes); candidate `f844bfdbb1a515ee5324b0e058e3176d823e632b2d687908c6e028d1a25cc7e3` (741634bytes); ActionB blocks=1; exact full reconstruction=**True**.
- `🧰️framework/Cargo.lock`: current `9afaf5e8f000295b4da31ad1adfd13e2c8fb2ebaff1d1928029b8caa8fdcd6d0` (305568bytes); candidate `9afaf5e8f000295b4da31ad1adfd13e2c8fb2ebaff1d1928029b8caa8fdcd6d0` (305568bytes); ActionB blocks=0; exact full reconstruction=**False**.

The root lock has all five newly declared ActionB dependencies; precise entries/current line positions are in the audit packet. The framework lock has no ActionB package block, so that bounded hypothesis cannot reconstruct it. No semantic equivalence or complete290byte seal is claimed. Root retains ownership of remaining lock investigation and runtime qualification. No rerun, Cargo, setup or cache action was performed.
