# AXON toolkit report (work in progress)

## Published first: the shared projection (stable, never changes)

```python
axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
axon.AXON                                   # {"ux": (0.866, 0.5), "uy": (-0.866, 0.5), "uz": (0.0, 1.0)}
axon.axon_point(x, y, z, point, scale)      # np.array([px, py, 0]) = point + scale * (x*ux + y*uy + z*uz)
```

`point` is the screen position of the **model origin** (grid corner of axis A/1 at ±0.00); x east, y north, z up, metres. `villa_axon(point, scale)` takes the **drawing centre** instead (see the final section once written).
