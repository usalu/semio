# Live Interactivity Gate

## Scope

The coordinator revalidated the repository-wide interactive-runtime policy after the mounted-reconcile implementation moved reservation ownership behind the thread-local `PATCHES` registry.

## Admission Repair

`📜️script.ts` now requires the exact production reservation boundary:

```text
match PATCHES.with(|patches| patches.reserve_mounted(surface))
```

The remaining contract stays unchanged: one production `drive_one` opportunity per iteration, exact source commit or cancellation, deferred saturation ownership, instance-close rearm, and retained reconcile work signalling. Existing hostile mutations continue to remove the inner reservation call and therefore remain rejected by the exact wrapper predicate.

## Evidence

Command:

```text
bun ./📜️script.ts verify interactivity
```

Observed result on 2026-08-27:

```text
exit 0
32 descriptors
101 app declarations
57 launch-only product surfaces
158 total surfaces
4,760 action rows
101/101 app contexts launch-covered
237 development launch surfaces
25 hostile/oracle self-tests
DENY mode — clean
```

The scanner also reported exactly one blocking-bridge finding, covered by the existing permitted structural boundary, with no unallowlisted violation.

The coordinator repeated this gate through the canonical task runner after the next app cohort landed:

```text
NX_DAEMON=false bun x nx run workspace:verify-interactivity --skip-nx-cache
exit 0
NX Successfully ran target verify-interactivity for project workspace
```

The 32 descriptors, 101/101 launch-covered app contexts, 237 development surfaces, 25 hostile/oracle checks, and deny-mode result were unchanged. This rerun did not execute the separate retained-tool-jobs or app runtime gates.

## Remaining Boundary

This result proves the repository-wide static interactivity policy and all-app discovery/launch coverage. It does not close the master refactor: retained command publication, app-owned operation jobs, native/Wasm runtime replay and timing gates, descriptor regeneration, and the zero-external-dependency gate remain separate required evidence.
