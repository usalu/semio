# Computed Fixture Read Audit

TypeScript AST inspection resolves literal filesystem reads, JSON property coordinates and lexical constants composed with join, resolve and dirname. Dynamic paths, parameters and runtime computations remain outside this bounded audit. Missing paths are candidates for review, not proof that every branch executes.

```json
{
  "files": 98,
  "observed": 370,
  "missing": []
}
```
