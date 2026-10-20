# Native CI Actual-Source RED Prepared

No command or test ran; Root63416 remains the sole live command. Workflow Source is unchanged.

The existing architecture deployment test now registers `provisions the native release frontend before every managed release command`. It consumes the actual workflow already parsed by the existing third-party yaml fixture port, checks check/site/proctor, requires native-repo after pinned Bun and before every run step, compares the reviewed conditional recovery recipe, and compares all13ordinary managed commands against explicit native argv DATA. The workflow currently has no native-repo step, so the intended first observed assertion is the actual-source missing-provisioning failure, not a fabricated fixture failure. Actual collection and RED remain unrun.

Changed only two test-owned files:

- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🔣️.json`:6499B/SHA256 `67da8556b13fbc86b4fa0064796c2aefc932617dca0b78fb00204404dbd421c0`. Added release.nativeIngress (recipe/envelope/ordinary argv DATA) derived from the authored ticket schema/fixture. Old release command expectations remain unchanged until actualRED.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🧩️compose-caddy-and-workflow/🟦️.ts`:9340B/SHA256 `025ea9407d277db60de4855e978f48d0aae47b9019b88744c4e815a8a04523ed`. Added one assertion registration, with no new import/dependency.

Unchanged workflow `.github/workflows/architecture-course.yml`:9010B/SHA256 `d6d1e450b1d8141b20c70485909d6fd5ccb2081f01db4216151a4809629ce75c`. No AGENTS, package, lockfile, target manifest, CLI/router/bootstrap implementation, generation-receipt implementation or private Source changed. Both edited files were already product-test inputs; this introduces no new CLI or dependency implementation import edge. A live planner may observe evolving architecture target inputs, but no dependency sync or CLI import-closure invalidation was introduced by this change.

The owning native target is `repo run @teaching/architecture-course:test --configuration=release -- quick`. The manifest selects its existing test router (Vitest native config loader), existing15-path node test roster, cache:true, default inputs and outputs[]. Parent must wait for the current command before scheduling it. A refusal before test collection is infrastructure evidence and cannot count actualRED. After actualRED, the smallest workflow patch and old readiness command expectation updates remain to be made, then the same owned gate must produce GREEN. The17case ticket corpus and schema remain prepared inputs; no neutral or actual-source test result is claimed.
