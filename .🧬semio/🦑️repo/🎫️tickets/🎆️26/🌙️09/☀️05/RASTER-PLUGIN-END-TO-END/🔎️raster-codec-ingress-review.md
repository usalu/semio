# Raster Codec Ingress Review

The newly mounted but not yet wired Base64Ingress and PngIngress own original text, partial output and decoder state. They use actual finite work selection, retain outputs before construction receipt reconciliation, and expose funded close paths. Source review identified two concrete issues, reported to controls:

- PngIngress's decoder birth takes the input Vec into PngDecodeInput and restores it after PngDecodeJob::new refusal, but the returned error carries zero retained progress despite the actual header transfers. The failure-path copy receipt must represent the actual transfers, not the successful-job birth quote.
- PNG demand methods, especially text_metadata_demands, can return an original PngDecodeError before the inner advance. The current map_err closures discard it and return only a generic invariant instead of retaining the exact cause in the ingress failure owner.

No native build was run; the source may change concurrently.
