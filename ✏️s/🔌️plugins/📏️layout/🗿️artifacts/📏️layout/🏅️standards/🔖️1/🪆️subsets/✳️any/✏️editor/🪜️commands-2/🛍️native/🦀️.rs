//! 🛍️ Native placements: the placeable 2D artifact kinds and the shell open triples.

/// 🛍️ Page and frame tools, then every placeable 2D artifact. The third column is the native artifact kind stored on the link.
pub const NATIVE_PLACEMENTS: &[(&str, &str, &str)] = &[
    ("png", "image", "s.stdio.png"),
    ("jpg", "image", "s.stdio.jpg"),
    ("gif", "image", "s.stdio.gif"),
    ("bmp", "image", "s.stdio.bmp"),
    ("tiff", "image", "s.stdio.tiff"),
    ("pdf", "file-text", "s.stdio.pdf"),
    ("svg", "spline", "s.stdio.svg"),
    ("drawing", "pen-tool", "s.draw.drawing"),
    ("dwg", "pen-tool", "s.stdio.dwg"),
    ("dxf", "pen-tool", "s.stdio.dxf"),
    ("raster", "image", "s.raster.raster"),
    ("bitmap", "image", "s.wfc.bitmap"),
    ("cad", "pen-tool", "s.cad.cad"),
    ("map", "map", "s.gis.gismap"),
    ("fem2d", "spline", "s.fem.fem2d"),
    ("note", "file-text", "s.note.note"),
    ("writer", "type", "s.writer.writer"),
    ("forms", "file-text", "s.forms.forms"),
    ("flow", "spline", "s.flow.flow"),
    ("equation", "type", "s.mathematical.equation"),
    ("puzzle2d", "square", "s.puzzle.puzzle2d"),
    ("block2d", "square", "s.block.block2d"),
    ("generation2d", "spline", "s.procedural.generation2d"),
    ("terrain", "map", "s.gis.gisterrain"),
    ("grid2d", "image", "s.wfc.grid2d"),
    ("wfc2d", "image", "s.wfc.wfc2d"),
    ("presentation", "file", "s.animate.presentation"),
    ("sequence", "file", "s.sequence.sequence"),
    ("wires", "spline", "s.reasoning.wires"),
];

pub fn native_artifact_kind(kind: &str) -> Option<&'static str> {
    NATIVE_PLACEMENTS.iter().find(|(candidate, _, _)| *candidate == kind).map(|(_, _, artifact_kind)| *artifact_kind)
}

/// 📂 Kind, standard, subset, and document schema the shell uses to open that artifact's editor.
pub const NATIVE_OPEN: &[(&str, &str, &str, &str)] = &[
    ("s.stdio.png", "1.2", "*", "stdio.png"),
    ("s.stdio.jpg", "jfif-1.01", "*", "stdio.jpg"),
    ("s.stdio.gif", "89a", "*", "stdio.gif"),
    ("s.stdio.bmp", "v3", "*", "stdio.bmp"),
    ("s.stdio.tiff", "6.0", "*", "stdio.tiff"),
    ("s.stdio.pdf", "1.7", "*", "stdio.pdf"),
    ("s.stdio.svg", "1.1", "*", "stdio.svg"),
    ("s.draw.drawing", "1", "*", "drawing.document"),
    ("s.stdio.dwg", "ac1018", "*", "stdio.dwg"),
    ("s.stdio.dxf", "r12", "*", "stdio.dxf"),
    ("s.raster.raster", "1", "*", "raster.document"),
    ("s.wfc.bitmap", "1", "*", "s.wfc.bitmap"),
    ("s.cad.cad", "1", "*", "cad.document"),
    ("s.gis.gismap", "1", "*", "gis.map"),
    ("s.fem.fem2d", "1", "*", "fem.2d"),
    ("s.note.note", "1", "*", "note.document"),
    ("s.writer.writer", "1", "*", "writer.document"),
    ("s.forms.forms", "1", "*", "forms.form"),
    ("s.flow.flow", "1", "*", "flow.host_snapshot"),
    ("s.mathematical.equation", "1", "*", "semio.equation/v1"),
    ("s.puzzle.puzzle2d", "1", "*", "board.ports.directed.v1"),
    ("s.block.block2d", "1", "*", "block.2d"),
    ("s.procedural.generation2d", "1", "*", "generation.2d"),
    ("s.gis.gisterrain", "1", "*", "gis.terrain"),
    ("s.wfc.grid2d", "1", "*", "s.wfc.grid2d"),
    ("s.wfc.wfc2d", "1", "*", "s.wfc.wfc2d"),
    ("s.animate.presentation", "1", "*", "animate.presentation"),
    ("s.sequence.sequence", "1", "*", "sequence.sequence"),
    ("s.reasoning.wires", "1", "*", "reasoning.wires.identity.snapshot"),
];

pub fn native_open(artifact_kind: &str) -> Option<(&'static str, &'static str, &'static str)> {
    NATIVE_OPEN.iter().find(|(kind, _, _, _)| *kind == artifact_kind).map(|(_, standard, subset, schema)| (*standard, *subset, *schema))
}
