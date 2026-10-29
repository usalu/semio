# Thesis Path and Metadata Audit

Bounded read of inventoried thesis/supporting files only; no broad scan or heavy process.

```json
{
  "files": 34,
  "maximumAbsoluteUnits": 161,
  "over256": [],
  "missing": [],
  "longest": [
    {
      "path": "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts",
      "units": 161
    },
    {
      "path": "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts",
      "units": 151
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🟦️.ts",
      "units": 142
    },
    {
      "path": "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️pdf-consumption.json",
      "units": 138
    }
  ],
  "privateBounds": {
    "name": "print-thesis-fixture",
    "maximumFiles": 2048,
    "maximumFileBytes": 67108864,
    "maximumBytes": 536870912,
    "maximumPathLength": 256,
    "package": {
      "name": "print-thesis-fixture",
      "private": true,
      "type": "module",
      "workspaces": [
        "packages/*"
      ],
      "semio": {
        "workspace": {
          "schemaVersion": 1,
          "members": [
            "packages/*"
          ],
          "owners": []
        }
      }
    },
    "project": {
      "name": "print-thesis-fixture",
      "targets": {}
    },
    "lock": {
      "lockfileVersion": 2,
      "configVersion": 1,
      "workspaces": {
        "": {
          "name": "print-thesis-fixture"
        },
        "packages/input": {
          "name": "print-thesis-input",
          "version": "0.0.0"
        }
      },
      "packages": {
        "print-thesis-input": [
          "print-thesis-input@workspace:packages/input"
        ]
      }
    },
    "module": "module(name = \"print_thesis_fixture\")\n",
    "inputPackage": {
      "name": "print-thesis-input",
      "version": "0.0.0",
      "private": true
    }
  }
}
```

## Native References and Metadata

```json
{
  "relativeImports": 29,
  "missing": [],
  "readmeThesis": true,
  "targets": {
    "contract": true,
    "render": true
  },
  "publication": [
    {
      "id": "thesis",
      "expected": [
        "Master’s Thesis",
        "Reproducible Research Through Explicit Design Models",
        "Alex Example",
        "Master of Science",
        "Architecture and Technology",
        "Abstract",
        "Kurzfassung",
        "Introduction",
        "Background and Related Work",
        "Research Method",
        "Results",
        "Discussion",
        "Conclusion and Future Work",
        "Evaluation Protocol",
        "Data Availability"
      ],
      "bibliography": [
        "Literate Programming",
        "Best Practices for Scientific Computing"
      ],
      "citationKeys": [
        "knuth1984",
        "wilson2014"
      ],
      "firstBodyMarker": "Research findings can only be checked"
    },
    {
      "id": "phd",
      "expected": [
        "Dissertation",
        "Semio Dissertation Template",
        "Semio Research",
        "Doctor of Engineering",
        "Abstract",
        "Kurzfassung",
        "Introduction",
        "Method",
        "Results",
        "Conclusion",
        "Supplementary Data"
      ],
      "bibliography": [
        "Print Framework Architecture",
        "The expl3 Programming Language",
        "Port-Based Assemblies as a Design Representation"
      ],
      "citationKeys": [
        "semio2026",
        "expl32024",
        "dissertation2026"
      ],
      "firstBodyMarker": "A dissertation is a"
    }
  ]
}
```

All inventoried authored absolute paths fit256units in this checkout, maximum161. Private projection additionally checks destination paths against256 and the managed private-root API preflights deeper store paths. Relative module references resolve exactly on case-sensitive Linux, avoiding Windows-only case assumptions; emoji source names follow the existing repository naming and are remapped to ASCII compiler names by the unchanged staging layer. No new reserved Windows device basename or trailing-dot/space name was introduced. Both root templates explicitly select English, and publication expectations now use the English master heading. The artifact-target underscore correction changes only compiler target DATA and not native source filenames or metadata keys. No PDF or cross-platform execution result is claimed by this source audit.

The subsequently reviewed compiler entry and neutral fixture/feature/test remain below the maximum inventoried path length. They use existing emoji taxonomy and relative imports, preserve the actual physical runtime directory, and route the genuine caller workspace through the existing ScriptRouter constructor. No TypeScript source projection or native filename workaround is introduced.
