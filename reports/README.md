# Reports

This folder contains generated reports from various linters, formatters, and validation tools.

Reports are generated automatically by pre-commit hooks and CI/CD pipelines.

## Available Reports

- `i18n.md` - i18n translation validation report
- `prettier.md` - Prettier formatting check report
- `eslint.md` - ESLint linting report
- `typescript.md` - TypeScript compiler check report
- `ruff.md` - Python Ruff linter report

## Generating Reports

Reports are automatically generated when running:

```bash
# Run all pre-commit hooks
pre-commit run --all-files

# Or run individual hooks
npx tsx hooks/i18n.ts
npx tsx hooks/prettier.ts
npx tsx hooks/eslint.ts
npx tsx hooks/typescript.ts
npx tsx hooks/ruff.ts
```

## Note

Report files (*.md) are gitignored except for this README.
