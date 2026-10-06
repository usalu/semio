# Current Mathematics Worker Host Proof — 2026-10-04

Root authorized an independently frozen worker closure while the separate Math set PDF checks continued. The actual enumerated closure contains 29 source paths and excludes Math stylesheets and family test helpers. It differs from the previous a422e2dd proof in exactly two released inputs: authored visualization catalog and print schema. The other 27 sources are unchanged. Initial hashes below were captured before dispatch and remained identical before the retry.

The existing retained 📜️async-runtime/📜️script.ts is unchanged. Its previous generated evidence was copied intact to 🗑️generated/async-runtime-before-math629 before output refresh. The first Nx exec dispatch, session 21368, reached terminal exit 1 before any host replay: Nx used the selected inference project cwd, while the harness resolves the print product from repository cwd. ENOENT occurred at schema load. The authoritative first terminal log is 🗑️generated/native-math629-worker-terminal.log. This is an invocation failure, not a worker test failure or PASS.

The corrected route uses bun nx exec --projects=@semio-tech/print-viz-inference -- -- bun --cwd=C:/git/semio <ticket>/📜️async-runtime/📜️script.ts. Bun help locally confirms that --cwd changes process cwd. Retry session 89766 is live; authoritative log is 🗑️generated/native-math629-worker-terminal-retry.log. No harness, product source or dependency is changed for this correction.

| Worker source | Initial SHA256 |
| --- | --- |
| C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🌗️mixing\🟦️.ts | 400624F3790F23CD71A69E13A2746EACC13007E0A661A7025E0EB85269E9DC79 |
| C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🔣️.json | 09E8B5AFB79A554B01429A7AB71D7A84B61195B9CA1738F1C0BDAC1DDE66AACA |
| C:\git\semio\🧰️framework\🔨️modules\🧬️schema\✅️validator\🟦️.ts | 2617C03029C26A0CB97B3CF7FAB1FA159B03C3F821D163BA55A55902C76170F3 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🎨print-design-token-paints\🧮️resolution\🟦️.ts | B3F26600887B2DE5B5FD755D989294BA84AB760E70B9E636D35260F0A30F46DD |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\📏️metrics\🔣️.json | 68829670A0124EBF1053816BEF7A05BF65684F71F33F28B31E6F4690AFFC12EB |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\📏️metrics\🟦️.ts | C2DE06B5DBE7B1D3CC6E7434F501F7D8739639C3CFFCBFE5EF079F825E5494A5 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\🔣️.json | B63EB4A1255760F5B19A02298DA33C34BF14C21A4135DB78EB344B02839AB451 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🖼️assets\🔣️viz-catalog.json | B5385563AADEAAFE81690E4065C83EBA6723CEAC60B4CDA20C5189ECF671D818 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\✅️validation\🟦️.ts | 75BBDFAC96B518A69A0E7C096C0AFC96718760AB44C17853D24F4A86FDEAA86B |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\✒️mark\🟦️.ts | B50EDBA869C3F21D35D58A854FC9601DC3972229229307341255F5242A7F748A |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌊flow\🟦️.ts | A0F20875427DBE943A80F74AA439652EFBB56F62A81EA296DEA60CC4466C4B01 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌍geo\🟦️.ts | 9D4D9DEB711813FA49F0446AD1D8FDC84BF0B40FFBF5C133C3990EBF37ABAF0C |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌳hierarchy\🟦️.ts | 4C84CEA6EE7F39110C071F32BE8D883B9103C8CB54CAB975C42968E9F99A08F8 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🎨theme\🟦️.ts | 7A65E22068CB4D9FAE540D3584D9E280056F3C74C2A60C9CE9DDAA1EF8232D64 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📍spatial\🟦️.ts | 58ADBB55E17D190F0D2092C320C3D102D776F3B9A8B1DFC5FAD778359B642EDF |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📐scale\🟦️.ts | 25837EBA287F63342521AE4123F79C609641E9EDC925F37091E78B60F74A1477 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📚️catalogue\🟦️.ts | DF531636E322FB46A7A85EAB4A6E8DF8FF8899BD000456B0878097779BD26B13 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🔢format\🟦️.ts | 6C6E25D5D6899030907782FC7245DBF7386FBF2997276BDAEB5DAC1B54101D0C |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🔣️.json | A1232C89DCF15C63C22962AF544E2521196529DC6A03B151141FDDA6A255D7D3 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🕸️network\🟦️.ts | DB0F836E4794EAC93FB4465BD2C0001337889CF5348F2F369DA0B14BDCF27668 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🖼️render\🟦️.ts | 83CBCA27822DB8ECB72CF041A18D999BE560122E8D19164934D8484B5B51DCA4 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🟦️.ts | EDB3DFE3639B08A14EF87D750E3AEE277A6DD08ABB647F91708A177224620DD9 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🥧shape\🟦️.ts | D854C5267131FD6B41FFFBF300097D87549AA02A250C172F5A8E75BED9363D1C |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧭coordinate\🟦️.ts | A71D09747C422927DB8B9D529A34A406A0599449887CB9C3409774562BD9E449 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧮transform\🟦️.ts | 631CD829B2B0F1CB4C0F00089D972008CB407D1153A6F5A283F9BAB1864C41D9 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧵️worker\🟦️.ts | 66D0167F473285D75E45AA214CCB1C3086521E8C62A089F6C5CF93BDAD75EA6E |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\📸️snapshot\📊️chart\🎨️color\🔣️.json | DEF3436D7F6EE7265D9E21DC44BCD9DDD22B18AA36485D2171133AFDA608B0B0 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\📸️snapshot\📊️chart\🟦️.ts | AB3977AF7BC621D2F1F379D646DD6E115556A95E5B507FE51910AE050164B405 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\🔣️.json | FBE6DDB5A8AD11E17FC4C9B5E158A59C8EF015C689C62F64CF10E6F51DC7555E |

