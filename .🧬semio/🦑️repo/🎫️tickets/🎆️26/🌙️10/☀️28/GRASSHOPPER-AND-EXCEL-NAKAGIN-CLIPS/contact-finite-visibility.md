# Current Contact Finite Visibility Proof

All three fresh finite producers and all three independent surface queries passed against visual source SHA256 4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2. Every complete producer and query endpoint map is retained below and unchanged. The maps consume no VisualContracts fixture, so the concurrent test-only expected-bolt correction does not affect these results.

The unchanged existing producer audit-opaque-coverage.py ran through repo tools exec uv and the ticket verify.py wrapper, with --render and the following exact options:

- --source capsule --resolution 256 --output-name contact-capsule256 --baseline-name capsule-postcut256
- --source capsule --resolution 512 --output-name contact-capsule512 --baseline-name capsule-postcut512
- --source capital --resolution 512 --azimuth 135 --output-name contact-capital512 --baseline-name capital-postcut

Unspecified defaults remain tower0, elevation24, exterior detail, Slash, glass opacity1, and capsule azimuth55. Interpreter: C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe; UV --no-project python -B. Verification labels are contact-capsule256-cache, contact-capsule512-cache, contact-capital512-cache. All three exact original geometry/retained-ID baseline assertions passed. Base source-ID, geometric and silhouette comparisons are exact; high-resolution geometric and silhouette comparisons at1024/2048 are exact. Normal-AA material differences remain explicit in complete control results below; no colour waiver is applied.

Queries used unchanged run-fragment-surface-audit.py with --input-dir generated/contact-{capsule256,capsule512,capital512}, distinct --output generated/contact-*-ray.json, and existing ROIs capsule-fragment-roi-256.json, capsule-fragment-roi-512.json, capital-fragment-roi.json. Capital also uses --include-steel. All actual paths are under this ticket generated directory. Verify labels: contact-ray-256-cache, contact-ray-512-cache, contact-capital-ray-cache. The existing producer/source/oracle maps include every captured geometry/raster/camera/ROI input.

Across1260pixel regions and31500centre/subpixel rays, there are zero analytic nearest-material, unsupported-source, layer-inversion or physical-culling errors. Capital has zero visible or nearest internal steel samples in this closed north/west view. This finite sampling does not prove every raster point or every other camera orientation.

Exactly two Cairo256 centre-owner discrepancies persist in all three stages: (52,84) Cairo63 versus centre42 and (78,90) Cairo64 versus centre63. The corresponding25sample grids split13/12 between the adjacent shell faces. Their literal shared edges, distances, sample counts and retained opaque source ownership are below. These are explicit raster-boundary attributions, not coplanar centre-depth ties or perfect centre matching. Every point-painter sample is correct.

The immutable validate-paint-carrier-suite.py replay passed13tests in0.537s with unchanged exact oracle endpoint maps. The tiny positive witness retains one triangle and carrier depth error5.551115123125783e-17. Before normal replay, resolved absolute source and destination paths were checked inside this ticket generated directory, then historical fixed-output directories were archived to generated/precontact-fragment-ray-oracle and generated/precontact-paint-lineage. Earlier markdown reports remain historical and unchanged. The new outputs occupy their original fixed locations; full fresh records and hashes follow. No production or harness code changed.

All owned command handles are terminal: producers19766/94769/18181, queries79183/23442 and the immediate capsule512 query, plus immediate13test suite.

## Complete Maps and Results

