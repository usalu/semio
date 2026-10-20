import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = sys.argv[2]
TB = "semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::"
rows = [
 (136, "write_layer_record", "Writes the main stream fields of a layer table record.", {"binds": {"value": "&" + TB + "DwgLayerTableRecord"}}),
 (153, "write_linetype_record", "Writes the main stream fields of a linetype table record.", {"binds": {"value": "&" + TB + "DwgLinetypeTableRecord"}}),
 (174, "write_block_header_record", "Writes the main stream fields of a block header table record.", {"binds": {"value": "&" + TB + "DwgBlockHeaderTableRecord"}}),
 (201, "write_viewport_record", "Writes the main stream fields of a viewport table record.", {"binds": {"value": "&" + TB + "DwgViewportTableRecord"}}),
 (253, "write_dimension_style_record", "Writes the main stream fields of a dimension style table record.", {"binds": {"value": "&" + TB + "DwgDimensionStyleTableRecord"}}),
]
armspec.build(F, "encode_r2010_table_record_frame", 123, rows, "()", types={"data": "&mut DwgBitWriter", "strings": "&mut DwgBitWriter"}, out=sys.argv[1], context=["object", "body", "data", "strings"])
