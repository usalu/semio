# Typed Codec Test Insertion Seams

A standalone Rust test target in the existing Semio artifact root package can invoke19explicit editor types while sharing a test-only generic helper. Root already depends all19subsets. Explicit dev-dependency model/snapshot testkit features are necessary for populated demos; do not rely on feature unification through unrelated SQLite test crates. Existing per-subset test targets are also legitimate: for example text root already enables text-model/text-mutations testkit as dev dependencies. Avoid a new runtime helper or production dependency solely for test reuse.

The helper should force actual editor natural_file_codec registration, encode a populated exact typed demo, decode through that editor, compare the full original snapshot, reject foreign subset bytes, and retain a language-neutral schema/app/nonempty-discriminator roster. An independent JSON/pack/format oracle must verify externally observable content; mere successful self-roundtrip is weaker than retained populated equality. Archive/member behavior remains a separate test scope.

DOCX strict/editor source133–149 and transitional138–154 implement distinct natural codec methods. Their existing editor-unit modules are mounted at282 and290 respectively. Extend those actual unit owners with their subset-local tests/fixtures/example-readme.docx, independent ZIP/XML checks, complete snapshot roundtrip and opposite-conformance refusal where source contract requires it. Base DocxEditor tests cannot prove either conformance editor.

## Concrete Product Example Bug

Text editor source87–91 parses base_examples/demo PRIMARY_TEXT into SemioTextSnapshot and uses unwrap_or_default. Its navbar examples171–172 publishes that base source. The base demo assets DSL is hex-encoded old scaffold JSON with schema stdio.semio and an object/text tagged payload, not stdio.semio.text/runs. A person selecting Demo can therefore receive an empty typed default silently. The same base-example fallback pattern occurs in many subset editors. Correct source wiring must point to exact typed populated examples and report invalid parsing; tests must not treat fallback default as a valid exemplar.

No source edits or builds were performed.
