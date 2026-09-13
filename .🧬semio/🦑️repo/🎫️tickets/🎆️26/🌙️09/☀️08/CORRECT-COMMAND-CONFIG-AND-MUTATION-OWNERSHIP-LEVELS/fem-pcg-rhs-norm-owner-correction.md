# PCG RHS Norm Owner Correction

Status: actual native RED followed by 16/16 native GREEN on the configured 2 MiB stack. Composed FEM3D validation awaits the active shared loader edits regaining compile coherence.

## Numerical Ownership

The retained PCG constructor adopted either a vector RHS or mounted scalar slots but initialized its convergence norm as sqrt(matrix order). That value belongs to a synthetic unit RHS. The actual assembled RHS owns the norm; FEM3D and renderers must not provide a separate derived copy.

The correction adds an inline norm accumulator and a NormalizeB phase to PcgJobConstruction. One opportunity consumes one actual RHS scalar with f64::hypot. The phase rejects non-finite values or an unrepresentable norm through the existing sticky constructor fault. Zero-vector allocation follows normalization; Complete only transfers the finished norm, floored at 1e-300. Both adopted-vector and mounted-source paths use the same phase. No whole-vector norm is computed in Complete.

## Schema-First Reference

The closed shared fixture contains eight cases: empty, zero, [3,4], [-3,4], four unit values, [3e-6,4e-6], 512 unit values and [3e-12,4e-12]. NumPy 2.0.2 was executed against the committed corpus for both numpy.linalg.norm and numpy.linalg.solve(identity, rhs); all eight agree with the recorded norms and solutions. Evidence: `🗑️generated/pcg-rhs-norm-numpy-corpus.log`.

TypeScript Math.hypot independently checks those norms and fast-json-patch checks the zero-norm floor transition. Neutral8 passed with strict TypeScript in 2.7 seconds. The native law drives both constructor paths for every case, retains observations, completes the actual solver and closes every transferred job and remaining builder before checking the results. The 512-scalar case also requires a separate scalar opportunity for normalization.

## Executed Runtime RED

`🗑️generated/fem-pcg-rhs-native-red-1.log` ran the registered native route on a 2 MiB stack: 15 passed, 1 failed, 973 filtered, 0.50 seconds Rust runtime; Nx exited nonzero in 2m19s. The sole failure is the new actual-RHS norm law.

Every one of the sixteen case/path combinations was driven and closed before the aggregate assertion. The constructor reported sqrt(2) for zero, [3,4], signed, small and tiny two-entry RHS values. The tiny RHS completed at zero iterations in both paths. The 512-entry vector path consumed only 3085 turns before the correction, below the normalization-inclusive bound. The failing assertion first reports the zero RHS norm instead of its 1e-300 floor.

The newly added dense CSR continuation and empty restore laws passed in this run: order64 with 14 checkpoint pages converged after two iterations; empty restore closed the exact 9856-byte payload backing. Existing construction, publication, malformed restore, identity and cancellation laws also passed. This is meaningful partial evidence, not a passing full route.

## Executed Runtime GREEN

The production NormalizeB correction was applied only after that runtime RED. `🗑️generated/fem-pcg-rhs-native-green-1.log` passes all 16 selected native tests, 973 filtered, in 0.25 seconds Rust runtime. Nx exits zero in 1m23s. All sixteen RHS case/path combinations agree with their independent norm and actual identity-matrix solution and close completely. The tiny cases now solve in one iteration; the 512-entry vector path takes 3598 retained constructor turns and the mounted path 5137. The observed maximum norm 22.62741699796953 agrees within the declared relative tolerance with NumPy's 22.627416997969522.

Four additional non-finite RHS cases now use exact IEEE754 bits in the same closed fixture: NaN, positive infinity, negative infinity and norm overflow from two finite maximum scalars. NumPy2.0.2 was executed against all four and agrees with the scalar-versus-derived-norm refusal classification (`🗑️generated/pcg-rhs-invalid-numpy-corpus.log`). Neutral9 passes the expanded schema/oracle and strict TypeScript in1.8s with0/1 cache hits. The new native law covers both constructor paths, sticky first fault, stable retained owners before close, no zero-vector allocation after invalid input, and complete bounded release. Its native execution is still pending.

The first composed route (`🗑️generated/fem3d-post-rhs-norm-native-1.log`) passed lower1/1, then failed while compiling the upper test binary with rustc-LLVM "No space left on device". Nx exited nonzero after16.5s; no upper tests ran. Root removed only completed task Nx workspace caches and recovered6.7GiB; see `completed-task-nx-cache-recovery.md`.

The second composed route (`🗑️generated/fem3d-post-rhs-norm-native-2.log`) stopped before Rust tests in the active retained config loader edits: `edit_index` was declared as both a retained lookup map and an integer cursor, causing duplicate-field and type errors. Nx exited nonzero after10.9s. The loader executor has the exact diagnostics and is restoring compile coherence. Neither failure establishes post-norm composed runtime behavior.

After the focused route passes, repeat the composed FEM3D route: the real assembled RHS now determines the relative convergence threshold. Keep the configured watchdog, deadline and normal stack, observe actual terminal close, and accept the resulting iteration count rather than preserve the old incorrect normalization.
