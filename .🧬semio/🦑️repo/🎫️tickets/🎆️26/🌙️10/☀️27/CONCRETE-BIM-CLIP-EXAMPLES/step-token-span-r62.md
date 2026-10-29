# Empty STEP Text Annotation R62

Capturedc53core has one additional failure in BimStepExchangeContract.test_spans_point_at_the_tokens_they_name for SL-1. The native property line is #12766=IFCPROPERTYSINGLEVALUE('SharedOwnerTags',$,IFCTEXT(''),$); its value span is line12672,start59,end59. The native text itself is valid; the annotation has zero width.

The exact producer is model/🔌️exchange/🐍️.py::_Writer.raw: it appends a role span even when chunk is empty. This is not a slab thickness, geometry, quantity or IFC identity defect. Retained language-agnostic cases and executable input step-empty-token-r62.json/.py reproduce it without constructing any slab. EmptyIFCTEXT reports start=end9; ordinary and apostrophe-escaped strings pass. No test limit or regex was weakened.

The writer should emit no annotation for nonexistent empty inner text, or annotate the actual quoted empty token. Every reported span must cover real serialized characters. Source and native IFC line bytes should stay unchanged if only empty annotation handling changes. Parent owns producer patch/timing and an exact token/span affected-subset recheck; a full geometry rerun is unnecessary for byte-identical serialization with corrected metadata.
