# Grammar file contract (`.grammar.semio`)

## Location

Per artifact facet:

- `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/🗣️dsl/📖️component.grammar.semio`
- `…/🔧️op/📖️component.grammar.semio`
- `…/🔺️diff/📖️component.grammar.semio`
- App config: `🎛️apps/<app>/🎚️config/📖️<ext>cfg.grammar.semio` where applicable.

## Header

```
dialect grammar
grammar <id>
extension <ext>
use <family-fragment>*
start <production>
```

## Body

EBNF productions over `dsl_core` terminals; macro calls (`edge`, `chain`, `expr`, `props`, …) resolve via `dsl_notation` / family crates.

## Normative role

The `.semio` file is normative; handcrafted Rust is the reference implementation. Divergence is fixed root-first in Rust or spec, never left silent.

## Conformance

`dsl_grammar::Recognizer` + fixture-sweep: corpus agreement, production coverage, generative sampling (parser accepts generated sentences).
