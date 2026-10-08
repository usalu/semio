# Resolved Prepared Seven-Owner Caller Census

Read-only OS WGPU renderer source SHA `fa638dd7549e43c3b1d8b41b9a3d6f59cf1cb1ae5905383a06510413d4397650`. No source/compiler/test mutation or runtime credit.

Resolved production edges in this file, using declared field types:

| Owner | Declared field | Ungranted call |
|---|---|---|
| InputRejected | input_rejected15156 |15236|
| AtlasPages | icon_pages15151 |15278|
| AtlasPages | glyph_pages15382 |15408|
| InputRejected | draw_rejected15132 |15449|
| Upload | upload_rejected15131 |15456|
| Producer | raster_rejected15514 |16188|
| JobRejected | job_rejected16329 |16380,16460|

Input is held resource_input15129/15155; its actual production retirement delegates through frame-owned input and the defining JobRejected2647 input.close_step call. Native tests directly admit and drain Input. RasterRejected direct calls were found in existing prepared unit tests288/289/432/438/456/461; no independently resolved production call outside prepared was found in this bounded owner-name census. Absence claim is bounded, not compiler exhaustiveness.

Current demand methods Producer1270, AtlasPages778, Upload1483, Input2113, Job2660 are private; external production callers cannot simply reuse them. Expose defining fallible full demand and close_step(grant)->typed receipt together, with exact actual owner semantics. InputRejected and RasterRejected need their own honest demand/physical close definitions. JobRejected must delegate the original Input grant/receipt rather than manufacture authority. Existing renderer original generic job rejected16389 and worker-session16493 already distinguish demand errors and consume typed receipts; they are useful local integration patterns, not aliases to these prepared owners.

Renderer parent close methods currently return bool and have no caller grant. Their migration must propagate a real owner grant across the declared child edge and preserve original Option until actual receipt fits and child terminal emptiness is proven. Do not adapt unrelated engine document/router/Gate booleans based only on spelling. Preserve mailbox generation, credit/permit identity and exact original rejected owners.

Existing selectors: @semio-tech/ui-rs:test-physical-close source and test-physical-close-native; native prepared unit filter suite resides targets-wgpu-prepared-unit. Existing direct tests producer276, rejected288–289, admitted304, pages322–326, upload/owner346/382/410 and rejected432/438/456/461 must pass explicit grants and assert denied identity, physical backing and terminal state. Tests must not quote demand and silently grant themselves inside production API. Actual normal baseline is Root-owned and pending, not evidence supplied by this census.
