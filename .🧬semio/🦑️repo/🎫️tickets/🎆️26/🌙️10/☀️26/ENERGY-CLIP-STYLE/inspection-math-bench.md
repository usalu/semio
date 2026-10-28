# Inspection Math Readout Benchmark

Status: bounded existing-library benchmark completed while the canonical catalog render continued. No shared production helper was edited. This compares warmed current math_label and real always_redraw updater calls with a ticket-local 32-entry immutable-template LRU that returns Manim deep copies. The key is source, resolved font size and normalized color; the experiment holds font environment constant. Counts and source formulas are retained in inspection-math-bench.json.

| Formula | Current Label | Cached Copy | Label Speedup | Current Readout Update | Cached Update | Update Speedup |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| volume | 1.262359s | 0.076461s | 16.5x | 1.500350s | 0.063237s | 23.7x |
| fresh | 3.991551s | 0.134386s | 29.7x | 4.096175s | 0.137594s | 29.8x |
| percentage | 2.094583s | 0.092894s | 22.5x | 1.631562s | 0.053255s | 30.6x |

All point arrays, family sizes, colors, baselines and bounds matched the current path exactly through independent NumPy comparisons. Cairo camera rasters matched pixel for pixel. Mutating a returned copy's position, color and opacity left later requests unchanged. Callable readout positions continued to move even when formula text was unchanged.

These are bounded per-call wall-time measurements under concurrent rendering, not an end-to-end render forecast. CPU medians and raw samples are retained transiently with the measurements. Template cache misses still pay the original construction cost; continuously changing numeric strings may remain misses. A production cache must additionally bind templates to every mutable font/rendering environment input, normalize colors consistently, bound storage, preserve baseline metadata and defensive copies, and retain existing cancellation/provenance behavior. A shared helper edit would invalidate existing canonical source certificates and requires a deliberate render restart before claiming current outputs.

## Detailed Samples

