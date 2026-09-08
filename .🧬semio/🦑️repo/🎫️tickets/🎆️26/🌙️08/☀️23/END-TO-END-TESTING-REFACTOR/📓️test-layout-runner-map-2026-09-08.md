# Canonical JavaScript and TypeScript Runner Map

Durable transcription of the executor’s current machine inventory. It records collection/dispatch routes, not an assertion that every case was executed. Categories overlap. No case in this snapshot is unreferenced. The independent final closeout audit sampled native suites, guarded registrations, explicit dispatch, and all ten feature adapters.

[Independent snapshot reconciliation](📓️runner-inventory-scope-audit-2026-09-08.md) accounts for the concurrent caching/Wasm case and its existing caller chain. [Runner closeout](📓️test-layout-final-closeout-audit-2026-09-08.md) records independent path and fixture checks.

```json
{
  "caseCount": 518,
  "counts": {
    "native-suite": 401,
    "guarded-production-registration": 140,
    "exported-self-test-invoked": 343,
    "feature-adapter": 10
  },
  "unreferenced": [],
  "rows": [
    {
      "path": ".storybook/🧪️tests/🔌️project-annotations/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/vitest.setup.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": ".storybook/🧪️tests/🧭️scope-resolution/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": ".storybook/🧪️tests/🧹️owned-ui-react-lint/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/.stage/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/.stage/🖼️assets/🧪️tests/🧪️metabolism-icon-codegen/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/.stage/🖼️assets/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "temp/merge/mit-bestand/präsentation/33.projektetage/🧪️tests/📽️deck/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "temp/merge/mit-bestand/präsentation/33.projektetage/js/index.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "♻️mit-bestand/🧺️demonstrator/🎭️playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "♻️mit-bestand/🧺️demonstrator/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "♻️mit-bestand/🧺️demonstrator/🪧️brand.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🎞️storybook-deck/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/🧪️tests/🧪️loadpresentationfromslideglob/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🧪️tests/🧪️owned-markdown-html-compiler/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🧪️tests/🧪️pdfcanvasresourceowner/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🧪️tests/🧪️compilemarkdowntohtml/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🖥️host/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🧬️schema/🟨️.js",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-live-visual-publication/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-numerical-microcursor/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🧪️tests/🧪️semio-tech-cad-js-runtime/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🧪️semio-tech-cad-js-module-aec-building-structure/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🧪️semio-tech-cad-js-module-aec-building/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧪️tests/🧪️semio-tech-cad-js-module-spatial-shape/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧪️tests/🧪️semio-tech-cad-js-module-aec-building-energy/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📐️cad/🧪️tests/🎨️storybook-renderer/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️high-consequence-office/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔥️retail-hydrocarbon-fire/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛢️liquid-retaining-fem-anchor/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🔩️high-strength-connection/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️loadbearing-wall/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️glulam-footbridge/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏠️aluminium-roof-purlin/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-600FF/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-610/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-620/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-630/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-640/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-650/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-900FF/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-910/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-920/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-930/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-940/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏛️bestest-950/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧪️tests/🧪️stdio-xml-valid-conformance-mirror/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🗄️stdio/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🧪️tests/🧭️action-cohort/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts",
      "categories": [
        "native-suite",
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧪️compose5d-preparetopologymodel/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/➡️hexagonal-cut-concrete-forest-right/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🧪️tests/🧪️semio-tech-cad-js-core-model-commit-mesh/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🌎️hub/🔨️modules/🛡️admin/🧪️tests/🛡️admin/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🌎️hub/🔨️modules/🛡️admin/🧪️tests/🧹️environment/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🧪️tests/🧪️admin-i18n/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🌎️hub/🧪️tests/🤝️integration/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🌎️hub/📦️packages/🟦️typescript/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧪️tests/🗿️artifact-runner/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/🧪️tests/🔬️watchdog-tail/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️tool-job-cooperative-maintenance/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/◻️2d/🧪️tests/🧪️semio-tech-s-2d-js/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/◻️2d/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧪️tests/🧪️kernelreturncontentframing-matches-the-shared-stream-and-independent-fra/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/✅️satisfy-version-requirements/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🚫️reject-malformed-version-input/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/📃️page/🧪️tests/🧪️actorbytepage-matches-shared-vectors-and-node-buffer-for-every-fixed-wor/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-load-natively-in-node-strip-only-mode-and-preserve-ev/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧪️tests/🧪️cold-pair-wit-status-codec-agrees-with-the-neutral-schema-and-rejects-ev/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🧪️tests/🧪️createboundedmailbox/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-publication-fixture-preserves-watchdog-and-te/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧪️tests/🧪️actor-ui-patch-receipt-matches-shared-canonical-vectors-and-the-independ/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧪️turnscheduler-lane-priority/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️tests/🧪️ownedactorturnoutput/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🧪️tests/🔬️wire-retirement/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️artifact-bootstrap-protocol/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/📡️replication/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️document-backbone-envelope-batch/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/📡️replication/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🔄️machine/🧪️tests/🧪️semio-tech-machine/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🔄️machine/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🧪️tests/🔬️parser-oracle/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎨️storybook/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🔀️merge-conflicting-utilities/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🧩️slot/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🧿️flatten-class-name-inputs/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🧬️style-variants/🧪️tests/🎨️compile-style-variants/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/✍️writer/🧪️tests/🔬️input-writer/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🔬️single-enqueue-publication/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/🧪️tests/🔬️input-commit-observer/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/🧪️tests/🔬️input-commit-observer/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧪️tests/🔬️input-root/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🌐️browser-host/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️surface-ownership/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️engagement-story-interaction/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/stories/ui/🎯Engagement.stories.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-types/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🔀️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/💭️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🕸️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/👤️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎀️Ribbon/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/✅️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/💬️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🎮️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/📨️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🪗️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🎚️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/📋️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/📑️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🌳️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🎛️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🍽️.story.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🧪️tests/📚️storybook/🟦️.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-prepared-raster-producer/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️conformance-corpus/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️fixed-list-storage/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🧪️tests/🧪️typednodefields-preflights-every-capture-before-transfer-under-private-r/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎀️Ribbon/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎭️storybook-interaction/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/stories/ui/⚙️Mode.stories.tsx",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/📚️storybook/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🖼️assets/🧪️tests/🧪️metabolism-icon-codegen/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧩️suite/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️hostresolvedargs/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧊️3d/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧬️schema/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🧬️schema/🧪️tests/📤️schema-export-entries/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-fixed-operation-registry/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🧪️tests/🧪️canvaseventbindingcontroller/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🧪️tests/🔬️interactivity-mcp-http-transport/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🌅️modern-era/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🏛️legacy-conformance/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💡️inference-bridge/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔄️end-to-end/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🧹️hygiene/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔓️open-ownership/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔓️open-ownership/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🧬️schema-oracle/🟨️.js",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️interactivity-store-sync/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️tool-job-artifact-envelope-rejection-transfer/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️initial-child-identity/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🧪️tests/🎚️canonical-os-ui-preferences/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📇️directory-home-bootstrap/🟦️.tsx",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-no-wasm/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📚️storybook-hosts-wasm/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📡️actor-backbone/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️artifact-creation-ready-opening/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧯️router-plugin-faults/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🩺️window-fault/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧩️component/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧪️browser-actor-action-handoff-validates-the-neutral-schema-and-exact-owne/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧫️fixtures/🌊️actor-import/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧪️tests/🧪️browser-actor-patch-handoff-validates-the-neutral-schema-and-exact-owner/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧪️tests/🪟️host-opening-context/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📖️generated-projection/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📚️storybook-plugins/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": ".storybook/playwright.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🪪️plugin-identity/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🧪️tests/🧪️pluginreturnwit-maps-every-canonical-drive-to-the-exact-wit-nesting-and/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🧪️tests/🧪️pluginpollcompositionwit-preserves-the-canonical-six-scalars-and-exact-n/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/♻️relay-lifecycle/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎚️mutate-os-config-identity/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️interactivity-shard-executor/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner-self-tests/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-live-fixed-replay/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-artifact-retained-command/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🧪️tests/🧪️rendertree/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/🧪️rendermedia/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🧪️tests/🧪️renderdocument/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🧪️tests/🧪️rendertable/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🧪️tests/🧪️renderimage/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🧪️tests/🧪️rendermesh/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🧪️semio-tech-framework-os-shell-reduce/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🧪️tests/🧪️os-shell-schema-module/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-artifact-submit/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-capability-open/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-bootstrap/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-read/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-create-catalog/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io-b1-b6/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-artifact-history/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-database-compaction/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-database-sync-hello/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-caller-migration/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-vcs-bridge/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️source-contract/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/💻️os/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🔬️schema/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript/vitest.config.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/🧪️tests/🔬️schema/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌎️hub/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️browser/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️service-readiness/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📇️artifacts/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️dependencies/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🚀️bootstrap/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-runtime-source/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-truth/🟦️.ts",
      "categories": [
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "📜️script.ts",
          "guarded": true,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/☂️frozen-coordinate-wildcard-coverage/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏃️run-vitest-config-argument-tokens/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏠️nx-workspace-root-file-reference/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📜️historical-document-evidence/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔍️filesystem/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔮️source-file-facts-oracle/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔮️typescript-declaration-facts-oracle/🟦️.ts",
      "categories": [
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥇️bare-reference-sibling-precedence/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
          "guarded": false,
          "dispatcher": false,
          "anotherCase": true
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission-io/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🔬️schema/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🔬️server-persistence/🟦️.ts",
      "categories": [
        "native-suite"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🖥️host-protocol-parity/🟦️.ts",
      "categories": [
        "feature-adapter"
      ],
      "references": []
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        },
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts",
      "categories": [
        "native-suite",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
          "guarded": false,
          "dispatcher": true,
          "anotherCase": false
        }
      ]
    },
    {
      "path": "🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts",
      "categories": [
        "native-suite",
        "guarded-production-registration",
        "exported-self-test-invoked"
      ],
      "references": [
        {
          "caller": "🧰️framework/📦️packages/🟦️typescript/🟦️.ts",
          "guarded": true,
          "dispatcher": false,
          "anotherCase": false
        }
      ]
    }
  ]
}
```
