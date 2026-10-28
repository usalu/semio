# Surviving Transition PNG Parameter Audit

Read-only actual PNG chunk scan; no source edits, render or extraction.

Concrete difference: first/last snapshots include pHYs=000000010000000100 (1:1 ratio, unit unspecified); browser/boundary/probe frames omit pHYs. Both families are RGB color type2,8-bit,480×270. No ICC/sRGB/gAMA/cHRM chunks found in the scanned live artifacts. This is an actual metadata difference that may yield unknown versus1:1 decoder SAR; executor owns native FFmpeg reinit proof. No causal filter claim is made from chunks alone.

Current live p-92c2ffb5 scan:

[
  {
    "path": "first.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": [
      "pHYs:000000010000000100"
    ]
  },
  {
    "path": "i/0000.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0001.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0002.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0003.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0004.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0005.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0006.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0007.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0008.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0009.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0010.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0011.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0012.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  },
  {
    "path": "i/0013.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": [
      "pHYs:000000010000000100"
    ]
  },
  {
    "path": "last.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": [
      "pHYs:000000010000000100"
    ]
  },
  {
    "path": "o/probe-58bc7faf-0.png",
    "width": 480,
    "height": 270,
    "depth": 8,
    "colorType": 2,
    "chunks": []
  }
]

The old EB07 published work and p-2b729f0e sequence were already cleaned before inspection. An earlier broad generated/i scan had34 PNGs: six snapshot-family pHYs files and28 browser-family no-pHYs files. Live work changes during publication; counts are checkpoint-specific. Root and executor notified with exact family difference.

## Exact EB07 Restore Axis Found By Executor

Executor captured actual failing restore sequence in generated/ie/varied/o/:0012.png and0014.png are RGBA with alpha242–255; surrounding images are RGB. Native debug graph reconfigures four times at those stream2 mode flips. This is stronger causal evidence than the focus-only pHYs difference above. The scout previous scan covered surviving focus/current boundary families, not the subsequently retained exact failing restore; it must not be read as proving uniform pixel mode across EB07. Executor owns RGB normalization and strict decoded clock guard; no scout production edit.
