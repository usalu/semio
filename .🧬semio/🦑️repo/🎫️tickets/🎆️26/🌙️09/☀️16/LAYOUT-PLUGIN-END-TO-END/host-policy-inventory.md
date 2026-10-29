# Host Policy Source Inventory

Canonical readRustSourceClosureV1 with macOS ARM64/unix flags, cfg(test) only for actual focused standalone targets. Normal libraries are measured without cfg(test); their existing broad unit suites are not silently selected. The Sourcing implementation lives in its declared current first-party delegate package, which is included here. This is source admission only, not compiler or native execution.

~~~json
[
  {
    "name": "semio-s-artifact-process-process3d-editor",
    "kind": "normal",
    "files": 6,
    "bytes": 130682,
    "lines": 1865,
    "fits": true
  },
  {
    "name": "semio-s-artifact-process-process3d-editor",
    "kind": "host-test",
    "files": 1,
    "bytes": 1671,
    "lines": 24,
    "fits": true
  },
  {
    "name": "semio-s-artifact-cad-cad-editor",
    "kind": "normal",
    "files": 11,
    "bytes": 130107,
    "lines": 1749,
    "fits": true
  },
  {
    "name": "semio-s-artifact-cad-cad-editor",
    "kind": "host-test",
    "files": 1,
    "bytes": 1061,
    "lines": 21,
    "fits": true
  },
  {
    "name": "semio-s-artifact-sourcing-curation",
    "kind": "normal",
    "files": 2,
    "bytes": 16979,
    "lines": 278,
    "fits": true
  },
  {
    "name": "semio-s-artifact-sourcing-curation",
    "kind": "host-test",
    "files": 1,
    "bytes": 1640,
    "lines": 24,
    "fits": true
  },
  {
    "name": "semio-s-artifact-sourcing-curation-tests",
    "kind": "normal",
    "files": 13,
    "bytes": 119237,
    "lines": 1741,
    "fits": true
  }
]
~~~
