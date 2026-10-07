# Native Ten Current Compiler Failure Audit

Current native-ten terminal records code 101. The closed lockfile receipt records exit 0; the separate test receipt records exit 101, stdout 0 bytes, stderr 1,042,984 bytes, process/lease/stdout/stderr closed. Compiler stderr reports fourteen OS-kernel errors, chiefly inverse mutation representation inconsistencies (`PagedList` versus `Vec`, slice indexing, and missing `try_reserve_exact`), plus replay-operation `M` requiring Debug through `expect`. No runtime assertions executed successfully are established.

Six owned Rust postchecks and five artifact schema postchecks are exact. Of 560 provider source postchecks, 559 are exact; the OS store main source changed during the run. It has advanced again since the captured postcheck. Accordingly terminal compiler output concerns a non-atomic live provider closure, and cannot establish a coherent unchanged source revision. Current helper still matches recorded producer. Whole-root acceptance, production descriptor execution, and controlled public-diff acceptance are all false.

No evidence or completion state from absent native nine is reconstructed.
