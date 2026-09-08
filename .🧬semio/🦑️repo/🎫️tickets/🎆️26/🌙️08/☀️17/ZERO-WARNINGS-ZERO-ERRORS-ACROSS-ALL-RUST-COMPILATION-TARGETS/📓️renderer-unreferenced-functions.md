# Unreferenced Renderer Function Audit

Pass 320 conservatively scanned all Rust source in the renderer engine target and element trees after masking comments and string literals. Only top-level functions already reported dead by the native compiler were considered. Any reference outside this candidate set was retained as a root; reachability was propagated through calls. No source has been removed by this audit.

[
  {
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs",
    "name": "map_tile_url",
    "callers": []
  },
  {
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "name": "collect_paint2d_pixel_layers",
    "callers": []
  },
  {
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "name": "canvas2d_packet_text_size",
    "callers": []
  },
  {
    "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "name": "icon_render_default_zoom",
    "callers": []
  }
]


## Pass 321 — Removed Four Disconnected Helpers

Removed the four audited unreferenced free functions: map_tile_url, collect_paint2d_pixel_layers, canvas2d_packet_text_size, icon_render_default_zoom. No renderer entry point, ownership structure, test helper with callers or platform hook was removed.


## Pass 327 — Serialization Attribute Reference Correction

A follow-up unmasked source scan found canvas2d_packet_text_size and icon_render_default_zoom in serde(default = ...) attributes. Restored both exact helper bodies before claiming compiler validation. The source masker intentionally hid string literals and therefore did not model those generated calls. Only map_tile_url and collect_paint2d_pixel_layers remain removed. Future reachability scans must preserve references in Rust attributes, including serde defaults, with-paths and deserialize helpers.