Retry 89766 reached actual terminal exit 0. All three actual hosts passed. The retained evidence timestamp is 
04/10/2026 21:09:39
. Initial/final closure SHA256 is 
8292de6bf5bea6647d58c256128ba966f078c9b52e35c9b7fece302de8a548ac
, stable across all 29 paths. An independent post-terminal disk rehash also found zero drift. The first dispatch failure remains separately retained and is not relabeled as a passing test.

| Actual host | Version | Deterministic complete result | Progress | Fonts | Result admissions | Font variants | Independent timer cancellation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Bun | 1.4.2 | PASS | 8 monotonic events, total 9 | 9 matching transported texts and native selectors | 3 owned/AJV PASS | 10 owned/AJV PASS | fired at 129/50007, atomic cancellation |
| Chromium | 151.0.7922.34 | PASS | 8 monotonic events, total 9 | 9 matching transported texts and native selectors | 3 owned/AJV PASS | 10 owned/AJV PASS | fired at 129/50007, atomic cancellation |
| Node | v24.14.1 | PASS | 8 monotonic events, total 9 | 9 matching transported texts and native selectors | 3 owned/AJV PASS | 10 owned/AJV PASS | fired at 129/50007, atomic cancellation |

Every host reported completion counts 0,1,2,3,6,7,8,9. Already-aborted input produced zero progress and no partial plan, scene or TikZ. The timer workload also returned no partial outputs with print.chart.cancelled. Axis/Palette retain Share Tech Mono; ticks and A/B retain Anta in both plan and scene. Browser and Node entries/workers were freshly bundled from these sources.

| Fresh bundle | Bytes | SHA256 |
| --- | ---: | --- |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\browser\main.mjs | 1555077 | 678e980f4b647e241a341b8cf0a556373d9ecb14ab5e007c46e061e9e4263f7e |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\browser\🧵️worker\🟦️.ts | 2842306 | 2fc739baae8ee6c3853648fd97a7ed0f63f0739c0f9cfd0990103fca03fd4e78 |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\node\main.mjs | 1555077 | 678e980f4b647e241a341b8cf0a556373d9ecb14ab5e007c46e061e9e4263f7e |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\node\🧵️worker\🟦️.ts | 2842306 | 2fc739baae8ee6c3853648fd97a7ed0f63f0739c0f9cfd0990103fca03fd4e78 |

The exact per-host records are retained below so the result remains reviewable independently of generated-output cleanup.

