# Package Body False Admission Reproduction

A coordinator read-only runtime probe called the live public classifier APIs with three source strings. All three contain domain implementation but were admitted as package wiring/metadata. The exact returned data is in the disposable coordinator/package-body-counterexamples.json output; the evidence is retained below.

| Input | Current API result | Required ownership result |
| --- | --- | --- |
| Python import os followed by def domain(): return 42 | declaration, evidence Python import or export declarations only | implementation |
| Vitest defineConfig import/default call plus unexported class Domain with compute(){ return 40+2; } | tool-metadata | reject metadata admission; report implementation |
| await runBundleScriptMain(new ScriptRouter()) plus the same unexported domain class | tool-metadata | reject metadata admission; report implementation |

The Python import-only predicate uses multiline anchors with RegExp.test, allowing a matching import line to admit the rest of the file. The two metadata validators use call/import presence without checking unrelated private declarations. These are confirmed current runtime false admissions, not only static concerns. The original Sol policy executor is correcting them during its active metadata follow-up, adding exact-disposition portable/native cases. The earlier 30-test result remains accurate for its covered cases and is not sufficient for acceptance of the broader policy.

Source strings were classified as data; their code was not executed. No repository implementation change was made by this probe. Further independent audit remains required after the correction, including whole-file coverage for other supported language grammars and precise tool-helper ownership.

## Registration Macro And C Directive Cases

A second live public-API probe confirmed two further false admissions in advertised grammars:

- Rust semio::register!({ let value = 40 + 2; value }); returns registration with evidence registration macro invocations only. The invocation name admits computation inside its token body.
- C #define COMPUTE(x) ((x)+1) returns declaration with evidence trivia-only source. The lexical layer discards the preprocessor directive before the C package classifier inspects it.

The active Sol correction now includes these exact cases and native/compiler oracles. Registration macro names alone must not admit arbitrary token bodies. C preprocessing directives are source structure and cannot be discarded as Python-style comments. Valid include/header/registration wiring remains narrowly distinguished from domain computations. No source strings were executed by these probes.

## Whole-Form Counterexamples During Correction

After the first five regression cases passed, another live public-API probe confirmed that token blacklists still admit executable macro content:

- `semio::register!({ execute(); });` returns `registration` with evidence `registration macro invocations only`.
- `#define COMPUTE(x) domain(x)` returns `declaration` with evidence `C or C++ declarations only`.

Both inputs omit the arithmetic/let tokens from the original regression. The correction must validate a closed admitted macro form and classify unknown bodies as unresolved or implementation; matching a familiar macro name or lacking selected operator tokens is insufficient. These exact results were sent to the active Sol executor before its final report. The independent Terra audit must include both variants alongside the original five. These source strings were classified as data and were not executed.
