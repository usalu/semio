# Bun Import Unicode Representation Review

Independently inspected actual `oct8-prepared-pair-dependency-order-red.log`: 4,610 bytes, SHA256 cc6e8833386568ab63a3ba25567e7ad65f87d856302aaf0125981c8ef0ed8e90, Nx1/3.2s. Failure is scanned Unicode import resolution before the intended dependency-order law, so this receipt cannot serve as that law’s meaningful RED.

A cheap read-only Bun1.3.14 scan/resolution probe reproduces the exact representation seam. Literal `./🧬️schema/🔣️.json` resolves the physical custody schema. scanImports returns `./ð§¬ï¸schema/ð£ï¸.json`, whose Unicode code units represent UTF-8 bytes as Latin1; it differs from the literal and cannot resolve. No files or test/build outputs were created by the probe.

Repair should bind actual source literal spelling using owned token parsing or a validated scanner-specific decoding rule, preserving genuine Latin1 literals and escapes rather than applying arbitrary normalization. Required neutral cases compare emoji import, actual Latin1 import and escaped Unicode spelling against independent source/parser resolution. Exact evidence sent to Runtime. No dependency-order or settled freeze pass is inferred.