```json
[
  {
    "host": "Bun",
    "version": "1.4.2",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  },
  {
    "host": "Chromium",
    "version": "151.0.7922.34",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  },
  {
    "host": "Node",
    "version": "v24.14.1",
    "pass": true,
    "evidence": {
      "complete": true,
      "deterministic": true,
      "progress": [
        {
          "completed": 0,
          "total": 9
        },
        {
          "completed": 1,
          "total": 9
        },
        {
          "completed": 2,
          "total": 9
        },
        {
          "completed": 3,
          "total": 9
        },
        {
          "completed": 6,
          "total": 9
        },
        {
          "completed": 7,
          "total": 9
        },
        {
          "completed": 8,
          "total": 9
        },
        {
          "completed": 9,
          "total": 9
        }
      ],
      "progressValid": true,
      "earlyProgress": 0,
      "earlyAtomic": true,
      "started": true,
      "fired": true,
      "startProgress": {
        "completed": 129,
        "total": 50007
      },
      "cancelAtomic": true,
      "texts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "sceneTexts": [
        [
          "0",
          "Anta"
        ],
        [
          "0.5",
          "Anta"
        ],
        [
          "1",
          "Anta"
        ],
        [
          "1.5",
          "Anta"
        ],
        [
          "2",
          "Anta"
        ],
        [
          "Axis",
          "Share Tech Mono"
        ],
        [
          "Palette",
          "Share Tech Mono"
        ],
        [
          "A",
          "Anta"
        ],
        [
          "B",
          "Anta"
        ]
      ],
      "fontTransport": true,
      "tikzFontSelectors": true
    },
    "admissions": [
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      },
      {
        "owned": true,
        "ajv": true,
        "errors": null
      }
    ],
    "fontAdmission": [
      {
        "surface": "plan",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "plan",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "absent",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "Custom Family",
        "expected": true,
        "owned": true,
        "ajv": true,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": 42,
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      },
      {
        "surface": "scene",
        "input": "non-text",
        "expected": false,
        "owned": false,
        "ajv": false,
        "pass": true
      }
    ]
  }
]
```

At the resumed source boundary, Catalogue reported a later route stopping before target because the workspace recipe requires Bun 1.3.14 while PATH resolves Bun 1.4.2. The observed worker proof above explicitly remains Bun 1.4.2, rather than a claim of execution under the recipe version. Read-only local checks resolve PATH through C:/Users/Ueli/AppData/Roaming/npm/bun.ps1 to C:/Users/Ueli/AppData/Roaming/npm/node_modules/bun/bin/bun.exe (1.4.2). A separate existing C:/Users/Ueli/.bun/bin/bun.exe reports 1.3.13; C:/git/semio/node_modules/bun/bin/bun.exe is absent. No dependency install, recipe modification or guard bypass was performed in this lane. Root owns the environment resolution for subsequent registered checks.

The configured Bun 1.3.14 replay is session 56564, currently live. Its official binary SHA256 was verified as 0187F68D843F825A72ADA4A7ECA60DB896ED753759A7F8252EDCD31AC1BF1B9C. PATH was changed only in the executing process. All successful 1.4.2 evidence was preserved at 🗑️generated/async-runtime-before-bun1314 before harness output refresh. This re-execution is required to validate the newly enforced configured runtime, while keeping the previous proof truthful at its actual version.

The following current dispatch context lies outside the historical 627-source input set. These hashes bind execution context; they do not assert ticket ownership of unrelated infrastructure changes or silently expand the 29-file runtime closure.

| Dispatch context | SHA256 |
| --- | --- |
| package.json | E2B5A359079249181D97257CB4456837DC4664CB6C4FF7FB63698A85E5214037 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts | 6EEF15D6F208D25D2D5EB6083F1C366DD94BC7BA10145AAAF24AF43353401884 |

Pinned dispatch 56564 reached terminal exit 1 before executing a host. Nx reported the project cycle @semio-tech/print-viz-inference → test-framework-products-print-da77cb-🎬️render-scene → @semio-tech/print-viz-inference. Local Nx exec source confirms that its default command graph recursively includes dependency projects and runs the same command in each. Its supported --excludeTaskDependencies option restricts this authored harness to the explicitly selected inference project. This does not alter the harness imports or its complete source fingerprint. The first pinned log remains 🗑️generated/native-math629-worker1314-terminal.log.

