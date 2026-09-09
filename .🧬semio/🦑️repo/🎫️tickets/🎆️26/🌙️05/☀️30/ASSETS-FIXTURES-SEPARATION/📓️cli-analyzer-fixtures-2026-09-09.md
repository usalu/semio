# CLI Analyzer Fixture Ownership

Fourteen intentionally malformed/fixed source examples were stored under production assets. CLI analyzer tests are their actual consumers, so the examples now live in the language-neutral CLI owner’s analyzer-path fixture bundle. Sample bytes and intentionally colliding generic directory identities are preserved. CLI consumers and the existing exact-path taxonomy exception now point at the canonical owner. Runtime verification now passes for the selected analyzer consumers, after fixing the scope matcher defect described below.

## Preserved Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🟦️.tsx",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🟦️.tsx",
    "sha256": "9c85d1c32fa064a0511684a9550217755c9c0861cb76e2067a6b723ff9db8fed"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx",
    "sha256": "b370f11700e665e24c510c87121bc2da7588e4fe64365779bec45e842fd4aa3f"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
    "sha256": "891eb3ce3a40b130d1cfca6b7230df6ba17ba4d482c92c6593ce1b9d440d46bb"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐍️.py",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🐍️.py",
    "sha256": "6554d6b61ec0250f7deafe817d4f8f50144f98da36b51d38641a28d45ad0e453"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐹️.go",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🐹️.go",
    "sha256": "c5f6b542a9a8590fcafc6b2b241d923a2c2956ebbb1518ddc3ddfdb7651a0064"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file/🟦️.tsx",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file/🟦️.tsx",
    "sha256": "2d5f32d341b1da719978f1996854d5ecfad3fa0cf790cf424b964337e390319d"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file/🔷️.cs",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file/🔷️.cs",
    "sha256": "a04a5021db708c155964e1cf51df2d3491c69806afb6d254ec3e5a01f3cc955d"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file/🐍️.py",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file/🐍️.py",
    "sha256": "ac7d1ffce1f4acb78c45a43d93dd3eda15848b37db9efe3094bc3d9f75bea34c"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx",
    "sha256": "edefa50320592b2a29dfe3944696d7cd4517b975b2108684e534c05a6ce5981f"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx",
    "sha256": "f9131700bad580d2a4481936409ca69b149fa77ed2459e2ef31186c70c1c9870"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
    "sha256": "b9b12b4d7fe9d49831f5d35c6950701dfa3f3f1d00e74720a4b8990218449900"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
    "sha256": "1797d057d407c39ef381c2908f6fecd44d86be5ae3d4af9a678636272f29eca3"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐍️.py",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🐍️.py",
    "sha256": "2c0ae605efdcbdfdf0a2cb9dccc41c22a4bfbf446fa4824cab092646f28127b0"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐹️.go",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🐹️.go",
    "sha256": "8b5b2178ae2ee5fe30fd00f4adb9a7c952b8f8df82cb5980c8fbb9840e7e4293"
  }
]
```

## Authored Paths

```json
[
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️cli-analyzer-fixtures-2026-09-09.md",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🔷️.cs",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐍️.py",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🐍️.py",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-invalid/🐹️.go",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-invalid/🐹️.go",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file/🔷️.cs",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file/🔷️.cs",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file/🐍️.py",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file/🐍️.py",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🟦️.tsx",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🔷️.cs",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐍️.py",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🐍️.py",
  "🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/🧪️file-fixed/🐹️.go",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some/📁️folder/🧪️file-fixed/🐹️.go"
]
```

## Runtime Evidence and Scope Matcher Repair

The actual Bun/Nx canonical Go runner initially failed both malformed-fixture checks. A temporary diagnostic proved that the moved input existed, yielded 380 bytes, parsed three sections, and produced 16 direct policy breaches; the public policy dispatcher selected no code policy. Its owned glob compiler did not support the brace alternation in `**/*.{ts,tsx,py,cs,go,rs}`. It also corrupted multibyte Unicode pattern literals by compiling individual bytes. The temporary diagnostic has been removed.

A new language-neutral schema and nine vectors cover policy language selection, negative extensions, Unicode paths, nested alternatives, directory alternatives, and literal/escaped braces. Before the repair, the real Go runner failed five vectors; the minimatch reference verified all nine expected results. The compiler now handles balanced comma alternatives and preserves Unicode runes. No runtime dependency was added. The CLI's existing test route runs the independent minimatch/Ajv oracle before the Go runner.

The final selected Bun/Nx Go run passed all nine glob vectors, both malformed/fixed language fixture checks, and three file-header extraction checks. Raw logs are temporary ticket outputs: `cli-glob-red.log`, `cli-analyzer-diagnostic.log`, and `cli-analyzer-green.log`. The current evidence does not claim that the complete CLI suite passed.

## Additional Authored Paths

```json
[
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/glob/🃏️glob.go",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🃏️glob/🐹️.go",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🃏️glob/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🃏️glob/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧬️schema/🃏️glob/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts"
]
```

## Fixture Immutability

The autofix consumer previously edited its checked-in fixture and restored it afterward. It now stages the example below the test temporary root before running the real policy and autofix implementation. The actual canonical Go run passed both `TestFixApplyAutofixes` and `TestFixNonAutofixableNotFixed`, with the nine-vector minimatch oracle also passing.
