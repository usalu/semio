# Default Physical Attachment Hardware

A final consumer audit found a concrete missing-detail path: capsule_axon defaults bolts to false, and the shared assembly item builder then omits actual bolt shafts and heads. Most standalone Grasshopper and Excel reference calls do not pass that option. The physical source model, tower renderer and orthographic renderer already retain the hardware, so a correct shared physical assembly alone did not prove these clip views were complete.

The portable reference_hardware case requires four bolt shafts and four heads for all six physical capsule families. Its regression exercises the real model and assembly-item dispatch before expensive painting, compares every hardware part bounding interval against the physical source through independent NumPy calculations, and retains an explicit false-option control for intentional staged diagrams. The initial run failed all six family cases in17.268 seconds: all eight attachment components were absent from every default call. The renderer default is now true and explicit suppression remains supported. The focused six-family hardware regression passed together with the actual shared-geometry render in a combined 132.337-second run. A historical broad visual assertion that excluded hardware was strengthened to require all physical source parts; its complete-suite rerun is pending. See cutaway-boundary-repair.md and frozen-source-hardware-test-amendment.md.

This changes reference completeness rather than nominal thermal/program quantities or physical source geometry. No legacy adapter or extra runtime dependency is introduced.

## Audited Clip Call Sites

The following bounded source census includes multiline starts; explicit options on continuation lines must be inspected with the source. It is evidence of the consumer path, not an assertion that every call lacks a later keyword.

```json
[
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 173,
    "call": "drawing = self.capsule_axon(params, at=v(0, (p[\"back\"] + l / 2) * rv[\"lift\"], h / 2 * rv[\"lift\"]), scale=SCALE, view=view, window=rv[\"window\"] > 0.5, dims=dims, base=_canvas.PREVIEW_FILL, dim_offset=0.4)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 322,
    "call": "drawing = self.capsule_axon(params, at=v(0, (p[\"back\"] + l / 2) * lift, h / 2 * lift), scale=SCALE, view=view, window=rv[\"window\"] > 0.5, dims=dims, base=_canvas.PREVIEW_FILL, dim_offset=0.4)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 394,
    "call": "drawing = self.capsule_axon(_nk.capsule_type(type_key), at=core.at(*centre), scale=SCALE_WRAP, view=(AZIMUTH - yaw, ELEVATION), base=color or _nkd.type_color(type_key), layout=layout, interior=\"study\")"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 415,
    "call": "drawing = self.capsule_axon(_nk.capsule_type(\"A\"), at=self._core.at(*centre), scale=SCALE_CORE, view=(AZIMUTH - yaw, ELEVATION), base=_canvas.PREVIEW_FILL)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 475,
    "call": "strip.add(_fade(self.capsule_axon(_nk.capsule_type(pick.tag[\"type\"]), at=(x, y, 0), scale=0.13, view=view, base=_nkd.type_color(pick.tag[\"type\"]),layout=pick.tag[\"family\"],interior=\"study\"), wv[\"caps\"]))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 541,
    "call": "group.add(self.capsule_axon(_nk.capsule_type(\"A\"), at=(x, y, 0), scale=0.075, base=_canvas.PREVIEW_FILL))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py",
    "line": 982,
    "call": "minis.add(self.capsule_axon(t, at=(x, -1.5, 0), scale=0.2, base=_nkd.type_color(t.key), cutaway=True, interior=\"study\"), self._say(self.tr(\"Type \", \"Typ \") + t.key, (x, -2.2), size=SMALL, color=_nkd.type_color(t.key)))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py",
    "line": 426,
    "call": "programs = self.capsule_axon(types[new_key], at=(VX, VY, 0), scale=.6, cutaway=True, interior=\"study\")"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py",
    "line": 280,
    "call": "drawing = self.capsule_axon(shape, at=VIEW + np.array([-0.2, 0.12, 0.0]), scale=state[\"scale\"], base=_canvas.PREVIEW_FILL, window=state[\"window\"],"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py",
    "line": 444,
    "call": "strip = _engine.VGroup(*[self.capsule_axon(t, at=_at(-3.0 + 2.0 * i, 0.2), scale=0.2, base=_canvas.PREVIEW_FILL, window=True) for i, t in enumerate(kinds)],"
  },
  {
    "path": "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py",
    "line": 357,
    "call": "cap = self.capsule_axon(_nakagin.capsule_type(\"A\"), at=PORT_AT + np.array([0, up + 0.15, 0]), scale=0.8, dims=dims, window=window, highlight=highlight, hatch=hatch)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py",
    "line": 234,
    "call": "return self.capsule_axon(self.cap, at=tuple(state.at), scale=state.scale, view=state.view, bolts=state.bolts, dims=dims, hatch=state.hatch, highlight=highlight)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py",
    "line": 287,
    "call": "drawing = self.capsule_axon(self.cap, at=tuple(at), scale=scale, view=state.view, bolts=state.bolts, dims=state.dims, highlight=colors, hatch=state.hatch)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py",
    "line": 443,
    "call": "big = self.capsule_axon(self.cap, at=(self.dx, 0.1, 0.0), scale=0.6)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py",
    "line": 716,
    "call": "group.add(self.capsule_axon(t, at=(x, at[1], 0.0), scale=0.14, window=True),"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py",
    "line": 782,
    "call": "group = _engine.VGroup(self.capsule_axon(self.cap, at=tuple(at), scale=0.28, view=\"plan\", hatch=(\"top\",), window=False))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py",
    "line": 236,
    "call": "views.append(self.capsule_axon(spec, at=centre, scale=0.3 if state[\"card\"] else 0.44, view=view, highlight=highlight, hatch=state[\"hatch\"], dims=state[\"dims\"] if view == \"axon\" else ()))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py",
    "line": 635,
    "call": "plan = self.capsule_axon(_nk.capsule_type(\"A\"), at=(RIGHT_X, -1.55, 0), scale=0.3, view=\"plan\", hatch=(\"top\",))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py",
    "line": 723,
    "call": "capsule = self.capsule_axon(_nk.capsule_type(key), at=(x, 0.7, 0), scale=0.2, view=\"axon\", base=_nd.type_color(key))"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py",
    "line": 159,
    "call": "cap = self.capsule_axon(t, at=CAP_AT, scale=CAP_SCALE, dims=s.dims)"
  },
  {
    "path": "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py",
    "line": 278,
    "call": "cap = self.capsule_axon({\"dimensional_basis\": \"nominal\", \"key\": \"A\", **self._outer}, at=CAP_AT, scale=CAP_SCALE, dims=(\"width\", \"length\", \"height\"))"
  }
]
```
