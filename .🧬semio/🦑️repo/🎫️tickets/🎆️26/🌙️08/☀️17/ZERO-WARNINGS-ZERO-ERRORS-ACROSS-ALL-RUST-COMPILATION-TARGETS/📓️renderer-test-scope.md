# Renderer Test Scope Review

Pass 329 applies a conservative reference graph to named items already diagnosed as unused. Rust attributes are preserved in the reference stream, so serde callback names count as generated-code dependencies. Existing cfg(test) and cfg(all(test,...)) scopes mark test roots; any other external reference marks a production root. Ambiguous names and unknown contexts remain excluded. Associated impl blocks are attached only to unambiguous named types. No source has been changed by this inventory.

[
  {
    "name": "POINTER_EDGE_STATE",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "static",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 1016,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 1029,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "WindowMeasuresRailOutcome",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 14938,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 14942,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 14946,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 14957,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 14963,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 14997,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 15006,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "INTRODUCED_PULSE_PERIOD_MS",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 10918,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 10918,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 17180,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 17181,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 17187,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "INTRODUCTION_INFO_BOX_GAP",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 10939,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 17224,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 17225,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 17232,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "Paint2dFlatLayer",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2008,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7064,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "PAINT2D_NAVIGATOR_PADDING",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2030,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2031,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "TableColumn",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2156,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "TableSortJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2160,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "TableCellPayload",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "enum",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2120,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2127,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2128,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2129,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2139,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "TableCellButtonPayload",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "TableCellPayload"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "BlockListBlockJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "BlockListStepJson"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "BlockListStepJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2410,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "BlockListPaletteEntryJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2411,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "DiffLineOperation",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "enum",
    "callers": [
      "DiffLine"
    ],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2794,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2796,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2797,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2799,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2800,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2816,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2820,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2823,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2828,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2832,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2878,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2881,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2884,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2892,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2893,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2894,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2914,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2922,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2923,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2924,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2932,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2933,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2934,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2942,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2943,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2944,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2958,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "DiffLine",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2788,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2794,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2796,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2797,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2799,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2800,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2816,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2820,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2823,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2828,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2832,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "DIFF_LCS_CELL_BUDGET",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 2790,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "EventFeedEntryJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3089,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3105,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3187,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3200,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3211,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3212,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "HistoryColumnAuthorJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "HistoryColumnJson"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "HistoryColumnJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3329,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3351,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3402,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3513,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3514,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3552,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "HISTORY_LANE_PITCH",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3336,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3345,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3345,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "HISTORY_LANE_PAD",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3336,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3345,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "HISTORY_AUTHOR_SLOT",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3412,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CanvasFillJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "CanvasLayer"
    ],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4282,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4319,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7013,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7033,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CanvasGradientStopJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "CanvasFillJson"
    ],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4077,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 6971,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 6971,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7019,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7019,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7038,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7038,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CanvasStrokeJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "CanvasLayer"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "CanvasImageFieldJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "CanvasLayer"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "CanvasTextFieldJson",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "CanvasLayer"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "CanvasLayer",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3794,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4336,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4422,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 6961,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 6963,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 6965,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7052,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "Canvas2dPacketText",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "Canvas2dPacketItem"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "Canvas2dPacketItem",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4370,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4414,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "canvas2d_packet_text_size",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "function",
    "callers": [
      "Canvas2dPacketText"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "decode_canvas_image_source",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "function",
    "callers": [
      "queue_canvas_image_upload_sized"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "decode_canvas_image_bytes",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "function",
    "callers": [
      "queue_canvas_image_upload_sized"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "queue_canvas_image_upload_sized",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "function",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 12,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 1985,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 3946,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CANVAS_GRADIENT_BANDS",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4300,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4301,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4302,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4305,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4324,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4325,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4326,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CANVAS_CIRCLE_SEGMENTS",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4243,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4251,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4328,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CANVAS2D_SELECTION_RING",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4507,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4510,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4578,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "CANVAS2D_SELECTION_GLOW",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4506,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4509,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 4579,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "INK_RESIZE_HANDLES",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "const",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 6390,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "IconRenderCameraFields",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "IconRenderRequestFields"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "icon_render_default_zoom",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "function",
    "callers": [
      "IconRenderCameraFields"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "IconRenderLightsFields",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "IconRenderRequestFields"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "IconRenderMaterialFields",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "IconRenderRequestFields"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "IconRenderRequestFields",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7739,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 7777,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "VfsDescriptorKind",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "VfsSchema"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "VfsFileNodeKind",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "VfsSchema"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "VfsDescriptorColumn",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "VfsFileNodeKind"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "VfsSchema",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8018,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8081,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8088,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8096,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "TextEditorCompletionItem",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8189,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8339,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8783,
        "test": true
      },
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8783,
        "test": true
      }
    ],
    "ranges": 1
  },
  {
    "name": "TextEditorSpan",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [
      "TextEditorRenameInfo"
    ],
    "external": [],
    "ranges": 1
  },
  {
    "name": "TextEditorRenameInfo",
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "kind": "struct",
    "callers": [],
    "external": [
      {
        "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "line": 8194,
        "test": true
      }
    ],
    "ranges": 1
  }
]