Configured retry 30701 is live through the same Nx exec route with --excludeTaskDependencies, explicit pinned Bun --cwd=C:/git/semio, and the single argument separator retained by Bun 1.3.14. All 29 initial input hashes remained identical before this retry. No source, graph, cache or dependency was modified to remove the cycle. The authoritative retry log is 🗑️generated/native-math629-worker1314-terminal-retry.log; actual host execution remains pending.

Pinned configuration counterexample: 30701 failed before host execution when Bun1.3.14 Transpiler import scanning produced mojibake Unicode module paths from the same source bytes. Both string and UTF8-byte inputs reproduced the failure. A bounded installed TypeScript AST scan agreed with the exact 29 non-type first-party import/export closure, excluding 17 type-only paths; no version branch or fallback was added. Old harness E76EB13BD25E6159DC7E6E307E31F0C0EE4DC210E229187C4AFBFD3CA149377D is retained in authored-inputs/native-worker-before-pinned-scanner/📜️script.ts; AST candidate 3A7BA6727665809EBCB3B1560E3918035DA95E51F01755E75CB1FBBF11927987 is retained in authored-inputs/native-worker-before-node-browser-controller/📜️script.ts.

Actual pinned AST route 74185 exited 1 after the Bun1.3.14 host completed the product evidence successfully, when in-process Playwright browser launch timed out at 180 seconds. Chromium and Node host completion are not claimed. A bounded existing-controller check launched Playwright through Node v24.14.1 and closed Chromium151.0.7922.34 successfully in 2.9s (native-worker1314-node-browser-controller-check.log). The uniform harness now uses that Node controller for the browser host, retaining the Bun-built actual browser worker and separately running the actual Node worker. Current authored harness SHA2608C9D70A395E23C2D6A363AA4FC68A252A2A5749CE54EC801DABA739FBFE08. Its next configured full proof must bind the repaired worker/validation source, so no previous 29 hash or earlier1.4.2 PASS is relabeled current.

Actual configured worker96190 exited0 at evidence timestamp 
04/10/2026 21:58:49
. Bun1.3.14, Chromium151.0.7922.34 and Nodev24.14.1 all passed. Each host published deterministic complete closed outputs,8 monotone progress records(0,1,2,3,6,7,8,9 of9), atomic pre-abort with0progress, actual50000-row timer cancellation after129/50007,9font entries transported from plan to scene, correct TikZ font selectors,3 owned/AJV wire admissions and10font-schema variant checks. The uniform Node browser controller completed actual browser WebWorkers; browser/Node worker bundles were compiled by pinned Bun.

Actual29 closure initial and final SHA256 73c362346738d0de2f30229ef7c1709cb9bc49faaa2e5b3e8d4b26a485800169, stable=true; independent terminal disk check found0drift. These are the repaired worker/validation bytes. Historical1.4.2 and earlier failed configured dispatches remain distinct.

