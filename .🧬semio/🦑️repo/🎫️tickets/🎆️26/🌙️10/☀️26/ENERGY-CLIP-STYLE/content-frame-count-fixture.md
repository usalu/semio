# Content Frame Count Fixture

The generated source is two seconds of eight fps H.264. Each selection uses input-side seeking, a 1.5-second duration and stream copying. `copied` is the FFmpeg null-output frame counter with video stream copy; `decoded` removes that copy codec; `pyav` counts images returned by the independent decoder. All experiments retain preroll samples that are absent from the displayed frame sequence. The permanent regression selects the 0.125-second case.

```json
[
  {
    "seek": 0.0625,
    "copied": 13,
    "decoded": 12,
    "pyav": 12
  },
  {
    "seek": 0.125,
    "copied": 13,
    "decoded": 12,
    "pyav": 12
  },
  {
    "seek": 0.1875,
    "copied": 14,
    "decoded": 12,
    "pyav": 12
  },
  {
    "seek": 0.25,
    "copied": 14,
    "decoded": 12,
    "pyav": 12
  },
  {
    "seek": 0.375,
    "copied": 15,
    "decoded": 12,
    "pyav": 12
  }
]
```
