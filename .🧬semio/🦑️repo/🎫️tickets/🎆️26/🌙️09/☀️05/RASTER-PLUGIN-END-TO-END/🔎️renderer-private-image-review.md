# Renderer Private PNG Image Review

OriginalPngTexture now retains the completed RasterImage privately, retires the original decoder under the caller's funded close, and only then performs one admitted donation into an empty DecodedReferenceImage output while clearing the source binding. This removes the earlier pointer-and-length content witness. Cancellation close selects the decoder, then the image, then the binding; each is kept until its funded turn completes.

The current fixture checks denied birth, source pointer preservation, successful decode, original pixel pointer transfer and paid decoder retirement before output donation. It does not yet prove denied image handoff, denied decoder close, receipt refusal after donation, or genuine worker-to-boundary identity. The mounted renderer uses the same-original-caller worker path as documented in 🔎️renderer-worker-close-audit.md. Source review only; no native build was run.
