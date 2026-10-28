# Independent Encoder Clock Investigation

The historical first chapter publication was rejected: its raw body decoded4079 frames, while the chapter and full lesson each decoded4089 frames despite a4120-frame declared clock. Native FFmpeg, MP4 packet/header counts and PyAV independently agreed. The failed chapter/full SHA256 was `1c605964e1ca0473e9c051905d71b61bf3d559a11d708c07adddf95c47b5eddd`. No pass or retained final artifact was issued.

The canonical raw input contains no internal frame format, SAR or size changes: all4079 frames are480×270 yuv420p, SAR0/1, with unchanged unspecified color metadata. Debug FFmpeg reports only initial decoder context setup. A2411-landmark frame-hash alignment found a dominant raw-to-window offset of6; the first ten frames at that alignment differed, so the offset alone does not prove whether navigation or the body head was lost. The exact historical cause remains unproven.

Current isolated oracles pass:

- Five independently encoded PyAV body frames plus26/15 real PNG phase frames decode46 frames with all body pixels and clocks exact.
- The same test with real FFmpeg snapshot PNGs carrying pHYs(1,1) at focus-last/restore-first also passes46. Mixed pHYs alone does not reproduce the loss.
- One fresh process rendering actual26/15 UI transitions around the five-frame body passes46 with every body pixel exact. Its first attempt was correctly rejected when browser source code changed during capture; the stable retry passed.
- The exact4079-frame raw body with the retained actual26/15 UI sequences passes4120 frames/1030s in a fresh encoder process, with13.6s composition time. This reran only FFmpeg composition, with no UI or authored scene redraw.

No production encoder patch was justified or made. Loaded-source fidelity guards and strict decoded acceptance remain required; a metadata target count is insufficient. The new all55 watcher rejects historical source bindings before decoding current publications. All reproduction inputs and reports remain in the ticket; disposable native logs/media are under generated/iv.
