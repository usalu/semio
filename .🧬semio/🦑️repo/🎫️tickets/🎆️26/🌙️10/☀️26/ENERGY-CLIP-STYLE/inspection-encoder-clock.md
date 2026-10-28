# Mixed PNG Decoder Format Reinitialization

## Actual Failure And Root Cause

The strict current EB07 publication decodes 1304 frames instead of 1336 (9 focus + 1309 authored body + 18 restore). All raw/final/certificate identities were stable. Independent forensic alignment established 17 lost body frames and 15 lost restore frames. This is a real finite-clock contract failure.

The actual raw body has SHA256 61d41002d6cbfcf74bbebc13a4571d6fe278df3b97357f51297a2a6c6bc73fb3. Native FFmpeg showinfo independently confirms one invariant parameter tuple across all 1309 decoded frames: yuv420p,480×270,SAR0/1,unknown color range/space/primaries/transfer.

Repeating one RGB browser PNG around the exact body passes 1336 under both info/debug logging. It fails to exercise the actual intermediate screenshots. A fresh actual varied UI capture reproduces 1287 frames, and replaying those exact retained PNG bytes reproduces the same 1287.

The exact restore sequence has RGBA frames 0012/0014 with alpha 242–255 among RGB frames. Debug replay explicitly reconfigures the entire shared filter graph four times: rgba at line 1289, rgb24 at 1393, rgba at 1669, rgb24 at 1780. Every change is stream 2 (restore). Body/video buffers and the finite audio clock restart; repeated non-monotonic DTS resets corroborate the reset. SAR metadata differences alone did not reproduce it.

## Schema-First Native RED

The portable fixture and driver are presentation/🧪️tests/🖼️stream/🔣️.json and🐍️.py. A distinguishable 64-frame native body with actual Manim SAR0/1 plus mixed RGB/RGBA restore frames decodes only 55 instead of 91. The missing normalization/count-postcondition APIs are RED too (three cases, 15.145 seconds).

## Narrow Production Fix

Only presentation Python and collection Python change. A private Path→Path image adapter uses the already installed Pillow implementation used by existing browser adapters, without exposing external types or adding dependencies. It writes fresh RGB8 PNG copies with no inherited info into owned normalized directories, through sibling temporaries and atomic replacement. Original RGB channel values are retained; alpha is dropped without compositing, matting, masking, resizing or gamma conversion. Browser/canonical assets are untouched.

The exact native decoded frame count and rate are now checked before chapter candidate replacement and before returning an assembled lesson. The existing body-frame probe is factored through the same FFmpeg port; raw bodies/runtime.py/clocks/cursor timing remain unchanged.

## GREEN

The actual retained varied sequence now decodes all 1336 frames with unchanged raw input and no filter-graph reconfiguration. Debug retains only initial decoder setup. The fresh bilingual portable suite passes three tests in 11.809 seconds: both locales preserve 91 frames and every 64-body RGB hash; PNG RGB values/source bytes are unchanged; a healthy wrong-count MP4 is rejected.

The original failed candidate and exact input PNG/header/log SHA256 manifest are retained in generated/ie/repro-inputs.json. Full actual body/PNG independent verification is separately recorded by the verifier; no stale old output substitutes for it.

Production source hashes:
- presentation/🐍️.py:1304ba6990e8251a1352c202cee42e80ac0e12772395f89ef3d47d6c20c8c165
- presentation/📚️collection/🐍️.py:6242f2457ffbf5bfe6ab7d15ebbc7323d517fc18074e04dbebc9befe3e534695

## Process Handoff

Root stopped only the old publisher 36299 normally. Native handles confirm publisher PID 37860 and supervisor 36408 exited; doctor confirmed no grants/waiters or owned producer/native/browser/FFmpeg descendants, only read-only watchers. Root started the sole fresh publisher 16002 under 8 GiB/8 CPU, no force, reusing valid native/raw caches. No additional production owner edit is planned.


## Independent Actual GREEN

The verifier independently passed in 7.59 seconds: all 27 normalized PNGs are RGB8/no metadata and pixel-exact with original.convertRGB; both original and normalized hashes stayed stable. The fixed movie decodes 1336 contiguous 4-fps frames, with every one of 1309 body RGB frames exact at offset 9, and complete silent AAC audio of 334 seconds with no internal gaps. See inspection-encoder-normalization-proof.md.

The final three-case bilingual native suite passed in 11.809 seconds. Its driver is registered in the normal authoring test command. Final collection verification remains mandatory after the fresh sole publisher finishes.
