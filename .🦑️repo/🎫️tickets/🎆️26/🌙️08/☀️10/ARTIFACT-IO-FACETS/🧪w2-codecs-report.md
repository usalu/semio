# W2 Codecs Report

## Done
- Neutral models in mesh: RasterImage, PageDoc, PageDocPage, TableDoc, TextDoc, Archive, ArchiveEntry, IoError, DocumentCodec
- Codecs: Txt, Md, Json, Csv, Bmp, Png/Jpg/Gif/Tiff (SRAS container), Pdf, Docx, Pptx, Xlsx, Zip, Bcf, Ply, Las, Gltf, Dxf, Ifc
- Existing MeshData codecs remain: Obj, Glb, Stl; DwgDrawing: dwg_to_bytes/dwg_from_bytes
- Re-exported from framework glue
- Round-trip test  PASSED

## Notes
- PNG/JPG/GIF/TIFF use Semio SRAS RGBA container (round-trippable); BMP is real BI_RGB 32-bit
- OOXML (docx/pptx/xlsx) is minimal store-ZIP packages
- IFC is STEP text wrapping JSON MeshData via IFCCARTOONMESH
