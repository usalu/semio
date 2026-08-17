# Notation style guide

## Arrows (graph family)

Fused edge token (canonical):

- `a -> b`, `a -- b`, `a <- b` (reversed sugar, normalized)
- `a -e1> b`, `a -c:Connection> b`, `a -:Kind> b`
- `a -e1- b` (undirected)
- Chains: `v1 -- v2 -- v3` (anonymous, prop-less runs re-chain on print)

Endpoints: `id[:Kind][@port]`. Props block `{ key=value … }` after the statement.

**Do not** use bracketed edge workaround `- [id:Kind] ->` in new grammars.

## Identifiers

Kebab-case allowed; space before `-` when an edge follows an ident (lexer `EdgeArrow`).

## Units

`210GPa` = `Float` + adjacent `Ident`; quantities use `dsl_notation::parse_quantity_text`.

## Statements vs tables

Graph artifacts: edges as arrow statements, not SoA `edges […]` tables in document DSL.

## Ops / diff

Ops: one line per variant or `use ops-header` from store fragment. Diff: patch lines mirroring op vocabulary.

## Config

`use config` fragment; attribute runs; ~10-line grammar per app.
