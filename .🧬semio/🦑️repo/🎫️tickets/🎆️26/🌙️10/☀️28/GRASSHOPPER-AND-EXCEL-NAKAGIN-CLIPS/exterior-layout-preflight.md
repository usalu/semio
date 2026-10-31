# Exterior Gallery Bounds Preflight

A geometry-only conservative preflight avoids spending a complete cold tower build on an invalid gallery crop. It constructs the real shared source items, includes all four ground rectangle corners, and projects every source vertex with the actual view basis. Occlusion may remove source material but cannot enlarge these bounds. The original0.092scale exceeded every literal pixel band; no image was accepted or cropped to hide this.

The ticket gallery now uses0.085scale from the start of rendering. The independent source support calculation proves at least0.03scene-unit horizontal margin in all four panels, retaining every physical element and using the correct pixel-coverage padding at the actual scale. No model, geometry, tessellation, global renderer or clip consumer changed. The actual worker retains its full drawing bounds assertion before each panel save. Fresh complete capture must bind the revised ticket input; previous full-capture attempts remain non-passes.

The source-only preflight command exited0. Corrected support intervals are below; the complete original record is generated/exterior-layout-preflight.json and the retained input is preflight-exterior-layout.py.

```json
{
  "items": 355232,
  "source_bounds": [
    [
      -22.43078593622583,
      -15.920226747962813,
      -0.18
    ],
    [
      14.21584916134264,
      10.344999999999999,
      53.5
    ]
  ],
  "views": [
    {
      "azimuth": 45,
      "scale": 0.092,
      "panel_x": [
        -8.0,
        -3.875
      ],
      "source_x": [
        -7.846328589890135,
        -3.7536714101098654
      ],
      "fits": false,
      "conservative_scale": 0.08519648352729124,
      "corrected_scale": 0.085,
      "corrected_x": [
        -7.690629675441972,
        -3.9093703245580276
      ]
    },
    {
      "azimuth": 135,
      "scale": 0.092,
      "panel_x": [
        -3.875,
        0.0
      ],
      "source_x": [
        -3.996328589890134,
        0.09632858989013449
      ],
      "fits": false,
      "conservative_scale": 0.08519648352729127,
      "corrected_scale": 0.085,
      "corrected_x": [
        -3.8406296754419715,
        -0.05937032455802771
      ]
    },
    {
      "azimuth": 225,
      "scale": 0.092,
      "panel_x": [
        0.0,
        3.875
      ],
      "source_x": [
        -0.09632858989013449,
        3.996328589890134
      ],
      "fits": false,
      "conservative_scale": 0.08519648352729127,
      "corrected_scale": 0.085,
      "corrected_x": [
        0.05937032455802771,
        3.8406296754419715
      ]
    },
    {
      "azimuth": 315,
      "scale": 0.092,
      "panel_x": [
        3.875,
        8.0
      ],
      "source_x": [
        3.7536714101098654,
        7.846328589890135
      ],
      "fits": false,
      "conservative_scale": 0.08519648352729126,
      "corrected_scale": 0.085,
      "corrected_x": [
        3.9093703245580276,
        7.690629675441972
      ]
    }
  ]
}
```
