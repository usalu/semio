# polyglot-units tools (wave 1021a)

- `staged/🧩️units/` is the module to create at `L/🏗️build/🧩️units/` (L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`): `🧬️schema/🔣️.json` (closed schemas), `🔣️.json` (policy: interface kinds, key text, go and python rules, EN/DE texts; written by `gen-policy.py`), `🕸️graph` (laws, order, layers, SCC, affected, critical path), `🔏️key` (canonical JSON, unit key, closures), `🏃️engine` (run with early cutoff, memory and file stores), `🏗️emit` (Nx targets, Bazel `repo_unit` rule and BUILD text), `🐹️go`, `🐍️python`, `🟦️typescript` and `🦀️rust` adapters, `🧪️tests` (vectors `🔣️.json`, `🟦️.ts`, `🐍️oracle/🐍️.py`, `🧫️fixtures/cargo-metadata.json`).
- `apply.py` dry run by default; `--apply`; `--undo`; `--no-dedupe` (skip the TypeScript engine SCC wrapper edit); `--ts-closure` (opt-in soundness edit of the TypeScript engine keys); `--root DIR` for a mirror.
- `gen-policy.py`, `gen-fixture.py` (hand-written vectors, key hex from the oracle), `check-oracle.py` (oracle vs hand-written expectations, stdlib only, fast), `gen-cargo-fixture.py` (path-only workspace, `cargo metadata` and `cargo tree` as oracles).
- `overlay.sh` builds `/private/tmp/polyglot-units/ov` (staged module at its real depth + symlinks to the real validator, live TypeScript units engine, node_modules); `cd` there for `node node_modules/typescript/bin/tsc -p tsconfig.json` and `bun test <module>/🧪️tests`.
- `units-cli.ts` plans the Go or Python units of a checkout read-only (`git ls-files`) and prints counts.
- Scratch: `/private/tmp/polyglot-units/` only.
