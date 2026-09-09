# Rustdoc Test Layout Audit

A complementary read-only audit inspected 19873 authored Rust files for executable documentation fences. It found 2 inline Rustdoc examples; text and ignored code blocks do not execute. These executable bodies require canonical extraction before completion.

```json
[
  {
    "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs",
    "line": 1457,
    "flag": "compile_fail"
  },
  {
    "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs",
    "line": 1463,
    "flag": ""
  }
]
```

## Post-extraction Audit and Runtime

At 2026-09-08T23:10:34.338693+00:00, a repeat audit enumerated 19874 authored Rust files with `rg --files --hidden`, excluding dependency, build, generated, and ticket trees. Executable line-doc fences: 0. Block-doc comments containing fences: 0. Documentation attributes using fences or source includes were inspected separately below.

The real UI-contract `cargo test --locked --doc -- --nocapture` consumer ran through public Bun/Nx and passed both canonical examples. The original examples referenced removed string/build APIs; they now use current `UiText`, `Label`, and `try_build`. The negative case is rejected specifically with E0599 because `ImageBuilder<NoAlt>` lacks the required conversion trait; the described and decorative positive case executes successfully.

```json
{
  "executableLineDocFences": [],
  "docAttributes": [
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs",
      "line": 1457,
      "source": "#[doc = concat!(\"```compile_fail\\n\", include_str!(\"../../🧪️tests/🚫️image-description-required/🦀️.rs\"), \"\\n```\\n\\n\")]"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs",
      "line": 1458,
      "source": "#[doc = concat!(\"```\\n\", include_str!(\"../../🧪️tests/🖼️image-described/🦀️.rs\"), \"\\n```\")]"
    }
  ],
  "fencedBlockDocs": []
}
```
