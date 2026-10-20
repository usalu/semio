### 🪞️ Parity verification

Nx and Bazel must produce the same results from the same neutral graph. `repo verify build-parity` proves it per sample instead of by inspection ({{@parity}}). `verify` is a root command, not a build operation: it first runs `repo dependencies ensure`, and where `repo` is not on the `PATH` it is `bun ./📜️script.ts verify build-parity` at the root.

```bash
repo verify build-parity --plans-only
repo verify build-parity --only=area:framework
repo verify build-parity --samples=<file> --out=<directory> --keep --json
repo verify build-parity --recover=<directory>
repo inputs @teaching/architecture-course:test
```

- **What one sample does.** The dry-run task graph of both tools and the closure (Bazel `cquery` against the planner); then `repo --tool=nx|bazel run <target>` fresh (`--skip-cache`) and restored from the cache; the declared outputs of each tool captured into separate roots and compared byte for byte (kind, mode, link target, SHA-256); exit codes, failed tasks and test status; the Nx `inputs` against the inputs and environment of Bazel's spawns (read from its execution log); and the cache restore of each tool cold against restored.
- **Only a rule explains a difference.** `🏗️build/🪞️parity/🔣️rules.json` holds 8 normalisations, each with a justification in English and German and its evidence, and the report counts how often each applied. Everything else is a divergence (35 codes, each with a help text) and the command exits non-zero. 27 samples in 6 areas (`🏗️build/🪞️parity/🔣️samples.json`: `framework`, `s-plugins`, `hub`, `teaching`, `mit-bestand`, `semio-tech`) are authored; `--only=<id>` or `--only=area:<area>` (comma separated) selects, `--samples=<file>` replaces them.
- **Safe for your tree.** Declared output roots are set aside before each run and restored in a `finally` (also on Ctrl+C); tracked roots are copied, untracked ones moved; `--recover=<directory>` restores from the journal after a crash. Progress is published per sample, tool and phase and every step is a cancellation checkpoint.
- **Reports.** `report.json`, `report.en.md` and `report.de.md` land in the evidence directory (`--out`, kept with `--keep`); `--lang=en|de` selects the help language. The Bazel side records its evidence (execution log, build events, closure) only when `SEMIO_BUILD_EVIDENCE_DIR` is set, which the verifier sets for its own Bazel runs.
- **`repo inputs <project>:<target>[:<configuration>]`** prints what Nx hashes for one task (files, environment, runtime values, dependency outputs, external packages) as JSON. Bazel does not implement it: its inputs are the spawn inputs in the parity evidence.
- **Not yet verified.** The real-tool answers: it was tested with fixtures, a fake `repo`, and Bazel evidence tests; the first live runs (the course build, the semio-tech play build, the mit-bestand demonstrator release build, a compact report print) are open. A heavy sample such as the demonstrator release build can exceed the 3,600 s sample limit. Run one process at a time; the host capacity governor applies to the child runs.
