# Remaining Stdio Diagnostic Review

The pass306 inventory includes 143 discarded futures, 124 large mutation enum variants and 345 pass-by-value diagnostics in Stdio. These require ownership and call-chain review; they were not part of the pass319 expression changes. Registration examples show synchronous wrappers creating unpolled register_composer_entries/register_document_codec futures, so explicit drop would preserve the defect. No future site has been changed by this audit.

Additional type and whitespace diagnostics are grouped in the generated diagnostic333 inventory. Review of existing synchronous document-codec registration and composer publication barriers must precede choosing the registration call-chain change. The framework currently has register_document_codec_now, while composer registration uses async signatures over the assembly barrier and standard-library registry locks.
