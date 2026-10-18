"""c6ts: domain table for the PDF 1.7 snapshot type and schema names."""
D = {
 "cos": ["ObjRef", "PdfDecimal", "PdfDictEntry", "PdfObject", "PdfStreamFilter", "PdfCcittParameters", "PdfPredictor"],
 "document": ["PdfIndirectObject", "PdfPage", "PdfNamedColorSpace", "PdfNamedProperties"],
 "metadata": ["PdfInfo", "PdfDate", "PdfEncryption", "PdfEncryptionAlgorithm", "PdfMarkInfo", "PdfEmbeddedFile", "PdfOutputIntent", "PdfViewerPreferences", "PdfPageMode", "PdfPageLayout"],
 "navigation": ["PdfOpenAction", "PdfAction", "PdfActionKind", "PdfFileSpecification", "PdfDestination", "PdfDestinationFit", "PdfNamedDestination", "PdfOutlineItem", "PdfPageLabelRange", "PdfPageLabelStyle"],
 "form": ["PdfOptionalContent", "PdfOptionalContentGroup", "PdfAcroForm", "PdfFormField", "PdfFormFieldKind"],
 "color": ["PdfColorSpace", "PdfFunction"],
 "content": ["PdfOp", "PdfPropertyList", "PdfInlineImage", "PdfTextString", "PdfTextArrayItem"],
 "image": ["PdfImage", "PdfImageMask", "PdfImageBody"],
 "shading": ["PdfShading", "PdfShadingKind", "PdfPattern", "PdfPatternKind"],
 "state": ["PdfExtGState", "PdfSoftMask", "PdfFormXObject", "PdfTransparencyGroup", "PdfLineJoin", "PdfLineCap"],
 "font": ["PdfFont", "PdfFontKind", "PdfFontDescriptor", "PdfFontProgram", "PdfSimpleEncoding", "PdfBaseEncoding", "PdfEncodingDifference", "PdfCharProc", "PdfToUnicode", "PdfToUnicodeMapping", "PdfCMap", "PdfEmbeddedCMap", "PdfCodespaceRange", "PdfCidFont", "PdfCidMapping", "PdfCidSystemInfo", "PdfCidToGid", "PdfCidWidthRun", "PdfCidVerticalRun"],
 "annotation": ["PdfAnnotation", "PdfMarkupAnnotation", "PdfAppearance", "PdfAppearanceEntry", "PdfAppearanceState", "PdfBorderStyle"],
 "annotation-kind": ["PdfAnnotationKind"],
 "roles": ["PdfGraphIdentity", "PdfStreamRoleValue", "PdfAdmittedStreamRole", "ArtifactRef"],
}