## Pass 330 — Restrict Test-Only Items

Applied cfg(test) to the 51 reviewed items and retained every original attribute/body. The serde default helpers move with the fixture types that reference them. Removed Paint2dAssetJson and VfsVisibleRow only after unmasked full renderer searches found no references beyond their definitions. No methods or live ownership structures were removed. Native and browser compiler verification is pending.


## Pass 395 — Additional Function Reachability Audit

Scanned 25 renderer source files. Candidate functions must have a unique name across those files, already have a dead-code diagnostic, and be reachable from existing cfg(test) scopes without any observed production root. String literals remain in reference scans so attribute callback strings are retained. Ambiguous names and trait declarations remain excluded. 48 candidates require source review before edits.

- try_reserve_fresh_packet — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- from_json — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
- from_typed — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
- submit_shell_io — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_plugin_examples — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- dock_tab_bars_for_drop — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- sync_status_label — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- chrome_text — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- window_overlay_max_width — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measure_window_measure_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measure_window_measures_body_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- register_tooltip — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- open_dialog — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- start_introduction — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- register_element_rect_fallback — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- element_rect_is_fallback — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_completion_suffix — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_ghost_accept_on_click — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- left_tabs — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- right_tabs — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_left_tab_id — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_right_tab_id — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_element_rects — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_window_silhouettes — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_element_rect — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_example_dropdown — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_action_list — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measures_for_kind — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_for_kind — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_utility_options_rail — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_window_measure_tree — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_window_measure_one — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_engagement_input — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_engagement_control — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_form_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_arg_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_arg_display_string — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_staged_text_field — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- paint_staged_input_box — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- context_menu_level_width — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- shell_panel_tab_label — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- is_visible — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs
- fire — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs
- pending_len — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs
- batch_is_empty — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- begin_shutdown — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- shutdown_step — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- render_presence_bar — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs


## Pass 397 — Additional Function Reachability Audit

Scanned 25 renderer source files. Candidate functions must have a unique name across those files, already have a dead-code diagnostic, and be reachable from existing cfg(test) scopes without any observed production root. String literals remain in reference scans so attribute callback strings are retained. Ambiguous names and trait declarations remain excluded. 53 candidates require source review before edits; functions with no test reachability are removal candidates so no compiled parent retains a call to a gated child.

- try_reserve_packet — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- try_reserve_fresh_packet — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- observe_engine_surface_packet_freshness — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- from_json — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
- from_typed — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
- submit_shell_io — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_plugin_examples — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- dock_tab_bars_for_drop — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- sync_status_label — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- publish_presence_heartbeat — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- chrome_text — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- window_overlay_max_width — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measure_window_measure_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measure_window_measures_body_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- register_tooltip — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- open_dialog — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- start_introduction — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- register_element_rect_fallback — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- element_rect_is_fallback — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_completion_suffix — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_ghost_accept_on_click — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- left_tabs — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- right_tabs — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_left_tab_id — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_right_tab_id — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_element_rects — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_window_silhouettes — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_element_rect — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_example_dropdown — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_action_list — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measures_for_kind — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_for_kind — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_utility_options_rail — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_window_measure_tree — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_window_measure_one — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_engagement_input — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_engagement_control — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_form_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_arg_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_arg_display_string — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_staged_text_field — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- paint_staged_input_box — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- context_menu_level_width — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- shell_panel_tab_label — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- is_visible — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs
- fire — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs
- pending_len — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs
- take_typed_operation_result — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- take_typed_operation_result_page — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- batch_is_empty — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- begin_shutdown — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- shutdown_step — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- render_presence_bar — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs


## Pass 398 — Additional Function Reachability Audit

Scanned 25 renderer source files. Candidate functions must have a unique name across those files, already have a dead-code diagnostic, and be reachable from existing cfg(test) scopes without any observed production root. String literals remain in reference scans so attribute callback strings are retained. Unlike preliminary passes395/397, cfg scopes are interpreted recursively: all must require test in at least one conjunct and any must require it in every branch; cfg(any(wasm,test)) is production code. No edits were based on those preliminary passes. Ambiguous names and trait declarations remain excluded. 53 candidates require source review before edits; functions with no test reachability are removal candidates so no compiled parent retains a call to a gated child.