| Actual first-party runtime input | SHA256 |
| --- | --- |
| C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🌗️mixing\🟦️.ts | 400624f3790f23cd71a69e13a2746eacc13007e0a661a7025e0eb85269e9dc79 |
| C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🔣️.json | 09e8b5afb79a554b01429a7ab71d7a84b61195b9ca1738f1c0bdac1dde66aaca |
| C:\git\semio\🧰️framework\🔨️modules\🧬️schema\✅️validator\🟦️.ts | 2617c03029c26a0cb97b3cf7fab1fa159b03c3f821d163ba55a55902c76170f3 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🎨print-design-token-paints\🧮️resolution\🟦️.ts | b3f26600887b2de5b5fd755d989294ba84ab760e70b9e636d35260f0a30f46dd |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\📏️metrics\🔣️.json | 68829670a0124ebf1053816bef7a05bf65684f71f33f28b31e6f4690affc12eb |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\📏️metrics\🟦️.ts | c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🔨️modules\🔤print-font-catalog\🔣️.json | b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🖼️assets\🔣️viz-catalog.json | b5385563aadeaafe81690e4065c83eba6723ceac60b4cda20c5189ecf671d818 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\✅️validation\🟦️.ts | 4482a28ee677bc8b85b930600ce4ab07b94df4dd56336a6347744f297f18d369 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\✒️mark\🟦️.ts | b50edba869c3f21d35d58a854fc9601dc3972229229307341255f5242a7f748a |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌊flow\🟦️.ts | a0f20875427dbe943a80f74aa439652efbb56f62a81ea296dea60cc4466c4b01 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌍geo\🟦️.ts | 9d4d9deb711813fa49f0446ad1d8fdc84bf0b40ffbf5c133c3990ebf37abaf0c |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🌳hierarchy\🟦️.ts | 4c84cea6ee7f39110c071f32be8d883b9103c8cb54cab975c42968e9f99a08f8 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🎨theme\🟦️.ts | 7a65e22068cb4d9fae540d3584d9e280056f3c74c2a60c9ce9ddaa1ef8232d64 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📍spatial\🟦️.ts | 58adbb55e17d190f0d2092c320c3d102d776f3b9a8b1dfc5fad778359b642edf |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📐scale\🟦️.ts | 25837eba287f63342521ae4123f79c609641e9edc925f37091e78b60f74a1477 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\📚️catalogue\🟦️.ts | df531636e322fb46a7a85eab4a6e8df8ff8899bd000456b0878097779bd26b13 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🔢format\🟦️.ts | 6c6e25d5d6899030907782fc7245dbf7386fbf2997276bdaeb5dac1b54101d0c |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🔣️.json | a1232c89dcf15c63c22962af544e2521196529dc6a03b151141fdda6a255d7d3 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🕸️network\🟦️.ts | db0f836e4794eac93fb4465bd2c0001337889cf5348f2f369da0b14bdcf27668 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🖼️render\🟦️.ts | 83cbca27822db8ecb72cf041a18d999be560122e8d19164934d8484b5b51dca4 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🟦️.ts | edb3dfe3639b08a14ef87d750e3aee277a6dd08abb647f91708a177224620dd9 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🥧shape\🟦️.ts | d854c5267131fd6b41fffbf300097d87549aa02a250c172f5a8e75bed9363d1c |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧭coordinate\🟦️.ts | a71d09747c422927db8b9d529a34a406a0599449887cb9c3409774562bd9e449 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧮transform\🟦️.ts | 631cd829b2b0f1cb4c0f00089d972008cb407d1153a6f5a283f9bab1864c41d9 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\💡️inferences\🧵️worker\🟦️.ts | 56a17cf00bfa0dff40560c2150a8434473ff6598aec3e6d6e97464bc437acbb8 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\📸️snapshot\📊️chart\🎨️color\🔣️.json | def3436d7f6ee7265d9e21dc44bcd9ddd22b18aa36485d2171133afda608b0b0 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\📸️snapshot\📊️chart\🟦️.ts | ab3977af7bc621d2f1f379d646dd6e115556a95e5b507fe51910ae050164b405 |
| C:\git\semio\🧰️framework\🛍️products\📓️print\🧬️schema\🔣️.json | fbe6ddb5a8ad11e17fc4c9b5e158a59c8ef015c689c62f64cf10e6f51dc7555e |

| Actual configured bundle | Bytes | SHA256 |
| --- | --- | --- |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\browser\main.mjs | 1555074 | f5b250366e6694ebd2339c5652d4b4cc753c011df57aeff45031341ffdbd16e1 |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\browser\🧵️worker\🟦️.ts | 2842815 | e1f5186e8fc1f7b2030bdf5cbad41897d56bded5e16d921b84566ac151f07570 |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\node\main.mjs | 1555074 | f5b250366e6694ebd2339c5652d4b4cc753c011df57aeff45031341ffdbd16e1 |
| C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️05\PRINT-VISUALIZATION-LIBRARY\🗑️generated\async-runtime-final\node\🧵️worker\🟦️.ts | 2842815 | e1f5186e8fc1f7b2030bdf5cbad41897d56bded5e16d921b84566ac151f07570 |
