# Runtime Examples Vs Fixtures — 2026-09-15

## Question

Do user-facing bundled examples load from `🖼️assets/` (runtime) or `🧫️fixtures/` (test-only)?

## Procedural generation3d (bundled picker examples)

| Path | Role |
| --- | --- |
| `📚️examples/<slug>/🖼️assets/<slug>/🗣️.dsl.semio` | **Runtime** — `PRIMARY_TEXT`, `GENERATION3D_EXAMPLE_*_TEXT`, `example_snapshot()` |
| `📚️examples/<slug>/🧫️fixtures/🧩️example/🔣️.json` | **Test oracle** — expected mesh/stats in `📚️examples/🧪️tests/🧩️geometry/` only |

`example_snapshot` / `default_snapshot` / `set-active-example` parse DSL from asset constants, not from `🧫️fixtures/`.

## Violations found

None in production editor modules under `✏️editor/` (non-`🧪️tests`) for `include_str!(…🧫️fixtures…)` — all such includes are in tests or language-neutral contract JSON.

## Follow-up

- Keep example **oracles** under `🧫️fixtures/🧩️example/`; do not wire them into `example_snapshot`.
- Optional: rename test-only JSON paths in docs/comments from “example fixture” to “example oracle” to reduce confusion (cosmetic).