- try_reserve_packet — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- try_reserve_fresh_packet — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- observe_engine_surface_packet_freshness — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs
- from_json — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
- from_typed — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs
- submit_shell_io — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_plugin_examples — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- dock_tab_bars_for_drop — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- sync_status_label — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- publish_presence_heartbeat — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- chrome_text — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- window_overlay_max_width — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measure_window_measure_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measure_window_measures_body_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- register_tooltip — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- open_dialog — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- start_introduction — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- register_element_rect_fallback — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- element_rect_is_fallback — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_completion_suffix — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_ghost_accept_on_click — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- left_tabs — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- right_tabs — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_left_tab_id — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- active_right_tab_id — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_element_rects — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_window_silhouettes — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- resolve_introduction_element_rect — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_example_dropdown — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_action_list — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- measures_for_kind — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- engagement_for_kind — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_utility_options_rail — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_window_measure_tree — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_window_measure_one — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_engagement_input — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_engagement_control — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_form_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_arg_height — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- staged_arg_display_string — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- render_staged_text_field — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- paint_staged_input_box — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- context_menu_level_width — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- shell_panel_tab_label — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
- is_visible — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs
- fire — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏰️deadlines/🦀️.rs
- pending_len — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs
- take_typed_operation_result — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- take_typed_operation_result_page — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- batch_is_empty — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- begin_shutdown — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- shutdown_step — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- render_presence_bar — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs


## Pass 400 — Apply the Refined Reachability Audit

Moved 48 existing renderer helper methods/functions into cfg(test), preserving every body and attached attribute. Removed five uniquely named private functions with no production or test root: try_reserve_packet, observe_engine_surface_packet_freshness, publish_presence_heartbeat, take_typed_operation_result, and take_typed_operation_result_page. The presence helper’s attached documentation and native-only attribute were removed with it. The incoming-reference closure includes the removed parents of test-only callees, avoiding compiled calls to gated children. No fields, trait declarations, serde callback strings, or ambiguous method names were removed. Native/browser compilation and existing renderer laws must validate this batch.


## Pass 403 — Engine Surface Fixture Dependencies

The engine surface snapshot, packet reservation, destination enum, seven private identity/reservation methods now compile with their existing fixture callers. Both private publish_reserved methods are accounted for: their calls are in cfg(test) surface/packet laws and the already-test-only ensure_surface helper. Removed get_token_mut only after finding its declaration as the sole local executable reference; its registry type is private to this module. Live packet disposal, surface identity, and retirement fields remain. Native/browser checks and engine fixture laws pending.


## Pass 404 — Text Editor and Measure Fixture State

Restricted the three text-editor state/menu types plus window-measure traversal frame/capacity to their existing test-only helpers. Removed both platform declarations of TEXT_EDITOR_UI_STATE after renderer-wide raw searches found no access beyond their declarations; tests own their state directly. Kept all state fields and test bodies. Fresh native/browser compiler and fixture validation pending.


## Pass 423 — Remaining Private Function Reachability

Rebuilt the unique-name function graph from the completed native 402 and browser 401 diagnostics. Already test-gated functions are included as candidates, and calls between dead helpers form graph edges before test/production root classification. Attribute strings remain observable references. This read-only pass identifies removal candidates, additional test-only helpers, and already scoped helpers to keep; ambiguous methods remain excluded.


## Pass 424 — Remove Unreachable Shell Helper Chains

Removed 55 private Shell functions/methods with no production or test root in the rebuilt graph. These included chains already gated to tests but never invoked by a test. Attached cfg attributes and documentation were removed with each item. Two additional helpers used by existing tests now share their test scope. Eight already scoped functions with test roots were retained. Guarded source edits preserve live renderer paths and ambiguous methods; fresh native/browser checks remain required.


## Pass 430 — Engine State and Introspection Scope

Removed the map-request cursor that had no constructor calls and whose two storage slots were always initialized to None. Its empty retirement phase now advances directly to Board; actual map ownership and retirement remain in MapHost. Removed the unused build-context DPR and duplicate published GPU dimensions; generation fields used only to make packet fixtures are scoped to tests, while the production empty resource sink uses Default. Eight introspection items consumed only by browser exports now compile only for wasm32; the native walk/path tests retain their shared helpers. Source guards and full-file concurrent checks cover all writes. Native/browser compilation must be rerun after these changes.


## Pass 431 — Store Only Consumed Scene Projection Fields

SceneSurfaceState's unused image-digest maps are removed. Paint2d interaction decoding now projects the camera only; layer data is not consumed by these handlers and remains in the original document owned by the paint surface. Valid document camera values and defaults are unchanged, while unrelated layer fields are ignored by this projection. The navigator's test record retains only the geometry it uses. The icon-chrome test projects width and height from its existing full request fixture. Five unread grid/pencil fields are removed from both Ink interaction projections and their fixture/default constructors; snap and eraser controls remain. These private Deserialize projections do not serialize or rewrite stored documents. Existing navigator, icon-chrome and Ink interaction laws are the relevant runtime checks; fresh compilation is required.


