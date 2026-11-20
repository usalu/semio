# PDF Type 3 glyph painting

The PDF page canvas now interprets authored Type 3 glyph programs. The painter selects a character procedure using the declared encoding and differences and places it using the glyph matrix, text transform, font size, widths, spacing, and adjustments from the source show operation. Glyph graphics use the existing page program interpreter and its clip layers. Canvas selection stays on the containing text item. The renderer shares the scene operation budget and stops recursive glyph reuse.

MuPDF verified both authored Type 3 colors and the adjusted gap from the shared fixture. The isolated Rust probe used the real glyph painter and page geometry with shallow type and paint-call stand-ins. It matched both glyph origins and the exact operation budget. The governed page regression constructs a genuine Type 3 PDF from the same fixture, imports it through the full PDF codec and checks the scene. That gate is still running.
