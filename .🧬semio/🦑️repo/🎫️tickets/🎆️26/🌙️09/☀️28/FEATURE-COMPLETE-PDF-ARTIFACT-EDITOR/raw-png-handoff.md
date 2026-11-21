# Original Picker PNG Decoder Handoff

The format work uses the current framework `PngDecodeJob` with an owned native `PagedBytes` source. The guarded stage preserves the existing IDAT range allocator, CRC and Adler checks, filters, palette and transparency behavior, sample conversion, Adam7 passes, raster progress and controlled retirement. It reads an eight-byte chunk header or four-byte CRC through fixed-width checked reads. IHDR, palette and transparency read only their bounded payloads. The CRC octet and IDAT cursor read directly from the original pages. The contiguous overload retains its source-bound text metadata entry point.

`RawPngWork` binds the source to the caller and the full issued operation, generation, instance, document and revision. It pays the decoder admission quote before moving the exact completed page owner out of `OriginalGuestMediaInput`. A denied quote leaves the complete source with the picker. Each advance and raster loan checks the current Original caller and captured document loan again. A stale revision or a canceled caller permanently revokes result access. The close path retires the decoder, its encoded pages, raster and work buffers under the same source before the caller returns the picker and command-origin loan.

## Product registration

Use the guarded `stage_decoder` function in `raw-png-handoff-stage.py` to materialize the codec change. The focused runner built the complete pixels-codecs crate with the candidate decode owner and live neighboring modules.

Wire `RawPngWork<ImagePlacement, { MAX_FILE_BYTES as usize }>` into the same retained picker state as `OriginalGuestMediaInput`. Pass `PngDecodeLimits { max_pixels: MAX_IMAGE_IMPORT_PIXELS, max_bytes: MAX_FILE_BYTES as usize, max_chunks: 1024 }` on admission. Use `demand` and `close_demand` from the job budget. The PDF document crate already imports the pixels facade and exports its decoder module through `pub use decode::*`. It can reexport `RasterImage`, `PngDecodeJob`, `PngDecodeLimits` and `PngDecodeProgress` from that facade for the Base editor adapter.

Keep the completed picker input beside the work item through publication so `raster` can revalidate the binding. After sampling completes, use the returned trusted `ImagePlacement` and `SampledRgba` with the existing `place_sampled_image` on the live snapshot. That function checks live placement and builds the optional soft mask, image and paint mutations together. The caller still owns the funded transaction and terminal response. Cancel during that interval must drive decoder close and picker close while the command-origin loan is live. Transport Complete establishes the source handoff. Product success requires the placement transaction to settle.

## Verification

The three focused native tests passed using the complete staged codec linked to exact `p5d2dcc` crate families. The inputs traversed the real retained caller, raw picker, command cursor, and physically funded fixed-page custody. The first decoder turn moved the original owner with zero reported backing allocation. A one-copy-byte-short grant left every original encoded byte with the picker. Tests also exercised a different Source with the same numeric operation and generation, a change to the final revision byte after decoder admission, actual caller cancellation and a corrupt IDAT CRC. All native close paths reached terminal. Observed close took 344–453 polling turns under the retained fixture budget, which includes the existing controlled decoder field cursors. Keep polling close until it reports terminal before returning the Original loan.

Pillow independently decoded these four files and compared the full output to the native decoder's captured raster. Pillow and the native codec both rejected the corrupt IDAT CRC specimen.

| Fixture | Encoded bytes | RGBA bytes | SHA-256 of RGBA |
| --- | ---: | ---: | --- |
| Exact issued 32×32 RGBA picker | 4,196 | 4,096 | `d67c656e01756650d77717b0839985a056ec28ffe174601d690fc407a2ceffca` |
| PLTE and tRNS crossing separate fixed-page boundaries | 8,758 | 1,024 | `ff0dc7f2e9cff4b2c84f17f447a621ca39722f940a97afaa188416542c98aed7` |
| Adam7 RGBA with IEND framing across a page boundary | 4,105 | 1,748 | `29c13313407d527294c77db93161a622a16adca98d0cf09a2c1dff2550c9873c` |
| Two consecutive IDAT chunks with split length and IEND CRC | 8,194 | 1,748 | `29c13313407d527294c77db93161a622a16adca98d0cf09a2c1dff2550c9873c` |

The language-neutral bounds and controls are in `raw-png-handoff.schema.json`. Generate the base fixture and matching Pillow rasters with `original-media-guest-oracle.py` and `raw-png-handoff-oracle.py` from a repo Python environment with Pillow. The real caller fixture consumes the shared `browser-law.rs` produced by `📜️original-media-paged-receiver-check.py`. Run `raw-png-handoff-build.py .cache/b/r/m/u/l/p5d2dcc/prototype/build` through the repo tool, then run `raw-png-handoff-oracle.py --compare`. The candidate is staged inside this ticket. Registered PDF placement and the WASI route are separate integration gates.
