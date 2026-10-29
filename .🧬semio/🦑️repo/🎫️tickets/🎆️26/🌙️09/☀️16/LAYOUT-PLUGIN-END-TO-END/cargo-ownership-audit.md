# Cargo Ownership Audit

2026-10-28. Read-only Python tomllib parsing of the bounded layout artifact Cargo tree and root build-owner registry. No compiler, graph or test launched. Checked exact package.workspace resolution, explicit workspace.members membership, unique package names within this artifact tree, one matching build-owner row, library entry existence, every declared workspace dependency alias, and relative dependency Cargo path/package identity for all six new owners.

```json
[
  {
    "name": "semio-s-artifact-layout-layout-editor-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 14,
    "owner_rows": 1,
    "absolute_manifest_length": 156,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-scene-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 19,
    "owner_rows": 1,
    "absolute_manifest_length": 166,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-engine-flow-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 5,
    "owner_rows": 1,
    "absolute_manifest_length": 165,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-diff-plan",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 2,
    "owner_rows": 1,
    "absolute_manifest_length": 166,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-sqlite-member-tests",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 5,
    "owner_rows": 1,
    "absolute_manifest_length": 166,
    "issues": []
  },
  {
    "name": "semio-s-artifact-layout-layout-mutations-decisions",
    "workspace": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout",
    "relative_dependency_paths_checked": 9,
    "owner_rows": 1,
    "absolute_manifest_length": 176,
    "issues": []
  }
]
```

The managed default Python is3.9without tomllib; the retained audit script ran under the installed Python3.12 through the managed repo tools Python entry.