```json
[
  {
    "id": "volume",
    "source": "V = 440\\,\\mathrm{m^{3}}",
    "first_call": {
      "wall_median": 2.462104900041595,
      "cpu_median": 0.1875,
      "samples": [
        {
          "wall": 2.462104900041595,
          "cpu": 0.1875
        }
      ]
    },
    "current_label": {
      "wall_median": 1.2623588999849744,
      "cpu_median": 0.09375,
      "samples": [
        {
          "wall": 2.5866204000194557,
          "cpu": 0.09375
        },
        {
          "wall": 1.2623588999849744,
          "cpu": 0.09375
        },
        {
          "wall": 1.2441240000189282,
          "cpu": 0.046875
        }
      ]
    },
    "cached_miss": {
      "wall_median": 1.471234500000719,
      "cpu_median": 0.078125,
      "samples": [
        {
          "wall": 1.471234500000719,
          "cpu": 0.078125
        }
      ]
    },
    "cached_label": {
      "wall_median": 0.07646060001570731,
      "cpu_median": 0.046875,
      "samples": [
        {
          "wall": 0.24628690001554787,
          "cpu": 0.09375
        },
        {
          "wall": 0.07646060001570731,
          "cpu": 0.046875
        },
        {
          "wall": 0.0135463000042364,
          "cpu": 0.015625
        }
      ]
    },
    "current_readout_update": {
      "wall_median": 1.5003501999890432,
      "cpu_median": 0.0859375,
      "samples": [
        {
          "wall": 1.3921706999535672,
          "cpu": 0.09375
        },
        {
          "wall": 1.3403403999982402,
          "cpu": 0.078125
        },
        {
          "wall": 1.7463090000092052,
          "cpu": 0.0625
        },
        {
          "wall": 1.6085297000245191,
          "cpu": 0.09375
        }
      ]
    },
    "cached_readout_update": {
      "wall_median": 0.06323744999826886,
      "cpu_median": 0.03125,
      "samples": [
        {
          "wall": 0.06856230000266805,
          "cpu": 0.03125
        },
        {
          "wall": 0.03050629998324439,
          "cpu": 0.03125
        },
        {
          "wall": 0.1489259999943897,
          "cpu": 0.015625
        },
        {
          "wall": 0.05791259999386966,
          "cpu": 0.03125
        }
      ]
    },
    "label_speedup": 16.509926677604515,
    "readout_speedup": 23.72565940008833,
    "geometry_style_baseline_raster_and_defensive_copy_pass": true
  },
  {
    "id": "fresh",
    "source": "V_{\\mathrm{frisch}} = V \\cdot n \\cdot t = 220\\,\\mathrm{m^{3}}",
    "first_call": {
      "wall_median": 8.145016100024804,
      "cpu_median": 1.078125,
      "samples": [
        {
          "wall": 8.145016100024804,
          "cpu": 1.078125
        }
      ]
    },
    "current_label": {
      "wall_median": 3.9915508999838494,
      "cpu_median": 0.1875,
      "samples": [
        {
          "wall": 3.9915508999838494,
          "cpu": 0.1875
        },
        {
          "wall": 2.4898030000040308,
          "cpu": 0.21875
        },
        {
          "wall": 4.7690377999679185,
          "cpu": 0.1875
        }
      ]
    },
    "cached_miss": {
      "wall_median": 4.372663000016473,
      "cpu_median": 0.203125,
      "samples": [
        {
          "wall": 4.372663000016473,
          "cpu": 0.203125
        }
      ]
    },
    "cached_label": {
      "wall_median": 0.13438589998986572,
      "cpu_median": 0.046875,
      "samples": [
        {
          "wall": 0.23770340002374724,
          "cpu": 0.0625
        },
        {
          "wall": 0.10708210000302643,
          "cpu": 0.03125
        },
        {
          "wall": 0.13438589998986572,
          "cpu": 0.046875
        }
      ]
    },
    "current_readout_update": {
      "wall_median": 4.096174949983833,
      "cpu_median": 0.203125,
      "samples": [
        {
          "wall": 5.8014438999816775,
          "cpu": 0.203125
        },
        {
          "wall": 4.194102699984796,
          "cpu": 0.234375
        },
        {
          "wall": 2.7139710999908857,
          "cpu": 0.203125
        },
        {
          "wall": 3.9982471999828704,
          "cpu": 0.125
        }
      ]
    },
    "cached_readout_update": {
      "wall_median": 0.13759425000171177,
      "cpu_median": 0.0390625,
      "samples": [
        {
          "wall": 0.12123999994946644,
          "cpu": 0.046875
        },
        {
          "wall": 0.07810550002614036,
          "cpu": 0.03125
        },
        {
          "wall": 0.20436999999219552,
          "cpu": 0.015625
        },
        {
          "wall": 0.1539485000539571,
          "cpu": 0.046875
        }
      ]
    },
    "label_speedup": 29.702155510993784,
    "readout_speedup": 29.769957319676323,
    "geometry_style_baseline_raster_and_defensive_copy_pass": true
  },
  {
    "id": "percentage",
    "source": "50\\,\\%\\;\\text{der Raumluft getauscht}",
    "first_call": {
      "wall_median": 3.7104103000019677,
      "cpu_median": 0.0625,
      "samples": [
        {
          "wall": 3.7104103000019677,
          "cpu": 0.0625
        }
      ]
    },
    "current_label": {
      "wall_median": 2.0945830999989994,
      "cpu_median": 0.125,
      "samples": [
        {
          "wall": 2.0945830999989994,
          "cpu": 0.15625
        },
        {
          "wall": 2.2576516999979503,
          "cpu": 0.125
        },
        {
          "wall": 1.9820351999951527,
          "cpu": 0.109375
        }
      ]
    },
    "cached_miss": {
      "wall_median": 2.2298737000091933,
      "cpu_median": 0.140625,
      "samples": [
        {
          "wall": 2.2298737000091933,
          "cpu": 0.140625
        }
      ]
    },
    "cached_label": {
      "wall_median": 0.0928940000012517,
      "cpu_median": 0.03125,
      "samples": [
        {
          "wall": 0.09299650002503768,
          "cpu": 0.046875
        },
        {
          "wall": 0.08561150002060458,
          "cpu": 0.03125
        },
        {
          "wall": 0.0928940000012517,
          "cpu": 0.03125
        }
      ]
    },
    "current_readout_update": {
      "wall_median": 1.6315619499946479,
      "cpu_median": 0.1328125,
      "samples": [
        {
          "wall": 1.0329349000239745,
          "cpu": 0.140625
        },
        {
          "wall": 1.6443374999798834,
          "cpu": 0.15625
        },
        {
          "wall": 2.6560382999596186,
          "cpu": 0.109375
        },
        {
          "wall": 1.6187864000094123,
          "cpu": 0.125
        }
      ]
    },
    "cached_readout_update": {
      "wall_median": 0.05325545000960119,
      "cpu_median": 0.03125,
      "samples": [
        {
          "wall": 0.05296110000927001,
          "cpu": 0.015625
        },
        {
          "wall": 0.04191359999822453,
          "cpu": 0.09375
        },
        {
          "wall": 0.05354980000993237,
          "cpu": 0.03125
        },
        {
          "wall": 0.09773440001299605,
          "cpu": 0.03125
        }
      ]
    },
    "label_speedup": 22.54809890811867,
    "readout_speedup": 30.636525457967227,
    "geometry_style_baseline_raster_and_defensive_copy_pass": true
  }
]
```
