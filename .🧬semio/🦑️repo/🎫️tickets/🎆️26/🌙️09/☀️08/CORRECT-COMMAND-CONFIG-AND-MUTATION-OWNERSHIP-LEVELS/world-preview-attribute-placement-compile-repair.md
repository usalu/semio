# World Preview Attribute Placement Compile Repair

Generation2d native validation reported duplicate Clone/Debug implementations on WorldCatalogueDropPreviewRecord and missing Clone/Default on WorldBrushPreviewRecord. Root reread the current World source and confirmed that the new catalogue record had been inserted between the existing brush record's documentation/derive/serde attributes and the brush struct. The duplicate derive and missing brush traits share that one placement error.

Root moved only the brush documentation, `Clone, Debug, Deserialize, Default` derive and camelCase serde attribute back immediately before WorldBrushPreviewRecord. The catalogue record retains its own Clone/Debug derive and all fields unchanged. No preview behavior or other logical work was rewritten. This is a narrow shared compile repair based on the actual compiler failure; fresh Gen2/Home/FEM native routes must confirm compilation.

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`.
