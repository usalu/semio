# Failed Cargo Build Diagnostics

The Flow endpoint49460 gate failed during compilation with code101 after21m10 and no retained diagnostic output. The bounded Cargo runner captured Nextest build stdout, but wrote metadata only after a successful await; failed build stdout was lost. The retained failed directory therefore contained no explanation.

Neutral cargo-failed-build-output.v1 declares three failure outputs: compiler E0046 JSON, a Unicode scalar split inside its UTF8 bytes, and a separate nonzero status7. The closed draft07 schema is Ajv validated. An independent Node child process supplies the exact stdout and status oracle. The canonical real bounded driver must fail, retain exact decoded bytes, create no successful binaries metadata and never enter assertions.

Red49706 actually failed:zero passed,one failed, after the canonical build port emitted a failure the expected retained build-failure.stdout.txt was absent (ENOENT). The repair now writes the bounded captured stdout to that diagnostic file before rethrowing the original error. Successful metadata behavior, caller retention/deletion policy, capture ceiling, build budget and assertion budget are unchanged. Green retry has not run yet.

Authored paths:

Full green58879 actually exited0:six tests passed,zero failed,226 assertions,14.58s. All three failure ports retain exact stdout including split Unicode, preserve failure propagation, create no successful metadata and never enter assertions. The same full suite reran real Cargo library/integration/default target routing, CLI option partitioning, reporter flags, bounded capture and empty-selection refusal. Launch11.56 is registered by the pipeline owner. Failed compiler capture now remains reviewable at its retained ticket artifact directory.

- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧫️fixtures/🚨️failed-build/🔣️.json
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧫️fixtures/🚨️failed-build/🧬️schema/🔣️.json
