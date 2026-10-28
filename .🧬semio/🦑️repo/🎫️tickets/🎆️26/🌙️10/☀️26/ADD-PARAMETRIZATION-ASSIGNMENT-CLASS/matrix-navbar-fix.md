# Shared Navbar Alias Investigation

The first English, German and dark compiler invocations reported an undefined SemioChromeNavbarLabelsCompute at the begin-document hook. No pages were produced.

## Investigation

- The only semio-window.sty under the print product contains the public alias invocation at line 1996 and the private function plus public export at lines 3671 through 3695.
- A comment-aware brace scan confirms these definitions are at top level. A top-level conditional scan finds no enclosing TeX conditional. The package contains no endinput and the assignment class does not load it inside a group.
- The compiler log allocates symbols before and after the alias definition, confirming that the complete package was read.
- A minimal original assignment-class probe printed the meanings of both the public alias and the private function after documentclass and at begindocument/before. Both were correctly defined protected long macros.
- The retained navbar-probe.tex then compiled the original shared geometry and original English content with expanded error context. The first TeX pass succeeded and the compiler proceeded through BibTeX and further TeX passes.
- The parent independently reran the original German source and also passed the initial TeX and BibTeX stages.

## Decision

No semio-window.sty change is justified: the settled product already exposes the actual implementation through the expected API. No fallback, no-op or compatibility code was added. The initial failures are consistent with a concurrent incomplete source snapshot; that explanation is an inference, while the fresh macro-visibility and actual compilation checks are direct evidence.

The final English verification uses the exact original parametrization-en.tex separately; see matrix-english.md. navbar-probe.tex is a retained diagnostic input, while all compiler output remains temporary under the ticket generated directory.

## Completed Diagnostic Compile

The retained navbar-probe.tex compile completed with exit 0 and wrote the actual PDF. Bundled pypdf independently confirmed five pages, title Finger Exercise: Parametrization and catalog language en-US. The output was 83,681 bytes. This probe contains no copied checked source.