```json
{
  "producers": {
    "contact-capsule256": {
      "provenance": {
        "start_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce"
        },
        "end_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce"
        },
        "unchanged": true
      },
      "controls": {
        "source_items": 2368,
        "retained_items": 470,
        "modes": {
          "256-geometric": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "256-silhouette": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "256-ids": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "256-aa-fill": {
            "stable_removed": {
              "channels": 527,
              "pixels": 182,
              "maximum": 46
            },
            "retained_order": {
              "channels": 198,
              "pixels": 107,
              "maximum": 2
            },
            "total": {
              "channels": 684,
              "pixels": 267,
              "maximum": 46
            }
          },
          "256-aa-strokes": {
            "stable_removed": {
              "channels": 550,
              "pixels": 198,
              "maximum": 46
            },
            "retained_order": {
              "channels": 409,
              "pixels": 160,
              "maximum": 9
            },
            "total": {
              "channels": 826,
              "pixels": 308,
              "maximum": 46
            }
          },
          "256-material": {
            "stable_removed": {
              "channels": 452,
              "pixels": 190,
              "maximum": 17
            },
            "retained_order": {
              "channels": 499,
              "pixels": 196,
              "maximum": 10
            },
            "total": {
              "channels": 837,
              "pixels": 330,
              "maximum": 17
            }
          },
          "1024-geometric": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "1024-silhouette": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "1024-material": {
            "stable_removed": {
              "channels": 1283,
              "pixels": 507,
              "maximum": 21
            },
            "retained_order": {
              "channels": 1649,
              "pixels": 612,
              "maximum": 7
            },
            "total": {
              "channels": 2853,
              "pixels": 1075,
              "maximum": 21
            }
          }
        }
      },
      "camera": {
        "towards": [
          0.5239881480393285,
          0.7483326291788591,
          0.4067366430758002
        ],
        "basis": [
          [
            -0.8191520442889918,
            0.573576436351046,
            0.0
          ],
          [
            -0.23329455426880485,
            -0.33317915266278375,
            0.9135454576426009
          ]
        ],
        "centre": [
          0.0,
          2.0,
          1.25
        ],
        "point": [
          0.0,
          0.0,
          0.0
        ],
        "scale": 1.0,
        "pixel_width": 256,
        "pixel_height": 256,
        "frame_width": 6,
        "frame_height": 6,
        "projection": "orthographic"
      }
    },
    "contact-capsule512": {
      "provenance": {
        "start_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1"
        },
        "end_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1"
        },
        "unchanged": true
      },
      "controls": {
        "source_items": 2368,
        "retained_items": 373,
        "modes": {
          "512-geometric": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "512-silhouette": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "512-ids": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "512-aa-fill": {
            "stable_removed": {
              "channels": 1077,
              "pixels": 374,
              "maximum": 46
            },
            "retained_order": {
              "channels": 485,
              "pixels": 231,
              "maximum": 3
            },
            "total": {
              "channels": 1502,
              "pixels": 567,
              "maximum": 46
            }
          },
          "512-aa-strokes": {
            "stable_removed": {
              "channels": 1056,
              "pixels": 381,
              "maximum": 46
            },
            "retained_order": {
              "channels": 895,
              "pixels": 339,
              "maximum": 9
            },
            "total": {
              "channels": 1719,
              "pixels": 625,
              "maximum": 46
            }
          },
          "512-material": {
            "stable_removed": {
              "channels": 846,
              "pixels": 349,
              "maximum": 19
            },
            "retained_order": {
              "channels": 982,
              "pixels": 380,
              "maximum": 10
            },
            "total": {
              "channels": 1701,
              "pixels": 653,
              "maximum": 19
            }
          },
          "2048-geometric": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "2048-silhouette": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "2048-material": {
            "stable_removed": {
              "channels": 2471,
              "pixels": 917,
              "maximum": 21
            },
            "retained_order": {
              "channels": 2399,
              "pixels": 959,
              "maximum": 7
            },
            "total": {
              "channels": 4868,
              "pixels": 1873,
              "maximum": 21
            }
          }
        }
      },
      "camera": {
        "towards": [
          0.5239881480393285,
          0.7483326291788591,
          0.4067366430758002
        ],
        "basis": [
          [
            -0.8191520442889918,
            0.573576436351046,
            0.0
          ],
          [
            -0.23329455426880485,
            -0.33317915266278375,
            0.9135454576426009
          ]
        ],
        "centre": [
          0.0,
          2.0,
          1.25
        ],
        "point": [
          0.0,
          0.0,
          0.0
        ],
        "scale": 1.0,
        "pixel_width": 512,
        "pixel_height": 512,
        "frame_width": 6,
        "frame_height": 6,
        "projection": "orthographic"
      }
    },
    "contact-capital512": {
      "provenance": {
        "start_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capital-postcut\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba"
        },
        "end_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capital-postcut\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba"
        },
        "unchanged": true
      },
      "controls": {
        "source_items": 2602,
        "retained_items": 957,
        "modes": {
          "512-geometric": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "512-silhouette": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "512-ids": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "512-aa-fill": {
            "stable_removed": {
              "channels": 8777,
              "pixels": 3163,
              "maximum": 15
            },
            "retained_order": {
              "channels": 50,
              "pixels": 32,
              "maximum": 2
            },
            "total": {
              "channels": 8807,
              "pixels": 3175,
              "maximum": 15
            }
          },
          "512-aa-strokes": {
            "stable_removed": {
              "channels": 6098,
              "pixels": 2707,
              "maximum": 6
            },
            "retained_order": {
              "channels": 51,
              "pixels": 46,
              "maximum": 2
            },
            "total": {
              "channels": 6113,
              "pixels": 2716,
              "maximum": 6
            }
          },
          "512-material": {
            "stable_removed": {
              "channels": 9596,
              "pixels": 4078,
              "maximum": 6
            },
            "retained_order": {
              "channels": 48,
              "pixels": 24,
              "maximum": 2
            },
            "total": {
              "channels": 9575,
              "pixels": 4075,
              "maximum": 6
            }
          },
          "2048-geometric": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "2048-silhouette": {
            "stable_removed": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            }
          },
          "2048-material": {
            "stable_removed": {
              "channels": 1980,
              "pixels": 1908,
              "maximum": 2
            },
            "retained_order": {
              "channels": 0,
              "pixels": 0,
              "maximum": 0
            },
            "total": {
              "channels": 1980,
              "pixels": 1908,
              "maximum": 2
            }
          }
        }
      },
      "camera": {
        "towards": [
          -0.645974188021251,
          0.6459741880212511,
          0.4067366430758002
        ],
        "basis": [
          [
            -0.7071067811865476,
            -0.7071067811865475,
            0.0
          ],
          [
            0.28760623847595074,
            -0.28760623847595074,
            0.9135454576426009
          ]
        ],
        "centre": [
          -11.162749999999999,
          0.053749999999999964,
          47.81756705853753
        ],
        "point": [
          0.0,
          0.0,
          0.0
        ],
        "scale": 1.0,
        "pixel_width": 512,
        "pixel_height": 512,
        "frame_width": 16,
        "frame_height": 16,
        "projection": "orthographic"
      }
    }
  },
  "queries": {
    "contact-capsule256": {
      "progress_journal": "🗑️generated\\contact-capsule256-ray.progress.jsonl",
      "progress_sha256": "a0f57c1bff5be4b2512ac9a00c7e86b1040c8028d42bb6412b1179c5356f7bc7",
      "elapsed_seconds": 6.914354799999273,
      "source_before": {
        "🗑️generated\\contact-capsule256\\1024-geometric-culled.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
        "🗑️generated\\contact-capsule256\\1024-geometric-full.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
        "🗑️generated\\contact-capsule256\\1024-geometric-stable.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
        "🗑️generated\\contact-capsule256\\1024-material-culled.png": "b2f2f295537ba3968caf9ae0af357c3304b3b2a6bcb3d0bcde6282a01bb7b413",
        "🗑️generated\\contact-capsule256\\1024-material-full.png": "3b7df958e7cdc024f9d034357e0918f6f385ea315107c48f88e5f7c7dda78bfe",
        "🗑️generated\\contact-capsule256\\1024-material-stable.png": "aabcf4e21ad1039502d0139a2470c1ffd12619908ea47f233babd66063d4430b",
        "🗑️generated\\contact-capsule256\\1024-silhouette-culled.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
        "🗑️generated\\contact-capsule256\\1024-silhouette-full.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
        "🗑️generated\\contact-capsule256\\1024-silhouette-stable.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
        "🗑️generated\\contact-capsule256\\256-aa-fill-culled.png": "1e3eb33625a5ec2c0895a594315f85a5d4a3de7acaca0a33990035de3163da59",
        "🗑️generated\\contact-capsule256\\256-aa-fill-full.png": "63448597d7248b0cba26dce58969b43847ebc2c179bffd4626b5f5ebd57c58da",
        "🗑️generated\\contact-capsule256\\256-aa-fill-stable.png": "0d5cc519da9d8976a82a626eaa9d5bbf7ed174317943522073ade57d4461bb4e",
        "🗑️generated\\contact-capsule256\\256-aa-strokes-culled.png": "657eab6b93306dd5b380a5c5ec56d22da3679fa746a2b5dfea794f5a396d4650",
        "🗑️generated\\contact-capsule256\\256-aa-strokes-full.png": "3deba40daab0544f67c5b53c2d7f2e5dd82a5e5559eed2ddb517e5683c5be325",
        "🗑️generated\\contact-capsule256\\256-aa-strokes-stable.png": "ba33e2c171639361b897923cb3d9c4f814b808f9c6d0fb4ce418a01f6a57e834",
        "🗑️generated\\contact-capsule256\\256-geometric-culled.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
        "🗑️generated\\contact-capsule256\\256-geometric-full.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
        "🗑️generated\\contact-capsule256\\256-geometric-stable.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
        "🗑️generated\\contact-capsule256\\256-ids-culled.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
        "🗑️generated\\contact-capsule256\\256-ids-full.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
        "🗑️generated\\contact-capsule256\\256-ids-stable.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
        "🗑️generated\\contact-capsule256\\256-material-culled.png": "04533916618013a0086ba105d9f07ae1b8b51f01bf78eead398f2c7bbe4941dc",
        "🗑️generated\\contact-capsule256\\256-material-full.png": "494c712a7ab3f4a74da1c39de798705e5bb401af87048c33ff5070e225c3ccd8",
        "🗑️generated\\contact-capsule256\\256-material-stable.png": "10315d5b4d38f4ac53261ad018bbf53b4fdb8a3f43b9a923e5c6d4f3ced2667c",
        "🗑️generated\\contact-capsule256\\256-silhouette-culled.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
        "🗑️generated\\contact-capsule256\\256-silhouette-full.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
        "🗑️generated\\contact-capsule256\\256-silhouette-stable.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
        "🗑️generated\\contact-capsule256\\coverage-aa-controls.json": "af79d35598191e5451898b3aaae9c6d774d65239731e980fdde39460e951f20a",
        "🗑️generated\\contact-capsule256\\coverage-camera.json": "75b599f98ffd63699a2c24976ab73615db7b4c2cc1ba98d6251cb7bf3f744ee4",
        "🗑️generated\\contact-capsule256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce",
        "🗑️generated\\contact-capsule256\\coverage-paint-fragments.json": "e8e0b9d86b4d8564f7b86a229618ffc56651fc6d51e39debdaf7b9b64b50910d",
        "🗑️generated\\contact-capsule256\\coverage-provenance.json": "3d91189035b2249923b0ae8dcee1975d75d857e9f8a20025fda98443d15a9dad",
        "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
        "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
        "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
        "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
        "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
        "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
        "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
        "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
        "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
        "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
        "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
        "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
        "🗑️generated\\capsule-fragment-roi-256.json": "ec164a488f13b3b172489909722578ce7d8acbb99d458c78c0004bdaac13da04"
      },
      "source_after": {
        "🗑️generated\\contact-capsule256\\1024-geometric-culled.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
        "🗑️generated\\contact-capsule256\\1024-geometric-full.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
        "🗑️generated\\contact-capsule256\\1024-geometric-stable.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
        "🗑️generated\\contact-capsule256\\1024-material-culled.png": "b2f2f295537ba3968caf9ae0af357c3304b3b2a6bcb3d0bcde6282a01bb7b413",
        "🗑️generated\\contact-capsule256\\1024-material-full.png": "3b7df958e7cdc024f9d034357e0918f6f385ea315107c48f88e5f7c7dda78bfe",
        "🗑️generated\\contact-capsule256\\1024-material-stable.png": "aabcf4e21ad1039502d0139a2470c1ffd12619908ea47f233babd66063d4430b",
        "🗑️generated\\contact-capsule256\\1024-silhouette-culled.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
        "🗑️generated\\contact-capsule256\\1024-silhouette-full.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
        "🗑️generated\\contact-capsule256\\1024-silhouette-stable.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
        "🗑️generated\\contact-capsule256\\256-aa-fill-culled.png": "1e3eb33625a5ec2c0895a594315f85a5d4a3de7acaca0a33990035de3163da59",
        "🗑️generated\\contact-capsule256\\256-aa-fill-full.png": "63448597d7248b0cba26dce58969b43847ebc2c179bffd4626b5f5ebd57c58da",
        "🗑️generated\\contact-capsule256\\256-aa-fill-stable.png": "0d5cc519da9d8976a82a626eaa9d5bbf7ed174317943522073ade57d4461bb4e",
        "🗑️generated\\contact-capsule256\\256-aa-strokes-culled.png": "657eab6b93306dd5b380a5c5ec56d22da3679fa746a2b5dfea794f5a396d4650",
        "🗑️generated\\contact-capsule256\\256-aa-strokes-full.png": "3deba40daab0544f67c5b53c2d7f2e5dd82a5e5559eed2ddb517e5683c5be325",
        "🗑️generated\\contact-capsule256\\256-aa-strokes-stable.png": "ba33e2c171639361b897923cb3d9c4f814b808f9c6d0fb4ce418a01f6a57e834",
        "🗑️generated\\contact-capsule256\\256-geometric-culled.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
        "🗑️generated\\contact-capsule256\\256-geometric-full.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
        "🗑️generated\\contact-capsule256\\256-geometric-stable.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
        "🗑️generated\\contact-capsule256\\256-ids-culled.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
        "🗑️generated\\contact-capsule256\\256-ids-full.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
        "🗑️generated\\contact-capsule256\\256-ids-stable.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
        "🗑️generated\\contact-capsule256\\256-material-culled.png": "04533916618013a0086ba105d9f07ae1b8b51f01bf78eead398f2c7bbe4941dc",
        "🗑️generated\\contact-capsule256\\256-material-full.png": "494c712a7ab3f4a74da1c39de798705e5bb401af87048c33ff5070e225c3ccd8",
        "🗑️generated\\contact-capsule256\\256-material-stable.png": "10315d5b4d38f4ac53261ad018bbf53b4fdb8a3f43b9a923e5c6d4f3ced2667c",
        "🗑️generated\\contact-capsule256\\256-silhouette-culled.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
        "🗑️generated\\contact-capsule256\\256-silhouette-full.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
        "🗑️generated\\contact-capsule256\\256-silhouette-stable.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
        "🗑️generated\\contact-capsule256\\coverage-aa-controls.json": "af79d35598191e5451898b3aaae9c6d774d65239731e980fdde39460e951f20a",
        "🗑️generated\\contact-capsule256\\coverage-camera.json": "75b599f98ffd63699a2c24976ab73615db7b4c2cc1ba98d6251cb7bf3f744ee4",
        "🗑️generated\\contact-capsule256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce",
        "🗑️generated\\contact-capsule256\\coverage-paint-fragments.json": "e8e0b9d86b4d8564f7b86a229618ffc56651fc6d51e39debdaf7b9b64b50910d",
        "🗑️generated\\contact-capsule256\\coverage-provenance.json": "3d91189035b2249923b0ae8dcee1975d75d857e9f8a20025fda98443d15a9dad",
        "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
        "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
        "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
        "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
        "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
        "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
        "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
        "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
        "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
        "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
        "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
        "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
        "🗑️generated\\capsule-fragment-roi-256.json": "ec164a488f13b3b172489909722578ce7d8acbb99d458c78c0004bdaac13da04"
      },
      "unchanged": true,
      "producer_provenance": {
        "start_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce"
        },
        "end_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce"
        },
        "unchanged": true
      },
      "resolution": 256,
      "source_items": 2368,
      "paint_piece_rows": 7094,
      "paint_unique_polygons": 4126,
      "physical_triangles": 3728,
      "warped_original_polygons": [
        [
          1762,
          0
        ],
        [
          1763,
          0
        ],
        [
          1764,
          0
        ],
        [
          1765,
          0
        ],
        [
          1766,
          0
        ],
        [
          1767,
          0
        ],
        [
          1768,
          0
        ],
        [
          1769,
          0
        ],
        [
          1770,
          0
        ],
        [
          1771,
          0
        ],
        [
          1772,
          0
        ],
        [
          1773,
          0
        ],
        [
          1774,
          0
        ],
        [
          1775,
          0
        ],
        [
          1776,
          0
        ],
        [
          1777,
          0
        ]
      ],
      "paint_triangles": 8128,
      "projection_residual": 2.8421709430404007e-13,
      "pixels": 319,
      "rays": 7975,
      "changed_id_pixels": {
        "full-stable": 0,
        "stable-culled": 0,
        "full-culled": 0
      },
      "visible_steel_id_pixels": 0,
      "summary": {
        "full_wrong_nearest_samples": 0,
        "full_unsupported_samples": 0,
        "full_layer_inversion_samples": 0,
        "stable_wrong_nearest_samples": 0,
        "stable_unsupported_samples": 0,
        "stable_layer_inversion_samples": 0,
        "culled_wrong_nearest_samples": 0,
        "culled_unsupported_samples": 0,
        "culled_layer_inversion_samples": 0,
        "physical_visible_steel_samples": 0,
        "physical_culling_nearest_changed_samples": 0,
        "full_cairo_physical_mismatch_centres": 2,
        "full_cairo_paint_mismatch_centres": 2,
        "stable_cairo_physical_mismatch_centres": 2,
        "stable_cairo_paint_mismatch_centres": 2,
        "culled_cairo_physical_mismatch_centres": 2,
        "culled_cairo_paint_mismatch_centres": 2
      },
      "limits": "Finite centre plus 5x5 rays over explicit prior ROI, every new source-ID difference and optional steel pixels. Physical nearest uses only original source polygons; actual point compositing uses clipped paint rows unioned once per rank. No Cairo rerender, colour tolerance or original scalar-rank fallback."
    },
    "contact-capsule512": {
      "progress_journal": "🗑️generated\\contact-capsule512-ray.progress.jsonl",
      "progress_sha256": "e4b519bcc4d056e8ea7af7d0252c8d63f4aec23bb671b90262167ed2aeb6020a",
      "elapsed_seconds": 0.07343880001280922,
      "source_before": {
        "🗑️generated\\contact-capsule512\\2048-geometric-culled.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
        "🗑️generated\\contact-capsule512\\2048-geometric-full.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
        "🗑️generated\\contact-capsule512\\2048-geometric-stable.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
        "🗑️generated\\contact-capsule512\\2048-material-culled.png": "bf3865501102f40c3b3dbddca37256a2480a8f70f1e19d4978ae8ae453a17436",
        "🗑️generated\\contact-capsule512\\2048-material-full.png": "e8ed9166c80c21bb8d207da43e19b2d0bde82dbea333daa487654795e8287851",
        "🗑️generated\\contact-capsule512\\2048-material-stable.png": "856fb3418aaaf0c177fb56009be43604c9fa47d8bf84516715daad480bebdc3e",
        "🗑️generated\\contact-capsule512\\2048-silhouette-culled.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
        "🗑️generated\\contact-capsule512\\2048-silhouette-full.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
        "🗑️generated\\contact-capsule512\\2048-silhouette-stable.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
        "🗑️generated\\contact-capsule512\\512-aa-fill-culled.png": "f11db3d7ddef75c98dd4522541a8d33d4ea48a4f830804c7ee4edbc9761e39b6",
        "🗑️generated\\contact-capsule512\\512-aa-fill-full.png": "52ed1a21cb76a0676e60f048d41a7e8772037c2755d66a767eef8c49a6bf9a83",
        "🗑️generated\\contact-capsule512\\512-aa-fill-stable.png": "c8b11818901ca85daf68d7949ef070c80407fdeec236c50f3128e1cc7426cf50",
        "🗑️generated\\contact-capsule512\\512-aa-strokes-culled.png": "bcb4302ed4a274497a7b6152dce52bb5f45b316086aa3fe53c97bf9d3eda6cdc",
        "🗑️generated\\contact-capsule512\\512-aa-strokes-full.png": "2fb5b4d1ef9a4ea6e1ce6aa8c76af7375257bf9da65fb2f3d1ebfeb9a7efba56",
        "🗑️generated\\contact-capsule512\\512-aa-strokes-stable.png": "3912ec29c56a0d606a9ff7bd15b5791f09f73994957dc37dab10e5ae7b5833a3",
        "🗑️generated\\contact-capsule512\\512-geometric-culled.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
        "🗑️generated\\contact-capsule512\\512-geometric-full.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
        "🗑️generated\\contact-capsule512\\512-geometric-stable.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
        "🗑️generated\\contact-capsule512\\512-ids-culled.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
        "🗑️generated\\contact-capsule512\\512-ids-full.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
        "🗑️generated\\contact-capsule512\\512-ids-stable.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
        "🗑️generated\\contact-capsule512\\512-material-culled.png": "574d1f7b4777229427d92c1b09b1d1d1cc7fdd342ba99cc8454341a6c96e2928",
        "🗑️generated\\contact-capsule512\\512-material-full.png": "3c063106db0fca2ca8d2017eefeeae3f0c5ff2447a352b9ffb246fdc396b0f3e",
        "🗑️generated\\contact-capsule512\\512-material-stable.png": "9a20fb5e927537561a84269e1e18a68f6c6ffd58993493e5d712ab17b9abd748",
        "🗑️generated\\contact-capsule512\\512-silhouette-culled.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
        "🗑️generated\\contact-capsule512\\512-silhouette-full.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
        "🗑️generated\\contact-capsule512\\512-silhouette-stable.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
        "🗑️generated\\contact-capsule512\\coverage-aa-controls.json": "1f32b8d5f44b37f2d094fd9e340ccdba33c3221858a66a993ed2a970eca00c15",
        "🗑️generated\\contact-capsule512\\coverage-camera.json": "fb6d460aa4428bf58bef8f54ac09205922491ee4f2c7e30014edae893c4bf65c",
        "🗑️generated\\contact-capsule512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1",
        "🗑️generated\\contact-capsule512\\coverage-paint-fragments.json": "a0903170152349afd9d884ae7ac98cb1059f52d8ba61eb6e01ae19f17587b4c3",
        "🗑️generated\\contact-capsule512\\coverage-provenance.json": "d84f76b5d0bb1b6cc15ef3299f010565dfe3b88336f3d81a2ad5c18a88fe8bc3",
        "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
        "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
        "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
        "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
        "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
        "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
        "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
        "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
        "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
        "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
        "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
        "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
        "🗑️generated\\capsule-fragment-roi-512.json": "0fce6a8ed99939f7818018a9010f762f6bbbafef28db0155b97796cd8fb524ed"
      },
      "source_after": {
        "🗑️generated\\contact-capsule512\\2048-geometric-culled.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
        "🗑️generated\\contact-capsule512\\2048-geometric-full.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
        "🗑️generated\\contact-capsule512\\2048-geometric-stable.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
        "🗑️generated\\contact-capsule512\\2048-material-culled.png": "bf3865501102f40c3b3dbddca37256a2480a8f70f1e19d4978ae8ae453a17436",
        "🗑️generated\\contact-capsule512\\2048-material-full.png": "e8ed9166c80c21bb8d207da43e19b2d0bde82dbea333daa487654795e8287851",
        "🗑️generated\\contact-capsule512\\2048-material-stable.png": "856fb3418aaaf0c177fb56009be43604c9fa47d8bf84516715daad480bebdc3e",
        "🗑️generated\\contact-capsule512\\2048-silhouette-culled.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
        "🗑️generated\\contact-capsule512\\2048-silhouette-full.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
        "🗑️generated\\contact-capsule512\\2048-silhouette-stable.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
        "🗑️generated\\contact-capsule512\\512-aa-fill-culled.png": "f11db3d7ddef75c98dd4522541a8d33d4ea48a4f830804c7ee4edbc9761e39b6",
        "🗑️generated\\contact-capsule512\\512-aa-fill-full.png": "52ed1a21cb76a0676e60f048d41a7e8772037c2755d66a767eef8c49a6bf9a83",
        "🗑️generated\\contact-capsule512\\512-aa-fill-stable.png": "c8b11818901ca85daf68d7949ef070c80407fdeec236c50f3128e1cc7426cf50",
        "🗑️generated\\contact-capsule512\\512-aa-strokes-culled.png": "bcb4302ed4a274497a7b6152dce52bb5f45b316086aa3fe53c97bf9d3eda6cdc",
        "🗑️generated\\contact-capsule512\\512-aa-strokes-full.png": "2fb5b4d1ef9a4ea6e1ce6aa8c76af7375257bf9da65fb2f3d1ebfeb9a7efba56",
        "🗑️generated\\contact-capsule512\\512-aa-strokes-stable.png": "3912ec29c56a0d606a9ff7bd15b5791f09f73994957dc37dab10e5ae7b5833a3",
        "🗑️generated\\contact-capsule512\\512-geometric-culled.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
        "🗑️generated\\contact-capsule512\\512-geometric-full.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
        "🗑️generated\\contact-capsule512\\512-geometric-stable.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
        "🗑️generated\\contact-capsule512\\512-ids-culled.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
        "🗑️generated\\contact-capsule512\\512-ids-full.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
        "🗑️generated\\contact-capsule512\\512-ids-stable.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
        "🗑️generated\\contact-capsule512\\512-material-culled.png": "574d1f7b4777229427d92c1b09b1d1d1cc7fdd342ba99cc8454341a6c96e2928",
        "🗑️generated\\contact-capsule512\\512-material-full.png": "3c063106db0fca2ca8d2017eefeeae3f0c5ff2447a352b9ffb246fdc396b0f3e",
        "🗑️generated\\contact-capsule512\\512-material-stable.png": "9a20fb5e927537561a84269e1e18a68f6c6ffd58993493e5d712ab17b9abd748",
        "🗑️generated\\contact-capsule512\\512-silhouette-culled.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
        "🗑️generated\\contact-capsule512\\512-silhouette-full.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
        "🗑️generated\\contact-capsule512\\512-silhouette-stable.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
        "🗑️generated\\contact-capsule512\\coverage-aa-controls.json": "1f32b8d5f44b37f2d094fd9e340ccdba33c3221858a66a993ed2a970eca00c15",
        "🗑️generated\\contact-capsule512\\coverage-camera.json": "fb6d460aa4428bf58bef8f54ac09205922491ee4f2c7e30014edae893c4bf65c",
        "🗑️generated\\contact-capsule512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1",
        "🗑️generated\\contact-capsule512\\coverage-paint-fragments.json": "a0903170152349afd9d884ae7ac98cb1059f52d8ba61eb6e01ae19f17587b4c3",
        "🗑️generated\\contact-capsule512\\coverage-provenance.json": "d84f76b5d0bb1b6cc15ef3299f010565dfe3b88336f3d81a2ad5c18a88fe8bc3",
        "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
        "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
        "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
        "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
        "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
        "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
        "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
        "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
        "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
        "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
        "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
        "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
        "🗑️generated\\capsule-fragment-roi-512.json": "0fce6a8ed99939f7818018a9010f762f6bbbafef28db0155b97796cd8fb524ed"
      },
      "unchanged": true,
      "producer_provenance": {
        "start_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1"
        },
        "end_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capsule-postcut512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1"
        },
        "unchanged": true
      },
      "resolution": 512,
      "source_items": 2368,
      "paint_piece_rows": 6115,
      "paint_unique_polygons": 3752,
      "physical_triangles": 3728,
      "warped_original_polygons": [
        [
          1762,
          0
        ],
        [
          1763,
          0
        ],
        [
          1764,
          0
        ],
        [
          1765,
          0
        ],
        [
          1766,
          0
        ],
        [
          1767,
          0
        ],
        [
          1768,
          0
        ],
        [
          1769,
          0
        ],
        [
          1770,
          0
        ],
        [
          1771,
          0
        ],
        [
          1772,
          0
        ],
        [
          1773,
          0
        ],
        [
          1774,
          0
        ],
        [
          1775,
          0
        ],
        [
          1776,
          0
        ],
        [
          1777,
          0
        ]
      ],
      "paint_triangles": 7377,
      "projection_residual": 5.684341886080801e-13,
      "pixels": 3,
      "rays": 75,
      "changed_id_pixels": {
        "full-stable": 0,
        "stable-culled": 0,
        "full-culled": 0
      },
      "visible_steel_id_pixels": 0,
      "summary": {
        "full_wrong_nearest_samples": 0,
        "full_unsupported_samples": 0,
        "full_layer_inversion_samples": 0,
        "stable_wrong_nearest_samples": 0,
        "stable_unsupported_samples": 0,
        "stable_layer_inversion_samples": 0,
        "culled_wrong_nearest_samples": 0,
        "culled_unsupported_samples": 0,
        "culled_layer_inversion_samples": 0,
        "physical_visible_steel_samples": 0,
        "physical_culling_nearest_changed_samples": 0,
        "full_cairo_physical_mismatch_centres": 0,
        "full_cairo_paint_mismatch_centres": 0,
        "stable_cairo_physical_mismatch_centres": 0,
        "stable_cairo_paint_mismatch_centres": 0,
        "culled_cairo_physical_mismatch_centres": 0,
        "culled_cairo_paint_mismatch_centres": 0
      },
      "limits": "Finite centre plus 5x5 rays over explicit prior ROI, every new source-ID difference and optional steel pixels. Physical nearest uses only original source polygons; actual point compositing uses clipped paint rows unioned once per rank. No Cairo rerender, colour tolerance or original scalar-rank fallback."
    },
    "contact-capital512": {
      "progress_journal": "🗑️generated\\contact-capital512-ray.progress.jsonl",
      "progress_sha256": "3c1cea1f972457644fbb31c9768e66c548fc913453950860ad3abf3d52ce4512",
      "elapsed_seconds": 18.835680400006822,
      "source_before": {
        "🗑️generated\\contact-capital512\\2048-geometric-culled.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
        "🗑️generated\\contact-capital512\\2048-geometric-full.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
        "🗑️generated\\contact-capital512\\2048-geometric-stable.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
        "🗑️generated\\contact-capital512\\2048-material-culled.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
        "🗑️generated\\contact-capital512\\2048-material-full.png": "d8f8909a1f14d8e0533af73a369c73de35c1c91bece51b09ea519bb8a25c62d9",
        "🗑️generated\\contact-capital512\\2048-material-stable.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
        "🗑️generated\\contact-capital512\\2048-silhouette-culled.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
        "🗑️generated\\contact-capital512\\2048-silhouette-full.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
        "🗑️generated\\contact-capital512\\2048-silhouette-stable.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
        "🗑️generated\\contact-capital512\\512-aa-fill-culled.png": "ca8c312edaff39fa660b910f347829c2a318c73544f253bfc6bc933defda0917",
        "🗑️generated\\contact-capital512\\512-aa-fill-full.png": "89f6195a0c0c2ebd82e42884fb5bf272bdd16541ddd5e6c6f29f742f8582f8ef",
        "🗑️generated\\contact-capital512\\512-aa-fill-stable.png": "0b709781d8acbf9bf046252b93206cdc00592dbe215e7f034e49392dcb9130ef",
        "🗑️generated\\contact-capital512\\512-aa-strokes-culled.png": "fc1895120a606f3fe0a8655d350176f30091522066365824fa7b9e1fc07e7e5c",
        "🗑️generated\\contact-capital512\\512-aa-strokes-full.png": "26453b906d6b827195aedc08e05d85dfb2cff261ceaaf6d8387a971a2201befb",
        "🗑️generated\\contact-capital512\\512-aa-strokes-stable.png": "8aca4be3da8d42f790a180edd35be4a437512e992154fbf2665a82373fad6e27",
        "🗑️generated\\contact-capital512\\512-geometric-culled.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
        "🗑️generated\\contact-capital512\\512-geometric-full.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
        "🗑️generated\\contact-capital512\\512-geometric-stable.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
        "🗑️generated\\contact-capital512\\512-ids-culled.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
        "🗑️generated\\contact-capital512\\512-ids-full.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
        "🗑️generated\\contact-capital512\\512-ids-stable.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
        "🗑️generated\\contact-capital512\\512-material-culled.png": "d88acb18ebacc1f6a40c5b95e1ca4b07ca7258868970146e1f46f71047d1769f",
        "🗑️generated\\contact-capital512\\512-material-full.png": "120e8b7b9e9ac5274e452a6d5559f9c75c4186ba403292e6d87a2c25b597e263",
        "🗑️generated\\contact-capital512\\512-material-stable.png": "65aee655dce05421f7db3c13937fffe023e63588e1ea117500f4d0749a8bedac",
        "🗑️generated\\contact-capital512\\512-silhouette-culled.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
        "🗑️generated\\contact-capital512\\512-silhouette-full.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
        "🗑️generated\\contact-capital512\\512-silhouette-stable.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
        "🗑️generated\\contact-capital512\\coverage-aa-controls.json": "42c9f518f77cab20e4ee8485fbadda7cfc420de8b84d3bbb244638ff00d32fee",
        "🗑️generated\\contact-capital512\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
        "🗑️generated\\contact-capital512\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba",
        "🗑️generated\\contact-capital512\\coverage-paint-fragments.json": "8b3b641a5fab3526e0a90002b2501c1ae16eaa2d8cb1bdb1a5763c03ffd2e57e",
        "🗑️generated\\contact-capital512\\coverage-provenance.json": "215d9133aedf9b117096ba9b6992aa6476f273636fb824516f230f2a7b6b7332",
        "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
        "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
        "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
        "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
        "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
        "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
        "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
        "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
        "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
        "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
        "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
        "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
        "🗑️generated\\capital-fragment-roi.json": "08e7796732e6beaaf0596f3b887055f53f26c516cbf7f4e7b3762049d2dd503f"
      },
      "source_after": {
        "🗑️generated\\contact-capital512\\2048-geometric-culled.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
        "🗑️generated\\contact-capital512\\2048-geometric-full.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
        "🗑️generated\\contact-capital512\\2048-geometric-stable.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
        "🗑️generated\\contact-capital512\\2048-material-culled.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
        "🗑️generated\\contact-capital512\\2048-material-full.png": "d8f8909a1f14d8e0533af73a369c73de35c1c91bece51b09ea519bb8a25c62d9",
        "🗑️generated\\contact-capital512\\2048-material-stable.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
        "🗑️generated\\contact-capital512\\2048-silhouette-culled.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
        "🗑️generated\\contact-capital512\\2048-silhouette-full.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
        "🗑️generated\\contact-capital512\\2048-silhouette-stable.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
        "🗑️generated\\contact-capital512\\512-aa-fill-culled.png": "ca8c312edaff39fa660b910f347829c2a318c73544f253bfc6bc933defda0917",
        "🗑️generated\\contact-capital512\\512-aa-fill-full.png": "89f6195a0c0c2ebd82e42884fb5bf272bdd16541ddd5e6c6f29f742f8582f8ef",
        "🗑️generated\\contact-capital512\\512-aa-fill-stable.png": "0b709781d8acbf9bf046252b93206cdc00592dbe215e7f034e49392dcb9130ef",
        "🗑️generated\\contact-capital512\\512-aa-strokes-culled.png": "fc1895120a606f3fe0a8655d350176f30091522066365824fa7b9e1fc07e7e5c",
        "🗑️generated\\contact-capital512\\512-aa-strokes-full.png": "26453b906d6b827195aedc08e05d85dfb2cff261ceaaf6d8387a971a2201befb",
        "🗑️generated\\contact-capital512\\512-aa-strokes-stable.png": "8aca4be3da8d42f790a180edd35be4a437512e992154fbf2665a82373fad6e27",
        "🗑️generated\\contact-capital512\\512-geometric-culled.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
        "🗑️generated\\contact-capital512\\512-geometric-full.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
        "🗑️generated\\contact-capital512\\512-geometric-stable.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
        "🗑️generated\\contact-capital512\\512-ids-culled.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
        "🗑️generated\\contact-capital512\\512-ids-full.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
        "🗑️generated\\contact-capital512\\512-ids-stable.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
        "🗑️generated\\contact-capital512\\512-material-culled.png": "d88acb18ebacc1f6a40c5b95e1ca4b07ca7258868970146e1f46f71047d1769f",
        "🗑️generated\\contact-capital512\\512-material-full.png": "120e8b7b9e9ac5274e452a6d5559f9c75c4186ba403292e6d87a2c25b597e263",
        "🗑️generated\\contact-capital512\\512-material-stable.png": "65aee655dce05421f7db3c13937fffe023e63588e1ea117500f4d0749a8bedac",
        "🗑️generated\\contact-capital512\\512-silhouette-culled.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
        "🗑️generated\\contact-capital512\\512-silhouette-full.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
        "🗑️generated\\contact-capital512\\512-silhouette-stable.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
        "🗑️generated\\contact-capital512\\coverage-aa-controls.json": "42c9f518f77cab20e4ee8485fbadda7cfc420de8b84d3bbb244638ff00d32fee",
        "🗑️generated\\contact-capital512\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
        "🗑️generated\\contact-capital512\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba",
        "🗑️generated\\contact-capital512\\coverage-paint-fragments.json": "8b3b641a5fab3526e0a90002b2501c1ae16eaa2d8cb1bdb1a5763c03ffd2e57e",
        "🗑️generated\\contact-capital512\\coverage-provenance.json": "215d9133aedf9b117096ba9b6992aa6476f273636fb824516f230f2a7b6b7332",
        "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
        "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
        "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
        "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
        "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
        "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
        "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
        "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
        "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
        "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
        "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
        "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
        "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
        "🗑️generated\\capital-fragment-roi.json": "08e7796732e6beaaf0596f3b887055f53f26c516cbf7f4e7b3762049d2dd503f"
      },
      "unchanged": true,
      "producer_provenance": {
        "start_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capital-postcut\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba"
        },
        "end_sources": {
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "20982173b6662e2a991b843a8cd1081019e45b89e039a911b1596df1b2f87822",
          "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "0b0123eba3a76a8a82ff8b981bd613d83b707ed670ef13e4f49ce97f83d696b4",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
          "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "f4f0216fe5c40181d645effe4e222a1ec014c14dab4eb6fa2e7884dc5411790d",
          ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\🗑️generated\\capital-postcut\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba"
        },
        "unchanged": true
      },
      "resolution": 512,
      "source_items": 2602,
      "paint_piece_rows": 14843,
      "paint_unique_polygons": 5578,
      "physical_triangles": 2392,
      "warped_original_polygons": [],
      "paint_triangles": 11084,
      "projection_residual": 1.8189894035458565e-12,
      "pixels": 938,
      "rays": 23450,
      "changed_id_pixels": {
        "full-stable": 0,
        "stable-culled": 0,
        "full-culled": 0
      },
      "visible_steel_id_pixels": 0,
      "summary": {
        "full_wrong_nearest_samples": 0,
        "full_unsupported_samples": 0,
        "full_layer_inversion_samples": 0,
        "stable_wrong_nearest_samples": 0,
        "stable_unsupported_samples": 0,
        "stable_layer_inversion_samples": 0,
        "culled_wrong_nearest_samples": 0,
        "culled_unsupported_samples": 0,
        "culled_layer_inversion_samples": 0,
        "physical_visible_steel_samples": 0,
        "physical_culling_nearest_changed_samples": 0,
        "full_cairo_physical_mismatch_centres": 0,
        "full_cairo_paint_mismatch_centres": 0,
        "stable_cairo_physical_mismatch_centres": 0,
        "stable_cairo_paint_mismatch_centres": 0,
        "culled_cairo_physical_mismatch_centres": 0,
        "culled_cairo_paint_mismatch_centres": 0
      },
      "limits": "Finite centre plus 5x5 rays over explicit prior ROI, every new source-ID difference and optional steel pixels. Physical nearest uses only original source polygons; actual point compositing uses clipped paint rows unioned once per rank. No Cairo rerender, colour tolerance or original scalar-rank fallback."
    }
  },
  "boundary_witnesses": [
    {
      "pixel": [
        52,
        84
      ],
      "cairo": {
        "full": 63,
        "stable": 63,
        "culled": 63
      },
      "centre_physical": {
        "layers": [
          {
            "depth": 2.17366992806212,
            "ids": [
              42
            ]
          }
        ],
        "visible_ids": [
          42
        ],
        "hit_ids": [
          23,
          42
        ],
        "nearest_depth": 2.17366992806212,
        "nearest_ids": [
          42
        ]
      },
      "nearest_counts": {
        "[63]": 12,
        "[42]": 13
      },
      "boundary_witnesses": [
        {
          "id": 42,
          "key": "shell-right",
          "role": "shell",
          "removed": false,
          "normal": [
            1.0,
            0.0,
            0.0
          ],
          "projected_edge_distance_pixels": 0.0014365413187008892,
          "extrapolated_plane_depth": 2.1736699280621203
        },
        {
          "id": 63,
          "key": "shell-edge-y-right-top-0",
          "role": "shell",
          "removed": false,
          "normal": [
            0.923879532511287,
            0.0,
            0.38268343236508945
          ],
          "projected_edge_distance_pixels": 0.0014365413187044584,
          "extrapolated_plane_depth": 2.1736954233873593
        }
      ]
    },
    {
      "pixel": [
        78,
        90
      ],
      "cairo": {
        "full": 64,
        "stable": 64,
        "culled": 64
      },
      "centre_physical": {
        "layers": [
          {
            "depth": 2.9240256272159186,
            "ids": [
              63
            ]
          }
        ],
        "visible_ids": [
          63
        ],
        "hit_ids": [
          23,
          63,
          1753
        ],
        "nearest_depth": 2.9240256272159186,
        "nearest_ids": [
          63
        ]
      },
      "nearest_counts": {
        "[64]": 12,
        "[63]": 13
      },
      "boundary_witnesses": [
        {
          "id": 63,
          "key": "shell-edge-y-right-top-0",
          "role": "shell",
          "removed": false,
          "normal": [
            0.923879532511287,
            0.0,
            0.38268343236508945
          ],
          "projected_edge_distance_pixels": 0.0039859360374941534,
          "extrapolated_plane_depth": 2.9240256272159195
        },
        {
          "id": 64,
          "key": "shell-edge-y-right-top-0",
          "role": "shell",
          "removed": false,
          "normal": [
            0.3826834323650909,
            0.0,
            0.9238795325112863
          ],
          "projected_edge_distance_pixels": 0.0039859360374941534,
          "extrapolated_plane_depth": 2.924144475529506
        }
      ]
    }
  ],
  "literal_shared_edges": [
    {
      "sources": [
        42,
        63
      ],
      "common_literal_vertices": [
        [
          1.3,
          0.1499999999999999,
          2.25
        ],
        [
          1.3,
          3.8499999999999996,
          2.25
        ]
      ]
    },
    {
      "sources": [
        63,
        64
      ],
      "common_literal_vertices": [
        [
          1.226776695296637,
          0.1499999999999999,
          2.426776695296637
        ],
        [
          1.226776695296637,
          3.8499999999999996,
          2.426776695296637
        ]
      ]
    }
  ],
  "oracle_tests": {
    "tests": 13,
    "success": true,
    "failures": 0,
    "errors": 0,
    "before": {
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "e2601dc9f34154c06c5a560e38e465153486592626cc73eba6705b28be450477",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "647e4e10aab12d55ddc0527209bba42447ca90125aaefef595d60e672e1470c6",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf"
    },
    "after": {
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "e2601dc9f34154c06c5a560e38e465153486592626cc73eba6705b28be450477",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "647e4e10aab12d55ddc0527209bba42447ca90125aaefef595d60e672e1470c6",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf"
    },
    "unchanged": true
  },
  "positive_carrier_witness": {
    "success": true,
    "kind": "retained-material",
    "triangles": 1,
    "independent_triangle_area": 6.211953942447299e-11,
    "carrier_depth_error": 5.551115123125783e-17,
    "error": null,
    "elapsed": 0.01600000000325963,
    "python": "3.12.11 (main, Aug 18 2025, 19:17:54) [MSC v.1944 64 bit (AMD64)]",
    "numpy": "2.5.4",
    "before": {
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-paint-lineage.py": "eb430ffa79107d6ee45d73ff0c3ae267bee810082d4b0fda4bde0fbe1945e213",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514"
    },
    "after": {
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-paint-lineage.py": "eb430ffa79107d6ee45d73ff0c3ae267bee810082d4b0fda4bde0fbe1945e213",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514"
    },
    "unchanged": true
  },
  "artifact_sha256": {
    "🗑️generated\\contact-capsule256\\1024-geometric-culled.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
    "🗑️generated\\contact-capsule256\\1024-geometric-full.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
    "🗑️generated\\contact-capsule256\\1024-geometric-stable.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
    "🗑️generated\\contact-capsule256\\1024-material-culled.png": "b2f2f295537ba3968caf9ae0af357c3304b3b2a6bcb3d0bcde6282a01bb7b413",
    "🗑️generated\\contact-capsule256\\1024-material-full.png": "3b7df958e7cdc024f9d034357e0918f6f385ea315107c48f88e5f7c7dda78bfe",
    "🗑️generated\\contact-capsule256\\1024-material-stable.png": "aabcf4e21ad1039502d0139a2470c1ffd12619908ea47f233babd66063d4430b",
    "🗑️generated\\contact-capsule256\\1024-silhouette-culled.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
    "🗑️generated\\contact-capsule256\\1024-silhouette-full.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
    "🗑️generated\\contact-capsule256\\1024-silhouette-stable.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
    "🗑️generated\\contact-capsule256\\256-aa-fill-culled.png": "1e3eb33625a5ec2c0895a594315f85a5d4a3de7acaca0a33990035de3163da59",
    "🗑️generated\\contact-capsule256\\256-aa-fill-full.png": "63448597d7248b0cba26dce58969b43847ebc2c179bffd4626b5f5ebd57c58da",
    "🗑️generated\\contact-capsule256\\256-aa-fill-stable.png": "0d5cc519da9d8976a82a626eaa9d5bbf7ed174317943522073ade57d4461bb4e",
    "🗑️generated\\contact-capsule256\\256-aa-strokes-culled.png": "657eab6b93306dd5b380a5c5ec56d22da3679fa746a2b5dfea794f5a396d4650",
    "🗑️generated\\contact-capsule256\\256-aa-strokes-full.png": "3deba40daab0544f67c5b53c2d7f2e5dd82a5e5559eed2ddb517e5683c5be325",
    "🗑️generated\\contact-capsule256\\256-aa-strokes-stable.png": "ba33e2c171639361b897923cb3d9c4f814b808f9c6d0fb4ce418a01f6a57e834",
    "🗑️generated\\contact-capsule256\\256-geometric-culled.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
    "🗑️generated\\contact-capsule256\\256-geometric-full.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
    "🗑️generated\\contact-capsule256\\256-geometric-stable.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
    "🗑️generated\\contact-capsule256\\256-ids-culled.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
    "🗑️generated\\contact-capsule256\\256-ids-full.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
    "🗑️generated\\contact-capsule256\\256-ids-stable.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
    "🗑️generated\\contact-capsule256\\256-material-culled.png": "04533916618013a0086ba105d9f07ae1b8b51f01bf78eead398f2c7bbe4941dc",
    "🗑️generated\\contact-capsule256\\256-material-full.png": "494c712a7ab3f4a74da1c39de798705e5bb401af87048c33ff5070e225c3ccd8",
    "🗑️generated\\contact-capsule256\\256-material-stable.png": "10315d5b4d38f4ac53261ad018bbf53b4fdb8a3f43b9a923e5c6d4f3ced2667c",
    "🗑️generated\\contact-capsule256\\256-silhouette-culled.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
    "🗑️generated\\contact-capsule256\\256-silhouette-full.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
    "🗑️generated\\contact-capsule256\\256-silhouette-stable.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
    "🗑️generated\\contact-capsule256\\coverage-aa-controls.json": "af79d35598191e5451898b3aaae9c6d774d65239731e980fdde39460e951f20a",
    "🗑️generated\\contact-capsule256\\coverage-camera.json": "75b599f98ffd63699a2c24976ab73615db7b4c2cc1ba98d6251cb7bf3f744ee4",
    "🗑️generated\\contact-capsule256\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce",
    "🗑️generated\\contact-capsule256\\coverage-paint-fragments.json": "e8e0b9d86b4d8564f7b86a229618ffc56651fc6d51e39debdaf7b9b64b50910d",
    "🗑️generated\\contact-capsule256\\coverage-provenance.json": "3d91189035b2249923b0ae8dcee1975d75d857e9f8a20025fda98443d15a9dad",
    "🗑️generated\\contact-capsule512\\2048-geometric-culled.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
    "🗑️generated\\contact-capsule512\\2048-geometric-full.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
    "🗑️generated\\contact-capsule512\\2048-geometric-stable.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
    "🗑️generated\\contact-capsule512\\2048-material-culled.png": "bf3865501102f40c3b3dbddca37256a2480a8f70f1e19d4978ae8ae453a17436",
    "🗑️generated\\contact-capsule512\\2048-material-full.png": "e8ed9166c80c21bb8d207da43e19b2d0bde82dbea333daa487654795e8287851",
    "🗑️generated\\contact-capsule512\\2048-material-stable.png": "856fb3418aaaf0c177fb56009be43604c9fa47d8bf84516715daad480bebdc3e",
    "🗑️generated\\contact-capsule512\\2048-silhouette-culled.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
    "🗑️generated\\contact-capsule512\\2048-silhouette-full.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
    "🗑️generated\\contact-capsule512\\2048-silhouette-stable.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
    "🗑️generated\\contact-capsule512\\512-aa-fill-culled.png": "f11db3d7ddef75c98dd4522541a8d33d4ea48a4f830804c7ee4edbc9761e39b6",
    "🗑️generated\\contact-capsule512\\512-aa-fill-full.png": "52ed1a21cb76a0676e60f048d41a7e8772037c2755d66a767eef8c49a6bf9a83",
    "🗑️generated\\contact-capsule512\\512-aa-fill-stable.png": "c8b11818901ca85daf68d7949ef070c80407fdeec236c50f3128e1cc7426cf50",
    "🗑️generated\\contact-capsule512\\512-aa-strokes-culled.png": "bcb4302ed4a274497a7b6152dce52bb5f45b316086aa3fe53c97bf9d3eda6cdc",
    "🗑️generated\\contact-capsule512\\512-aa-strokes-full.png": "2fb5b4d1ef9a4ea6e1ce6aa8c76af7375257bf9da65fb2f3d1ebfeb9a7efba56",
    "🗑️generated\\contact-capsule512\\512-aa-strokes-stable.png": "3912ec29c56a0d606a9ff7bd15b5791f09f73994957dc37dab10e5ae7b5833a3",
    "🗑️generated\\contact-capsule512\\512-geometric-culled.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
    "🗑️generated\\contact-capsule512\\512-geometric-full.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
    "🗑️generated\\contact-capsule512\\512-geometric-stable.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
    "🗑️generated\\contact-capsule512\\512-ids-culled.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
    "🗑️generated\\contact-capsule512\\512-ids-full.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
    "🗑️generated\\contact-capsule512\\512-ids-stable.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
    "🗑️generated\\contact-capsule512\\512-material-culled.png": "574d1f7b4777229427d92c1b09b1d1d1cc7fdd342ba99cc8454341a6c96e2928",
    "🗑️generated\\contact-capsule512\\512-material-full.png": "3c063106db0fca2ca8d2017eefeeae3f0c5ff2447a352b9ffb246fdc396b0f3e",
    "🗑️generated\\contact-capsule512\\512-material-stable.png": "9a20fb5e927537561a84269e1e18a68f6c6ffd58993493e5d712ab17b9abd748",
    "🗑️generated\\contact-capsule512\\512-silhouette-culled.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
    "🗑️generated\\contact-capsule512\\512-silhouette-full.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
    "🗑️generated\\contact-capsule512\\512-silhouette-stable.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
    "🗑️generated\\contact-capsule512\\coverage-aa-controls.json": "1f32b8d5f44b37f2d094fd9e340ccdba33c3221858a66a993ed2a970eca00c15",
    "🗑️generated\\contact-capsule512\\coverage-camera.json": "fb6d460aa4428bf58bef8f54ac09205922491ee4f2c7e30014edae893c4bf65c",
    "🗑️generated\\contact-capsule512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1",
    "🗑️generated\\contact-capsule512\\coverage-paint-fragments.json": "a0903170152349afd9d884ae7ac98cb1059f52d8ba61eb6e01ae19f17587b4c3",
    "🗑️generated\\contact-capsule512\\coverage-provenance.json": "d84f76b5d0bb1b6cc15ef3299f010565dfe3b88336f3d81a2ad5c18a88fe8bc3",
    "🗑️generated\\contact-capital512\\2048-geometric-culled.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
    "🗑️generated\\contact-capital512\\2048-geometric-full.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
    "🗑️generated\\contact-capital512\\2048-geometric-stable.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
    "🗑️generated\\contact-capital512\\2048-material-culled.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
    "🗑️generated\\contact-capital512\\2048-material-full.png": "d8f8909a1f14d8e0533af73a369c73de35c1c91bece51b09ea519bb8a25c62d9",
    "🗑️generated\\contact-capital512\\2048-material-stable.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
    "🗑️generated\\contact-capital512\\2048-silhouette-culled.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
    "🗑️generated\\contact-capital512\\2048-silhouette-full.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
    "🗑️generated\\contact-capital512\\2048-silhouette-stable.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
    "🗑️generated\\contact-capital512\\512-aa-fill-culled.png": "ca8c312edaff39fa660b910f347829c2a318c73544f253bfc6bc933defda0917",
    "🗑️generated\\contact-capital512\\512-aa-fill-full.png": "89f6195a0c0c2ebd82e42884fb5bf272bdd16541ddd5e6c6f29f742f8582f8ef",
    "🗑️generated\\contact-capital512\\512-aa-fill-stable.png": "0b709781d8acbf9bf046252b93206cdc00592dbe215e7f034e49392dcb9130ef",
    "🗑️generated\\contact-capital512\\512-aa-strokes-culled.png": "fc1895120a606f3fe0a8655d350176f30091522066365824fa7b9e1fc07e7e5c",
    "🗑️generated\\contact-capital512\\512-aa-strokes-full.png": "26453b906d6b827195aedc08e05d85dfb2cff261ceaaf6d8387a971a2201befb",
    "🗑️generated\\contact-capital512\\512-aa-strokes-stable.png": "8aca4be3da8d42f790a180edd35be4a437512e992154fbf2665a82373fad6e27",
    "🗑️generated\\contact-capital512\\512-geometric-culled.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
    "🗑️generated\\contact-capital512\\512-geometric-full.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
    "🗑️generated\\contact-capital512\\512-geometric-stable.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
    "🗑️generated\\contact-capital512\\512-ids-culled.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\contact-capital512\\512-ids-full.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\contact-capital512\\512-ids-stable.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\contact-capital512\\512-material-culled.png": "d88acb18ebacc1f6a40c5b95e1ca4b07ca7258868970146e1f46f71047d1769f",
    "🗑️generated\\contact-capital512\\512-material-full.png": "120e8b7b9e9ac5274e452a6d5559f9c75c4186ba403292e6d87a2c25b597e263",
    "🗑️generated\\contact-capital512\\512-material-stable.png": "65aee655dce05421f7db3c13937fffe023e63588e1ea117500f4d0749a8bedac",
    "🗑️generated\\contact-capital512\\512-silhouette-culled.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
    "🗑️generated\\contact-capital512\\512-silhouette-full.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
    "🗑️generated\\contact-capital512\\512-silhouette-stable.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
    "🗑️generated\\contact-capital512\\coverage-aa-controls.json": "42c9f518f77cab20e4ee8485fbadda7cfc420de8b84d3bbb244638ff00d32fee",
    "🗑️generated\\contact-capital512\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
    "🗑️generated\\contact-capital512\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba",
    "🗑️generated\\contact-capital512\\coverage-paint-fragments.json": "8b3b641a5fab3526e0a90002b2501c1ae16eaa2d8cb1bdb1a5763c03ffd2e57e",
    "🗑️generated\\contact-capital512\\coverage-provenance.json": "215d9133aedf9b117096ba9b6992aa6476f273636fb824516f230f2a7b6b7332",
    "🗑️generated\\contact-capsule256-ray.json": "a5359f6fe12265817a851952bd8fdde237c19bcaad46e9547dcd715942793709",
    "🗑️generated\\contact-capsule256-ray.progress.jsonl": "a0f57c1bff5be4b2512ac9a00c7e86b1040c8028d42bb6412b1179c5356f7bc7",
    "🗑️generated\\contact-capsule512-ray.json": "2855009c9b6b9ea10f8ce0d5ee5f8370ed00ce46cf73cc85387bf3c0e7a022a7",
    "🗑️generated\\contact-capsule512-ray.progress.jsonl": "e4b519bcc4d056e8ea7af7d0252c8d63f4aec23bb671b90262167ed2aeb6020a",
    "🗑️generated\\contact-capital512-ray.json": "c68dc7eab0d9a984f210692d664c10da740ce2ea1c0405e92555a652ca07203d",
    "🗑️generated\\contact-capital512-ray.progress.jsonl": "3c1cea1f972457644fbb31c9768e66c548fc913453950860ad3abf3d52ce4512",
    "🗑️generated\\fragment-ray-oracle\\result.json": "683a5eb6e818ab3ac5abfa435ed288f6c32cd679b84bfe9d656db8c0301ffd91",
    "🗑️generated\\fragment-ray-oracle\\tests.log": "b672d217ec1d7654656e6a5a84cff531e61bb83d50e6093cd87032be5b27a5d1",
    "🗑️generated\\paint-lineage\\green.json": "a361a0a4df6ac7cf90d3670d641a2a13e9906f8551391a7d7a23461b31c79485"
  }
}
```
