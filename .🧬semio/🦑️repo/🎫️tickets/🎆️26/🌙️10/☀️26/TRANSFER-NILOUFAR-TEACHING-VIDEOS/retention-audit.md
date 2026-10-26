# Generated Input Retention Audit

Read-only inventory of generated top-level entries and shallow task-only directories. Vendor runtimes, virtual environments, UV state, font/text caches and media/render trees were excluded. No file was moved, removed, copied, executed, or modified except this retained audit report.

## Retain Three Unique Authored Inputs

| Existing generated path | Suggested ticket-root path | Purpose | Root byte-identical duplicate |
| --- | --- | --- | --- |
| `🗑️generated/compare-segments.py` | `compare-segments.py` | Independent AST comparison of original playlists with all ordered manifest segments | None |
| `🗑️generated/segment-audit.py` | `segment-audit.py` | Original AST playlist/build-command inventory | None |
| `🗑️generated/glyph-all.py` | `glyph-all.py` | Original fontTools glyph/literal coverage audit | None |

These are small task-authored audit programs, not copies of canonical product scenes or helpers. Existing root glyph-probe/glyph-after and plan-segments programs serve different steps and are not byte-identical replacements.

Each of the three scripts currently writes JSON with `Path(__file__).with_suffix(".json")`. Once retained at ticket root, their output path should explicitly target the ticket `🗑️generated` directory rather than create tool output beside retained inputs. Their source-only audits may remain Windows-specific historical test inputs; no production dependency should be created from them.

## Generated Configurations And Data

No additional unique authored configuration input was identified in the bounded scope. The three recovery `render.cfg` files are generated Manim configurations containing absolute per-run media paths; the retained root recovery runner/fixture already reproduces them. JSON files inspected are command reports, process/index snapshots, cache-operation reports, decoded-frame observations, or derived AST/plan data. `segment-plan.json` is semantically identical to the canonical product `video_manifest.json` and should not be retained as a second source. Generated data and media remain cleanup candidates once the parent has preserved final human-readable evidence.

## Markdown References To Update

- `glyph-coverage.md:43` currently says the audit script and JSON reside only under generated and can both be removed. Change this to identify the retained root `glyph-all.py` input and distinguish removable generated JSON output.
- `segment-plan-audit.md:33` cites generated `compare-segments.json`. Keep the retained pass result in prose, add the retained root `compare-segments.py` input, and remove or clarify the transient JSON reference after cleanup.

- `segment-plan-audit.md:35` cites generated `segment-audit.json`; identify retained root `segment-audit.py` and clarify that its JSON is transient output.

No existing Markdown reference to the `segment-audit.py` input itself was found. No live run or renderer/cache file was altered, and this audit launched no processes.

## Retention Completed

Parent preserved all three unique audit inputs at ticket root and redirected their JSON output into `🗑️generated`. No canonical product source was copied. Derived data remains disposable after final evidence is retained.
