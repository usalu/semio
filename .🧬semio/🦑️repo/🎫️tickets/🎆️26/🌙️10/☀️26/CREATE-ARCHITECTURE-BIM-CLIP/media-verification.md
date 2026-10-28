# BIM Preview Media Verification

The final four-worker run completed **24 of 24** independent render jobs with no failures. Each of the six topics was rendered in English and German for both Speaker and Spectator, using the documented repository authoring interface. Preview settings were 640 × 360 at 1 frame/s; production quality options remain unchanged.

PyAV independently decoded every final movie completely. All files have the expected dimensions, one unique topic/language/audience output, a positive complete frame count and a valid audio stream. The shared presentation supplies the audio track; no prerecorded narration assets were authored in this task. The English and German narration texts are complete.

| Topic | English Speaker / Spectator Frames | German Speaker / Spectator Frames |
| --- | ---: | ---: |
| Data and Information | 67 / 67 | 64 / 64 |
| Object Orientation | 87 / 87 | 85 / 85 |
| Object Representations | 74 / 74 | 67 / 67 |
| Partial Models and Translation | 88 / 88 | 83 / 83 |
| Interfaces and Translation | 98 / 98 | 90 / 90 |
| Consistency across Models | 87 / 87 | 81 / 81 |

A total of 1,942 frames were decoded. Within each language, the Speaker and Spectator variants have identical frame counts for every topic. Cross-language duration differences follow their independently authored narration clocks.

All generated paths were scanned: maximum absolute length **172**, below the 256-character limit. The source inventory maximum is 123.

Independent visual review covered all six canonical scenes and representative fully composed movies. The actual hierarchy shows the BIM topic and six individual localized video titles; content and captions fit the window. Pillow found the first composed movie's first and last decoded images pixel-identical. Encoded language/caption policy comparisons are documented separately in media-audience.md.

The final proof uses current source certificates; earlier interrupted or concurrently changed inputs were never certified as completed media. Generated previews and logs are temporary verification outputs and are removed at ticket closure according to repository policy. Source files, test cases, input scripts and Markdown audits remain.