## Pass 432 — Shell Fixture Scope and Empty Action Registry

Scoped the synchronous preference test oracle (capture, load/write helpers and grouped lock values) to tests; production uses its existing resumable preference steps. PrefsStore now follows its browser/test implementers. Removed two unconstructed measure-render fixtures, their unused traversal capacity, and the never-populated measure action registry. Its unreachable dispatch alternatives are removed; populated widget_maps select/toggle/slider dispatch remains. Removed unused tutorial bookkeeping and Dock's already-applied corner value from the resulting layout record. Removed imports left without consumers by pass 424. Full-file guards precede writes; new native/browser diagnostics remain the acceptance check.


## Pass 434 — Isolate the Unrouted Kernel-Mailbox Fixture

The kernel_seam module's own documentation and the repository call graph agree: its default exchange echoes a surface, no production caller submits intents or drains outcomes, and the host only installs an unused waker. The module is now test-only. Removed that idle fixture from OsHost construction/retirement and the two platform waker installations. Actual RuntimeMailbox wakeups, frame completions and kernel-runtime retirement remain. Its three mailbox/wakeup laws are retained. Updated module documentation to state the current ownership boundary. All sixteen Stdio 433 laws passed; the 419 link receipt also contains one macOS unwind-section warning, now recorded accurately in the verification report.


## Pass 435 — Remaining Platform and Test Helper Boundaries

Removed the now-unused DPR argument from the pre-input frame step and the already-applied bootstrap DPR cache. Window-based boot is native-only; browser boot uses its separate OffscreenCanvas bootstrap. Removed three unreachable browser fallback helpers whose callers are native-only, and scoped sync-document/FFmpeg helpers accordingly. Engine timing helpers had no callers and were removed. Local action constructors/argument mergers, scene double-click timing and three associated state fields are scoped to their existing tests. Browser 426 completed with 50 warnings on source preceding 432/434; that inventory includes one new unused DPR argument corrected here. The brief malformed module-doc boundary introduced in 434 was immediately repaired with an exact patch before this pass.


## Pass 436 — Remaining Private Function Reachability

Rebuilt the unique-name function graph from the completed native 402 and browser 401 diagnostics. Already test-gated functions are included as candidates, and calls between dead helpers form graph edges before test/production root classification. Attribute strings remain observable references. This read-only pass identifies removal candidates, additional test-only helpers, and already scoped helpers to keep; ambiguous methods remain excluded.


## Pass 438 — Remove Remaining Uncalled Test Rendering Chain

Rebuilt reachability after the Shell cleanup, correcting nested cfg-expression classification in the audit. Eight functions now have no call roots: the synchronous test document renderer, its pointer/offset/composition helpers, and two former Shell color helpers. Removed those definitions and their attached attributes/docs; current retained document-step rendering and the separately called geometry/introspection tests remain. Restored DockDropZone's test import and removed four diagnosed unused import groups after native 425 reported the missing symbol. Native 425 had one compiler error, 35 dead-code, four unused-import and two unused-variable diagnostics; fresh acceptance checks are required.


## Pass 439 — Remove Unused Scene-Host Borrows

FrameworkSceneHost now retains only the five input/theme/widget-state borrows consumed by its scene-slot paint implementation. Removed seven unused state/resource parameters from document-step traversal and the two forwarded resource arguments through Shell's main-window, panel and chrome methods. All definitions and call sites were arity-checked, with the removed arguments verified by name. Existing resource-presence checks remain at the frame boundary. Removed the unused chrome child y coordinate and fallback flag; fallback priority already follows HashMap entry insertion versus primary replacement. Fresh checks are required.


## Pass 443 — Native Continuation Boundaries

The standalone interaction continuation is only submitted by native plugin reload. Removed its unused wasm methods, scoped its restoration variant/match arm to native, and followed native-only reservation/cancellation and maintenance callers. The bounded completion queue retains corresponding native/test APIs. Removed the empty browser completion-waker method and an uncalled frame-stage getter. RuntimeDispatchCursor's standalone close helper is test-only, matching its event-retirement law; production drains events via take_next. No cancellation check or active continuation was removed. Fresh checks remain required.

Pass 455 removes three unread duplicate metadata fields from the progress presentation lease, retained replay request, and retained document build, plus the replay field's unused getter. Exact lease index/epoch checks, the raw replay job identity, the live request rejection identity, and the generation already carried by UiDocumentBuilder remain in use. Native compile validation is pending.

Pass 456 removes unused generation/timestamp copies and always-None hit-test/damage placeholders from RenderSnapshot. Constructors and the seven snapshot sink tests use the smaller cursor/IME/revision record. The existing generation comparison in AppPresentStep::Complete is unchanged and still rejects stale frames before publication. The sink publication revision remains; its production-read warning is still under review. Compiler/runtime verification is pending.
